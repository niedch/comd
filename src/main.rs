use color_eyre::Result;
use futures::StreamExt;
use ratatui::TerminalOptions;
use rig::{agent::{MultiTurnStreamItem, Text}, client::{CompletionClient, ProviderClient}, completion::Prompt, providers::gemini, streaming::{StreamedAssistantContent, StreamingPrompt}};
use tokio::sync::mpsc::{self};

use crate::{
    actions::{StartRequest, StreamResult},
    app::App,
    worker::spawn_worker,
};

mod actions;
mod app;
mod config;
mod worker;

#[tokio::main]
async fn main() -> Result<()> {
    // color_eyre::install()?;
    // let mut inline_terminal = ratatui::init_with_options(TerminalOptions {
    //     viewport: ratatui::Viewport::Inline(6),
    // });

    let config = config::load_config()?;
    
    let client = gemini::Client::new(&config.global.gemini_api_key)?;
    let agent = client.agent("gemini-2.5-flash").preamble("You are a comedian").build();
    let mut response_stream = agent.stream_prompt("Write me a joke").await;

    while let Some(item) = response_stream.next().await {
        match item {
            Ok(MultiTurnStreamItem::StreamAssistantItem(
                StreamedAssistantContent::Text(result),
            )) => {
                print!("|{result}|");
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error in stream: {}", e);
                break;
            }
        }
    }

    
    Ok(())

    // let (tx_start, rx_start) = mpsc::unbounded_channel::<StartRequest>();
    // let (tx_result, rx_result) = mpsc::unbounded_channel::<StreamResult>();
    //
    // spawn_worker(&config, tx_result, rx_start);
    //
    // let result = App::new(tx_start, rx_result)
    //     .run(&mut inline_terminal)
    //     .await;
    //
    // ratatui::restore();
    //
    // return match result {
    //     None => Ok(()),
    //     Some(out) => {
    //         println!("\n\nOutput {:?}", out);
    //         Ok(())
    //     }
    // };
}
