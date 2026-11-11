import { useEffect, useState } from "react";
import { api, type FirmwareArtifact } from "@/lib/api";
import { useT } from "@/i18n";
import { useAppStore } from "@/stores/appStore";
import { useDeviceStore } from "@/stores/deviceStore";
import { useToastStore } from "@/stores/toastStore";
import { Button } from "@/components/ui/Button";
import { Tag } from "@/components/ui/Badge";
import { ConfirmDialog } from "@/components/ui/ConfirmDialog";

const FLASH_STEPS = [
  "flash.step.identify",
  "flash.step.compat",
  "flash.step.validate",
  "flash.step.backup",
  "flash.step.bootloader",
  "flash.step.erase",
  "flash.step.write",
  "flash.step.verify",
  "flash.step.reboot",
  "flash.step.reenum",
] as const;

export function FlashManagerPage() {
  const t = useT();
  const subSection = useAppStore((s) => s.subSection);
  const pushLog = useAppStore((s) => s.pushLog);
  const setBottomStatus = useAppStore((s) => s.setBottomStatus);
  const pushToast = useToastStore((s) => s.pushToast);
  const selected = useDeviceStore((s) => s.selectedDevice());
  const [artifacts, setArtifacts] = useState<FirmwareArtifact[]>([]);
  const [artifactId, setArtifactId] = useState("");
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [compat, setCompat] = useState("");
  const [confirmOpen, setConfirmOpen] = useState(false);
  const [activeStep, setActiveStep] = useState(-1);

  useEffect(() => {
    void api
      .listFirmwareArtifacts()
      .then(setArtifacts)
      .catch((e) => pushLog("ERROR", String(e)));
  }, [pushLog]);

  const artifact = artifacts.find((a) => a.id === artifactId);

  async function prepare() {
    if (!selected || !artifactId) return;
    try {
      setActiveStep(0);
      setBottomStatus(t("flash.preparing"));
      const report = await api.evaluateCompatibility(selected.id, artifactId);
      setCompat(JSON.stringify(report, null, 2));
      setActiveStep(1);
      if (!report.compatible) {
        pushLog("ERROR", t("fw.compatBlocked"));
        pushToast({ title: t("fw.compatBlocked"), tone: "error" });
        setBottomStatus(t("fw.compatBlocked"));
        return;
      }
      setActiveStep(2);
      const session = await api.prepareFlashSession({
        deviceId: selected.id,
        artifactId,
      });
      setSessionId(String(session.id));
      setActiveStep(3);
      pushLog("INFO", t("fw.prepared", { id: String(session.id) }));
      pushToast({ title: t("fw.prepared", { id: String(session.id) }), tone: "success" });
      setBottomStatus(t("app.ready"));
    } catch (e) {
      pushLog("ERROR", String(e));
      pushToast({ title: String(e), tone: "error" });
    }
  }

  async function doFlash() {
    if (!sessionId) return;
    setConfirmOpen(false);
    try {
      for (let i = 4; i < FLASH_STEPS.length; i++) {
        setActiveStep(i);
        setBottomStatus(t(FLASH_STEPS[i]));
      }
      const result = await api.startFlashSession({ sessionId, confirm: true });
      const logs = await api.getFlashStepLogs(sessionId);
      setCompat(JSON.stringify({ result, logs }, null, 2));
      pushLog("INFO", t("fw.flashStatus", { status: String(result.status) }));
      pushToast({
        title: t("fw.flashStatus", { status: String(result.status) }),
        tone: String(result.status).toLowerCase().includes("success") ? "success" : "warning",
      });
      setBottomStatus(t("app.ready"));
    } catch (e) {
      pushLog("ERROR", String(e));
      pushToast({ title: String(e), tone: "error" });
    }
  }

  if (subSection === "history" || subSection === "profiles" || subSection === "settings") {
    return (
      <div className="overflow-auto p-4 text-sm text-graphite-400">
        <h2 className="text-lg font-semibold text-graphite-100">{t(`flash.${subSection}.title`)}</h2>
        <p className="mt-1">{t(`flash.${subSection}.help`)}</p>
        <pre className="panel-inset mt-4 overflow-auto p-3 font-mono text-[11px]">
          {compat || t("flash.noSessionData")}
        </pre>
      </div>
    );
  }

  return (
    <div className="flex h-full flex-col gap-4 overflow-auto p-4">
      <div>
        <h2 className="text-lg font-semibold">{t("flash.title")}</h2>
        <p className="text-sm text-graphite-400">{t("flash.subtitle")}</p>
      </div>

      <div className="grid gap-3 xl:grid-cols-[1.1fr_0.9fr]">
        <section className="panel-inset space-y-3 p-4">
          <div className="text-xs text-graphite-400">
            {t("fw.device")}:{" "}
            <span className="text-graphite-100">{selected?.name || t("fw.none")}</span>{" "}
            {selected?.isMock && <Tag tone="warn">{t("fw.mockBlocked")}</Tag>}
          </div>
          <div className="grid grid-cols-2 gap-2 text-xs">
            <div>
              <div className="text-graphite-500">{t("right.comPort")}</div>
              <div className="font-mono">{selected?.comPort || "—"}</div>
            </div>
            <div>
              <div className="text-graphite-500">{t("right.mcu")}</div>
              <div className="font-mono">{selected?.mcu || "—"}</div>
            </div>
            <div>
              <div className="text-graphite-500">SHA-256</div>
              <div className="font-mono">{artifact?.sha256?.slice(0, 16) || "—"}</div>
            </div>
            <div>
              <div className="text-graphite-500">{t("flash.size")}</div>
              <div className="font-mono">{artifact?.fileSize ?? "—"}</div>
            </div>
          </div>
          <select
            className="w-full rounded-[var(--afs-radius)] border border-graphite-700 bg-graphite-950 px-2 py-2 text-sm"
            value={artifactId}
            onChange={(e) => setArtifactId(e.target.value)}
          >
            <option value="">{t("fw.selectArtifact")}</option>
            {artifacts.map((a) => (
              <option key={a.id} value={a.id}>
                {a.version} ({a.sha256.slice(0, 8)})
              </option>
            ))}
          </select>
          <div className="flex flex-wrap gap-2">
            <Button onClick={() => void prepare()} disabled={!selected || !artifactId}>
              {t("fw.compatPrepare")}
            </Button>
            <Button
              variant="danger"
              disabled={!sessionId}
              onClick={() => setConfirmOpen(true)}
            >
              {t("fw.confirmFlash")}
            </Button>
          </div>
          <pre className="max-h-64 overflow-auto font-mono text-[11px]">{compat}</pre>
        </section>

        <section className="panel-inset p-4">
          <h3 className="mb-3 text-sm font-semibold">{t("flash.pipeline")}</h3>
          <ol className="space-y-1.5">
            {FLASH_STEPS.map((key, index) => (
              <li
                key={key}
                className={`flex items-center gap-2 rounded-[var(--afs-radius)] px-2 py-1.5 text-xs ${
                  index === activeStep
                    ? "bg-accent-teal/15 text-accent-teal"
                    : index < activeStep
                      ? "text-success"
                      : "text-graphite-400"
                }`}
              >
                <span className="font-mono w-5">{index + 1}</span>
                {t(key)}
              </li>
            ))}
          </ol>
        </section>
      </div>

      <ConfirmDialog
        open={confirmOpen}
        title={t("flash.confirmTitle")}
        message={t("flash.confirmMessage")}
        confirmLabel={t("fw.confirmFlash")}
        danger
        onCancel={() => setConfirmOpen(false)}
        onConfirm={() => void doFlash()}
      />
    </div>
  );
}
