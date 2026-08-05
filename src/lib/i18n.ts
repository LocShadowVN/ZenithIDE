import { writable } from 'svelte/store';
export const lang = writable<'en' | 'vi'>('en');

interface TranslationDict {
  [key: string]: { [key: string]: string };
}

const translations: TranslationDict = {
  en: { explorer: 'Explorer', search: 'Search', git: 'Git', settings: 'Settings', terminal: 'Terminal', ai: 'AI Assistant', run: 'Run' },
  vi: { explorer: 'Trình duyệt', search: 'Tìm kiếm', git: 'Git', settings: 'Cài đặt', terminal: 'Terminal', ai: 'Trợ lý AI', run: 'Chạy' }
};

export function t(key: string, currentLang: string) {
  return translations[currentLang]?.[key] || translations['en'][key] || key;
}
