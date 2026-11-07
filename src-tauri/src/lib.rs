// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// audio shit
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;
use cpal::traits::DeviceTrait;

#[tauri::command]
fn play_audio_to_device(audio_bytes: Vec<u8>, device_name_filter: String) -> Result<(), String> {
    use cpal::traits::HostTrait;

    let host = cpal::default_host();
    let devices = host.output_devices().map_err(|e| e.to_string())?;

    let mut target_device = None;
    for device in devices {
        if let Ok(name) = device.name() {
            if name.contains(&device_name_filter) {
                target_device = Some(device);
                break;
            }
        }
    }

    let device = match target_device {
        Some(d) => d,
        None => host.default_output_device().ok_or("No audio output device found")?,
    };

    let (_stream, stream_handle) = OutputStream::try_from_device(&device).map_err(|e| e.to_string())?;
    let sink = Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;

    let cursor = Cursor::new(audio_bytes);
    let source = Decoder::new(cursor).map_err(|e| e.to_string())?;
    
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, play_audio_to_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}