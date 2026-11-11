use crate::error::{AppError, AppResult};
use crate::flashing::{
    pipeline_steps, prepare_flash_session, start_flash_session, FlashSessionView, PrepareFlashRequest,
    StartFlashRequest,
};
use crate::state::AppState;
use tauri::{AppHandle, State};

#[tauri::command]
pub fn prepare_flash_session_cmd(
    state: State<'_, AppState>,
    request: PrepareFlashRequest,
) -> AppResult<crate::database::FlashSessionRecord> {
    let device = state
        .devices
        .read()
        .iter()
        .find(|d| d.id == request.device_id)
        .cloned()
        .ok_or_else(|| AppError::DeviceNotFound(request.device_id.clone()))?;
    prepare_flash_session(&state.db, &state.ports, &device, &request.artifact_id)
}

#[tauri::command]
pub fn start_flash_session_cmd(
    app: AppHandle,
    state: State<'_, AppState>,
    request: StartFlashRequest,
) -> AppResult<crate::database::FlashSessionRecord> {
    let session = state
        .db
        .get_flash_session(&request.session_id)?
        .ok_or_else(|| AppError::Message("Flash session not found".into()))?;

    let device_id = session
        .device_id
        .clone()
        .ok_or_else(|| AppError::Message("Flash session missing device".into()))?;
    let artifact_id = session
        .artifact_id
        .clone()
        .ok_or_else(|| AppError::Message("Flash session missing artifact".into()))?;

    let device = state
        .devices
        .read()
        .iter()
        .find(|d| d.id == device_id)
        .cloned()
        .ok_or_else(|| AppError::DeviceNotFound(device_id))?;

    start_flash_session(
        &app,
        &state.db,
        &state.ports,
        &device,
        &artifact_id,
        &session,
        request.confirm,
    )
}

#[tauri::command]
pub fn get_flash_session(
    state: State<'_, AppState>,
    session_id: String,
) -> AppResult<FlashSessionView> {
    let session = state
        .db
        .get_flash_session(&session_id)?
        .ok_or_else(|| AppError::Message("Flash session not found".into()))?;
    Ok(FlashSessionView {
        session,
        steps: pipeline_steps(),
    })
}
