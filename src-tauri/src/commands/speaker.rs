use shared::speaker::{SpeakerControlArgs, SpeakerStartArgs};

use crate::state::SharedState;

#[tauri::command]
pub async fn start_speech(
    args: SpeakerStartArgs,
    state: tauri::State<'_, SharedState>,
) -> Result<(), String> {
    let state = state.lock().await;
    if let Some(speaker) = state.speaker_manager.get_client(args.name.clone()) {
        let _stream = speaker
            .speak(state.document.body.clone(), args.options)
            .await
            .map_err(|e| e.to_string())?;
        // TODO: handle stream
        Ok(())
    } else {
        Err(format!("Unsupported speaker: {:?}", args.name))
    }
}

#[tauri::command]
pub async fn pause_speech(
    args: SpeakerControlArgs,
    state: tauri::State<'_, SharedState>,
) -> Result<(), String> {
    let state = state.lock().await;
    if let Some(speaker) = state.speaker_manager.get_client(args.name.clone()) {
        speaker.pause().await.map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("Unsupported speaker: {:?}", args.name))
    }
}

#[tauri::command]
pub async fn resume_speech(
    args: SpeakerControlArgs,
    state: tauri::State<'_, SharedState>,
) -> Result<(), String> {
    let state = state.lock().await;
    if let Some(speaker) = state.speaker_manager.get_client(args.name.clone()) {
        speaker.resume().await.map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err(format!("Unsupported speaker: {:?}", args.name))
    }
}
