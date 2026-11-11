import {
  Activity,
  AlertTriangle,
  Cable,
  Cpu,
  Radio,
  Wrench,
} from "lucide-react";
import { useDeviceStore } from "@/stores/deviceStore";
import { useAppStore } from "@/stores/appStore";
import { StatusBadge, Tag } from "@/components/ui/Badge";
import { formatDeviceType, formatTime } from "@/lib/format";
import { Button } from "@/components/ui/Button";
import { useLocale, useT } from "@/i18n";

function Stat({
  label,
  value,
  icon: Icon,
  tone,
}: {
  label: string;
  value: string | number;
  icon: React.ComponentType<{ className?: string }>;
  tone: "teal" | "blue" | "neutral";
}) {
  return (
    <div className="panel-inset rounded-lg p-3">
      <div className="mb-2 flex items-center justify-between">
        <span className="text-[11px] uppercase tracking-wide text-graphite-400">{label}</span>
        <Icon
          className={
            tone === "teal"
              ? "h-4 w-4 text-accent-teal"
              : tone === "blue"
                ? "h-4 w-4 text-accent-blue"
                : "h-4 w-4 text-graphite-400"
          }
        />
      </div>
      <div className="font-mono text-2xl font-semibold text-graphite-100">{value}</div>
    </div>
  );
}

export function DashboardPage() {
  const t = useT();
  const locale = useLocale();
  const dashboard = useDeviceStore((s) => s.dashboard);
  const selectDevice = useDeviceStore((s) => s.selectDevice);
  const setSection = useAppStore((s) => s.setSection);

  if (!dashboard) {
    return (
      <div className="flex h-full items-center justify-center text-sm text-graphite-400">
        {t("dashboard.loading")}
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col gap-4 overflow-auto p-4">
      <div className="flex items-end justify-between gap-3">
        <div>
          <h2 className="text-lg font-semibold">{t("dashboard.title")}</h2>
          <p className="text-sm text-graphite-400">{t("dashboard.subtitle")}</p>
        </div>
        <Button size="sm" onClick={() => setSection("device_manager")}>
          {t("dashboard.openDeviceManager")}
        </Button>
      </div>

      <div className="panel-inset flex flex-wrap gap-2 p-3">
        <span className="mr-2 self-center text-[11px] uppercase tracking-wide text-graphite-400">
          {t("nav.sub.quickActions")}
        </span>
        <Button size="sm" onClick={() => setSection("device_manager")}>
          {t("dash.qa.scan")}
        </Button>
        <Button size="sm" onClick={() => setSection("firmware_studio")}>
          {t("dash.qa.firmware")}
        </Button>
        <Button size="sm" onClick={() => setSection("flash_manager")}>
          {t("dash.qa.flash")}
        </Button>
        <Button size="sm" onClick={() => setSection("diagnostics")}>
          {t("dash.qa.diag")}
        </Button>
        <Button size="sm" onClick={() => setSection("backup_manager")}>
          {t("dash.qa.backup")}
        </Button>
      </div>

      <div className="grid grid-cols-2 gap-3 xl:grid-cols-4">
        <Stat label={t("dashboard.detected")} value={dashboard.detectedDevices} icon={Cable} tone="teal" />
        <Stat label={t("dashboard.connectedRx")} value={dashboard.connectedRx} icon={Radio} tone="blue" />
        <Stat label={t("dashboard.connectedTx")} value={dashboard.connectedTx} icon={Cpu} tone="blue" />
        <Stat
          label={t("dashboard.fwUpdates")}
          value={dashboard.firmwareUpdatesAvailable}
          icon={Activity}
          tone="neutral"
        />
      </div>

      <div className="grid grid-cols-1 gap-3 xl:grid-cols-3">
        <div className="panel-inset rounded-lg p-3 xl:col-span-2">
          <div className="mb-3 flex items-center justify-between">
            <h3 className="text-sm font-semibold">{t("dashboard.connectedDevices")}</h3>
            {dashboard.mockDeviceMode && <Tag tone="warn">{t("dashboard.includesMock")}</Tag>}
          </div>
          <div className="overflow-hidden rounded border border-graphite-700">
            <table className="w-full text-left text-xs">
              <thead className="bg-graphite-900 text-[10px] uppercase tracking-wide text-graphite-400">
                <tr>
                  <th className="px-3 py-2">{t("dashboard.colName")}</th>
                  <th className="px-3 py-2">{t("dashboard.colType")}</th>
                  <th className="px-3 py-2">{t("dashboard.colMcu")}</th>
                  <th className="px-3 py-2">{t("dashboard.colCom")}</th>
                  <th className="px-3 py-2">{t("dashboard.colFirmware")}</th>
                  <th className="px-3 py-2">{t("dashboard.colStatus")}</th>
                </tr>
              </thead>
              <tbody>
                {dashboard.devices.length === 0 ? (
                  <tr>
                    <td colSpan={6} className="px-3 py-6 text-center text-graphite-500">
                      {t("dashboard.noDevices")}
                    </td>
                  </tr>
                ) : (
                  dashboard.devices.map((device) => (
                    <tr
                      key={device.id}
                      className="cursor-pointer border-t border-graphite-800 hover:bg-graphite-800/60"
                      onClick={() => selectDevice(device.id)}
                    >
                      <td className="px-3 py-2">
                        <div className="font-medium text-graphite-100">{device.name}</div>
                        <div className="text-graphite-500">
                          {device.manufacturer || t("dashboard.unknownMfr")}
                        </div>
                      </td>
                      <td className="px-3 py-2 font-mono">
                        {formatDeviceType(device.deviceType, locale)}
                      </td>
                      <td className="px-3 py-2 font-mono">{device.mcu || "—"}</td>
                      <td className="px-3 py-2 font-mono">{device.comPort || "—"}</td>
                      <td className="px-3 py-2 font-mono">{device.firmwareVersion || "—"}</td>
                      <td className="px-3 py-2">
                        <StatusBadge status={device.status} />
                      </td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>
        </div>

        <div className="space-y-3">
          <div className="panel-inset rounded-lg p-3">
            <h3 className="mb-2 text-sm font-semibold">{t("dashboard.system")}</h3>
            <div className="space-y-2 text-xs">
              <div className="flex justify-between">
                <span className="text-graphite-400">{t("dashboard.usb")}</span>
                <span className={dashboard.usbOk ? "text-success" : "text-danger"}>
                  {dashboard.usbOk ? t("dashboard.online") : t("dashboard.error")}
                </span>
              </div>
              <div className="flex justify-between">
                <span className="text-graphite-400">{t("dashboard.uart")}</span>
                <span className={dashboard.uartOk ? "text-success" : "text-danger"}>
                  {dashboard.uartOk ? t("dashboard.online") : t("dashboard.error")}
                </span>
              </div>
              <div className="flex justify-between gap-2">
                <span className="text-graphite-400">{t("dashboard.lastFirmware")}</span>
                <span className="truncate font-mono text-graphite-200">
                  {dashboard.lastFirmwareUsed || "—"}
                </span>
              </div>
              <div className="flex justify-between gap-2">
                <span className="text-graphite-400">{t("dashboard.buildTools")}</span>
                <span className="truncate text-graphite-200">{dashboard.buildToolsStatus}</span>
              </div>
            </div>
          </div>

          <div className="panel-inset rounded-lg p-3">
            <div className="mb-2 flex items-center gap-2">
              <AlertTriangle className="h-4 w-4 text-warning" />
              <h3 className="text-sm font-semibold">{t("dashboard.errors")}</h3>
            </div>
            {dashboard.errors.length === 0 ? (
              <p className="text-xs text-graphite-500">{t("dashboard.noErrors")}</p>
            ) : (
              <ul className="space-y-1 text-xs text-danger">
                {dashboard.errors.map((err, idx) => (
                  <li key={`${err}-${idx}`} className="truncate">
                    {err}
                  </li>
                ))}
              </ul>
            )}
          </div>

          <div className="panel-inset rounded-lg p-3">
            <div className="mb-2 flex items-center gap-2">
              <Wrench className="h-4 w-4 text-accent-blue" />
              <h3 className="text-sm font-semibold">{t("dashboard.recentOps")}</h3>
            </div>
            <ul className="space-y-2">
              {dashboard.recentOperations.length === 0 ? (
                <li className="text-xs text-graphite-500">{t("dashboard.noOps")}</li>
              ) : (
                dashboard.recentOperations.slice(0, 6).map((op) => (
                  <li key={op.id} className="text-xs">
                    <div className="flex items-center justify-between gap-2">
                      <span className="font-medium text-graphite-200">{op.operationType}</span>
                      <span className="font-mono text-graphite-500">{op.status}</span>
                    </div>
                    <div className="truncate text-graphite-400">{op.description}</div>
                    <div className="text-[10px] text-graphite-600">
                      {formatTime(op.startedAt, locale)}
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
