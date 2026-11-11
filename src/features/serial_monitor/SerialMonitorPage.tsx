import { useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { api } from "@/lib/api";
import { useT } from "@/i18n";
import { useAppStore } from "@/stores/appStore";
import { useDeviceStore } from "@/stores/deviceStore";
import { Button } from "@/components/ui/Button";

export function SerialMonitorPage() {
  const t = useT();
  const comPorts = useDeviceStore((s) => s.comPorts);
  const pushLog = useAppStore((s) => s.pushLog);
  const [port, setPort] = useState("");
  const [baud, setBaud] = useState("420000");
  const [mode, setMode] = useState("ascii");
  const [lines, setLines] = useState<string[]>([]);
  const [tx, setTx] = useState("");
  const [crsf, setCrsf] = useState("");

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    void listen<{ chunk: string; comPort: string }>("serial-data", (event) => {
      setLines((prev) => [`[${event.payload.comPort}] ${event.payload.chunk}`, ...prev].slice(0, 500));
    }).then((fn) => {
      unlisten = fn;
    });
    return () => unlisten?.();
  }, []);

  return (
    <div className="flex h-full flex-col gap-4 overflow-auto p-4">
      <div>
        <h2 className="text-lg font-semibold">{t("serial.title")}</h2>
        <p className="text-sm text-graphite-400">{t("serial.subtitle")}</p>
      </div>
      <div className="flex flex-wrap items-end gap-2">
        <label className="text-xs">
          {t("serial.com")}
          <select
            className="mt-1 block rounded border border-graphite-600 bg-graphite-950 px-2 py-2"
            value={port}
            onChange={(e) => setPort(e.target.value)}
          >
            <option value="">{t("serial.select")}</option>
            {comPorts.map((p) => (
              <option key={p.name} value={p.name}>
                {p.name}
              </option>
            ))}
          </select>
        </label>
        <label className="text-xs">
          {t("serial.baud")}
          <input
            className="mt-1 block rounded border border-graphite-600 bg-graphite-950 px-2 py-2 font-mono"
            value={baud}
            onChange={(e) => setBaud(e.target.value)}
          />
        </label>
        <label className="text-xs">
          {t("serial.mode")}
          <select
            className="mt-1 block rounded border border-graphite-600 bg-graphite-950 px-2 py-2"
            value={mode}
            onChange={(e) => setMode(e.target.value)}
          >
            <option value="ascii">ASCII</option>
            <option value="hex">HEX</option>
          </select>
        </label>
        <Button
          variant="primary"
          onClick={async () => {
            try {
              await api.openSerialMonitor({
                comPort: port,
                baudRate: Number(baud) || 115200,
                mode,
              });
              pushLog("HARDWARE", t("serial.opened", { port }));
            } catch (e) {
              pushLog("ERROR", String(e));
            }
          }}
        >
          {t("serial.open")}
        </Button>
        <Button
          onClick={async () => {
            try {
              await api.closeSerialMonitor();
              pushLog("HARDWARE", t("serial.closed"));
            } catch (e) {
              pushLog("ERROR", String(e));
            }
          }}
        >
          {t("serial.close")}
        </Button>
      </div>
      <div className="flex gap-2">
        <input
          className="flex-1 rounded border border-graphite-600 bg-graphite-950 px-2 py-2 font-mono text-sm"
          value={tx}
          onChange={(e) => setTx(e.target.value)}
          placeholder={mode === "hex" ? t("serial.hexBytes") : t("serial.asciiPayload")}
        />
        <Button
          onClick={async () => {
            try {
              await api.sendSerialMonitor(tx, mode === "hex");
            } catch (e) {
              pushLog("ERROR", String(e));
            }
          }}
        >
          {t("serial.send")}
        </Button>
      </div>
      <pre className="panel-inset min-h-48 flex-1 overflow-auto rounded-lg p-3 font-mono text-[11px]">
        {lines.join("\n") || t("serial.rxEmpty")}
      </pre>
      <section className="panel-inset space-y-2 rounded-lg p-3">
        <h3 className="text-sm font-semibold">{t("serial.crsf")}</h3>
        <input
          className="w-full rounded border border-graphite-600 bg-graphite-950 px-2 py-2 font-mono text-xs"
          placeholder={t("serial.pasteHex")}
          value={crsf}
          onChange={(e) => setCrsf(e.target.value)}
        />
        <Button
          size="sm"
          onClick={async () => {
            try {
              const frames = await api.decodeCrsfPreview(crsf);
              pushLog("INFO", t("serial.crsfFrames", { frames: JSON.stringify(frames) }));
            } catch (e) {
              pushLog("ERROR", String(e));
            }
          }}
        >
          {t("serial.decode")}
        </Button>
      </section>
    </div>
  );
}
