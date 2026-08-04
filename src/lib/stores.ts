import { writable } from 'svelte/store';

export interface FileTab {
    path: string; name: string; content: string; language: string;
}

export const openTabs = writable<FileTab[]>([]);
export const activeTabPath = writable<string | null>(null);
