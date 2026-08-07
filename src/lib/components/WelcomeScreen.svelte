<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { openTabs, activeTabPath } from '../stores';
  
  let step = 'main'; 
  let selectedLang = ''; 
  let isInstalled = false; 
  let isInstalling = false; 
  let installStatus = ''; 
  let installProgress = 0;
  
  const langMap: Record<string, { name: string; icon: string }> = { 
    'c': { name: 'C', icon: 'C' }, 
    'cpp': { name: 'C++', icon: 'C++' }, 
    'rust': { name: 'Rust', icon: 'R' }, 
    'html': { name: 'HTML', icon: '<>' } 
  };

  async function checkCompiler(lang: string) {
    selectedLang = lang;
    if (lang === 'html') { isInstalled = true; step = 'setup_compiler'; return; }
    try { await invoke('get_compiler_path', { lang }); isInstalled = true; } catch (e) { isInstalled = false; }
    step = 'setup_compiler';
  }

  async function setupCompiler() {
    isInstalling = true;
    try { await invoke('install_compiler', { lang: selectedLang }); isInstalled = true; } catch (e) { installStatus = `Error: ${e}`; }
    isInstalling = false;
  }

  async function createAndOpenFile() {
    try {
      const path = await invoke<string>('create_new_file', { lang: selectedLang });
      const content = await invoke<string>('read_file', { path });
      const name = path.split(/[\\/]/).pop() || 'untitled';
      openTabs.update(tabs => { if (!tabs.find(t => t.path === path)) tabs.push({ path, name, content, language: selectedLang }); return tabs; });
      activeTabPath.set(path);
    } catch (e) { console.error(e); }
  }

  listen('compiler-status', (e) => installStatus = e.payload as string);
  listen('compiler-progress', (e) => installProgress = e.payload as number);
</script>

<div class="w-full h-full flex bg-[#1e1e1e] text-[#cccccc]">
  <!-- Cột trái: Các nút bấm -->
  <div class="flex-1 p-12 overflow-y-auto">
    <h1 class="text-5xl font-extralight mb-2 text-white">ZenithIDE</h1>
    <p class="text-sm text-[#888] mb-12">Editing evolved</p>
    
    {#if step === 'main'}
      <h2 class="text-sm font-semibold uppercase text-[#888] mb-4">Start</h2>
      <div class="flex flex-col gap-3 text-sm">
        <button class="text-[#3794ff] hover:underline text-left w-fit" on:click={() => step = 'select_lang'}>New File...</button>
      </div>
    {:else if step === 'select_lang'}
      <h2 class="text-sm font-semibold uppercase text-[#888] mb-4">Select Language</h2>
      <div class="flex flex-wrap gap-4">
        {#each Object.entries(langMap) as [lang, info]}
          <button class="w-32 h-32 bg-[#252526] hover:bg-[#2d2d2d] border border-transparent hover:border-[#3794ff] text-white rounded-lg flex flex-col items-center justify-center gap-2 transition-all" on:click={() => checkCompiler(lang)}>
            <span class="text-3xl font-mono text-[#3794ff]">{info.icon}</span>
            <span class="text-sm">{info.name}</span>
          </button>
        {/each}
      </div>
      <button class="mt-8 text-sm text-[#888] hover:text-white" on:click={() => step = 'main'}>← Back</button>
    {:else if step === 'setup_compiler'}
      <h2 class="text-sm font-semibold uppercase text-[#888] mb-4">Setup {langMap[selectedLang].name}</h2>
      {#if isInstalling}
        <div class="w-80 flex flex-col gap-3">
          <div class="w-full bg-[#3c3c3c] h-1 rounded-full overflow-hidden">
            <div class="bg-[#3794ff] h-full transition-all" style="width: {installProgress}%"></div>
          </div>
          <p class="text-sm text-[#888]">{installStatus} {installProgress > 0 && installProgress < 100 ? `${installProgress}%` : ''}</p>
        </div>
      {:else if isInstalled}
        <p class="text-green-400 mb-6 text-sm">{selectedLang === 'html' ? 'No compiler needed.' : 'Compiler is ready!'}</p>
        <button class="bg-[#0e639c] hover:bg-[#1177bb] text-white px-4 py-2 rounded text-sm w-fit" on:click={createAndOpenFile}>Create & Open File</button>
      {:else}
        <p class="text-[#888] mb-6 max-w-sm text-sm">Compiler not found. Please download it to compile and run code.</p>
        <button class="bg-[#0e639c] hover:bg-[#1177bb] text-white px-4 py-2 rounded text-sm w-fit mb-3" on:click={setupCompiler}>Download {langMap[selectedLang].name} Compiler</button>
        <button class="text-[#3794ff] hover:underline text-sm" on:click={createAndOpenFile}>Skip & Create File</button>
      {/if}
      <button class="mt-8 text-sm text-[#888] hover:text-white" on:click={() => step = 'select_lang'}>← Back</button>
    {/if}
  </div>

  <!-- Cột phải: Logo và thông tin -->
  <div class="w-1/3 flex flex-col items-center justify-center bg-[#252526] border-l border-black/40 p-10">
    <!-- Logo Z -->
    <svg width="120" height="120" viewBox="0 0 100 100" class="mb-6">
      <rect width="100" height="100" rx="20" fill="#1e1e1e" stroke="#3c3c3c" stroke-width="2"/>
      <path d="M30 30 H70 L35 70 H70" stroke="#3794ff" stroke-width="8" fill="none" stroke-linecap="round" stroke-linejoin="round"/>
    </svg>
    <h2 class="text-xl font-light text-white mb-2">ZenithIDE</h2>
    <p class="text-xs text-[#888] text-center">Powered by Rust & SvelteKit</p>
  </div>
</div>
