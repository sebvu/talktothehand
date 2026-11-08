// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// audio shit
use tts::Tts;

#[tauri::command]
fn speak_local_tts(text: String, channel_id: String) -> Result<(), String> {
    if text.trim().is_empty() {
        return Err("Text cannot be empty".to_string());
    }

    // 1. Initialize your system's native offline TTS engine 
    // (Uses SAPI on Windows, AVFoundation on Mac, SpeechDispatcher on Linux)
    let mut tts = Tts::default().map_err(|e| e.to_string())?;

    // 2. Speak the text out loud natively
    tts.speak(text, true).map_err(|e| e.to_string())?;

    Ok(())
}

use std::process::Command;
use std::io::Write;
use rodio::{Decoder, OutputStream, Sink};
use std::io::Cursor;

#[tauri::command]
fn play_audio_to_device(audio_bytes: Vec<u8>, device_name_filter: String) -> Result<(), String> {
    // ==========================================
    // LINUX PIPELINE (Native System Player)
    // ==========================================
    if cfg!(target_os = "linux") {
        // 1. Save the raw audio bytes to a temporary file
        let mut tmp_file = std::env::temp_dir();
        tmp_file.push("tauri_tts_speech.wav"); 
        
        let mut file = std::fs::File::create(&tmp_file).map_err(|e| e.to_string())?;
        file.write_all(&audio_bytes).map_err(|e| e.to_string())?;

        let file_path = tmp_file.to_str().ok_or("Invalid temp file path")?;

        // 2. Execute mpv, telling it exactly which audio output device channel to hit
        // Note: --audio-device expects "pulse/device_name" or "pipewire/device_name"
        let status = Command::new("mpv")
            .arg(format!("--audio-device=pulse/{}", device_name_filter)) 
            .arg("--no-video")
            .arg(file_path)
            .status()
            .or_else(|_| {
                // Fallback to default output channel if device isn't found or fails
                Command::new("mpv")
                    .arg("--no-video")
                    .arg(file_path)
                    .status()
            });

        // 3. Ultimate Fallback (if mpv isn't installed, try VLC's headless streamer)
        if status.is_err() || !status.unwrap().success() {
            let _ = Command::new("cvlc")
                .arg("--play-and-exit")
                .arg(file_path)
                .status();
        }

        // Clean up the temporary file after playback completes
        let _ = std::fs::remove_file(tmp_file);
        return Ok(());
    }

    // ==========================================
    // WINDOWS/MAC PIPELINE (Standard Rodio)
    // ==========================================
    use rodio::cpal::traits::{HostTrait, DeviceTrait};
    let host = rodio::cpal::default_host();
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

    let rodio_device: rodio::Device = device.into();
    let (_stream, stream_handle) = OutputStream::try_from_device(&rodio_device).map_err(|e| e.to_string())?;
    let sink = Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;

    let cursor = Cursor::new(audio_bytes);
    let source = Decoder::new(cursor).map_err(|e| e.to_string())?;
    
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if cfg!(target_os = "linux") {
        unsafe {
            std::env::set_var("CPAL_AUDIO_API", "pulse");
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, play_audio_to_device, speak_local_tts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}