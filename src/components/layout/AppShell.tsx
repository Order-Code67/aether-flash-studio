import { TopBar } from "@/components/layout/TopBar";
import { SideNav } from "@/components/layout/SideNav";
import { RightPanel } from "@/components/layout/RightPanel";
import { StatusBar } from "@/components/layout/StatusBar";
import { SectionTabs } from "@/components/layout/SectionTabs";
import { ToastHost } from "@/components/ui/ToastHost";
import { DashboardPage } from "@/features/dashboard/DashboardPage";
import { DeviceManagerPage } from "@/features/device_manager/DeviceManagerPage";
import { SettingsPage } from "@/features/settings/SettingsPage";
import { DeviceWorkbenchPage } from "@/features/rx_tx/DeviceWorkbenchPage";
import { FirmwareStudioPage } from "@/features/firmware/FirmwareStudioPage";
import { FlashManagerPage } from "@/features/flash_manager/FlashManagerPage";
import { UnlockRecoveryPage } from "@/features/unlock_recovery/UnlockRecoveryPage";
import { ConfigurationPage } from "@/features/configuration/ConfigurationPage";
import { BindingManagerPage } from "@/features/binding/BindingManagerPage";
import { SerialMonitorPage } from "@/features/serial_monitor/SerialMonitorPage";
import { DiagnosticsPage } from "@/features/diagnostics/DiagnosticsPage";
import { BackupManagerPage } from "@/features/backup_manager/BackupManagerPage";
import { useAppStore } from "@/stores/appStore";
import { getMenuItem } from "@/navigation/menu";

interface AppShellProps {
  onRefresh: () => void;
  refreshing: boolean;
}

export function AppShell({ onRefresh, refreshing }: AppShellProps) {
  const section = useAppStore((s) => s.section);
  const subSection = useAppStore((s) => s.subSection);
  const setSubSection = useAppStore((s) => s.setSubSection);
  const ui = useAppStore((s) => s.ui);
  const menu = getMenuItem(section);

  let content: React.ReactNode;
  switch (section) {
    case "dashboard":
      content = <DashboardPage />;
      break;
    case "device_manager":
      content = <DeviceManagerPage />;
      break;
    case "rx_manager":
      content = <DeviceWorkbenchPage kind="rx" />;
      break;
    case "tx_manager":
      content = <DeviceWorkbenchPage kind="tx" />;
      break;
    case "firmware_studio":
      content = <FirmwareStudioPage />;
      break;
    case "flash_manager":
      content = <FlashManagerPage />;
      break;
    case "recovery":
    case "unlock_recovery":
      content = <UnlockRecoveryPage />;
      break;
    case "configuration":
      content = <ConfigurationPage />;
      break;
    case "binding":
      content = <BindingManagerPage />;
      break;
    case "serial_monitor":
      content = <SerialMonitorPage />;
      break;
    case "diagnostics":
      content = <DiagnosticsPage />;
      break;
    case "backup_manager":
      content = <BackupManagerPage />;
      break;
    case "settings":
      content = <SettingsPage />;
      break;
    default:
      content = <DashboardPage />;
  }

  return (
    <div className="flex h-full flex-col bg-[var(--afs-bg)]">
      <TopBar onRefresh={onRefresh} refreshing={refreshing} />
      <div className="flex min-h-0 flex-1">
        <SideNav />
        <main className="workspace-surface flex min-w-0 flex-1 flex-col">
          {menu && menu.children.length > 0 && (
            <SectionTabs
              items={menu.children}
              active={subSection}
              onChange={setSubSection}
            />
          )}
          <div className="min-h-0 flex-1 overflow-hidden">{content}</div>
        </main>
        {ui.contextPanelOpen && <RightPanel />}
      </div>
      <StatusBar />
      <ToastHost />
    </div>
  );
}
