// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod errors;
mod model;

use epub::doc::EpubDoc;
use errors::Error;
use model::Epub;
use std::{
    fs::{create_dir, File, self},
    io::{Write, Cursor},
    path::Path,
};
use tauri::AppHandle;
use uuid::Uuid;

#[tauri::command]
fn get_epub_data(epub: String, app_handle: AppHandle) -> Result<Epub, Error> {
    let epub_path = Path::new(&epub);
    let mut doc = EpubDoc::new(epub_path)?;

    match app_handle.path_resolver().app_cache_dir() {
        Some(dir) => {
            if !dir.exists() {
                create_dir(&dir)?;
            }

            let epub_name = epub_path.file_stem().ok_or(Error::FailedToGetEpubName)?;
            let (cover_data, mimetype) = doc.get_cover().ok_or(Error::FailedToGetEpubCover)?;

            let extension = match mimetype.as_str() {
                "image/jpeg" => "jpg",
                "image/png" => "png",
                _ => "",
            };

            let cover_path = dir.join(epub_name).with_extension(extension);
            let mut cover_file = File::create(&cover_path)?;
            cover_file.write_all(&cover_data)?;

            Ok(Epub {
                number_of_pages: doc.get_num_pages(),
                cover_path: cover_path
                    .to_str()
                    .ok_or(Error::FailedToResolveEpubCoverPath)?
                    .to_string(),
                title: doc.mdata("title").ok_or(Error::FailedToGetEpubTitle)?,
                author: doc.mdata("creator").ok_or(Error::FailedToGetEpubCreator)?,
            })
        }
        None => Err(Error::FailedToResolveAppCacheDir),
    }
}

#[tauri::command]
fn generate_uuid() -> String {
    let id = Uuid::new_v4();
    return id.to_string();
}

#[tauri::command]
fn unzip(uuid: String, app_handle: AppHandle) -> Result<(), Error> {
    match app_handle.path_resolver().app_data_dir() {
        Some(dir) => {
            let epub_path = dir.join("books").join(uuid);
            let epub_data = fs::read(epub_path.with_extension("epub"))?;
            zip_extract::extract(Cursor::new(epub_data), epub_path.as_path(), true)?; 
            Ok(())
        },
        None => Err(Error::FailedToResolveAppDataDir),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_epub_data, generate_uuid, unzip])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
