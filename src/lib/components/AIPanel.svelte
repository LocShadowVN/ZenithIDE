<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Icon from './icons.svelte';
  import { editorContent } from '../stores';
  import { lang } from '../i18n';

  let apiKey = ''; 
  let prompt = ''; 
  let response = ''; 
  let loading = false;
  let includeCode = true; // Tự động gửi kèm code
  
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
    
    let finalPrompt = prompt;
    if (includeCode && $editorContent) {
      finalPrompt = `${prompt}\n\nCurrent Code:\n\`\`\`\n${$editorContent}\n\`\`\``;
    }

    try {
      const res = await invoke<string>('ask_ai', {
        apiUrl: 'https://api.openai.com/v1/chat/completions',
        apiKey: apiKey,
        model: 'gpt-3.5-turbo',
        prompt: finalPrompt
      });
      response = res;
    } catch (e) { 
      response = `Error: ${e}`; 
    }
    loading = false;
  }
</script>

<div class="h-full flex flex-col bg-[#0f172a] border-l border-[#1e293b]">
  <!-- Header -->
  <div class="h-10 flex items-center px-3 text-[11px] uppercase tracking-wide text-slate-400 font-semibold border-b border-[#1e293b] bg-[#1e293b] gap-2">
    <Icon name="ai" size={14} /> {t.ask}
  </div>

  <!-- API Key Input -->
  <div class="p-3 border-b border-[#1e293b] bg-[#1e293b]">
    <input 
      type="password" 
      bind:value={apiKey} 
      placeholder="Paste API Key (sk-...)" 
      class="w-full bg-[#0f172a] text-white px-3 py-2 text-xs rounded-md border border-slate-700 focus:outline-none focus:border-blue-500 transition-colors" 
    />
  </div>

  <!-- Chat History Area -->
  <div class="flex-1 overflow-y-auto p-4 text-sm text-slate-300 whitespace-pre-wrap">
    {#if loading}
      <div class="flex items-center gap-2 text-blue-400">
        <span class="w-4 h-4 border-2 border-blue-400/30 border-t-blue-400 rounded-full animate-spin"></span>
        AI is thinking...
      </div>
    {:else if response}
      {response}
    {:else}
      <div class="text-center text-slate-600 mt-10">
        <Icon name="ai" size={32} />
        <p class="mt-2 text-xs">Your AI responses will appear here.</p>
      </div>
    {/if}
  </div>

  <!-- Prompt Input Area -->
  <div class="p-3 border-t border-[#1e293b] bg-[#1e293b]">
    <div class="flex items-center mb-2">
      <label class="flex items-center gap-2 text-xs text-slate-400 cursor-pointer">
        <input type="checkbox" bind:checked={includeCode} class="accent-blue-500 rounded" />
        Include current code
      </label>
    </div>
    <textarea 
      bind:value={prompt} 
      placeholder={t.placeholder} 
      class="w-full h-24 bg-[#0f172a] text-white p-3 text-xs rounded-md border border-slate-700 focus:outline-none focus:border-blue-500 resize-none transition-colors mb-2"
    ></textarea>
    <button 
      on:click={ask} 
      class="w-full modern-btn text-white py-2 rounded-md text-xs font-semibold flex items-center justify-center gap-2 disabled:opacity-50"
      disabled={loading || !apiKey || !prompt}
    >
      {#if loading}Waiting...{:else}<Icon name="ai" size={14} /> {t.ask}{/if}
    </button>
  </div>
</div>
