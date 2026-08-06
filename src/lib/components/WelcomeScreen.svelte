<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { openTabs, activeTabPath } from '../stores';
  export let enterIDE: () => void;
  let step = 'main'; let selectedLang = ''; let isInstalled = false; let isInstalling = false; let installStatus = ''; let installProgress = 0;
  const langMap: Record<string, string> = { 'c': 'C', 'cpp': 'C++', 'rust': 'Rust', 'html': 'HTML' };

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
      enterIDE();
    } catch (e) { console.error(e); }
  }
  listen('compiler-status', (e) => installStatus = e.payload as string);
  listen('compiler-progress', (e) => installProgress = e.payload as number);
</script>
<div class="w-full h-full flex flex-col items-center justify-center bg-[#1e1e1e] text-[#cccccc]">
  {#if step === 'main'}
    <h1 class="text-5xl font-bold mb-10 text-white tracking-wide">ZenithIDE</h1>
    <div class="flex gap-4">
      <button class="bg-[#007acc] hover:bg-[#1f8ad2] text-white px-8 py-4 rounded text-lg font-semibold" on:click={() => step = 'select_lang'}>New File</button>
      <button class="bg-[#3c3c3c] hover:bg-[#4c4c4c] text-white px-8 py-4 rounded text-lg font-semibold" on:click={enterIDE}>Open Workspace</button>
    </div>
  {:else if step === 'select_lang'}
    <h2 class="text-2xl mb-8 text-white">Select Language</h2>
    <div class="grid grid-cols-2 gap-4">
      {#each Object.entries(langMap) as [lang, name]}
        <button class="bg-[#2d2d2d] hover:bg-[#3c3c3c] text-white px-10 py-5 rounded text-xl font-medium w-48" on:click={() => checkCompiler(lang)}>{name}</button>
      {/each}
    </div>
    <button class="mt-10 text-[#858585] hover:text-white" on:click={() => step = 'main'}>Back</button>
  {:else if step === 'setup_compiler'}
    <h2 class="text-2xl mb-8 text-white">Setup {langMap[selectedLang]}</h2>
    {#if isInstalling}
      <div class="w-96 flex flex-col items-center gap-3">
        <div class="w-full bg-[#3c3c3c] h-2 rounded-full overflow-hidden"><div class="bg-[#007acc] h-full transition-all duration-300" style="width: {installProgress}%"></div></div>
        <p class="text-sm text-[#858585]">{installStatus} {installProgress > 0 && installProgress < 100 ? `${installProgress}%` : ''}</p>
      </div>
    {:else if isInstalled}
      <p class="text-green-400 mb-8 text-lg">{selectedLang === 'html' ? 'No compiler needed for HTML.' : 'Compiler is ready!'}</p>
      <button class="bg-[#007acc] hover:bg-[#1f8ad2] text-white px-8 py-3 rounded font-semibold" on:click={createAndOpenFile}>Create & Open File</button>
    {:else}
      <p class="text-[#858585] mb-8 max-w-md text-center text-lg">Compiler not found. Please download it to compile and run code.</p>
      <button class="bg-[#007acc] hover:bg-[#1f8ad2] text-white px-8 py-3 rounded font-semibold mb-4" on:click={setupCompiler}>Download {langMap[selectedLang]} Compiler</button>
      <button class="text-[#858585] hover:text-white" on:click={createAndOpenFile}>Skip & Create File</button>
    {/if}
    <button class="mt-10 text-[#858585] hover:text-white" on:click={() => step = 'select_lang'}>Back</button>
  {/if}
</div>
