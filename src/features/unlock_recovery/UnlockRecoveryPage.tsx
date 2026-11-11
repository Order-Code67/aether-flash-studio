import { useState } from "react";
import { api } from "@/lib/api";
import { useT } from "@/i18n";
import { useAppStore } from "@/stores/appStore";
import { useDeviceStore } from "@/stores/deviceStore";
import { Button } from "@/components/ui/Button";

export function UnlockRecoveryPage() {
  const t = useT();
  const subSection = useAppStore((s) => s.subSection);
  const selected = useDeviceStore((s) => s.selectedDevice());
  const pushLog = useAppStore((s) => s.pushLog);
  const [boot, setBoot] = useState<Record<string, unknown> | null>(null);
  const [recovery, setRecovery] = useState<Record<string, unknown> | null>(null);
  const [unlock, setUnlock] = useState<Record<string, unknown> | null>(null);

  if (!selected) {
    return <div className="p-4 text-sm text-graphite-400">{t("unlock.selectFirst")}</div>;
  }

  return (
    <div className="flex h-full flex-col gap-4 overflow-auto p-4">
      <div>
        <h2 className="text-lg font-semibold">{t("nav.recovery")}</h2>
        <p className="text-sm text-graphite-400">{t("unlock.subtitle")}</p>
      </div>

      {(subSection === "guide" || subSection === "history") && (
        <div className="panel-inset max-w-2xl p-4 text-sm text-graphite-400">
          {subSection === "guide" ? t("unlock.subtitle") : t("flash.history.help")}
        </div>
      )}

      <div className="grid gap-3 xl:grid-cols-3">
        <section className="panel-inset space-y-2 rounded-lg p-4">
          <h3 className="text-sm font-semibold">{t("unlock.bootloader")}</h3>
          <div className="flex flex-wrap gap-2">
            <Button
              size="sm"
              onClick={async () => {
                try {
                  setBoot(await api.inspectBootloader(selected.id));
                } catch (e) {
                  pushLog("ERROR", String(e));
                }
              }}
            >
              {t("unlock.inspect")}
            </Button>
            <Button
              size="sm"
              onClick={async () => {
                try {
                  setBoot(await api.enterBootloader(selected.id));
                } catch (e) {
                  pushLog("ERROR", String(e));
                }
              }}
            >
              {t("unlock.enter")}
            </Button>
            <Button
              size="sm"
              onClick={async () => {
                try {
                  setBoot(await api.exitBootloader(selected.id));
                } catch (e) {
                  pushLog("ERROR", String(e));
                }
              }}
            >
              {t("unlock.exit")}
            </Button>
          </div>
          <pre className="max-h-48 overflow-auto text-[11px]">{JSON.stringify(boot, null, 2)}</pre>
        </section>

        <section className="panel-inset space-y-2 rounded-lg p-4">
          <h3 className="text-sm font-semibold">{t("unlock.recovery")}</h3>
          <Button
            size="sm"
            onClick={async () => {
              try {
                setRecovery(await api.diagnoseRecovery(selected.id));
              } catch (e) {
                pushLog("ERROR", String(e));
              }
            }}
          >
            {t("unlock.diagnose")}
          </Button>
          <pre className="max-h-48 overflow-auto text-[11px]">
            {JSON.stringify(recovery, null, 2)}
          </pre>
        </section>

        <section className="panel-inset space-y-2 rounded-lg p-4">
          <h3 className="text-sm font-semibold">{t("unlock.unlockMgr")}</h3>
          <Button
            size="sm"
            onClick={async () => {
              try {
                setUnlock(await api.inspectUnlock(selected.id));
              } catch (e) {
                pushLog("ERROR", String(e));
              }
            }}
          >
            {t("unlock.inspectCaps")}
          </Button>
          <pre className="max-h-48 overflow-auto text-[11px]">{JSON.stringify(unlock, null, 2)}</pre>
        </section>
      </div>
    </div>
  );
}
