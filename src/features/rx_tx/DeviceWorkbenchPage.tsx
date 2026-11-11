import { useEffect, useState } from "react";
import { api } from "@/lib/api";
import { formatDeviceType, formatVidPid } from "@/lib/format";
import { useLocale, useT } from "@/i18n";
import { useAppStore } from "@/stores/appStore";
import { useDeviceStore } from "@/stores/deviceStore";
import { Button } from "@/components/ui/Button";
import { StatusBadge, Tag } from "@/components/ui/Badge";
import type { DetectedDevice } from "@/types/device";

export function DeviceWorkbenchPage({ kind }: { kind: "rx" | "tx" }) {
  const t = useT();
  const locale = useLocale();
  const selectDevice = useDeviceStore((s) => s.selectDevice);
  const selectedId = useDeviceStore((s) => s.selectedDeviceId);
  const pushLog = useAppStore((s) => s.pushLog);
  const setBottomStatus = useAppStore((s) => s.setBottomStatus);
  const [devices, setDevices] = useState<DetectedDevice[]>([]);
  const [config, setConfig] = useState<string>("");
  const [busy, setBusy] = useState(false);

  async function refresh() {
    const list = kind === "rx" ? await api.listRxDevices() : await api.listTxDevices();
    setDevices(list);
  }

  useEffect(() => {
    void refresh().catch((err) => pushLog("ERROR", String(err)));
  }, [kind]);

  const selected = devices.find((d) => d.id === selectedId) ?? devices[0];

  async function run(label: string, fn: () => Promise<void>) {
    setBusy(true);
    setBottomStatus(label);
    try {
      await fn();
      pushLog("INFO", label);
    } catch (err) {
      pushLog("ERROR", `${label}: ${err instanceof Error ? err.message : String(err)}`);
    } finally {
      setBusy(false);
      setBottomStatus(t("app.ready"));
    }
  }

  return (
    <div className="flex h-full flex-col gap-4 overflow-auto p-4">
      <div>
        <h2 className="text-lg font-semibold">{kind === "rx" ? t("rx.title") : t("tx.title")}</h2>
        <p className="text-sm text-graphite-400">
          {kind === "rx" ? t("rxtx.subtitleRx") : t("rxtx.subtitleTx")}
        </p>
      </div>

      <div className="grid gap-3 xl:grid-cols-[1fr_1.2fr]">
        <div className="space-y-2">
          {devices.length === 0 ? (
            <div className="panel-inset rounded-lg p-4 text-sm text-graphite-500">
              {t("rxtx.empty", { kind: kind.toUpperCase() })}
            </div>
          ) : (
            devices.map((device) => (
              <button
                key={device.id}
                className={`w-full rounded border px-3 py-2 text-left ${
                  selected?.id === device.id
                    ? "border-accent-teal/50 bg-accent-teal/10"
                    : "border-graphite-700 hover:bg-graphite-800/50"
                }`}
                onClick={() => selectDevice(device.id)}
              >
                <div className="mb-1 flex items-center justify-between gap-2">
                  <span className="font-semibold">{device.name}</span>
                  <StatusBadge status={device.status} />
                </div>
                <div className="grid grid-cols-2 gap-1 text-[11px] text-graphite-400">
                  <span>{device.model || "—"}</span>
                  <span className="font-mono">{device.comPort || "—"}</span>
                  <span className="font-mono">{device.mcu || "—"}</span>
                  <span className="font-mono">{device.firmwareVersion || "—"}</span>
                </div>
              </button>
            ))
          )}
        </div>

        <div className="panel-inset space-y-3 rounded-lg p-4">
          {!selected ? (
            <p className="text-sm text-graphite-500">{t("rxtx.select")}</p>
          ) : (
            <>
              <div className="flex flex-wrap items-center gap-2">
                <h3 className="font-semibold">{selected.name}</h3>
                {selected.isMock && <Tag tone="warn">{t("right.mock")}</Tag>}
                <Tag tone="blue">{formatDeviceType(selected.deviceType, locale)}</Tag>
              </div>
              <dl className="grid grid-cols-2 gap-2 text-xs">
                <div>
                  <dt className="text-graphite-500">{t("right.manufacturer")}</dt>
                  <dd>{selected.manufacturer || "—"}</dd>
                </div>
                <div>
                  <dt className="text-graphite-500">{t("right.band")}</dt>
                  <dd>{selected.frequencyBand || "—"}</dd>
                </div>
                <div>
                  <dt className="text-graphite-500">{t("right.vidPid")}</dt>
                  <dd className="font-mono">{formatVidPid(selected.vid, selected.pid)}</dd>
                </div>
                <div>
                  <dt className="text-graphite-500">{t("right.flash")}</dt>
                  <dd>{selected.flashMethod || "—"}</dd>
                </div>
              </dl>
              <div className="flex flex-wrap gap-2">
                <Button
                  size="sm"
                  disabled={busy}
                  onClick={() =>
                    run(t("rxtx.readConfig"), async () => {
                      const cfg = await api.readDeviceConfiguration(selected.id);
                      setConfig(JSON.stringify(cfg, null, 2));
                    })
                  }
                >
                  {t("rxtx.readConfig")}
                </Button>
                <Button
                  size="sm"
                  disabled={busy}
                  onClick={() =>
                    run(t("rxtx.backupConfig"), async () => {
                      const path = await api.backupDeviceConfiguration(selected.id);
                      pushLog("INFO", t("rxtx.backupSaved", { path }));
                    })
                  }
                >
                  {t("rxtx.backupConfig")}
                </Button>
                <Button
                  size="sm"
                  disabled={busy}
                  onClick={() =>
                    run(t("rxtx.reconnect"), async () => {
                      const d = await api.reconnectDevice(selected.id);
                      selectDevice(d.id);
                      await refresh();
                    })
                  }
                >
                  {t("rxtx.reconnect")}
                </Button>
              </div>
              <pre className="max-h-64 overflow-auto rounded border border-graphite-700 bg-graphite-950 p-2 font-mono text-[11px] text-graphite-300">
                {config || t("rxtx.configPlaceholder")}
              </pre>
            </>
          )}
        </div>
      </div>
    </div>
  );
}
