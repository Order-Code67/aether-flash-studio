import { useAppStore } from "@/stores/appStore";
import { useDeviceStore } from "@/stores/deviceStore";
import { formatTime } from "@/lib/format";
import { useLocale, useT } from "@/i18n";

export function BottomPanel() {
  const t = useT();
  const locale = useLocale();
  const logs = useAppStore((s) => s.logs);
  const bottomStatus = useAppStore((s) => s.bottomStatus);
  const dashboard = useDeviceStore((s) => s.dashboard);
  const comPorts = useDeviceStore((s) => s.comPorts);

  return (
    <footer className="panel flex h-40 flex-col border-x-0 border-b-0">
      <div className="flex items-center justify-between border-b border-graphite-700 px-3 py-1.5 text-[11px]">
        <div className="flex items-center gap-3 text-graphite-300">
          <span>
            {t("bottom.status")}: <span className="text-graphite-100">{bottomStatus}</span>
          </span>
          <span>
            {t("bottom.comPorts")}:{" "}
            <span className="font-mono text-accent-teal">{comPorts.length}</span>
          </span>
          <span>
            {t("bottom.devices")}:{" "}
            <span className="font-mono text-accent-blue">
              {dashboard?.detectedDevices ?? 0}
            </span>
          </span>
        </div>
        <span className="text-graphite-500">{t("bottom.journal")}</span>
      </div>
      <div className="flex-1 overflow-auto px-3 py-2 font-mono text-[11px]">
        {logs.length === 0 ? (
          <div className="text-graphite-500">{t("bottom.empty")}</div>
        ) : (
          logs.map((log) => (
            <div key={log.id} className="grid grid-cols-[150px_90px_1fr] gap-2 py-0.5">
              <span className="text-graphite-500">{formatTime(log.at, locale)}</span>
              <span
                className={
                  log.level === "ERROR"
                    ? "text-danger"
                    : log.level === "WARNING"
                      ? "text-warning"
                      : log.level === "HARDWARE"
                        ? "text-accent-blue"
                        : "text-accent-teal"
                }
              >
                [{log.level}]
              </span>
              <span className="text-graphite-200">{log.message}</span>
            </div>
          ))
        )}
      </div>
    </footer>
  );
}
