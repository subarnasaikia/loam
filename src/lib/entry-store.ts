import { isIsoDate } from "./types";
import { writeEntry, readEntry, listEntries } from "./commands";

export interface Entry {
  date: string;
  body: string;
}

export class EntryStore {
  async getEntry(date: string): Promise<Entry | null> {
    if (!isIsoDate(date)) {
      throw new Error(`invalid date: ${date}`);
    }
    const body = await readEntry(date);
    if (body === null) return null;
    return { date, body };
  }

  async saveEntry(date: string, body: string): Promise<string> {
    if (!isIsoDate(date)) {
      throw new Error(`invalid date: ${date}`);
    }
    return writeEntry(date, body);
  }

  async listEntries(): Promise<string[]> {
    return listEntries();
  }
}
