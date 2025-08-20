use crate::app::App;
use std::io::{self, Error, ErrorKind};

pub mod app;
pub mod event;
pub mod sentences;
pub mod ui;
pub mod my_logger;

#[tokio::main]
async fn main() -> io::Result<()> {
    color_eyre::install().map_err(|e| Error::new(ErrorKind::Other, e))?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
