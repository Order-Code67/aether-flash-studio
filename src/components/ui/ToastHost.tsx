import { useEffect } from "react";
import { X } from "lucide-react";
import { useToastStore } from "@/stores/toastStore";
import clsx from "clsx";

export function ToastHost() {
  const toasts = useToastStore((s) => s.toasts);
  const dismissToast = useToastStore((s) => s.dismissToast);

  useEffect(() => {
    if (toasts.length === 0) return;
    const timers = toasts.map((toast) =>
      window.setTimeout(() => dismissToast(toast.id), 4500),
    );
    return () => timers.forEach((id) => window.clearTimeout(id));
  }, [toasts, dismissToast]);

  if (toasts.length === 0) return null;

  return (
    <div className="pointer-events-none fixed right-4 bottom-14 z-50 flex w-80 flex-col gap-2">
      {toasts.map((toast) => (
        <div
          key={toast.id}
          className={clsx(
            "toast-enter pointer-events-auto border px-3 py-2 shadow-lg",
            "bg-[var(--afs-card)] border-[var(--afs-border)]",
            toast.tone === "success" && "border-l-success",
            toast.tone === "warning" && "border-l-warning",
            toast.tone === "error" && "border-l-danger",
            toast.tone === "info" && "border-l-accent-teal",
            "border-l-4",
          )}
        >
          <div className="flex items-start justify-between gap-2">
            <div>
              <div className="text-sm font-medium text-graphite-100">{toast.title}</div>
              {toast.message && (
                <div className="mt-0.5 text-xs text-graphite-400">{toast.message}</div>
              )}
            </div>
            <button
              className="text-graphite-400 hover:text-graphite-100"
              onClick={() => dismissToast(toast.id)}
            >
              <X className="h-3.5 w-3.5" />
            </button>
          </div>
        </div>
      ))}
    </div>
  );
}
