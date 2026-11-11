import type { DeviceStatus, DeviceType } from "@/types/device";
import { translate, type Locale } from "@/i18n";

export function formatVidPid(vid?: number | null, pid?: number | null): string {
  if (vid == null || pid == null) return "—";
  return `${vid.toString(16).padStart(4, "0").toUpperCase()}:${pid
    .toString(16)
    .padStart(4, "0")
    .toUpperCase()}`;
}

export function formatDeviceType(type: DeviceType, locale: Locale = "ru"): string {
  return translate(locale, `type.${type}`);
}

export function formatStatus(status: DeviceStatus, locale: Locale = "ru"): string {
  return translate(locale, `status.${status}`);
}

export function formatTime(iso?: string | null, locale: Locale = "ru"): string {
  if (!iso) return "—";
  const date = new Date(iso);
  if (Number.isNaN(date.getTime())) return iso;
  return date.toLocaleString(locale === "ru" ? "ru-RU" : "en-US");
}
