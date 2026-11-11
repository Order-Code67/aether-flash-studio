import { useCallback, useEffect, useState } from "react";
import { listen } from "@tauri-apps/api/event";
import { AppShell } from "@/components/layout/AppShell";
import { api } from "@/lib/api";
import { useAppStore } from "@/stores/appStore";
import { useDeviceStore } from "@/stores/deviceStore";
import { translate, useT } from "@/i18n";

export default function App() {
  const t = useT();
  const locale = useAppStore((s) => s.locale);
  const setAppInfo = useAppStore((s) => s.setAppInfo);
  const setSettings = useAppStore((s) => s.setSettings);
  const pushLog = useAppStore((s) => s.pushLog);
  const setBottomStatus = useAppStore((s) => s.setBottomStatus);

  const setDevices = useDeviceStore((s) => s.setDevices);
  const setComPorts = useDeviceStore((s) => s.setComPorts);
  const setUsbDevices = useDeviceStore((s) => s.setUsbDevices);
  const setDashboard = useDeviceStore((s) => s.setDashboard);
  const setScanning = useDeviceStore((s) => s.setScanning);

  const [refreshing, setRefreshing] = useState(false);
  const [bootError, setBootError] = useState<string | null>(null);

  const refresh = useCallback(async () => {
    setRefreshing(true);
    setScanning(true);
    setBottomStatus(translate(locale, "app.refreshing"));
    try {
      const [info, settings, devices, ports, usb, dashboard] = await Promise.all([
        api.getAppInfo(),
        api.getSettings(),
        api.scanDevices(),
        api.listComPorts(),
        api.listUsbDevices(),
        api.getDashboard(),
      ]);
      setAppInfo(info);
      setSettings(settings);
      document.documentElement.lang = settings.locale === "en" ? "en" : "ru";
      setDevices(devices);
      setComPorts(ports);
      setUsbDevices(usb);
      setDashboard(dashboard);
      const activeLocale = settings.locale === "en" ? "en" : "ru";
      pushLog(
        "INFO",
        translate(activeLocale, "app.workspaceReady", {
          count: devices.length,
          db: info.databasePath,
        }),
      );
      setBottomStatus(translate(activeLocale, "app.ready"));
      setBootError(null);
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setBootError(message);
      pushLog("ERROR", translate(locale, "app.refreshFailed", { error: message }));
      setBottomStatus(translate(locale, "dashboard.error"));
    } finally {
      setRefreshing(false);
      setScanning(false);
    }
  }, [
    locale,
    pushLog,
    setAppInfo,
    setBottomStatus,
    setComPorts,
    setDashboard,
    setDevices,
    setScanning,
    setSettings,
    setUsbDevices,
  ]);

  useEffect(() => {
    void refresh();
  }, [refresh]);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    void listen<{ reason: string; deviceCount: number }>("devices-changed", (event) => {
      const currentLocale = useAppStore.getState().locale;
      pushLog(
        "HARDWARE",
        translate(currentLocale, "app.hotplug", {
          reason: event.payload.reason,
          count: event.payload.deviceCount,
        }),
      );
      setBottomStatus(translate(currentLocale, "app.hotplugRefresh"));
      void (async () => {
        try {
          const [devices, ports, usb, dashboard] = await Promise.all([
            api.getCachedDevices(),
            api.listComPorts(),
            api.listUsbDevices(),
            api.getDashboard(),
          ]);
          setDevices(devices);
          setComPorts(ports);
          setUsbDevices(usb);
          setDashboard(dashboard);
        } catch (err) {
          const message = err instanceof Error ? err.message : String(err);
          pushLog(
            "ERROR",
            translate(useAppStore.getState().locale, "app.hotplugFailed", { error: message }),
          );
        }
      })();
    }).then((fn) => {
      unlisten = fn;
    });
    return () => {
      unlisten?.();
    };
  }, [pushLog, setBottomStatus, setComPorts, setDashboard, setDevices, setUsbDevices]);

  const dashboard = useDeviceStore((s) => s.dashboard);

  if (bootError && !dashboard) {
    return (
      <div className="flex h-full items-center justify-center p-6">
        <div className="panel max-w-lg rounded-lg p-5">
          <h1 className="text-lg font-semibold">Aether Flash Studio</h1>
          <p className="mt-2 text-sm text-danger">{t("app.bootError")}</p>
          <p className="mt-2 font-mono text-xs text-graphite-300">{bootError}</p>
          <button
            className="mt-4 rounded border border-accent-teal/40 bg-accent-teal/15 px-3 py-2 text-sm text-accent-teal"
            onClick={() => void refresh()}
          >
            {t("app.retry")}
          </button>
        </div>
      </div>
    );
  }

  return <AppShell onRefresh={() => void refresh()} refreshing={refreshing} />;
}
