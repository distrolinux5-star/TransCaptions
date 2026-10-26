use tauri::Manager;

#[tauri::command]
pub async fn run_whisper(app: tauri::AppHandle, audio_path: String) -> Result<String, String> {
    // Risolve la cartella delle risorse dell'app in modo sicuro
    let resource_dir = app.path().resource_dir()
        .map_err(|e| format!("Errore resource_dir: {}", e))?;

    // Determina il suffisso del target in base all'architettura corrente di compilazione
    let target_suffix = if cfg!(target_arch = "aarch64") {
        "aarch64-apple-darwin"
    } else {
        "x86_64-apple-darwin"
    };

    let binary_name = format!("binaries/whisper-cli-{}", target_suffix);
    let binary_path = resource_dir.join(binary_name);

    // Esegue il processo locale con i parametri desiderati (es. modello e file audio)
    let output = std::process::Command::new(&binary_path)
        .arg("-m")
        .arg(resource_dir.join("models/ggml-small.bin").to_str().unwrap())
        .arg("-f")
        .arg(audio_path)
        .output()
        .map_err(|e| format!("Impossibile eseguire whisper-cli: {}", e))?;

    if output.status.success() {
        let result = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(result)
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!("Errore da whisper: {}", error_msg))
    }
}