// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use named_lock::{Error, NamedLock};
use std::env;

mod config;
mod shell;

#[cfg(windows)]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[macro_export]
macro_rules! dprintln {
    ($($arg:tt)*) => (#[cfg(debug_assertions)] println!("[D] {}", format!($($arg)*)));
}

pub trait CustomBuild {
    fn setup_platform(self) -> Self;
}

pub fn build_window<R: tauri::Runtime>(app: &tauri::App<R>) -> tauri::Window<R> {
    match tauri::WindowBuilder::new(app, "main", tauri::WindowUrl::default())
        .title("Pengu Loader")
        .inner_size(940.0, 560.0)
        .disable_file_drop_handler()
        .resizable(false)
        .maximizable(false)
        .center()
        .focused(true)
    {
        builder => {
            #[cfg(windows)]
            {
                builder.decorations(false)
            }
            #[cfg(target_os = "macos")]
            {
                builder.title_bar_style(tauri::TitleBarStyle::Overlay)
            }
        }
    }
    .build()
    .expect("error while building main window")
}

fn main() -> Result<(), Error> {
    #[cfg(windows)]
    windows::do_entry();

    let lock = NamedLock::create("989d2110-46da-4c8d-84c1-c4a42e43c424")?;
    let _guard = lock.try_lock()?;

    tauri::Builder::default()
        .setup_platform()
        .plugin(config::init())
        .plugin(shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
