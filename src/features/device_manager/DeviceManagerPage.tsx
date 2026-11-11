import { useMemo, useState } from "react";
import { PlugZap, RefreshCw, Search } from "lucide-react";
import { api } from "@/lib/api";
import { formatDeviceType, formatVidPid } from "@/lib/format";
import { useLocale, useT } from "@/i18n";
import { useAppStore } from "@/stores/appStore";
import { useDeviceStore } from "@/stores/deviceStore";
import { Button } from "@/components/ui/Button";
import { StatusBadge, Tag } from "@/components/ui/Badge";
import type { DetectedDevice } from "@/types/device";

export function DeviceManagerPage() {
  const t = useT();
  const locale = useLocale();
  const devices = useDeviceStore((s) => s.devices);
  const comPorts = useDeviceStore((s) => s.comPorts);
  const usbDevices = useDeviceStore((s) => s.usbDevices);
  const selectedDeviceId = useDeviceStore((s) => s.selectedDeviceId);
  const selectDevice = useDeviceStore((s) => s.selectDevice);
  const setDevices = useDeviceStore((s) => s.setDevices);
  const setComPorts = useDeviceStore((s) => s.setComPorts);
  const setUsbDevices = useDeviceStore((s) => s.setUsbDevices);
  const setDashboard = useDeviceStore((s) => s.setDashboard);
  const scanning = useDeviceStore((s) => s.scanning);
  const setScanning = useDeviceStore((s) => s.setScanning);
  const pushLog = useAppStore((s) => s.pushLog);
  const setBottomStatus = useAppStore((s) => s.setBottomStatus);
  const settings = useAppStore((s) => s.settings);

  const [query, setQuery] = useState("");
  const [comPort, setComPort] = useState("");
  const [baudRate, setBaudRate] = useState(settings?.uartDefaultBaud?.toString() ?? "460800");
  const [mode, setMode] = useState("UART");
  const [busy, setBusy] = useState(false);

  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    if (!q) return devices;
    return devices.filter((d) =>
      [d.name, d.manufacturer, d.model, d.comPort, d.mcu, d.radioSystem]
        .filter(Boolean)
        .some((v) => String(v).toLowerCase().includes(q)),
    );
  }, [devices, query]);

  async function refreshAll() {
    setScanning(true);
    setBottomStatus(t("devices.scanning"));
    try {
      const [scanned, ports, usb] = await Promise.all([
        api.scanDevices(),
        api.listComPorts(),
        api.listUsbDevices(),
      ]);
      setDevices(scanned);
      setComPorts(ports);
      setUsbDevices(usb);
      const dash = await api.getDashboard();
      setDashboard(dash);
      pushLog(
        "HARDWARE",
        t("devices.scanLog", { devices: scanned.length, ports: ports.length }),
      );
      setBottomStatus(t("devices.scanComplete"));
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      pushLog("ERROR", `${t("devices.scanFailed")}: ${message}`);
      setBottomStatus(t("devices.scanFailed"));
    } finally {
      setScanning(false);
    }
  }

  async function handleManualConnect() {
    if (!comPort) return;
    setBusy(true);
    setBottomStatus(t("devices.connecting", { port: comPort }));
    try {
      const device = await api.manualConnect({
        comPort,
        baudRate: Number(baudRate) || 115200,
        mode,
      });
      const next = [...devices.filter((d) => d.id !== device.id), device];
      setDevices(next);
      selectDevice(device.id);
      const dash = await api.getDashboard();
      setDashboard(dash);
      pushLog("HARDWARE", `${t("devices.manualOk")}: ${device.name} · ${device.comPort}`);
      setBottomStatus(t("devices.manualOk"));
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      pushLog("ERROR", `${t("devices.manualFail")}: ${message}`);
      setBottomStatus(t("devices.manualFail"));
    } finally {
      setBusy(false);
    }
  }

  return (
    <div className="flex h-full flex-col gap-4 overflow-auto p-4">
      <div className="flex flex-wrap items-end justify-between gap-3">
        <div>
          <h2 className="text-lg font-semibold">{t("devices.title")}</h2>
          <p className="text-sm text-graphite-400">{t("devices.subtitle")}</p>
        </div>
        <Button onClick={refreshAll} disabled={scanning}>
          <RefreshCw className={`h-4 w-4 ${scanning ? "animate-spin" : ""}`} />
          {t("devices.rescan")}
        </Button>
      </div>

      <div className="grid grid-cols-1 gap-3 xl:grid-cols-3">
        <div className="panel-inset rounded-lg p-3 xl:col-span-2">
          <div className="mb-3 flex flex-wrap items-center gap-2">
            <div className="relative min-w-[220px] flex-1">
              <Search className="pointer-events-none absolute top-2.5 left-2.5 h-3.5 w-3.5 text-graphite-500" />
              <input
                className="w-full rounded border border-graphite-600 bg-graphite-950 py-2 pr-3 pl-8 text-sm outline-none focus:border-accent-teal"
                placeholder={t("devices.filter")}
                value={query}
                onChange={(e) => setQuery(e.target.value)}
              />
            </div>
            <Tag tone="teal">
              {filtered.length} {t("devices.shown")}
            </Tag>
            {settings?.mockDeviceMode && <Tag tone="warn">{t("devices.mockOn")}</Tag>}
          </div>

          <div className="space-y-2">
            {filtered.length === 0 ? (
              <div className="rounded border border-dashed border-graphite-700 px-3 py-8 text-center text-sm text-graphite-500">
                {t("devices.empty")}
              </div>
            ) : (
              filtered.map((device) => (
                <DeviceRow
                  key={device.id}
                  device={device}
                  active={device.id === selectedDeviceId}
                  onSelect={() => selectDevice(device.id)}
                  locale={locale}
                  t={t}
                />
              ))
            )}
          </div>
        </div>

        <div className="space-y-3">
          <div className="panel-inset rounded-lg p-3">
            <h3 className="mb-2 text-sm font-semibold">{t("devices.manualConnect")}</h3>
            <div className="space-y-2 text-xs">
              <label className="block">
                <span className="mb-1 block text-graphite-400">{t("devices.comPort")}</span>
                <select
                  className="w-full rounded border border-graphite-600 bg-graphite-950 px-2 py-2 outline-none focus:border-accent-blue"
                  value={comPort}
                  onChange={(e) => setComPort(e.target.value)}
                >
                  <option value="">{t("devices.selectPort")}</option>
                  {comPorts.map((port) => (
                    <option key={port.name} value={port.name}>
                      {port.name}
                      {port.locked ? ` ${t("devices.locked")}` : ""}
                      {port.product ? ` — ${port.product}` : ""}
                    </option>
                  ))}
                </select>
              </label>
              <label className="block">
                <span className="mb-1 block text-graphite-400">{t("devices.baud")}</span>
                <input
                  className="w-full rounded border border-graphite-600 bg-graphite-950 px-2 py-2 font-mono outline-none focus:border-accent-blue"
                  value={baudRate}
                  onChange={(e) => setBaudRate(e.target.value)}
                />
              </label>
              <label className="block">
                <span className="mb-1 block text-graphite-400">{t("devices.mode")}</span>
                <select
                  className="w-full rounded border border-graphite-600 bg-graphite-950 px-2 py-2 outline-none focus:border-accent-blue"
                  value={mode}
                  onChange={(e) => setMode(e.target.value)}
                >
                  <option>UART</option>
                  <option>USB CDC</option>
                  <option>Bootloader</option>
                  <option>Debug</option>
                </select>
              </label>
              <Button
                className="w-full"
                variant="primary"
                disabled={!comPort || busy}
                onClick={handleManualConnect}
              >
                <PlugZap className="h-4 w-4" />
                {t("devices.connect")}
              </Button>
            </div>
          </div>

          <div className="panel-inset rounded-lg p-3">
            <h3 className="mb-2 text-sm font-semibold">{t("devices.profile")}</h3>
            <Button
              className="w-full"
              size="sm"
              disabled={!selectedDeviceId || busy}
              onClick={async () => {
                const device = devices.find((d) => d.id === selectedDeviceId);
                if (!device) return;
                setBusy(true);
                try {
                  await api.upsertDeviceProfile({
                    deviceId: device.id,
                    name: device.name,
                    manufacturer: device.manufacturer,
                    model: device.model,
                    preferredComPort: device.comPort,
                    preferredBaud: device.baudRate ?? (Number(baudRate) || 460800),
                    notes: "Saved from Device Manager",
                  });
                  pushLog("INFO", t("devices.profileSaved", { name: device.name }));
                } catch (err) {
                  pushLog("ERROR", String(err));
                } finally {
                  setBusy(false);
                }
              }}
            >
              {t("devices.saveProfile")}
            </Button>
          </div>

          <div className="panel-inset rounded-lg p-3">
            <h3 className="mb-2 text-sm font-semibold">{t("devices.comPorts")}</h3>
            <ul className="max-h-40 space-y-1 overflow-auto text-xs">
              {comPorts.length === 0 ? (
                <li className="text-graphite-500">{t("devices.noCom")}</li>
              ) : (
                comPorts.map((port) => (
                  <li key={port.name} className="flex items-center justify-between gap-2">
                    <span className="font-mono text-graphite-200">{port.name}</span>
                    <span className="truncate text-graphite-500">
                      {formatVidPid(port.vid, port.pid)}
                    </span>
                  </li>
                ))
              )}
            </ul>
          </div>

          <div className="panel-inset rounded-lg p-3">
            <h3 className="mb-2 text-sm font-semibold">{t("devices.usbCdc")}</h3>
            <ul className="max-h-40 space-y-1 overflow-auto text-xs">
              {usbDevices.length === 0 ? (
                <li className="text-graphite-500">{t("devices.noUsb")}</li>
              ) : (
                usbDevices.map((usb) => (
                  <li key={usb.id}>
                    <div className="font-medium text-graphite-200">{usb.name}</div>
                    <div className="font-mono text-graphite-500">
                      {formatVidPid(usb.vid, usb.pid)} · {usb.comPort || "—"}
                    </div>
                  </li>
                ))
              )}
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}

function DeviceRow({
  device,
  active,
  onSelect,
  locale,
  t,
}: {
  device: DetectedDevice;
  active: boolean;
  onSelect: () => void;
  locale: "ru" | "en";
  t: (key: string, params?: Record<string, string | number | null | undefined>) => string;
}) {
  return (
    <button
      className={`w-full rounded border px-3 py-2.5 text-left transition-colors ${
        active
          ? "border-accent-teal/50 bg-accent-teal/10"
          : "border-graphite-700 bg-graphite-950/40 hover:border-graphite-600 hover:bg-graphite-800/50"
      }`}
      onClick={onSelect}
    >
      <div className="mb-1 flex items-center justify-between gap-2">
        <div className="flex items-center gap-2">
          <span className="text-sm font-semibold">{device.name}</span>
          {device.isMock && <Tag tone="warn">{t("right.mock")}</Tag>}
          {device.bootloaderMode && <Tag tone="blue">{t("devices.bootloader")}</Tag>}
        </div>
        <StatusBadge status={device.status} />
      </div>
      <div className="grid grid-cols-2 gap-x-3 gap-y-1 text-[11px] text-graphite-400 md:grid-cols-4">
        <span>{device.manufacturer || t("devices.unknownMfr")}</span>
        <span className="font-mono">{formatDeviceType(device.deviceType, locale)}</span>
        <span className="font-mono">{device.comPort || t("devices.noComShort")}</span>
        <span className="font-mono">{formatVidPid(device.vid, device.pid)}</span>
        <span className="font-mono">{device.mcu || t("devices.mcuUnknown")}</span>
        <span>{device.flashMethod || t("devices.flashUnknown")}</span>
        <span>{device.radioSystem || t("devices.radioUnknown")}</span>
        <span className="font-mono">{device.firmwareVersion || t("devices.fwUnknown")}</span>
      </div>
    </button>
  );
}
