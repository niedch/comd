use color_eyre::Result;
use ratatui::TerminalOptions;
use tokio::sync::mpsc::{self};

use crate::{
    actions::{StartRequest, StreamResult},
    app::App,
    worker::spawn_worker,
};

mod actions;
mod app;
mod worker;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let mut inline_terminal = ratatui::init_with_options(TerminalOptions {
        viewport: ratatui::Viewport::Inline(6),
    });

    let (tx_start, rx_start) = mpsc::unbounded_channel::<StartRequest>();
    let (tx_result, rx_result) = mpsc::unbounded_channel::<StreamResult>();

    spawn_worker(tx_result, rx_start);

    let result = App::new(tx_start, rx_result)
        .run(&mut inline_terminal)
        .await;

    ratatui::restore();

    return match result {
        None => Ok(()),
        Some(out) => {
            println!("\n\nOutput {:?}", out);
            Ok(())
        }
    };
}
