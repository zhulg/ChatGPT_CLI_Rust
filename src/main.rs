mod gptcli_net;
mod gptcli_utils;
use clap::{Arg, ArgAction, Command};
use colored::*;
use gptcli_net::{send_gpt_request, GptRequestParams};
use gptcli_utils::{read_api_key, show_logo, show_progressbar};
use rustyline::error::ReadlineError;

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
            Arg::new("prompt")
                .action(ArgAction::Set)
                .short('p')
                .long("prompt")
                .default_value("")
                .help("Sets the prompt for this session."),
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

    // show_logo();
    let domain_name = matches.get_one::<String>("DomainName").unwrap();
    let max_tokens = matches.get_one::<String>("max_tokens").unwrap();
    let model = matches.get_one::<String>("model").unwrap();
    let temperature = matches.get_one::<String>("temperature").unwrap();
    let api_key_cli = matches.get_one::<String>("APIKey").unwrap();
    let url = format!("https://{}/v1/chat/completions", domain_name);
    let mut api_key = String::new();
    if api_key_cli.is_empty() {
        api_key = read_api_key();
    }

    let prompt = matches.get_one::<String>("prompt").unwrap();
    if !prompt.is_empty() {
        println!("{}{}", "your prompt is:".on_blue(), prompt.on_blue());
        request_gpt(GptRequestParams {
            url: &url,
            api_key: &api_key,
            line: &prompt,
            max_tokens: max_tokens.parse().unwrap(),
            model: &model,
            temperature: temperature.parse().unwrap(),
        })
        .await?;
    }

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
                request_gpt(GptRequestParams {
                    url: &url,
                    api_key: &api_key,
                    line: &line,
                    max_tokens: max_tokens.parse().unwrap(),
                    model: &model,
                    temperature: temperature.parse().unwrap(),
                })
                .await?;
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

async fn request_gpt(params: GptRequestParams<'_>) -> Result<(), Box<dyn std::error::Error>> {
    let pb = show_progressbar();
    let response_content = send_gpt_request(params).await?;
    pb.finish_and_clear();
    println!("{}", format!("ChatGPT:{}", response_content).green());
    Ok(())
}
