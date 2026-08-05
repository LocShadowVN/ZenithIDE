<script lang="ts">
  import Icon from './icons.svelte';
  import { lang } from '../i18n';
  
  export let activeTab: string;
  export let setActive: (tab: string) => void;
  
  let currentLang: 'en' | 'vi' = 'en';
  lang.subscribe((v: 'en' | 'vi') => currentLang = v);

  const dict: Record<string, { explorer: string; search: string; git: string; settings: string }> = {
    en: { explorer: 'Explorer', search: 'Search', git: 'Git', settings: 'Settings' },
    vi: { explorer: 'Trình duyệt', search: 'Tìm kiếm', git: 'Git', settings: 'Cài đặt' }
  };
  $: t = dict[currentLang] || dict.en;
</script>

<nav class="w-12 bg-[#333333] flex flex-col items-center py-2 gap-1 border-r border-black/40">
  <button class="p-3 hover:text-white {activeTab === 'files' ? 'text-white' : 'text-[#858585]'}" on:click={() => setActive('files')} title={t.explorer}><Icon name="files" size={24} /></button>
  <button class="p-3 hover:text-white text-[#858585]" on:click={() => setActive('search')} title={t.search}><Icon name="search" size={24} /></button>
  <button class="p-3 hover:text-white text-[#858585]" on:click={() => setActive('git')} title={t.git}><Icon name="git" size={24} /></button>
  <div class="flex-1"></div>
  <button class="p-3 hover:text-white text-[#858585]" on:click={() => setActive('settings')} title={t.settings}><Icon name="settings" size={24} /></button>
</nav>
