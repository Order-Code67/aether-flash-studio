import { create } from "zustand";
import type { AppInfo, AppSection, AppSettings, UiLocale } from "@/types/device";
import { defaultSubSection } from "@/navigation/menu";
import { loadUiPrefs, saveUiPrefs, type UiPrefs } from "@/lib/uiPrefs";

interface LogEntry {
  id: string;
  level: "INFO" | "WARNING" | "ERROR" | "HARDWARE" | "DEBUG";
  message: string;
  at: string;
}

interface AppStore {
  section: AppSection;
  subSection: string;
  appInfo: AppInfo | null;
  settings: AppSettings | null;
  locale: UiLocale;
  bottomStatus: string;
  logs: LogEntry[];
  ui: UiPrefs;
  setSection: (section: AppSection) => void;
  setSubSection: (subSection: string) => void;
  setAppInfo: (info: AppInfo) => void;
  setSettings: (settings: AppSettings) => void;
  setLocale: (locale: UiLocale) => void;
  setBottomStatus: (status: string) => void;
  pushLog: (level: LogEntry["level"], message: string) => void;
  patchUi: (patch: Partial<UiPrefs>) => void;
}

const initialUi = loadUiPrefs();

function normalizeSection(section: AppSection): AppSection {
  if (section === "unlock_recovery") return "recovery";
  if (section === "firmware_library") return "firmware_studio";
  return section;
}

export const useAppStore = create<AppStore>((set, get) => ({
  section: normalizeSection(initialUi.lastSection),
  subSection:
    initialUi.subsections[normalizeSection(initialUi.lastSection)] ??
    defaultSubSection(normalizeSection(initialUi.lastSection)),
  appInfo: null,
  settings: null,
  locale: "ru",
  bottomStatus: "Ожидание",
  logs: [],
  ui: initialUi,
  setSection: (section) => {
    const next = normalizeSection(section);
    const sub = get().ui.subsections[next] ?? defaultSubSection(next);
    const ui = { ...get().ui, lastSection: next, subsections: { ...get().ui.subsections, [next]: sub } };
    saveUiPrefs(ui);
    set({ section: next, subSection: sub, ui });
  },
  setSubSection: (subSection) => {
    const section = get().section;
    const ui = {
      ...get().ui,
      subsections: { ...get().ui.subsections, [section]: subSection },
    };
    saveUiPrefs(ui);
    set({ subSection, ui });
  },
  setAppInfo: (appInfo) => set({ appInfo }),
  setSettings: (settings) =>
    set({
      settings,
      locale: settings.locale === "en" ? "en" : "ru",
    }),
  setLocale: (locale) => set({ locale }),
  setBottomStatus: (bottomStatus) => set({ bottomStatus }),
  pushLog: (level, message) =>
    set((state) => ({
      logs: [
        {
          id: crypto.randomUUID(),
          level,
          message,
          at: new Date().toISOString(),
        },
        ...state.logs,
      ].slice(0, 200),
    })),
  patchUi: (patch) => {
    const ui = { ...get().ui, ...patch };
    saveUiPrefs(ui);
    set({ ui });
  },
}));
