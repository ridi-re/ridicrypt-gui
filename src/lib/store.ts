import { LazyStore } from "@tauri-apps/plugin-store";

declare global {
    var __APP_STORE__: LazyStore | undefined;
}

const instance =
    globalThis.__APP_STORE__ ??
    (globalThis.__APP_STORE__ = new LazyStore("config.json", {
        autoSave: true,
        defaults: {},
    }));

export async function get<T>(key: string): Promise<T | null> {
    return (await instance.get<T>(key)) ?? null;
}

export async function set<T>(key: string, value: T): Promise<void> {
    await instance.set(key, value);
}

export function save(): Promise<void> {
    return instance.save();
}

export { instance as store };
