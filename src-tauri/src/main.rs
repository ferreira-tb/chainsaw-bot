// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![feature(try_blocks)]

mod command;
pub mod database;
mod error;
mod prelude;

use error::BoxResult;
use sea_orm::DatabaseConnection;
use tauri::{App, Manager};

#[cfg(debug_assertions)]
use std::sync::OnceLock;

pub struct AppState {
  pub db: DatabaseConnection,
}

fn main() {
  #[cfg(debug_assertions)]
  let worker = OnceLock::new();

  #[cfg(debug_assertions)]
  {
    use tracing_appender::rolling;
    use tracing_subscriber::fmt::time::ChronoLocal;
    use tracing_subscriber::fmt::writer::MakeWriterExt;
    use tracing_subscriber::fmt::Layer;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::{EnvFilter, Registry};

    const TIMESTAMP: &str = "%F %T%.3f %:z";

    let filter = EnvFilter::builder()
      .from_env()
      .unwrap()
      .add_directive("kotori=trace".parse().unwrap())
      .add_directive("tauri_plugin_manatsu=trace".parse().unwrap());

    let appender = rolling::daily("../.temp", "log");
    let (writer, guard) = tracing_appender::non_blocking(appender);
    worker.set(guard).unwrap();

    let file = Layer::default()
      .with_ansi(false)
      .with_timer(ChronoLocal::new(TIMESTAMP.into()))
      .with_writer(writer.with_max_level(tracing::Level::TRACE))
      .pretty();

    let stderr = Layer::default()
      .with_ansi(true)
      .with_timer(ChronoLocal::new(TIMESTAMP.into()))
      .with_writer(std::io::stderr)
      .pretty();

    let subscriber = Registry::default()
      .with(file)
      .with(stderr)
      .with(filter);

    tracing::subscriber::set_global_default(subscriber).unwrap();
  }

  tauri::Builder::default()
    .plugin(tauri_plugin_http::init())
    .plugin(tauri_plugin_manatsu::init())
    .setup(setup)
    .invoke_handler(tauri::generate_handler![
      command::close_window,
      command::show_window
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn setup(app: &mut App) -> BoxResult<()> {
  let handle = app.handle();
  let state = AppState {
    db: database::connect(handle)?,
  };

  app.manage(state);

  Ok(())
}
