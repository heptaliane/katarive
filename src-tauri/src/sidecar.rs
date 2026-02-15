use shared::sidecar::SidecarMessage;
use tauri::AppHandle;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

pub async fn spawn_sidecar(app: &AppHandle, name: &str) -> Result<u16, String> {
    let (mut rx, _) = app
        .shell()
        .sidecar(name)
        .map_err(|e| format!("Sidecar {} is not found: {}", name, e))?
        .spawn()
        .map_err(|e| format!("Failed to spawn {}: {}", name, e))?;

    while let Some(event) = rx.recv().await {
        if let CommandEvent::Stdout(line) = event {
            return match serde_json::from_slice(&line) {
                Ok(SidecarMessage::Ready { port }) => Ok(port),
                Ok(SidecarMessage::Status { level, message }) => {
                    Err(format!("[{}] ({}) {}", name, level, message))
                }
                Err(e) => Err(format!("Unsupported sidecar message format: {}", e)),
            };
        }
    }
    Err(format!("Sidecar {} failed to report port", name))
}
