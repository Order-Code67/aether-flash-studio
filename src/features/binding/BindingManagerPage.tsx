import { useState } from "react";
import { api } from "@/lib/api";
import { useT } from "@/i18n";
import { useAppStore } from "@/stores/appStore";
import { useDeviceStore } from "@/stores/deviceStore";
import { useToastStore } from "@/stores/toastStore";
import { Button } from "@/components/ui/Button";

export function BindingManagerPage() {
  const t = useT();
  const subSection = useAppStore((s) => s.subSection);
  const selected = useDeviceStore((s) => s.selectedDevice());
  const pushLog = useAppStore((s) => s.pushLog);
  const pushToast = useToastStore((s) => s.pushToast);
  const [bindingName, setBindingName] = useState("");
  const [bindPhrase, setBindPhrase] = useState("");

  return (
    <div className="flex h-full flex-col gap-4 overflow-auto p-4">
      <div>
        <h2 className="text-lg font-semibold">{t("binding.title")}</h2>
        <p className="text-sm text-graphite-400">{t("binding.subtitle")}</p>
      </div>

      {(subSection === "profiles" || subSection === "setup") && (
        <section className="panel-inset max-w-xl space-y-3 p-4">
          <h3 className="text-sm font-semibold">
            {subSection === "setup" ? t("binding.setup") : t("config.binding")}
          </h3>
          <input
            className="w-full rounded-[var(--afs-radius)] border border-graphite-700 bg-graphite-950 px-2 py-2 text-sm"
            placeholder={t("config.defaultBindName")}
            value={bindingName}
            onChange={(e) => setBindingName(e.target.value)}
          />
          <input
            className="w-full rounded-[var(--afs-radius)] border border-graphite-700 bg-graphite-950 px-2 py-2 text-sm"
            type="password"
            placeholder={t("config.bindPhrase")}
            value={bindPhrase}
            onChange={(e) => setBindPhrase(e.target.value)}
          />
          <p className="text-xs text-graphite-500">{t("binding.dpapiNote")}</p>
          <Button
            variant="primary"
            onClick={async () => {
              try {
                await api.saveBindingProfile({
                  name: bindingName || t("config.defaultBindName"),
                  deviceId: selected?.id ?? null,
                  bindPhrase: bindPhrase || null,
                });
                setBindPhrase("");
                pushLog("INFO", t("config.bindingSaved"));
                pushToast({ title: t("config.bindingSaved"), tone: "success" });
              } catch (e) {
                pushLog("ERROR", String(e));
                pushToast({ title: String(e), tone: "error" });
              }
            }}
          >
            {t("config.saveBinding")}
          </Button>
        </section>
      )}

      {(subSection === "history" || subSection === "diagnostics") && (
        <div className="panel-inset max-w-2xl p-4 text-sm text-graphite-400">
          {t(`binding.${subSection}.help`)}
        </div>
      )}
    </div>
  );
}
