import type { AppSection } from "@/types/device";

export type SubSectionId = string;

export interface MenuSubItem {
  id: SubSectionId;
  labelKey: string;
}

export interface MenuItem {
  id: AppSection;
  labelKey: string;
  phase: string;
  children: MenuSubItem[];
}

export const MENU_ITEMS: MenuItem[] = [
  {
    id: "dashboard",
    labelKey: "nav.dashboard",
    phase: "P1",
    children: [
      { id: "overview", labelKey: "nav.sub.overview" },
      { id: "connected", labelKey: "nav.sub.connectedDevices" },
      { id: "operations", labelKey: "nav.sub.recentOps" },
      { id: "quick", labelKey: "nav.sub.quickActions" },
      { id: "system", labelKey: "nav.sub.systemStatus" },
    ],
  },
  {
    id: "device_manager",
    labelKey: "nav.device_manager",
    phase: "P2",
    children: [
      { id: "all", labelKey: "nav.sub.allDevices" },
      { id: "connected", labelKey: "nav.sub.connected" },
      { id: "disconnected", labelKey: "nav.sub.disconnected" },
      { id: "profiles", labelKey: "nav.sub.deviceProfiles" },
      { id: "history", labelKey: "nav.sub.deviceHistory" },
    ],
  },
  {
    id: "rx_manager",
    labelKey: "nav.rx_manager",
    phase: "P3",
    children: [
      { id: "overview", labelKey: "nav.sub.rxOverview" },
      { id: "configuration", labelKey: "nav.sub.rxConfig" },
      { id: "firmware", labelKey: "nav.sub.rxFirmware" },
      { id: "diagnostics", labelKey: "nav.sub.rxDiagnostics" },
      { id: "backup", labelKey: "nav.sub.rxBackup" },
    ],
  },
  {
    id: "tx_manager",
    labelKey: "nav.tx_manager",
    phase: "P3",
    children: [
      { id: "overview", labelKey: "nav.sub.txOverview" },
      { id: "configuration", labelKey: "nav.sub.txConfig" },
      { id: "firmware", labelKey: "nav.sub.txFirmware" },
      { id: "diagnostics", labelKey: "nav.sub.txDiagnostics" },
      { id: "backup", labelKey: "nav.sub.txBackup" },
    ],
  },
  {
    id: "firmware_studio",
    labelKey: "nav.firmware_studio",
    phase: "P4",
    children: [
      { id: "library", labelKey: "nav.sub.fwLibrary" },
      { id: "sources", labelKey: "nav.sub.sourceManager" },
      { id: "advanced", labelKey: "nav.sub.advancedBuilder" },
      { id: "custom", labelKey: "nav.sub.customBuilder" },
      { id: "history", labelKey: "nav.sub.buildHistory" },
      { id: "settings", labelKey: "nav.sub.buildSettings" },
    ],
  },
  {
    id: "flash_manager",
    labelKey: "nav.flash_manager",
    phase: "P5",
    children: [
      { id: "flash", labelKey: "nav.sub.flashFirmware" },
      { id: "history", labelKey: "nav.sub.flashHistory" },
      { id: "profiles", labelKey: "nav.sub.flashProfiles" },
      { id: "settings", labelKey: "nav.sub.flashSettings" },
    ],
  },
  {
    id: "recovery",
    labelKey: "nav.recovery",
    phase: "P6",
    children: [
      { id: "overview", labelKey: "nav.sub.recoveryOverview" },
      { id: "bootloader", labelKey: "nav.sub.bootloaderTools" },
      { id: "esp", labelKey: "nav.sub.espRecovery" },
      { id: "history", labelKey: "nav.sub.recoveryHistory" },
      { id: "guide", labelKey: "nav.sub.recoveryGuide" },
    ],
  },
  {
    id: "configuration",
    labelKey: "nav.configuration",
    phase: "P7",
    children: [
      { id: "device", labelKey: "nav.sub.deviceConfig" },
      { id: "rx", labelKey: "nav.sub.rxSettings" },
      { id: "tx", labelKey: "nav.sub.txSettings" },
      { id: "profiles", labelKey: "nav.sub.configProfiles" },
      { id: "compare", labelKey: "nav.sub.configCompare" },
    ],
  },
  {
    id: "binding",
    labelKey: "nav.binding",
    phase: "P7",
    children: [
      { id: "profiles", labelKey: "nav.sub.bindingProfiles" },
      { id: "setup", labelKey: "nav.sub.bindingSetup" },
      { id: "history", labelKey: "nav.sub.bindingHistory" },
      { id: "diagnostics", labelKey: "nav.sub.bindingDiagnostics" },
    ],
  },
  {
    id: "serial_monitor",
    labelKey: "nav.serial_monitor",
    phase: "P7",
    children: [
      { id: "terminal", labelKey: "nav.sub.serialTerminal" },
      { id: "crsf", labelKey: "nav.sub.crsfMonitor" },
      { id: "profiles", labelKey: "nav.sub.connectionProfiles" },
      { id: "logs", labelKey: "nav.sub.serialLogs" },
    ],
  },
  {
    id: "backup_manager",
    labelKey: "nav.backup_manager",
    phase: "P7",
    children: [
      { id: "all", labelKey: "nav.sub.allBackups" },
      { id: "device", labelKey: "nav.sub.deviceBackups" },
      { id: "config", labelKey: "nav.sub.configBackups" },
      { id: "firmware", labelKey: "nav.sub.firmwareBackups" },
      { id: "compare", labelKey: "nav.sub.compare" },
      { id: "restore", labelKey: "nav.sub.restore" },
    ],
  },
  {
    id: "diagnostics",
    labelKey: "nav.diagnostics",
    phase: "P6",
    children: [
      { id: "system", labelKey: "nav.sub.systemDiag" },
      { id: "device", labelKey: "nav.sub.deviceDiag" },
      { id: "usb", labelKey: "nav.sub.usbDiag" },
      { id: "uart", labelKey: "nav.sub.uartDiag" },
      { id: "firmware", labelKey: "nav.sub.firmwareDiag" },
      { id: "errors", labelKey: "nav.sub.errorLogs" },
      { id: "operations", labelKey: "nav.sub.operationLogs" },
    ],
  },
  {
    id: "settings",
    labelKey: "nav.settings",
    phase: "P1",
    children: [
      { id: "general", labelKey: "nav.sub.general" },
      { id: "appearance", labelKey: "nav.sub.appearance" },
      { id: "language", labelKey: "nav.sub.language" },
      { id: "interface", labelKey: "nav.sub.interface" },
      { id: "tools", labelKey: "nav.sub.firmwareTools" },
      { id: "serial", labelKey: "nav.sub.serialPorts" },
      { id: "storage", labelKey: "nav.sub.storage" },
      { id: "security", labelKey: "nav.sub.security" },
      { id: "notifications", labelKey: "nav.sub.notifications" },
      { id: "about", labelKey: "nav.sub.about" },
    ],
  },
];

export function getMenuItem(section: AppSection): MenuItem | undefined {
  return MENU_ITEMS.find((item) => item.id === section);
}

export function defaultSubSection(section: AppSection): SubSectionId {
  return getMenuItem(section)?.children[0]?.id ?? "overview";
}
