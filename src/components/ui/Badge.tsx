import clsx from "clsx";
import type { DeviceStatus } from "@/types/device";
import { formatStatus } from "@/lib/format";
import { useLocale } from "@/i18n";

const statusClass: Record<DeviceStatus, string> = {
  online: "status-online bg-success/10 border-success/30",
  offline: "status-offline bg-graphite-800 border-graphite-600",
  bootloader: "status-bootloader bg-accent-blue/10 border-accent-blue/30",
  updating: "status-updating bg-warning/10 border-warning/30",
  recovery: "status-recovery bg-warning/10 border-warning/30",
  error: "status-error bg-danger/10 border-danger/30",
  unknown: "status-unknown bg-graphite-800 border-graphite-600",
};

export function StatusBadge({ status }: { status: DeviceStatus }) {
  const locale = useLocale();
  return (
    <span
      className={clsx(
        "inline-flex items-center gap-1.5 rounded border px-2 py-0.5 text-[11px] font-medium uppercase tracking-wide",
        statusClass[status],
      )}
    >
      <span className="h-1.5 w-1.5 rounded-full bg-current" />
      {formatStatus(status, locale)}
    </span>
  );
}

export function Tag({
  children,
  tone = "default",
}: {
  children: React.ReactNode;
  tone?: "default" | "teal" | "blue" | "warn";
}) {
  return (
    <span
      className={clsx(
        "inline-flex rounded border px-1.5 py-0.5 text-[11px] font-medium",
        tone === "default" && "border-graphite-600 bg-graphite-800 text-graphite-300",
        tone === "teal" && "border-accent-teal/40 bg-accent-teal/10 text-accent-teal",
        tone === "blue" && "border-accent-blue/40 bg-accent-blue/10 text-accent-blue",
        tone === "warn" && "border-warning/40 bg-warning/10 text-warning",
      )}
    >
      {children}
    </span>
  );
}
