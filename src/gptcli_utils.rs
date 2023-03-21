use console::Style;
use dialoguer::{console, Input};
use indicatif::{ProgressBar, ProgressStyle};
use std::{env, time::Duration};
use text2art::{BasicFonts, Font, Printer};

pub fn show_progressbar() -> ProgressBar {
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

pub fn show_logo() {
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

pub fn read_api_key() -> String {
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
