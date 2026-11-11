import { useDeviceStore } from "@/stores/deviceStore";
import { StatusBadge, Tag } from "@/components/ui/Badge";
import { formatDeviceType, formatVidPid } from "@/lib/format";
import { useLocale, useT } from "@/i18n";

function Row({ label, value }: { label: string; value?: string | null }) {
  return (
    <div className="grid grid-cols-[110px_1fr] gap-2 border-b border-graphite-800 py-1.5 text-xs last:border-0">
      <dt className="text-graphite-400">{label}</dt>
      <dd className="truncate font-mono text-graphite-100">{value || "—"}</dd>
    </div>
  );
}

export function RightPanel() {
  const t = useT();
  const locale = useLocale();
  const selected = useDeviceStore((s) => s.selectedDevice());

  return (
    <aside className="panel flex w-72 shrink-0 flex-col border-y-0 border-r-0">
      <div className="border-b border-graphite-700 px-3 py-2 text-[10px] font-semibold uppercase tracking-[0.14em] text-graphite-400">
        {t("right.title")}
      </div>
      <div className="flex-1 overflow-auto p-3">
        {!selected ? (
          <div className="panel-inset rounded p-3 text-sm text-graphite-400">{t("right.empty")}</div>
        ) : (
          <div className="space-y-3">
            <div>
              <div className="mb-1 flex items-center gap-2">
                <h2 className="text-sm font-semibold">{selected.name}</h2>
                {selected.isMock && <Tag tone="warn">{t("right.mock")}</Tag>}
              </div>
              <StatusBadge status={selected.status} />
            </div>
            <dl>
              <Row label={t("right.manufacturer")} value={selected.manufacturer} />
              <Row label={t("right.type")} value={formatDeviceType(selected.deviceType, locale)} />
              <Row label={t("right.model")} value={selected.model} />
              <Row label={t("right.mcu")} value={selected.mcu} />
              <Row label={t("right.comPort")} value={selected.comPort} />
              <Row label={t("right.vidPid")} value={formatVidPid(selected.vid, selected.pid)} />
              <Row label={t("right.usbSerial")} value={selected.usbSerial} />
              <Row label={t("right.radio")} value={selected.radioSystem} />
              <Row label={t("right.band")} value={selected.frequencyBand} />
              <Row label={t("right.firmware")} value={selected.firmwareVersion} />
              <Row label={t("right.bootloader")} value={selected.bootloaderVersion} />
              <Row label={t("right.mode")} value={selected.connectionMode} />
              <Row label={t("right.flash")} value={selected.flashMethod} />
            </dl>
            {selected.notes && (
              <p className="panel-inset rounded p-2 text-xs text-graphite-300">{selected.notes}</p>
            )}
          </div>
        )}
      </div>
    </aside>
  );
}
