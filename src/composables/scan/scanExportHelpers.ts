import { invoke, invokeRaw } from "@/utils/invoke";
import { useNotificationStore } from "@/stores/notifications";

export { invoke, invokeRaw, useNotificationStore };

export function kbStr(v: number): string {
  return v >= 1024 ? `${(v / 1024).toFixed(0)} MB` : `${v} KB`;
}

export function fullRegPath(location: string, name?: string): string {
  let p = location
    .replace(/^HKCU(\\|$)/, "HKEY_CURRENT_USER$1")
    .replace(/^HKLM(\\|$)/, "HKEY_LOCAL_MACHINE$1")
    .replace(/^HKCR(\\|$)/, "HKEY_CLASSES_ROOT$1")
    .replace(/^HKU(\\|$)/, "HKEY_USERS$1");
  if (name) p = p + (p.endsWith("\\") ? "" : "\\") + name;
  return p;
}

export interface Solution {
  problem: string;
  action: string;
  repairKey?: string;
  severity: "critical" | "warning" | "info";
}
