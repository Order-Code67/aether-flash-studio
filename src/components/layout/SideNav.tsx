import clsx from "clsx";
import {
  Activity,
  Archive,
  Cable,
  ChevronLeft,
  ChevronRight,
  Cpu,
  HardDrive,
  KeyRound,
  LayoutDashboard,
  Radio,
  Settings,
  ShieldAlert,
  Terminal,
  Upload,
  Wrench,
} from "lucide-react";
import type { AppSection } from "@/types/device";
import { useAppStore } from "@/stores/appStore";
import { useT } from "@/i18n";
import { MENU_ITEMS } from "@/navigation/menu";

const ICONS: Record<AppSection, React.ComponentType<{ className?: string }>> = {
  dashboard: LayoutDashboard,
  device_manager: Cable,
  rx_manager: Radio,
  tx_manager: Cpu,
  firmware_studio: Wrench,
  flash_manager: Upload,
  recovery: ShieldAlert,
  unlock_recovery: ShieldAlert,
  configuration: HardDrive,
  binding: KeyRound,
  firmware_library: Wrench,
  serial_monitor: Terminal,
  diagnostics: Activity,
  backup_manager: Archive,
  settings: Settings,
};

export function SideNav() {
  const t = useT();
  const section = useAppStore((s) => s.section);
  const setSection = useAppStore((s) => s.setSection);
  const ui = useAppStore((s) => s.ui);
  const patchUi = useAppStore((s) => s.patchUi);

  const width = ui.sideNavCollapsed ? 56 : ui.sideNavWidth;

  return (
    <nav
      className="sidebar-surface relative flex shrink-0 flex-col border-r border-graphite-700"
      style={{ width }}
    >
      <div className="flex items-center justify-between border-b border-graphite-700 px-2 py-2">
        {!ui.sideNavCollapsed && (
          <span className="px-1 text-[10px] font-semibold uppercase tracking-[0.14em] text-graphite-400">
            {t("nav.navigation")}
          </span>
        )}
        <button
          className="ml-auto rounded border border-transparent p-1 text-graphite-400 hover:border-graphite-700 hover:text-graphite-100"
          onClick={() => patchUi({ sideNavCollapsed: !ui.sideNavCollapsed })}
          title={ui.sideNavCollapsed ? t("nav.expand") : t("nav.collapse")}
        >
          {ui.sideNavCollapsed ? (
            <ChevronRight className="h-4 w-4" />
          ) : (
            <ChevronLeft className="h-4 w-4" />
          )}
        </button>
      </div>

      <ul className="flex-1 space-y-0.5 overflow-auto p-1.5">
        {MENU_ITEMS.map((item) => {
          const Icon = ICONS[item.id];
          const active = section === item.id;
          return (
            <li key={item.id}>
              <button
                className={clsx(
                  "flex w-full items-center gap-2 rounded-[var(--afs-radius)] px-2.5 py-2 text-left text-sm transition-colors",
                  active
                    ? "bg-accent-teal/12 text-accent-teal"
                    : "text-graphite-300 hover:bg-graphite-800 hover:text-graphite-100",
                )}
                onClick={() => setSection(item.id)}
                title={t(item.labelKey)}
              >
                <Icon className="h-4 w-4 shrink-0" />
                {!ui.sideNavCollapsed && (
                  <>
                    <span className="flex-1 truncate">{t(item.labelKey)}</span>
                    <span
                      className={clsx(
                        "font-mono text-[10px]",
                        active ? "text-accent-teal/80" : "text-graphite-600",
                      )}
                    >
                      {item.phase}
                    </span>
                  </>
                )}
              </button>
            </li>
          );
        })}
      </ul>

      {!ui.sideNavCollapsed && (
        <div
          className="absolute top-0 right-0 h-full w-1 cursor-col-resize hover:bg-accent-teal/40"
          onMouseDown={(e) => {
            e.preventDefault();
            const startX = e.clientX;
            const startW = ui.sideNavWidth;
            const onMove = (ev: MouseEvent) => {
              const next = Math.min(320, Math.max(180, startW + (ev.clientX - startX)));
              patchUi({ sideNavWidth: next });
            };
            const onUp = () => {
              window.removeEventListener("mousemove", onMove);
              window.removeEventListener("mouseup", onUp);
            };
            window.addEventListener("mousemove", onMove);
            window.addEventListener("mouseup", onUp);
          }}
        />
      )}
    </nav>
  );
}
