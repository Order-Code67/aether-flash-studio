import { useCallback } from "react";
import { useAppStore } from "@/stores/appStore";
import { en } from "./en";
import { ru } from "./ru";
import type { Locale, TranslationDict } from "./types";

const dictionaries: Record<Locale, TranslationDict> = { ru, en };

export type { Locale };

export function translate(
  locale: Locale,
  key: string,
  params?: Record<string, string | number | null | undefined>,
): string {
  const dict = dictionaries[locale] ?? ru;
  let text = dict[key] ?? dictionaries.en[key] ?? key;
  if (params) {
    for (const [k, v] of Object.entries(params)) {
      text = text.replaceAll(`{${k}}`, v == null ? "" : String(v));
    }
  }
  return text;
}

export function useT() {
  const locale = useAppStore((s) => s.locale);
  return useCallback(
    (key: string, params?: Record<string, string | number | null | undefined>) =>
      translate(locale, key, params),
    [locale],
  );
}

export function useLocale() {
  return useAppStore((s) => s.locale);
}
