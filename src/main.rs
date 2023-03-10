use clap::{Arg, ArgAction, Command};
use colored::*;
use console::Style;
use dialoguer::{console, Input};
use reqwest;
use rustyline::error::ReadlineError;
use serde_json::{json, Value};
use std::env;
use text2art::{BasicFonts, Font, Printer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Decompile APK")
        .author("lg.json@gmail.com")
        .version("1.0.0")
        .about(
            "x\n
                    ChatGPT CLI Create by zhulg (lg.json@gmail.com)                    
            | 1.you just need to input your api key, the cli version V1.0.0    |
            | 2.no need access internet with VPN  and enjoy it.                |
            | 3.if you want to use it in China,you can use my api key.         |                                                   |
            |------------------------------------------------------------------|",
        )
        .after_help(
            "Longer explanation to appear after the options when \
                  displaying the help information from --help or -h",
        )
        .get_matches();

    let api_key = read_api_key();
    show_logo();
    // let url = "https://api.openai.com/v1/completions";
    let url = "https://openapi.ssiic.com/v1/completions";
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
                requestgpt(url, &api_key, &line).await?;
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
) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "text-davinci-001",
            // "prompt": "hello chatgpt",
            "prompt": format!("{}", line),
            "max_tokens":200,
            "temperature": 0.5
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;
    println!("{:?}", response["choices"][0]["text"].clone());
    let text1: String = response["choices"][0]["text"]
        .as_str()
        .unwrap_or_default()
        .to_owned();
    println!("{}", format!("ChatGPT: {}", text1).green());
    Ok(())
}

fn show_logo() {
    let font = match Font::from_basic(BasicFonts::Big) {
        Ok(font) => font,
        Err(_) => panic!("something wrong with font"),
    };
    let prntr = Printer::with_font(font);
    prntr.print_to_stdio("ChatGPT By Rust ").ok();
}

fn read_api_key() -> String {
    // If the OPENAI_API_KEY environment variable is not set,
    // ask the user to input the API key and save it to the
    // environment variables for future use.
    let api_key = env::var("OPENAI_API_KEY2").unwrap_or_else(|_| {
        console::set_colors_enabled(true);
        let prompt_style = Style::new().yellow();
        let api_key: String = Input::new()
            .with_prompt(prompt_style.apply_to("请输入你的API KEY").to_string())
            .interact_text()
            .unwrap();
        env::set_var("OPENAI_API_KEY", &api_key);
        api_key
    });
    api_key
}
