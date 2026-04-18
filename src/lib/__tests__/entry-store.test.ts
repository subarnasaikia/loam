import { describe, it, expect, vi, beforeEach } from "vitest";

const mockInvoke = vi.fn();
vi.mock("@tauri-apps/api/core", () => ({
  invoke: (...args: unknown[]) => mockInvoke(...args),
}));

import { EntryStore } from "../entry-store";

describe("EntryStore", () => {
  beforeEach(() => {
    mockInvoke.mockReset();
  });

  it("getEntry returns null when no file exists", async () => {
    mockInvoke.mockResolvedValueOnce(null);
    const store = new EntryStore();
    const entry = await store.getEntry("2026-04-17");
    expect(entry).toBeNull();
  });

  it("getEntry returns { date, body } when file exists", async () => {
    mockInvoke.mockResolvedValueOnce("hello world");
    const store = new EntryStore();
    const entry = await store.getEntry("2026-04-17");
    expect(entry).toEqual({ date: "2026-04-17", body: "hello world" });
  });

  it("saveEntry rejects invalid date format", async () => {
    const store = new EntryStore();
    await expect(store.saveEntry("2026-4-17", "x")).rejects.toThrow(/invalid date/i);
  });

  it("saveEntry calls write_entry with correct args", async () => {
    mockInvoke.mockResolvedValueOnce("/path/2026-04-17.md");
    const store = new EntryStore();
    const path = await store.saveEntry("2026-04-17", "hi");
    expect(mockInvoke).toHaveBeenCalledWith("write_entry", {
      date: "2026-04-17",
      body: "hi",
    });
    expect(path).toBe("/path/2026-04-17.md");
  });

  it("listEntries returns sorted string array", async () => {
    mockInvoke.mockResolvedValueOnce(["2026-04-17", "2026-04-18"]);
    const store = new EntryStore();
    const list = await store.listEntries();
    expect(list).toEqual(["2026-04-17", "2026-04-18"]);
  });
});
