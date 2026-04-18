import { describe, it, expect, vi, beforeEach } from "vitest";

const mockInvoke = vi.fn();

vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

import * as cmd from "../commands";
import { DEFAULT_SETTINGS } from "../types";

describe("commands", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  it("ensureLoamDir invokes ensure_loam_dir and returns root path", async () => {
    mockInvoke.mockResolvedValueOnce("/Users/me/Documents/Loam");
    const root = await cmd.ensureLoamDir();
    expect(mockInvoke).toHaveBeenCalledWith("ensure_loam_dir");
    expect(root).toBe("/Users/me/Documents/Loam");
  });

  it("writeEntry passes date and body", async () => {
    mockInvoke.mockResolvedValueOnce("/path/2026-04-17.md");
    const p = await cmd.writeEntry("2026-04-17", "hello");
    expect(mockInvoke).toHaveBeenCalledWith("write_entry", {
      date: "2026-04-17",
      body: "hello",
    });
    expect(p).toBe("/path/2026-04-17.md");
  });

  it("readEntry returns null for missing entry", async () => {
    mockInvoke.mockResolvedValueOnce(null);
    const body = await cmd.readEntry("2026-04-17");
    expect(body).toBeNull();
  });

  it("readEntry returns body string when present", async () => {
    mockInvoke.mockResolvedValueOnce("hello");
    const body = await cmd.readEntry("2026-04-17");
    expect(body).toBe("hello");
  });

  it("listEntries returns string array", async () => {
    mockInvoke.mockResolvedValueOnce(["2026-04-17", "2026-04-18"]);
    const list = await cmd.listEntries();
    expect(list).toEqual(["2026-04-17", "2026-04-18"]);
  });

  it("loadSettings returns parsed Settings", async () => {
    mockInvoke.mockResolvedValueOnce(DEFAULT_SETTINGS);
    const s = await cmd.loadSettings();
    expect(s).toEqual(DEFAULT_SETTINGS);
  });

  it("saveSettings passes new_settings arg", async () => {
    mockInvoke.mockResolvedValueOnce(undefined);
    await cmd.saveSettings(DEFAULT_SETTINGS);
    expect(mockInvoke).toHaveBeenCalledWith("save_settings", {
      newSettings: DEFAULT_SETTINGS,
    });
  });
});
