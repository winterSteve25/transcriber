// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use tauri::{CustomMenuItem, Menu, MenuItem};
use tauri::api::dialog;

fn main() {
    let open_file = CustomMenuItem::new("open_file", "Open File");

    let menu = Menu::new()
        .add_item(open_file)
        .add_native_item(MenuItem::Quit);

    tauri::Builder::default()
        // .menu(menu)
        // .on_menu_event(move |event| {
            // match event.menu_item_id() {
            //     "open_file" => {
            //         dialog::FileDialogBuilder::default()
            //             .add_filter("Book", &["pdf", "epub"])
            //             .pick_file(|path_buf| match path_buf {
            //                 Some(path) => {}
            //                 None => {}
            //             });
            //     }
            //     _ => {}
            // }
        // })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
