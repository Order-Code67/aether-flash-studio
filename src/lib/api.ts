import { invoke } from "@tauri-apps/api/core";
import type {
  AppInfo,
  AppSettings,
  UiLocale,
  ComPortInfo,
  DashboardSnapshot,
  DetectedDevice,
  DeviceProfile,
  HardwareServiceStatus,
  ManualConnectRequest,
  OperationRecord,
  UpsertDeviceProfile,
  UsbDeviceInfo,
} from "@/types/device";

export interface FirmwareArtifact {
  id: string;
  firmwareVersionId: string;
  hardwareTargetId?: string | null;
  filePath: string;
  sha256: string;
  fileSize: number;
  verified: boolean;
  manufacturer: string;
  radioSystem: string;
  version: string;
  channel: string;
  createdAt: string;
}

export interface BackupRecord {
  id: string;
  deviceId?: string | null;
  backupType: string;
  filePath?: string | null;
  sha256?: string | null;
  model?: string | null;
  firmwareVersion?: string | null;
  description?: string | null;
  createdAt: string;
}

export const api = {
  getAppInfo: () => invoke<AppInfo>("get_app_info"),
  getHardwareStatus: () => invoke<HardwareServiceStatus>("get_hardware_status"),
  getOperationHistory: (limit?: number) =>
    invoke<OperationRecord[]>("get_operation_history", { limit }),
  getSettings: () => invoke<AppSettings>("get_settings"),
  setMockDeviceMode: (enabled: boolean) =>
    invoke<AppSettings>("set_mock_device_mode", { enabled }),
  setUiLocale: (locale: UiLocale) => invoke<AppSettings>("set_ui_locale", { locale }),
  listComPorts: () => invoke<ComPortInfo[]>("list_com_ports_cmd"),
  listUsbDevices: () => invoke<UsbDeviceInfo[]>("list_usb_devices_cmd"),
  scanDevices: () => invoke<DetectedDevice[]>("scan_devices"),
  getCachedDevices: () => invoke<DetectedDevice[]>("get_cached_devices"),
  manualConnect: (request: ManualConnectRequest) =>
    invoke<DetectedDevice>("manual_connect_device", { request }),
  getLockedPorts: () => invoke<string[]>("get_locked_ports"),
  getDashboard: () => invoke<DashboardSnapshot>("get_dashboard"),
  listDeviceProfiles: () => invoke<DeviceProfile[]>("list_device_profiles"),
  upsertDeviceProfile: (profile: UpsertDeviceProfile) =>
    invoke<DeviceProfile>("upsert_device_profile", { profile }),
  deleteDeviceProfile: (id: string) => invoke<void>("delete_device_profile", { id }),

  listRxDevices: () => invoke<DetectedDevice[]>("list_rx_devices"),
  listTxDevices: () => invoke<DetectedDevice[]>("list_tx_devices"),
  getDeviceDetail: (id: string) => invoke<DetectedDevice>("get_device_detail", { id }),
  readDeviceConfiguration: (id: string) => invoke<unknown>("read_device_configuration", { id }),
  backupDeviceConfiguration: (id: string) =>
    invoke<string>("backup_device_configuration", { id }),
  reconnectDevice: (id: string) => invoke<DetectedDevice>("reconnect_device", { id }),

  listFirmwareArtifacts: (filter?: Record<string, unknown>) =>
    invoke<FirmwareArtifact[]>("list_firmware_artifacts", { filter: filter ?? null }),
  importFirmwareFile: (path: string, meta: Record<string, unknown>) =>
    invoke<FirmwareArtifact>("import_firmware_file_cmd", { path, meta }),
  validateFirmwareArtifact: (id: string) =>
    invoke<{ valid: boolean; message: string }>("validate_firmware_artifact", { id }),
  checkBuildTools: () => invoke<Record<string, unknown>>("check_build_tools_cmd"),
  startFirmwareBuild: (request: Record<string, unknown>) =>
    invoke<string>("start_firmware_build_cmd", { request }),

  prepareFlashSession: (request: { deviceId: string; artifactId: string }) =>
    invoke<Record<string, unknown>>("prepare_flash_session_cmd", { request }),
  startFlashSession: (request: { sessionId: string; confirm: boolean }) =>
    invoke<Record<string, unknown>>("start_flash_session_cmd", { request }),
  getFlashSession: (id: string) => invoke<Record<string, unknown>>("get_flash_session", { id }),

  inspectBootloader: (id: string) => invoke<Record<string, unknown>>("inspect_bootloader", { id }),
  enterBootloader: (id: string) => invoke<Record<string, unknown>>("enter_bootloader_cmd", { id }),
  exitBootloader: (id: string) => invoke<Record<string, unknown>>("exit_bootloader_cmd", { id }),
  diagnoseRecovery: (id: string) => invoke<Record<string, unknown>>("diagnose_recovery", { id }),
  inspectUnlock: (id: string) => invoke<Record<string, unknown>>("inspect_unlock", { id }),
  applyUnlock: (id: string, capabilityId: string) =>
    invoke<string>("apply_unlock_cmd", { id, capabilityId }),
  generateDiagnosticReport: () =>
    invoke<{ id: string; createdAt: string; summary: unknown }>("generate_diagnostic_report"),

  saveConfiguration: (request: Record<string, unknown>) =>
    invoke<Record<string, unknown>>("save_configuration_cmd", { request }),
  listConfigurations: (deviceId?: string | null) =>
    invoke<Record<string, unknown>[]>("list_configurations_cmd", { deviceId: deviceId ?? null }),
  compareConfigurations: (left: unknown, right: unknown) =>
    invoke<unknown>("compare_configurations_cmd", { left, right }),

  listBindingProfiles: () => invoke<Record<string, unknown>[]>("list_binding_profiles"),
  saveBindingProfile: (request: Record<string, unknown>) =>
    invoke<Record<string, unknown>>("save_binding_profile", { request }),
  compareBindingProfiles: (a: string, b: string) =>
    invoke<unknown>("compare_binding_profiles", { a, b }),

  listBackups: () => invoke<BackupRecord[]>("list_backups_cmd"),
  createBackup: (request: Record<string, unknown>) =>
    invoke<BackupRecord>("create_backup_cmd", { request }),
  deleteBackup: (id: string) => invoke<void>("delete_backup_cmd", { id }),
  compareBackups: (a: string, b: string) => invoke<unknown>("compare_backups_cmd", { a, b }),
  exportBackup: (id: string, destination: string) =>
    invoke<string>("export_backup_cmd", { id, destination }),
  importBackup: (path: string) => invoke<BackupRecord>("import_backup_cmd", { path }),

  openSerialMonitor: (request: { comPort: string; baudRate: number; mode: string }) =>
    invoke<void>("open_serial_monitor", { request }),
  closeSerialMonitor: () => invoke<void>("close_serial_monitor"),
  sendSerialMonitor: (data: string, hex: boolean) =>
    invoke<void>("send_serial_monitor", { data, hex }),
  decodeCrsfPreview: (bytesHex: string) =>
    invoke<Array<{ frameType: string; summary: string }>>("decode_crsf_preview", { bytesHex }),

  listElrsReleases: () => invoke<unknown[]>("list_elrs_releases"),
  listElrsTags: () => invoke<unknown[]>("list_elrs_tags"),
  fetchElrsSource: (request: { versionOrRef: string; kind: string }) =>
    invoke<{
      id: string;
      versionOrRef: string;
      localPath: string;
      commitSha?: string | null;
      integrityOk: boolean;
    }>("fetch_elrs_source", { request }),
  listElrsLocalSources: () => invoke<unknown[]>("list_elrs_local_sources"),
  verifyElrsLocalSource: (path: string) => invoke<boolean>("verify_elrs_local_source", { path }),
  workspaceListTree: (root: string, maxEntries?: number) =>
    invoke<Array<{ path: string; name: string; isDir: boolean; size?: number }>>(
      "workspace_list_tree",
      { root, maxEntries: maxEntries ?? null },
    ),
  workspaceReadFile: (root: string, relative: string) =>
    invoke<{ path: string; content: string; language: string; size: number; readonly: boolean }>(
      "workspace_read_file",
      { root, relative },
    ),
  workspaceWriteFile: (root: string, relative: string, content: string) =>
    invoke<void>("workspace_write_file", { root, relative, content }),
  workspaceRevertFile: (root: string, relative: string) =>
    invoke<void>("workspace_revert_file", { root, relative }),
  workspaceSearch: (root: string, query: string) =>
    invoke<string[]>("workspace_search", { root, query }),
  workspaceDiff: (root: string, relative: string) =>
    invoke<string>("workspace_diff", { root, relative }),
  runCustomFirmwareBuild: (request: Record<string, unknown>) =>
    invoke<Record<string, unknown>>("run_custom_firmware_build", { request }),
  evaluateCompatibility: (deviceId: string, artifactId: string) =>
    invoke<{ compatible: boolean; issues: Array<{ code: string; severity: string; message: string }> }>(
      "evaluate_compatibility",
      { deviceId, artifactId },
    ),
  getFlashStepLogs: (sessionId: string) =>
    invoke<unknown[]>("get_flash_step_logs", { sessionId }),
  revealBindPhrase: (id: string) => invoke<string>("reveal_bind_phrase_cmd", { id }),
};
