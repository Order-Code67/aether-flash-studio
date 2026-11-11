import { ChevronUp } from "lucide-react";
import { useAppStore } from "@/stores/appStore";
import { useDeviceStore } from "@/stores/deviceStore";
import { formatTime } from "@/lib/format";
import { useLocale, useT } from "@/i18n";

export function StatusBar() {
  const t = useT();
  const locale = useLocale();
  const logs = useAppStore((s) => s.logs);
  const bottomStatus = useAppStore((s) => s.bottomStatus);
  const appInfo = useAppStore((s) => s.appInfo);
  const ui = useAppStore((s) => s.ui);
  const patchUi = useAppStore((s) => s.patchUi);
  const dashboard = useDeviceStore((s) => s.dashboard);
  const comPorts = useDeviceStore((s) => s.comPorts);
  const selected = useDeviceStore((s) => s.selectedDevice());

  return (
    <footer className="panel flex flex-col border-x-0 border-b-0">
      <div className="flex h-8 items-center justify-between gap-3 px-3 text-[11px]">
        <div className="flex min-w-0 flex-wrap items-center gap-x-3 gap-y-1 text-graphite-300">
          <span>
            USB:{" "}
            <span className={dashboard?.usbOk ? "text-success" : "text-danger"}>
              {dashboard?.usbOk ? t("top.ok") : t("top.err")}
            </span>
          </span>
          <span>
            COM: <span className="font-mono text-accent-teal">{comPorts.length}</span>
          </span>
          <span className="truncate">
            {t("bottom.device")}:{" "}
            <span className="text-graphite-100">{selected?.name ?? "—"}</span>
          </span>
          <span className="truncate">
            {t("bottom.operation")}: <span className="text-graphite-100">{bottomStatus}</span>
          </span>
        </div>
        <div className="flex shrink-0 items-center gap-2">
          <span className="font-mono text-graphite-500">v{appInfo?.version ?? "0.3.0"}</span>
          <button
            className="inline-flex items-center gap-1 text-graphite-400 hover:text-graphite-100"
            onClick={() => patchUi({ journalOpen: !ui.journalOpen })}
          >
            {t("bottom.journal")}
            <ChevronUp className={`h-3.5 w-3.5 transition ${ui.journalOpen ? "" : "rotate-180"}`} />
          </button>
        </div>
      </div>

      {ui.journalOpen && (
        <div className="h-36 border-t border-graphite-700">
          <div className="h-full overflow-auto px-3 py-2 font-mono text-[11px]">
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
        </div>
      )}
    </footer>
  );
}
