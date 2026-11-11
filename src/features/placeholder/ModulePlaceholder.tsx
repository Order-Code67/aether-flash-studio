import { useAppStore } from "@/stores/appStore";
import { Button } from "@/components/ui/Button";
import { useT } from "@/i18n";

/** Legacy placeholder kept for compatibility; main modules are fully routed. */
export function ModulePlaceholder({ section }: { section: string }) {
  const t = useT();
  const setSection = useAppStore((s) => s.setSection);

  return (
    <div className="flex h-full flex-col items-start gap-3 overflow-auto p-4">
      <h2 className="text-lg font-semibold">{section}</h2>
      <p className="text-sm text-graphite-400">{t("dashboard.subtitle")}</p>
      <Button size="sm" onClick={() => setSection("dashboard")}>
        {t("nav.dashboard")}
      </Button>
    </div>
  );
}
