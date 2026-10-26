use tauri_plugin_shell::ShellExt;

#[tauri::command]
async fn transrivi_audio(app: tauri::AppHandle, file_path: String) -> Result<String, String> {
    // Modello ggml-small.bin, ottimizzato per le prestazioni dei chip Apple Silicon M2/M3
    let model_path = "models/ggml-small.bin";

    // 1. Inizializza il comando sidecar per whisper-cli
    let sidecar = app.shell().sidecar("whisper-cli")
        .map_err(|e| format!("Errore inizializzazione sidecar: {}", e))?;

    // 2. Esegue il comando passando il modello, il file da trascrivere e la lingua italiana
    let output = sidecar
        .args(["-m", model_path, "-f", &file_path, "-l", "it", "--output-txt"])
        .output()
        .await
        .map_err(|e| format!("Errore durante l'esecuzione di whisper: {}", e))?;

    if output.status.success() {
        let result_str = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(result_str)
    } else {
        let err_str = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Whisper ha restituito un errore: {}", err_str))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![transrivi_audio])
        .run(tauri::generate_context!())
        .expect("Errore durante l'avvio dell'applicazione Tauri");
}