import type { AppSection } from "@/types/device";

const KEY = "afs.ui.prefs.v1";

export interface UiPrefs {
  sideNavCollapsed: boolean;
  sideNavWidth: number;
  lastSection: AppSection;
  subsections: Partial<Record<AppSection, string>>;
  journalOpen: boolean;
  contextPanelOpen: boolean;
}

export const DEFAULT_UI_PREFS: UiPrefs = {
  sideNavCollapsed: false,
  sideNavWidth: 220,
  lastSection: "dashboard",
  subsections: {},
  journalOpen: false,
  contextPanelOpen: true,
};

export function loadUiPrefs(): UiPrefs {
  try {
    const raw = localStorage.getItem(KEY);
    if (!raw) return { ...DEFAULT_UI_PREFS };
    const parsed = JSON.parse(raw) as Partial<UiPrefs>;
    return {
      ...DEFAULT_UI_PREFS,
      ...parsed,
      subsections: { ...DEFAULT_UI_PREFS.subsections, ...parsed.subsections },
      sideNavWidth: Math.min(320, Math.max(180, parsed.sideNavWidth ?? 220)),
    };
  } catch {
    return { ...DEFAULT_UI_PREFS };
  }
}

export function saveUiPrefs(prefs: UiPrefs): void {
  localStorage.setItem(KEY, JSON.stringify(prefs));
}
