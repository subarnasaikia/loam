export type Aesthetic = "paper" | "nocturnal";
export type Classifier = "heuristic" | "llm";

export interface Settings {
  aesthetic: Aesthetic;
  typewriter_mode: boolean;
  distraction_free: boolean;
  ambient_sound: boolean;
  ambient_volume: number;
  classifier: Classifier;
  llm_model: string | null;
  prompt_packs_enabled: string[];
  loam_path: string | null;
  autosave_debounce_ms: number;
  onboarding_complete: boolean;
}

export const DEFAULT_SETTINGS: Settings = {
  aesthetic: "paper",
  typewriter_mode: true,
  distraction_free: true,
  ambient_sound: false,
  ambient_volume: 0.4,
  classifier: "heuristic",
  llm_model: null,
  prompt_packs_enabled: ["canon"],
  loam_path: null,
  autosave_debounce_ms: 300,
  onboarding_complete: false,
};

export const ISO_DATE = /^\d{4}-\d{2}-\d{2}$/;
export function isIsoDate(s: string): boolean {
  return ISO_DATE.test(s);
}
