# ChatGPT CLI


- A tool for chatting using the ChatGPT API, written in Rust CLI.
- You can use this tool to chat, just by setting your API Key.
- You can modify the API domain and other API parameters when you start the chat.
- If you like this tool, please join me to complete the TODO list, and let's improve this tool together


## QuickStart 
- **Download** [GitHub Release](https://github.com/zhulg/ChatGPT_CLI_Rust/releases)

## Usage

- Linux/MacOS Run the following command in your terminal:

  ```
  chatgpt_rust
  ```
![](./img/chatgpt_cli.png)


## cargo Install

- **If you have Rust installed, you can install the CLI using cargo:**

```
cargo install chatgpt_rust
```


## Install from source code

  1. build code 
```
cargo build
```
  2. cd target/debug 
    
```
   ./chatgpt_rust
```



## User Guide


- chatgpt_rust --help 

```

                    ChatGPT CLI Create by zhulg (lg.json@gmail.com)
            | 1.You just need to input your api key, the cli version    |
            | 2.No need access internet with VPN, and just enjoy it.    |
            | 3.If you want to use it in China, you can use my api key. |
            | --------------------------------------------------------- |

Usage: chatgpt_rust [OPTIONS]

Options:
  -d, --Domain <DomainName>  Sets the API Domain name. [default: api.openai.com]
  -k, --key <APIKey>         Sets the API key. If not provided, the cli will ask for it,
                             You can also set the OPENAI_API_KEY environment variable. [default: ]
  -t, --tokens <max_tokens>  sets the max_tokens, default is 1000 [default: 1000]
  -h, --help                 Print help
  -V, --version              Print version
 

```

## Option:
 Set your ‘OPENAI_API_KEY’ Environment Variable using zsh,  No set will ask the user to input the API key in the terminal.

1. Run the following command in your terminal, replacing yourkey with your API key. 

```
echo "export OPENAI_API_KEY='yourkey'" >> ~/.zshrc

```
2. Update the shell with the new variable:

```
source ~/.zshrc
```
3. Confirm that you have set your environment variable using the following command. 
   
```
echo $OPENAI_API_KEY
```
The value of your API key will be the resulting output.

## TODO:
- [ ] Add more API parameters support
- [ ] Support save message to file
- [ ] import chat message from file
- [ ] Support read prompt from file