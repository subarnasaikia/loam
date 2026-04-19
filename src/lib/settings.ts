import { loadSettings as loadCmd, saveSettings as saveCmd } from "./commands";
import { DEFAULT_SETTINGS, type Settings } from "./types";

export async function loadAppSettings(): Promise<Settings> {
  return loadCmd();
}

export async function saveAppSettings(partial: Partial<Settings>): Promise<void> {
  const merged: Settings = { ...DEFAULT_SETTINGS, ...partial };
  return saveCmd(merged);
}
