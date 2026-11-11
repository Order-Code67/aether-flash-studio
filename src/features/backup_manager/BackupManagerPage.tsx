import { useEffect, useState } from "react";
import { api, type BackupRecord } from "@/lib/api";
import { useT } from "@/i18n";
import { useAppStore } from "@/stores/appStore";
import { useDeviceStore } from "@/stores/deviceStore";
import { Button } from "@/components/ui/Button";

export function BackupManagerPage() {
  const t = useT();
  const pushLog = useAppStore((s) => s.pushLog);
  const selected = useDeviceStore((s) => s.selectedDevice());
  const [items, setItems] = useState<BackupRecord[]>([]);

  async function refresh() {
    setItems(await api.listBackups());
  }

  useEffect(() => {
    void refresh().catch((e) => pushLog("ERROR", String(e)));
  }, []);

  return (
    <div className="flex h-full flex-col gap-4 overflow-auto p-4">
      <div className="flex items-end justify-between gap-3">
        <div>
          <h2 className="text-lg font-semibold">{t("backup.title")}</h2>
          <p className="text-sm text-graphite-400">{t("backup.subtitle")}</p>
        </div>
        <Button
          variant="primary"
          onClick={async () => {
            try {
              await api.createBackup({
                deviceId: selected?.id ?? null,
                backupType: "manual",
                description: "Manual backup",
                payloadJson: JSON.stringify({
                  device: selected,
                  createdBy: "Backup Manager",
                }),
                model: selected?.model ?? null,
                firmwareVersion: selected?.firmwareVersion ?? null,
              });
              await refresh();
              pushLog("INFO", t("backup.created"));
            } catch (e) {
              pushLog("ERROR", String(e));
            }
          }}
        >
          {t("backup.create")}
        </Button>
      </div>
      <div className="space-y-2">
        {items.length === 0 ? (
          <div className="panel-inset rounded-lg p-4 text-sm text-graphite-500">{t("backup.empty")}</div>
        ) : (
          items.map((b) => (
            <div
              key={b.id}
              className="panel-inset flex items-center justify-between gap-3 rounded-lg px-3 py-2 text-xs"
            >
              <div>
                <div className="font-semibold text-sm">{b.backupType}</div>
                <div className="text-graphite-400">
                  {b.model || "—"} · {b.createdAt}
                </div>
                <div className="font-mono text-graphite-500">{b.sha256?.slice(0, 16)}</div>
              </div>
              <Button
                size="sm"
                variant="danger"
                onClick={async () => {
                  await api.deleteBackup(b.id);
                  await refresh();
                }}
              >
                {t("backup.delete")}
              </Button>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
