<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import { openTabs, activeTabPath } from '../stores';
  export let enterIDE: () => void;
  
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
      enterIDE();
    } catch (e) { console.error(e); }
  }

  listen('compiler-status', (e) => installStatus = e.payload as string);
  listen('compiler-progress', (e) => installProgress = e.payload as number);
</script>

<div class="w-full h-full flex flex-col items-center justify-center bg-[#1a1b1e] text-[#d4d4d4] relative overflow-hidden">
  <!-- Hiệu ứng nền tròn ánh sáng -->
  <div class="absolute top-[-10%] left-[20%] w-[500px] h-[500px] bg-blue-600/10 rounded-full blur-[120px]"></div>
  <div class="absolute bottom-[-10%] right-[20%] w-[400px] h-[400px] bg-purple-600/10 rounded-full blur-[120px]"></div>

  <div class="z-10 flex flex-col items-center w-full max-w-md p-8 glass-panel rounded-2xl shadow-2xl">
    {#if step === 'main'}
      <div class="w-16 h-16 flex items-center justify-center rounded-2xl bg-gradient-to-br from-blue-500 to-purple-600 text-3xl font-bold text-white mb-6 shadow-lg">Z</div>
      <h1 class="text-3xl font-bold text-white mb-2 tracking-wide">ZenithIDE</h1>
      <p class="text-sm text-gray-400 mb-10">Code at the speed of thought.</p>
      
      <div class="flex flex-col gap-3 w-full">
        <button class="modern-btn text-white py-3 rounded-lg font-semibold" on:click={() => step = 'select_lang'}>New File</button>
        <button class="bg-white/5 hover:bg-white/10 text-white py-3 rounded-lg font-semibold transition-all" on:click={enterIDE}>Open Workspace</button>
      </div>
    {:else if step === 'select_lang'}
      <h2 class="text-xl font-bold text-white mb-6">Select Language</h2>
      <div class="grid grid-cols-2 gap-3 w-full">
        {#each Object.entries(langMap) as [lang, info]}
          <button class="bg-white/5 hover:bg-blue-600/20 border border-white/5 hover:border-blue-500/50 text-white py-6 rounded-xl font-medium transition-all flex flex-col items-center gap-2" on:click={() => checkCompiler(lang)}>
            <span class="text-2xl font-mono text-blue-400">{info.icon}</span>
            {info.name}
          </button>
        {/each}
      </div>
      <button class="mt-8 text-sm text-gray-400 hover:text-white transition" on:click={() => step = 'main'}>← Back</button>
    {:else if step === 'setup_compiler'}
      <h2 class="text-xl font-bold text-white mb-6">Setup {langMap[selectedLang].name}</h2>
      
      {#if isInstalling}
        <div class="w-full flex flex-col items-center gap-4 py-4">
          <div class="w-full bg-white/5 h-2 rounded-full overflow-hidden">
            <div class="bg-gradient-to-r from-blue-500 to-purple-500 h-full transition-all duration-300" style="width: {installProgress}%"></div>
          </div>
          <p class="text-sm text-blue-300">{installStatus} {installProgress > 0 && installProgress < 100 ? `${installProgress}%` : ''}</p>
        </div>
      {:else if isInstalled}
        <div class="flex flex-col items-center gap-6 py-4">
          <div class="w-16 h-16 flex items-center justify-center rounded-full bg-green-500/20 text-green-400 text-3xl">✓</div>
          <p class="text-white text-lg">{selectedLang === 'html' ? 'No compiler needed.' : 'Compiler is ready!'}</p>
          <button class="modern-btn text-white px-8 py-3 rounded-lg font-semibold w-full" on:click={createAndOpenFile}>Create & Open File</button>
        </div>
      {:else}
        <div class="flex flex-col items-center gap-6 py-4 text-center">
          <p class="text-gray-400 max-w-xs">Compiler not found. Please download it to compile and run code.</p>
          <button class="modern-btn text-white px-8 py-3 rounded-lg font-semibold w-full" on:click={setupCompiler}>Download {langMap[selectedLang].name} Compiler</button>
          <button class="text-sm text-gray-400 hover:text-white transition" on:click={createAndOpenFile}>Skip & Create File</button>
        </div>
      {/if}
      <button class="mt-8 text-sm text-gray-400 hover:text-white transition" on:click={() => step = 'select_lang'}>← Back</button>
    {/if}
  </div>
</div>
