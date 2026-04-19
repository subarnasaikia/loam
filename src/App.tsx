import { useEffect, useState } from "react";
import { ensureLoamDir } from "./lib/commands";
import { EntryStore } from "./lib/entry-store";
import { loadAppSettings } from "./lib/settings";
import type { Settings } from "./lib/types";

const store = new EntryStore();

export default function App() {
  const [root, setRoot] = useState<string | null>(null);
  const [settings, setSettings] = useState<Settings | null>(null);
  const [entries, setEntries] = useState<string[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      try {
        const r = await ensureLoamDir();
        setRoot(r);
        const s = await loadAppSettings();
        setSettings(s);
        const list = await store.listEntries();
        setEntries(list);
      } catch (e) {
        setError(String(e));
      }
    })();
  }, []);

  return (
    <main
      style={{
        fontFamily: "Georgia, serif",
        padding: 48,
        color: "#3d2817",
        background: "#f4ecd8",
        minHeight: "100vh",
      }}
    >
      <h1>loam</h1>
      <p style={{ opacity: 0.6 }}>M1 — shell + storage</p>
      {error && <p style={{ color: "#a33" }}>error: {error}</p>}
      {root && (
        <section style={{ marginTop: 32 }}>
          <h3>root</h3>
          <code>{root}</code>
        </section>
      )}
      {settings && (
        <section style={{ marginTop: 24 }}>
          <h3>settings</h3>
          <pre style={{ fontSize: 12 }}>{JSON.stringify(settings, null, 2)}</pre>
        </section>
      )}
      {entries && (
        <section style={{ marginTop: 24 }}>
          <h3>entries ({entries.length})</h3>
          <ul>
            {entries.map((d) => (
              <li key={d}>{d}</li>
            ))}
          </ul>
        </section>
      )}
    </main>
  );
}
