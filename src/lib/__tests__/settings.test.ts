import { describe, it, expect, vi, beforeEach } from "vitest";

const mockInvoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

import { loadAppSettings, saveAppSettings } from "../settings";
import { DEFAULT_SETTINGS } from "../types";

describe("settings module", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  it("loadAppSettings returns Rust-provided settings", async () => {
    mockInvoke.mockResolvedValueOnce({ ...DEFAULT_SETTINGS, aesthetic: "nocturnal" });
    const s = await loadAppSettings();
    expect(s.aesthetic).toBe("nocturnal");
    expect(s.typewriter_mode).toBe(true);
  });

  it("saveAppSettings passes through to save_settings command", async () => {
    mockInvoke.mockResolvedValueOnce(undefined);
    const next = { ...DEFAULT_SETTINGS, onboarding_complete: true };
    await saveAppSettings(next);
    expect(mockInvoke).toHaveBeenCalledWith("save_settings", { newSettings: next });
  });

  it("saveAppSettings merges partial updates with defaults", async () => {
    mockInvoke.mockResolvedValueOnce(undefined);
    await saveAppSettings({ aesthetic: "nocturnal" });
    const call = mockInvoke.mock.calls[0];
    expect(call[0]).toBe("save_settings");
    expect(call[1].newSettings.aesthetic).toBe("nocturnal");
    expect(call[1].newSettings.typewriter_mode).toBe(true);
  });
});
