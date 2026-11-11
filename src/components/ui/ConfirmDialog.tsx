import { Button } from "@/components/ui/Button";
import { useT } from "@/i18n";

interface ConfirmDialogProps {
  open: boolean;
  title: string;
  message: string;
  confirmLabel?: string;
  danger?: boolean;
  onConfirm: () => void;
  onCancel: () => void;
}

export function ConfirmDialog({
  open,
  title,
  message,
  confirmLabel,
  danger,
  onConfirm,
  onCancel,
}: ConfirmDialogProps) {
  const t = useT();
  if (!open) return null;

  return (
    <div className="fixed inset-0 z-[60] flex items-center justify-center bg-black/55 p-4">
      <div className="panel w-full max-w-md border p-4 shadow-2xl">
        <h3 className="text-base font-semibold text-graphite-100">{title}</h3>
        <p className="mt-2 text-sm text-graphite-400">{message}</p>
        <div className="mt-4 flex justify-end gap-2">
          <Button size="sm" onClick={onCancel}>
            {t("dialog.cancel")}
          </Button>
          <Button size="sm" variant={danger ? "danger" : "primary"} onClick={onConfirm}>
            {confirmLabel ?? t("dialog.confirm")}
          </Button>
        </div>
      </div>
    </div>
  );
}
