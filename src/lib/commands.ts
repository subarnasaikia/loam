import { invoke } from "@tauri-apps/api/core";
import type { Settings } from "./types";

export async function ensureLoamDir(): Promise<string> {
  return invoke<string>("ensure_loam_dir");
}

export async function writeEntry(date: string, body: string): Promise<string> {
  return invoke<string>("write_entry", { date, body });
}

export async function readEntry(date: string): Promise<string | null> {
  return invoke<string | null>("read_entry", { date });
}

export async function listEntries(): Promise<string[]> {
  return invoke<string[]>("list_entries");
}

export async function loadSettings(): Promise<Settings> {
  return invoke<Settings>("load_settings");
}

export async function saveSettings(newSettings: Settings): Promise<void> {
  return invoke<void>("save_settings", { newSettings });
}
