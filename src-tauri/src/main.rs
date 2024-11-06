// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command as StdCommand;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State, WindowEvent};
use tauri_plugin_shell::process::{CommandChild, CommandEvent};

use tauri_plugin_shell::ShellExt;

struct APIManagerState {
    child: Arc<Mutex<Option<CommandChild>>>,
}

#[tauri::command]
async fn start_server(state: Arc<State<'_, APIManagerState>>, app_handle: AppHandle) -> Result<String, String> {
    let mut child_lock = state
        .child
        .lock()
        .map_err(|e| format!("Failed to lock mutex: {}", e))?;

    if child_lock.is_some() {
        println!("API server is already running");
        return Ok("API server is already running".into());
    }

    println!("Attempting to start API server...");

    let (mut rx, child) = app_handle.shell().sidecar("api")
        .expect("failed to create `api` binary command")
        .spawn()
        .map_err(|e| format!("Failed to spawn API server: {}", e))?;

    println!("API server process spawned successfully");

    tauri::async_runtime::spawn(async move {
        println!("Starting to listen for API server events");
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => print!("API: {}", String::from_utf8_lossy(line.as_ref())),
                CommandEvent::Stderr(line) => eprint!("API Info: {}", String::from_utf8_lossy(line.as_ref())),
                CommandEvent::Error(error) => eprint!("API Process Error: {}", error),
                CommandEvent::Terminated(status) => {
                    println!("API Process Terminated with status: {:?}", status)
                }
                _ => {}
            }
        }
        println!("Stopped listening for API server events");
    });

    *child_lock = Some(child);
    println!("API server started successfully");
    Ok("API server started successfully".into())
}


#[tauri::command]
async fn stop_server(state: State<'_, APIManagerState>) -> Result<String, String> {
    let mut child_lock = state
        .child
        .lock()
        .map_err(|e| format!("Failed to lock mutex: {}", e))?;

    if let Some(child) = child_lock.take() {
        println!("Attempting to stop API server");

        // Get the process ID
        let pid = child.pid();

        // On Unix-like systems (macOS, Linux)
        #[cfg(unix)]
        {
            // Use pkill to terminate all child processes
            if let Err(e) = StdCommand::new("pkill")
                .args(&["-P", &pid.to_string()])
                .output()
            {
                eprintln!("Failed to terminate child processes: {}", e);
            }
        }

        // On Windows
        #[cfg(windows)]
        {
            // Use taskkill to terminate the process tree
            if let Err(e) = StdCommand::new("taskkill")
                .args(&["/F", "/T", "/PID", &pid.to_string()])
                .output()
            {
                eprintln!("Failed to terminate process tree: {}", e);
            }
        }

        // Now kill the main process
        match child.kill() {
            Ok(_) => {
                println!("API server stopped successfully");
                Ok("API server stopped successfully".into())
            }
            Err(e) => {
                eprintln!("Failed to stop API server: {}", e);
                Err(format!("Failed to stop API server: {}", e))
            }
        }
    } else {
        println!("API server is not running");
        Ok("API server is not running".into())
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .manage(APIManagerState {
            child: Arc::new(Mutex::new(None))
        })
        .setup(move |app| {
            //let app_handle = app.handle();
            //let state = Arc::new(app_handle.state::<APIManagerState>());
            let app_handle = app.app_handle().clone();
            tauri::async_runtime::spawn(async move {
                let state = Arc::new(app_handle.state::<APIManagerState>());    
                match start_server(state, app_handle.clone()).await {
                    Ok(msg) => println!("{}", msg),
                    Err(e) => eprintln!("Failed to start API server: {}", e),
                }
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::Destroyed = event {
                let state = window.state::<APIManagerState>();
                tauri::async_runtime::block_on(async {
                    match stop_server(state).await {
                        Ok(msg) => println!("{}", msg),
                        Err(e) => eprintln!("Error stopping API server: {}", e),
                    }
                });
            }
        })
        .invoke_handler(tauri::generate_handler![greet, stop_server])
        .plugin(tauri_plugin_shell::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
