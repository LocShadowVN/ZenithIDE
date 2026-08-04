<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Icon from './icons.svelte';
  import { lang } from '../i18n';

  let apiKey = ''; let prompt = ''; let response = ''; let loading = false;
  let currentLang: string; lang.subscribe(v => currentLang = v);
  const dict = { en: { ask: 'Ask AI', placeholder: 'Ask AI anything about code...' }, vi: { ask: 'Hỏi AI', placeholder: 'Hỏi AI về code...' } };
  $: t = dict[currentLang] || dict.en;

  async function ask() {
    if (!apiKey || !prompt) return;
    loading = true; response = '';
    try {
      const res = await invoke<string>('ask_ai', {
        apiUrl: 'https://api.openai.com/v1/chat/completions',
        apiKey: apiKey, model: 'gpt-3.5-turbo', prompt: prompt
      });
      response = res;
    } catch (e) { response = `Error: ${e}`; }
    loading = false;
  }
</script>
<div class="h-full flex flex-col bg-[#252526] border-l border-black/40">
  <div class="h-9 flex items-center px-3 text-[11px] uppercase tracking-wide text-[#bbbbbb] font-semibold border-b border-black/40 flex items-center gap-2">
    <Icon name="ai" size={14} /> {t.ask}
  </div>
  <div class="p-2 border-b border-black/40">
    <input type="password" bind:value={apiKey} placeholder="API Key" class="w-full bg-[#1e1e1e] text-white px-2 py-1 text-xs border border-black/50 focus:outline-none focus:border-blue-500" />
  </div>
  <div class="flex-1 overflow-y-auto p-2 text-xs text-gray-300 whitespace-pre-wrap">
    {loading ? 'Thinking...' : response}
  </div>
  <div class="p-2 border-t border-black/40">
    <textarea bind:value={prompt} placeholder={t.placeholder} class="w-full h-20 bg-[#1e1e1e] text-white p-2 text-xs border border-black/50 focus:outline-none focus:border-blue-500 resize-none"></textarea>
    <button on:click={ask} class="w-full mt-1 bg-blue-600 hover:bg-blue-700 text-white py-1 text-xs font-semibold">{t.ask}</button>
  </div>
</div>
