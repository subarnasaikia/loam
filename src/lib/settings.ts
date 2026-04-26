import { loadSettings as loadCmd, saveSettings as saveCmd } from "./commands";
import type { Settings } from "./types";

export async function loadAppSettings(): Promise<Settings> {
  return loadCmd();
}

// Callers must load current settings first, apply overrides, then pass the
// full Settings object — avoids silently resetting unspecified fields to defaults.
export async function saveAppSettings(s: Settings): Promise<void> {
  return saveCmd(s);
}
