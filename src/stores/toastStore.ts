import { create } from "zustand";

export type ToastTone = "info" | "success" | "warning" | "error";

export interface ToastItem {
  id: string;
  title: string;
  message?: string;
  tone: ToastTone;
  createdAt: number;
}

interface ToastStore {
  toasts: ToastItem[];
  pushToast: (input: { title: string; message?: string; tone?: ToastTone }) => void;
  dismissToast: (id: string) => void;
}

export const useToastStore = create<ToastStore>((set) => ({
  toasts: [],
  pushToast: ({ title, message, tone = "info" }) =>
    set((state) => ({
      toasts: [
        {
          id: crypto.randomUUID(),
          title,
          message,
          tone,
          createdAt: Date.now(),
        },
        ...state.toasts,
      ].slice(0, 5),
    })),
  dismissToast: (id) => set((state) => ({ toasts: state.toasts.filter((t) => t.id !== id) })),
}));
