mod adapters;
mod backup;
mod binding;
mod bootloader;
mod commands;
mod compatibility;
mod configuration;
mod database;
mod device_detection;
mod diagnostics;
mod elrs_sources;
mod error;
mod firmware;
mod flashing;
mod hardware;
mod hotplug;
mod logging;
mod recovery;
mod security;
mod serial;
mod serial_monitor;
mod state;
mod unlock;
mod usb;
mod workspace;

use state::AppState;
use std::path::PathBuf;

fn resolve_data_dir() -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("AetherFlashStudio")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    logging::init();

    let data_dir = resolve_data_dir();
    let app_state = AppState::new(data_dir).expect("failed to initialize application state");

    tracing::info!(
        category = "INFO",
        data_dir = %app_state.data_dir.display(),
        "Starting Aether Flash Studio"
    );

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .setup(|app| {
            hotplug::start(&app.handle());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app::get_app_info,
            commands::app::get_hardware_status,
            commands::app::get_operation_history,
            commands::settings::get_settings,
            commands::settings::set_mock_device_mode,
            commands::settings::set_ui_locale,
            commands::devices::list_com_ports_cmd,
            commands::devices::list_usb_devices_cmd,
            commands::devices::scan_devices,
            commands::devices::get_cached_devices,
            commands::devices::manual_connect_device,
            commands::devices::get_locked_ports,
            commands::dashboard::get_dashboard,
            commands::profiles::list_device_profiles,
            commands::profiles::upsert_device_profile,
            commands::profiles::delete_device_profile,
            commands::rx_tx::list_rx_devices,
            commands::rx_tx::list_tx_devices,
            commands::rx_tx::get_device_detail,
            commands::rx_tx::read_device_configuration,
            commands::rx_tx::backup_device_configuration,
            commands::rx_tx::reconnect_device,
            commands::firmware::list_firmware_artifacts,
            commands::firmware::import_firmware_file_cmd,
            commands::firmware::validate_firmware_artifact,
            commands::firmware::check_build_tools_cmd,
            commands::firmware::start_firmware_build_cmd,
            commands::flashing::prepare_flash_session_cmd,
            commands::flashing::start_flash_session_cmd,
            commands::flashing::get_flash_session,
            commands::maintenance::inspect_bootloader,
            commands::maintenance::enter_bootloader_cmd,
            commands::maintenance::exit_bootloader_cmd,
            commands::maintenance::diagnose_recovery,
            commands::maintenance::inspect_unlock,
            commands::maintenance::apply_unlock_cmd,
            commands::maintenance::generate_diagnostic_report,
            commands::maintenance::save_configuration_cmd,
            commands::maintenance::list_configurations_cmd,
            commands::maintenance::compare_configurations_cmd,
            commands::maintenance::list_binding_profiles,
            commands::maintenance::save_binding_profile,
            commands::maintenance::compare_binding_profiles,
            commands::maintenance::list_backups_cmd,
            commands::maintenance::create_backup_cmd,
            commands::maintenance::delete_backup_cmd,
            commands::maintenance::compare_backups_cmd,
            commands::maintenance::export_backup_cmd,
            commands::maintenance::import_backup_cmd,
            commands::maintenance::open_serial_monitor,
            commands::maintenance::close_serial_monitor,
            commands::maintenance::send_serial_monitor,
            commands::maintenance::decode_crsf_preview,
            commands::studio::list_elrs_releases,
            commands::studio::list_elrs_tags,
            commands::studio::fetch_elrs_source,
            commands::studio::list_elrs_local_sources,
            commands::studio::verify_elrs_local_source,
            commands::studio::workspace_list_tree,
            commands::studio::workspace_read_file,
            commands::studio::workspace_write_file,
            commands::studio::workspace_revert_file,
            commands::studio::workspace_search,
            commands::studio::workspace_diff,
            commands::studio::run_custom_firmware_build,
            commands::studio::evaluate_compatibility,
            commands::studio::get_flash_step_logs,
            commands::studio::reveal_bind_phrase_cmd,
            commands::studio::check_build_tools_studio,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Aether Flash Studio");
}
