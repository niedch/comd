use color_eyre::Result;
use ratatui::TerminalOptions;
use std::fs;
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
    color_eyre::install()?;
    let mut inline_terminal = ratatui::init_with_options(TerminalOptions {
        viewport: ratatui::Viewport::Inline(6),
    });

    let config = config::load_config()?;

    let (tx_start, rx_start) = mpsc::unbounded_channel::<StartRequest>();
    let (tx_result, rx_result) = mpsc::unbounded_channel::<StreamResult>();

    spawn_worker(config, tx_result, rx_start);

    let result = App::new(tx_start, rx_result)
        .run(&mut inline_terminal)
        .await;

    ratatui::restore();

    let zsh_buf = std::env::var_os("COMD_ZSH_BUFFER_FILE");

    match &result {
        None => {}
        Some(out) if zsh_buf.is_none() => {
            println!("\n\n{out:?}");
        }
        Some(_out) => {}
    }

    if let Some(path) = zsh_buf {
        match &result {
            None => {
                let _ = fs::write(path, "");
            }
            Some(s) => {
                let _ = fs::write(path, s);
            }
        }
    }

    Ok(())
}
