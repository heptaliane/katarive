use log;

#[tauri::command]
pub async fn fetch_document(url: String) -> Result<String, String> {
    log::debug!("fetch_document is called (url = {:?})", url);
    Ok("Not Implemented".into())
}
