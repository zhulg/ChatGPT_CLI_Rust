use clap::{Arg, ArgAction, Command};
use colored::*;
use console::Style;
use dialoguer::{console, Input};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{self};
use rustyline::error::ReadlineError;
use serde_json::{json, Value};
use std::env;
use std::time::Duration;
use text2art::{BasicFonts, Font, Printer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("ChatGPT CLI")
        .author("lg.json@gmail.com")
        .version("1.0.0")
        .about(
            "x\n
                    ChatGPT CLI Create by zhulg (lg.json@gmail.com)
            | 1.You just need to input your api key, the cli version V0.1.1     |
            | 2.No need access internet with VPN, and just enjoy it.            |
            | 3.If you want to use it in China, you can use my api key.         |                                                   |
            |-------------------------------------------------------------------|",
        )
        .arg(
            Arg::new("DomainName")
                .action(ArgAction::Set)
                .short('d')
                .long("Domain")
                .default_value("api.openai.com")
                .help("Sets the API Domain name."),
        )
        .arg(
            Arg::new("APIKey")
                .action(ArgAction::Set)
                .short('k').
                long("key")
                .default_value("")
            .help("Sets the API key. If not provided, the cli will ask for it,\n\
             You can also set the OPENAI_API_KEY environment variable."),
        )
        .arg(
            Arg::new("model")
                .action(ArgAction::Set)
                .short('m')
                .long("model")
                .default_value("gpt-3.5-turbo")
                .help("Sets the GPT model to use. gpt-3.5-turbo or gpt-3.5-turbo-0301"),
        )
        .arg(
            Arg::new("temperature")
                .action(ArgAction::Set)
                .short('t')
                .long("temperature")
                .default_value("0.5")
                .help("Sets the temperature for text generation."),
        )
        .arg(
            Arg::new("max_tokens")
                .action(ArgAction::Set)
                .short('l')
                .long("length")
                .default_value("1000")
                .help("sets the max_tokens, default is 1000"),
        )
        .after_help(
            "Longer explanation to appear after the options when \
                  displaying the help information from --help or -h",
        )
        .get_matches();

    let domain_name = matches.get_one::<String>("DomainName").unwrap();
    let api_key_cli = matches.get_one::<String>("APIKey").unwrap();
    let max_tokens = matches.get_one::<String>("max_tokens").unwrap();
    let model = matches.get_one::<String>("model").unwrap();
    let temperature = matches.get_one::<String>("temperature").unwrap();
    show_logo();
    let mut api_key = String::new();
    if api_key_cli.is_empty() {
        api_key = read_api_key();
    }
    let url = format!("https://{}/v1/chat/completions", domain_name);
    let mut rl = rustyline::DefaultEditor::new()?;

    loop {
        let readline = rl.readline("enter your message:");
        match readline {
            Ok(line) => {
                if line == "exit" {
                    break;
                }
                if line.is_empty() {
                    continue;
                }
                requestgpt(&url, &api_key, &line, &max_tokens,model,temperature).await?;
            }
            Err(ReadlineError::Interrupted) => {
                println!("Control+C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Control+D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

async fn requestgpt(
    url: &str,
    api_key: &String,
    line: &String,
    max_tokens: &String,
    model: &String,
    temperature: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let pb = show_progressbar();
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": model,
            "max_tokens": max_tokens.parse::<i32>().unwrap(),
            "temperature": temperature.parse::<f32>().unwrap(),
            "messages": [{"role": "user", "content": line}]
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;
    // dbg!(response);
    if response["choices"].is_null() {
        println!(
            "{}",
            "ChatGPT: Something wrong with your api key or network errors, please check it.".red()
        );
        pb.finish_and_clear();
        return Ok(());
    }
    pb.finish_and_clear();
    let response_content: String = response["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_default()
        .to_owned();
    println!("{}", format!("ChatGPT:{}", response_content).green());
    Ok(())
}

fn show_progressbar() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(125));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} {msg}")
            .unwrap()
            .tick_strings(&["∙∙∙", "●∙∙", "∙●∙", "∙∙●", "∙∙∙"]),
    );
    pb.set_message("waiting for response...");
    pb
}

fn show_logo() {
    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("something wrong with font"),
    };
    let prntr = Printer::with_font(font);
    prntr
        .print_to_stdio(
            "
                ChatGPT CLI     ",
        )
        .ok();
}

fn read_api_key() -> String {
    // If the OPENAI_API_KEY environment variable is not set,
    // ask the user to input the API key and save it to the
    // environment variables for future use.
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        console::set_colors_enabled(true);
        let prompt_style = Style::new().yellow();
        let api_key: String = Input::new()
            .with_prompt(prompt_style.apply_to("Input your API key").to_string())
            .interact_text()
            .unwrap();
        env::set_var("OPENAI_API_KEY", &api_key);
        api_key
    });
    api_key
}
