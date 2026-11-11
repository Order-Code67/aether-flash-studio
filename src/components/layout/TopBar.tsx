import { RefreshCw, Settings } from "lucide-react";
import { useAppStore } from "@/stores/appStore";
import { useDeviceStore } from "@/stores/deviceStore";
import { Button } from "@/components/ui/Button";
import { Tag } from "@/components/ui/Badge";
import { useT } from "@/i18n";

interface TopBarProps {
  onRefresh: () => void;
  refreshing: boolean;
}

export function TopBar({ onRefresh, refreshing }: TopBarProps) {
  const t = useT();
  const appInfo = useAppStore((s) => s.appInfo);
  const settings = useAppStore((s) => s.settings);
  const setSection = useAppStore((s) => s.setSection);
  const dashboard = useDeviceStore((s) => s.dashboard);
  const selected = useDeviceStore((s) => s.selectedDevice());

  return (
    <header className="panel flex h-12 items-center justify-between border-x-0 border-t-0 px-3">
      <div className="flex min-w-0 items-center gap-3">
        <img
          src="/brand/icon.svg"
          alt="Aether Flash Studio"
          className="h-7 w-7 shrink-0"
          draggable={false}
        />
        <div className="min-w-0">
          <div className="flex items-baseline gap-2">
            <h1 className="truncate text-sm font-semibold tracking-[0.04em] text-graphite-100">
              {t("app.name")}
            </h1>
            <span className="font-mono text-[11px] text-graphite-400">
              v{appInfo?.version ?? "0.3.0"}
            </span>
          </div>
          <p className="truncate text-[11px] text-graphite-400">{t("app.tagline")}</p>
        </div>
      </div>

      <div className="flex items-center gap-2">
        {settings?.mockDeviceMode && <Tag tone="warn">{t("top.mockOn")}</Tag>}
        <Tag tone={dashboard?.usbOk ? "teal" : "default"}>
          {t("top.usb", { state: dashboard?.usbOk ? t("top.ok") : t("top.err") })}
        </Tag>
        <Tag tone={dashboard?.uartOk ? "blue" : "default"}>
          {t("top.uart", { state: dashboard?.uartOk ? t("top.ok") : t("top.err") })}
        </Tag>
        <Tag tone={selected ? "teal" : "default"}>
          {selected ? selected.name : t("top.noDevice")}
        </Tag>
        <Button size="sm" onClick={onRefresh} disabled={refreshing}>
          <RefreshCw className={`h-3.5 w-3.5 ${refreshing ? "animate-spin" : ""}`} />
          {t("top.refresh")}
        </Button>
        <Button size="sm" variant="ghost" onClick={() => setSection("settings")}>
          <Settings className="h-3.5 w-3.5" />
        </Button>
      </div>
    </header>
  );
}
