import { useState } from "react";
import { api } from "@/lib/api";
import { useT } from "@/i18n";
import { useAppStore } from "@/stores/appStore";
import { Button } from "@/components/ui/Button";

export function DiagnosticsPage() {
  const t = useT();
  const pushLog = useAppStore((s) => s.pushLog);
  const [report, setReport] = useState<unknown>(null);

  return (
    <div className="flex h-full flex-col gap-4 overflow-auto p-4">
      <div>
        <h2 className="text-lg font-semibold">{t("diag.title")}</h2>
        <p className="text-sm text-graphite-400">{t("diag.subtitle")}</p>
      </div>
      <Button
        variant="primary"
        onClick={async () => {
          try {
            const r = await api.generateDiagnosticReport();
            setReport(r);
            pushLog("INFO", t("diag.created", { id: r.id }));
          } catch (e) {
            pushLog("ERROR", String(e));
          }
        }}
      >
        {t("diag.generate")}
      </Button>
      <pre className="panel-inset flex-1 overflow-auto rounded-lg p-3 font-mono text-[11px]">
        {report ? JSON.stringify(report, null, 2) : t("diag.empty")}
      </pre>
    </div>
  );
}
