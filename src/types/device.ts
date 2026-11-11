export type DeviceType = "rx" | "tx" | "generic" | "unknown";

export type DeviceStatus =
  | "online"
  | "offline"
  | "bootloader"
  | "updating"
  | "recovery"
  | "error"
  | "unknown";

export interface DetectedDevice {
  id: string;
  name: string;
  manufacturer?: string | null;
  deviceType: DeviceType;
  model?: string | null;
  mcu?: string | null;
  comPort?: string | null;
  baudRate?: number | null;
  vid?: number | null;
  pid?: number | null;
  usbSerial?: string | null;
  radioSystem?: string | null;
  frequencyBand?: string | null;
  firmwareVersion?: string | null;
  bootloaderVersion?: string | null;
  connectionMode?: string | null;
  flashMethod?: string | null;
  bootloaderMode: boolean;
  status: DeviceStatus;
  isMock: boolean;
  notes?: string | null;
  profileId?: string | null;
}

export interface DeviceProfile {
  id: string;
  deviceId?: string | null;
  name: string;
  manufacturer?: string | null;
  model?: string | null;
  notes?: string | null;
  preferredComPort?: string | null;
  preferredBaud?: number | null;
  createdAt: string;
  updatedAt: string;
}

export interface UpsertDeviceProfile {
  id?: string | null;
  deviceId?: string | null;
  name: string;
  manufacturer?: string | null;
  model?: string | null;
  notes?: string | null;
  preferredComPort?: string | null;
  preferredBaud?: number | null;
}

export interface ComPortInfo {
  name: string;
  portType: string;
  vid?: number | null;
  pid?: number | null;
  serialNumber?: string | null;
  manufacturer?: string | null;
  product?: string | null;
  locked: boolean;
}

export interface UsbDeviceInfo {
  id: string;
  name: string;
  manufacturer?: string | null;
  product?: string | null;
  vid?: number | null;
  pid?: number | null;
  serialNumber?: string | null;
  comPort?: string | null;
  busPath?: string | null;
  status?: string | null;
}

export interface ManualConnectRequest {
  comPort: string;
  baudRate: number;
  mode: string;
}

export interface OperationRecord {
  id: string;
  operationType: string;
  deviceId?: string | null;
  status: string;
  startedAt: string;
  finishedAt?: string | null;
  description: string;
  errorMessage?: string | null;
}

export interface DashboardSnapshot {
  detectedDevices: number;
  connectedRx: number;
  connectedTx: number;
  usbOk: boolean;
  uartOk: boolean;
  lastFirmwareUsed?: string | null;
  recentOperations: OperationRecord[];
  errors: string[];
  firmwareUpdatesAvailable: number;
  buildToolsStatus: string;
  mockDeviceMode: boolean;
  devices: DetectedDevice[];
}

export interface AppInfo {
  name: string;
  version: string;
  dataDir: string;
  databasePath: string;
  platform: string;
}

export type UiLocale = "ru" | "en";

export interface AppSettings {
  mockDeviceMode: boolean;
  uartDefaultBaud: number;
  lastFirmwareUsed?: string | null;
  locale: UiLocale;
}

export interface HardwareServiceStatus {
  usbEnumeration: boolean;
  serialEnumeration: boolean;
  portLocking: boolean;
  flashingEngine: boolean;
  mockDeviceModeAvailable: boolean;
}

export type AppSection =
  | "dashboard"
  | "device_manager"
  | "rx_manager"
  | "tx_manager"
  | "firmware_studio"
  | "flash_manager"
  | "recovery"
  | "unlock_recovery"
  | "configuration"
  | "binding"
  | "firmware_library"
  | "serial_monitor"
  | "diagnostics"
  | "backup_manager"
  | "settings";
