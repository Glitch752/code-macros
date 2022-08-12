#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
use tauri::Manager;

fn main() {
  println!("Main ran");

  tauri::Builder::default()
    .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
      println!("{}, {argv:?}, {cwd}", app.package_info().name);
      let window = app.get_window("main").unwrap();
      window.show();
    }))
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .system_tray(
      SystemTray::new().with_menu(
        SystemTrayMenu::new()
          .add_item(
            CustomMenuItem::new("open".to_string(), "Open")
          )
          .add_native_item(SystemTrayMenuItem::Separator)
          .add_item(
            CustomMenuItem::new("quit".to_string(), "Quit")
          )
      )
    )
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
        // let item_handle = app.tray_handle().get_item(&id);
        match id.as_str() {
          "quit" => {
            let window = app.get_window("main").unwrap();
            window.close().unwrap();
          }
          "open" => {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
          }
          _ => {}
        }
      }
      _ => {}
    })
    .setup(|_| {
      initialize_macro_listeners();
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn my_custom_command() {
  println!("I was invoked from JS!");
}

fn initialize_macro_listeners() {
  println!("Todo: Initialize macro listeners");
}