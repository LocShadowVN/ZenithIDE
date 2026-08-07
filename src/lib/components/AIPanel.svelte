<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Icon from './icons.svelte';
  import { lang } from '../i18n';

  let apiKey = ''; 
  let prompt = ''; 
  let response = ''; 
  let loading = false;
  let currentLang: 'en' | 'vi' = 'en';
  lang.subscribe((v: 'en' | 'vi') => currentLang = v);
  
  const dict: Record<string, { ask: string; placeholder: string }> = { 
    en: { ask: 'Ask AI', placeholder: 'Ask AI anything about code...' }, 
    vi: { ask: 'Hỏi AI', placeholder: 'Hỏi AI về code...' } 
  };
  $: t = dict[currentLang] || dict.en;

  async function ask() {
    if (!apiKey || !prompt) return;
    loading = true; 
    response = '';
    try {
      const res = await invoke<string>('ask_ai', {
        apiUrl: 'https://api.openai.com/v1/chat/completions',
        apiKey: apiKey,
        model: 'gpt-3.5-turbo',
        prompt: prompt
      });
      response = res;
    } catch (e) { 
      response = `Error: ${e}`; 
    }
    loading = false;
  }
</script>

<div class="h-full flex flex-col bg-[#1e1e1e] border-l border-black/40">
  <!-- Header -->
  <div class="h-10 flex items-center px-3 text-[11px] uppercase tracking-wide text-[#bbbbbb] font-semibold border-b border-black/40 bg-[#252526] gap-2">
    <Icon name="ai" size={14} /> {t.ask}
  </div>

  <!-- API Key Input (Collapsible look) -->
  <div class="p-3 border-b border-black/40 bg-[#252526]">
    <input 
      type="password" 
      bind:value={apiKey} 
      placeholder="Paste API Key (sk-...)" 
      class="w-full bg-[#1e1e1e] text-white px-3 py-2 text-xs rounded-md border border-white/10 focus:outline-none focus:border-blue-500 transition-colors" 
    />
  </div>

  <!-- Chat History Area -->
  <div class="flex-1 overflow-y-auto p-4 text-sm text-gray-300 whitespace-pre-wrap">
    {#if loading}
      <div class="flex items-center gap-2 text-blue-400">
        <span class="w-4 h-4 border-2 border-blue-400/30 border-t-blue-400 rounded-full animate-spin"></span>
        AI is thinking...
      </div>
    {:else if response}
      {response}
    {:else}
      <div class="text-center text-gray-600 mt-10">
        <Icon name="ai" size={32} />
        <p class="mt-2 text-xs">Your AI responses will appear here.</p>
      </div>
    {/if}
  </div>

  <!-- Prompt Input Area -->
  <div class="p-3 border-t border-black/40 bg-[#252526]">
    <textarea 
      bind:value={prompt} 
      placeholder={t.placeholder} 
      class="w-full h-24 bg-[#1e1e1e] text-white p-3 text-xs rounded-md border border-white/10 focus:outline-none focus:border-blue-500 resize-none transition-colors mb-2"
    ></textarea>
    <button 
      on:click={ask} 
      class="w-full bg-[#007acc] hover:bg-[#1f8ad2] text-white py-2 rounded-md text-xs font-semibold flex items-center justify-center gap-2 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
      disabled={loading || !apiKey || !prompt}
    >
      {#if loading}Waiting...{:else}<Icon name="ai" size={14} /> {t.ask}{/if}
    </button>
  </div>
</div>
