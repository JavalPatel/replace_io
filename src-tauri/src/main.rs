// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]
use rdev::listen;
mod callback;
mod read_write;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// use tauri::{Manager};
#[allow(dead_code)]
#[tauri::command]
fn get_all_pattern() -> Vec<read_write::PatternStruct> {
    println!("reading all the structs");
    // app_handle.emit_all("Backend", Payload { message: "hello there!".into() }).unwrap();
    let temp = read_write::read_struct("./asd.json".to_string());
    println!("Done reading!!");
    temp
}

#[allow(dead_code)]
#[tauri::command]
fn add_pattern(asd: read_write::PatternStruct) {
    println!("Adding a pattern {:?}", asd);
    read_write::append_struct(asd, "./asd.json".to_string());
    // app_handle.emit_all("Backend", Payload { message: "hello there!".into() }).unwrap();
    // let temp = read_write::read_struct("./asd.json".to_string());
    // tempnm
}

#[allow(dead_code)]
#[tauri::command]
fn delete_pattern(delete_id: i32) {
    println!("Deleting a pattern of id {:?}", delete_id);
    read_write::remove_struct(delete_id, "./asd.json".to_string());
    // read_write::append_struct(asd,"./asd.json".to_string());
    // app_handle.emit_all("Backend", Payload { message: "hello there!".into() }).unwrap();
    // let temp = read_write::read_struct("./asd.json".to_string());
    // tempnm
}

#[tauri::command]
fn start_pattern_matching() {
    println!("Pattern matching started!");
    if let Err(error) = listen(callback::callback) {
        println!("Error: {:?}", error)
    }
}
// use tauri::async_runtime::Runtime;
// use tokio;
use std::thread;

#[warn(unused_imports)]
use tauri::{
    CustomMenuItem, Manager,
    SystemTray, SystemTrayEvent, SystemTrayMenu, WindowBuilder, WindowUrl
};
fn main() {
    thread::spawn(|| {
        start_pattern_matching();
    });
    let quit = CustomMenuItem::new("exit_app".to_string(), "Quit");
    // let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
    .add_item(quit);
    // .add_native_item(SystemTrayMenuItem::Separator)
    // .add_item(hide);
    let tray = SystemTray::new().with_menu(tray_menu);
    // let mut rt = tokio::runtime::Runtime::new().unwrap();
    let app = tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                // println!("Here {:?}",app.get_window("main"));
                // let window = app.get_window("main").unwrap();
                // let window = app_handle.get_window("main").unwrap();
                    // window.set_title("New title!").unwrap();
                // println!()
                let window = app.get_window("main");
                if window == None{
                    let _window = WindowBuilder::new(
                        app,
                        "new",
                        WindowUrl::App("index.html".into()),
                    ).build();
                }else{
                    let _a = window.unwrap();
                    _a.show().unwrap();
                    _a.set_focus().unwrap();
                    println!("hello")
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
                // let item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "exit_app" => {
                        // exit the app
                        app.exit(0);
                    },
                    _ =>{
                        println!("Something is wrong!");
                    }
                }
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("failed to run app");
        
    app.run(|_app_handle, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            println!("here at exit");
            api.prevent_exit();
        }
        _ => {}
      });
}
