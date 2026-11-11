import { api } from "@/lib/api";
import { useAppStore } from "@/stores/appStore";
import { useDeviceStore } from "@/stores/deviceStore";
import { Button } from "@/components/ui/Button";
import { Tag } from "@/components/ui/Badge";
import { translate, useT } from "@/i18n";
import type { UiLocale } from "@/types/device";

export function SettingsPage() {
  const t = useT();
  const appInfo = useAppStore((s) => s.appInfo);
  const settings = useAppStore((s) => s.settings);
  const locale = useAppStore((s) => s.locale);
  const setSettings = useAppStore((s) => s.setSettings);
  const pushLog = useAppStore((s) => s.pushLog);
  const setBottomStatus = useAppStore((s) => s.setBottomStatus);
  const setDevices = useDeviceStore((s) => s.setDevices);
  const setDashboard = useDeviceStore((s) => s.setDashboard);
  const setComPorts = useDeviceStore((s) => s.setComPorts);
  const setUsbDevices = useDeviceStore((s) => s.setUsbDevices);
  const subSection = useAppStore((s) => s.subSection);
  const ui = useAppStore((s) => s.ui);
  const patchUi = useAppStore((s) => s.patchUi);

  async function toggleMock(enabled: boolean) {
    try {
      const next = await api.setMockDeviceMode(enabled);
      setSettings(next);
      const [devices, ports, usb, dash] = await Promise.all([
        api.scanDevices(),
        api.listComPorts(),
        api.listUsbDevices(),
        api.getDashboard(),
      ]);
      setDevices(devices);
      setComPorts(ports);
      setUsbDevices(usb);
      setDashboard(dash);
      pushLog("INFO", enabled ? t("settings.mockOn") : t("settings.mockOff"));
      setBottomStatus(enabled ? t("settings.enabled") : t("settings.disabled"));
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      pushLog("ERROR", t("settings.mockFail", { error: message }));
    }
  }

  async function changeLocale(next: UiLocale) {
    try {
      const saved = await api.setUiLocale(next);
      setSettings(saved);
      document.documentElement.lang = next === "ru" ? "ru" : "en";
      pushLog(
        "INFO",
        translate(next, "settings.languageChanged", {
          lang: translate(next, `settings.lang.${next}`),
        }),
      );
      setBottomStatus(translate(next, "app.ready"));
    } catch (err) {
      pushLog("ERROR", String(err));
    }
  }

  return (
    <div className="flex h-full flex-col gap-4 overflow-auto p-4">
      <div>
        <h2 className="text-lg font-semibold">{t("settings.title")}</h2>
        <p className="text-sm text-graphite-400">{t("settings.subtitle")}</p>
      </div>

      <div className="grid max-w-3xl gap-3">
        {(subSection === "interface" || subSection === "appearance") && (
          <section className="panel-inset rounded-lg p-4">
            <h3 className="mb-3 text-sm font-semibold">{t("nav.sub.interface")}</h3>
            <div className="flex flex-wrap gap-2">
              <Button
                size="sm"
                variant={ui.sideNavCollapsed ? "primary" : "secondary"}
                onClick={() => patchUi({ sideNavCollapsed: !ui.sideNavCollapsed })}
              >
                {ui.sideNavCollapsed ? t("nav.expand") : t("nav.collapse")}
              </Button>
              <Button
                size="sm"
                variant={ui.contextPanelOpen ? "primary" : "secondary"}
                onClick={() => patchUi({ contextPanelOpen: !ui.contextPanelOpen })}
              >
                {ui.contextPanelOpen ? t("settings.hideContext") : t("settings.showContext")}
              </Button>
              <Button
                size="sm"
                variant={ui.journalOpen ? "primary" : "secondary"}
                onClick={() => patchUi({ journalOpen: !ui.journalOpen })}
              >
                {t("bottom.journal")}
              </Button>
            </div>
          </section>
        )}

        <section className="panel-inset rounded-lg p-4">
          <h3 className="mb-3 text-sm font-semibold">{t("settings.language")}</h3>
          <p className="mb-3 text-xs text-graphite-400">{t("settings.languageHelp")}</p>
          <div className="flex gap-2">
            <Button
              variant={locale === "ru" ? "primary" : "secondary"}
              onClick={() => void changeLocale("ru")}
            >
              {t("settings.lang.ru")}
            </Button>
            <Button
              variant={locale === "en" ? "primary" : "secondary"}
              onClick={() => void changeLocale("en")}
            >
              {t("settings.lang.en")}
            </Button>
          </div>
        </section>

        <section className="panel-inset rounded-lg p-4">
          <h3 className="mb-3 text-sm font-semibold">{t("settings.application")}</h3>
          <dl className="space-y-2 text-xs">
            <div className="flex justify-between gap-3">
              <dt className="text-graphite-400">{t("settings.name")}</dt>
              <dd className="font-mono">{appInfo?.name ?? "Aether Flash Studio"}</dd>
            </div>
            <div className="flex justify-between gap-3">
              <dt className="text-graphite-400">{t("settings.version")}</dt>
              <dd className="font-mono">{appInfo?.version ?? "0.3.0"}</dd>
            </div>
            <div className="flex justify-between gap-3">
              <dt className="text-graphite-400">{t("settings.platform")}</dt>
              <dd className="font-mono">{appInfo?.platform ?? "—"}</dd>
            </div>
            <div className="flex justify-between gap-3">
              <dt className="text-graphite-400">{t("settings.dataDir")}</dt>
              <dd className="max-w-[65%] truncate font-mono" title={appInfo?.dataDir}>
                {appInfo?.dataDir ?? "—"}
              </dd>
            </div>
            <div className="flex justify-between gap-3">
              <dt className="text-graphite-400">{t("settings.sqlite")}</dt>
              <dd className="max-w-[65%] truncate font-mono" title={appInfo?.databasePath}>
                {appInfo?.databasePath ?? "—"}
              </dd>
            </div>
          </dl>
        </section>

        <section className="panel-inset rounded-lg p-4">
          <div className="mb-3 flex items-center justify-between gap-3">
            <div>
              <h3 className="text-sm font-semibold">{t("settings.mockTitle")}</h3>
              <p className="mt-1 text-xs text-graphite-400">{t("settings.mockHelp")}</p>
            </div>
            {settings?.mockDeviceMode ? (
              <Tag tone="warn">{t("settings.enabled")}</Tag>
            ) : (
              <Tag>{t("settings.disabled")}</Tag>
            )}
          </div>
          <div className="flex gap-2">
            <Button
              variant={settings?.mockDeviceMode ? "secondary" : "primary"}
              onClick={() => toggleMock(true)}
              disabled={!!settings?.mockDeviceMode}
            >
              {t("settings.enableMock")}
            </Button>
            <Button
              variant={settings?.mockDeviceMode ? "danger" : "secondary"}
              onClick={() => toggleMock(false)}
              disabled={!settings?.mockDeviceMode}
            >
              {t("settings.disableMock")}
            </Button>
          </div>
        </section>

        <section className="panel-inset rounded-lg p-4">
          <h3 className="mb-2 text-sm font-semibold">{t("settings.defaults")}</h3>
          <div className="text-xs text-graphite-300">
            {t("settings.defaultBaud")}{" "}
            <span className="font-mono text-graphite-100">
              {settings?.uartDefaultBaud ?? 460800}
            </span>
          </div>
        </section>
      </div>
    </div>
  );
}
