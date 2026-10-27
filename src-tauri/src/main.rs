// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_shell::ShellExt;

#[tauri::command]
async fn transrivi_audio(app: tauri::AppHandle, file_path: String) -> Result<String, String> {
    // Ottiene il percorso della cartella delle risorse dell'app dove si trova il modello ggml-small.bin
    let resource_path = app.path().resource_dir()
        .map_err(|e| format!("Impossibile trovare la cartella risorse: {}", e))?;
    
    let model_path = resource_path.join("models/ggml-small.bin");
    let model_path_str = model_path.to_str()
        .ok_or("Percorso del modello non valido")?;

    // Invocazione del sidecar "whisper-cli" configurato in Tauri v2
    let output = app.shell()
        .sidecar("whisper-cli")
        .map_err(|e| format!("Errore nell'inizializzazione del sidecar whisper-cli: {}", e))?
        .args([
            "-m",
            model_path_str,
            "-f",
            &file_path,
            "-ovtt", // Genera automaticamente il file .vtt dei sottotitoli
            "-l",
            "auto"
        ])
        .output()
        .await
        .map_err(|e| format!("Errore durante l'esecuzione del processo di trascrizione: {}", e))?;

    if output.status.success() {
        let transcript = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(transcript)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Whisper ha restituito un errore: {}", error_message))
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![transrivi_audio])
        .run(tauri::generate_context!()) // <-- NOTA IL PUNTO ESCLAMATIVO AGGIUNTO QUI
        .expect("Errore durante l'avvio dell'applicazione Tauri");
}