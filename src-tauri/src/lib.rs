use tauri_plugin_shell::ShellExt;
use tauri::Manager; // Importante per usare .path()

#[tauri::command]
async fn transrivi_audio(app: tauri::AppHandle, file_path: String) -> Result<String, String> {
    // 1. Recupera dinamicamente il percorso della risorsa nel pacchetto Tauri
    let resource_path = app.path().resource_dir()
        .map_err(|e| format!("Impossibile trovare la cartella delle risorse: {}", e))?
        .join("models/ggml-small.bin");

    let model_path = resource_path.to_str()
        .ok_or("Percorso del modello non valido")?;

    // 2. Inizializzazione del sidecar di whisper-cli
    let sidecar = app.shell().sidecar("whisper-cli")
        .map_err(|e| format!("Errore nell'inizializzazione del sidecar whisper-cli: {}", e))?;

    // 3. Esecuzione del comando con il path risolto del modello
    let output = sidecar
        .args(["-m", model_path, "-f", &file_path, "-l", "it", "--output-txt"])
        .output()
        .await
        .map_err(|e| format!("Errore durante l'esecuzione del processo Whisper: {}", e))?;

    if output.status.success() {
        let transcript = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(transcript)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Whisper ha restituito un errore: {}", error_message))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![transrivi_audio])
        .run(tauri::generate_context!())
        .expect("Errore imprevisto durante l'avvio dell'applicazione Tauri");
}