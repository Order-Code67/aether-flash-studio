import { useEffect, useState } from "react";
import { api } from "@/lib/api";
import { useT } from "@/i18n";
import { useAppStore } from "@/stores/appStore";
import { useDeviceStore } from "@/stores/deviceStore";
import { Button } from "@/components/ui/Button";

export function ConfigurationPage() {
  const t = useT();
  const selected = useDeviceStore((s) => s.selectedDevice());
  const pushLog = useAppStore((s) => s.pushLog);
  const [payload, setPayload] = useState('{\n  "category": "general"\n}');
  const [items, setItems] = useState<Record<string, unknown>[]>([]);
  const [bindingName, setBindingName] = useState("");
  const [bindPhrase, setBindPhrase] = useState("");
  const [bindNameReady, setBindNameReady] = useState(false);

  async function refresh() {
    setItems(await api.listConfigurations(selected?.id));
  }

  useEffect(() => {
    if (!bindNameReady) {
      setBindingName(t("config.defaultBindName"));
      setBindNameReady(true);
    }
  }, [t, bindNameReady]);

  useEffect(() => {
    void refresh().catch((e) => pushLog("ERROR", String(e)));
  }, [selected?.id]);

  return (
    <div className="flex h-full flex-col gap-4 overflow-auto p-4">
      <div>
        <h2 className="text-lg font-semibold">{t("config.title")}</h2>
        <p className="text-sm text-graphite-400">{t("config.subtitle")}</p>
      </div>
      <div className="grid gap-3 xl:grid-cols-2">
        <section className="panel-inset space-y-2 rounded-lg p-4">
          <h3 className="text-sm font-semibold">{t("config.saveJson")}</h3>
          <textarea
            className="h-40 w-full rounded border border-graphite-600 bg-graphite-950 p-2 font-mono text-xs"
            value={payload}
            onChange={(e) => setPayload(e.target.value)}
          />
          <Button
            onClick={async () => {
              try {
                await api.saveConfiguration({
                  deviceId: selected?.id ?? null,
                  category: "general",
                  payload: JSON.parse(payload),
                  firmwareVersion: selected?.firmwareVersion ?? null,
                });
                await refresh();
                pushLog("INFO", t("config.saved"));
              } catch (e) {
                pushLog("ERROR", String(e));
              }
            }}
          >
            {t("config.save")}
          </Button>
          <div className="max-h-40 overflow-auto text-xs">
            {items.map((item) => (
              <div key={String(item.id)} className="border-b border-graphite-800 py-1">
                {String(item.category)} · {String(item.createdAt)}
              </div>
            ))}
          </div>
        </section>
        <section className="panel-inset space-y-2 rounded-lg p-4">
          <h3 className="text-sm font-semibold">{t("config.binding")}</h3>
          <input
            className="w-full rounded border border-graphite-600 bg-graphite-950 px-2 py-2 text-sm"
            value={bindingName}
            onChange={(e) => setBindingName(e.target.value)}
          />
          <input
            className="w-full rounded border border-graphite-600 bg-graphite-950 px-2 py-2 text-sm"
            type="password"
            placeholder={t("config.bindPhrase")}
            value={bindPhrase}
            onChange={(e) => setBindPhrase(e.target.value)}
          />
          <Button
            variant="primary"
            onClick={async () => {
              try {
                await api.saveBindingProfile({
                  name: bindingName,
                  deviceId: selected?.id ?? null,
                  bindPhrase: bindPhrase || null,
                });
                setBindPhrase("");
                pushLog("INFO", t("config.bindingSaved"));
              } catch (e) {
                pushLog("ERROR", String(e));
              }
            }}
          >
            {t("config.saveBinding")}
          </Button>
        </section>
      </div>
    </div>
  );
}
