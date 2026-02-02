use log;

#[tauri::command]
pub async fn start_speech() -> Result<(), String> {
    log::debug!("start_speech is called");
    Ok(())
}

#[tauri::command]
pub async fn pause_speech() -> Result<(), String> {
    log::debug!("pause_speech is called");
    Ok(())
}
