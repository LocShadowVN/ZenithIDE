<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import ActivityBar from '$lib/components/ActivityBar.svelte';
  import Sidebar from '$lib/components/Sidebar.svelte';
  import EditorArea from '$lib/components/EditorArea.svelte';
  import Terminal from '$lib/components/Terminal.svelte';
  import AIPanel from '$lib/components/AIPanel.svelte';
  import Icon from '$lib/components/icons.svelte';
  import { lang } from '$lib/i18n';
  import { activeTabPath, openTabs } from '$lib/stores';

  let activeView = 'files';
  let showTerminal = true;
  let showAI = false;
  let workspacePath = './';
  
  let currentLang: 'en' | 'vi' = 'en';
  lang.subscribe((v: 'en' | 'vi') => currentLang = v);

  let isInstalling = false;
  let installStatus = '';
  let installProgress = 0;

  const dict: Record<string, { terminal: string; run: string }> = {
    en: { terminal: 'Terminal', run: 'Run' },
    vi: { terminal: 'Terminal', run: 'Chạy' }
  };
  $: t = dict[currentLang] || dict.en;

  onMount(async () => { 
    workspacePath = await invoke('get_default_workspace'); 
    await listen('compiler-status', (e) => installStatus = e.payload as string);
    await listen('compiler-progress', (e) => installProgress = e.payload as number);
  });

  async function setupCompiler(lang: string) {
    isInstalling = true;
    try {
      await invoke('install_compiler', { lang });
    } catch (e) {
      installStatus = `Error: ${e}`;
    }
    isInstalling = false;
  }

  async function runCode() {
    showTerminal = true;
    const tab = $openTabs.find(t => t.path === $activeTabPath);
    if (!tab) return;
    
    let cmd = '';
    const isWindows = navigator.userAgent.includes('Windows');
    const execExt = isWindows ? '.exe' : '';
    const outputFile = `zenith_run${execExt}`;
    const fileName = tab.name;
    
    try {
      if (tab.path.endsWith('.c')) {
        const gcc = await invoke<string>('get_compiler_path', { lang: 'c' });
        cmd = `"${gcc}" "${fileName}" -o "${outputFile}" && ./"${outputFile}"\n`;
      } else if (tab.path.endsWith('.cpp')) {
        const gpp = await invoke<string>('get_compiler_path', { lang: 'cpp' });
        cmd = `"${gpp}" "${fileName}" -o "${outputFile}" && ./"${outputFile}"\n`;
      } else if (tab.path.endsWith('.rs')) {
        const rustc = await invoke<string>('get_compiler_path', { lang: 'rust' });
        cmd = `"${rustc}" "${fileName}" -o "${outputFile}" && ./"${outputFile}"\n`;
      } else if (tab.path.endsWith('.html')) {
        if (isWindows) cmd = `start "" "${fileName}"\n`;
        else if (navigator.userAgent.includes('Mac')) cmd = `open "${fileName}"\n`;
        else cmd = `xdg-open "${fileName}"\n`;
      } else {
        cmd = `echo "Unsupported file type for compilation."\n`;
      }
    } catch (e: any) {
      if (String(e).includes("Not Installed")) {
        cmd = `echo "Compiler not found. Please click the Setup button."\n`;
      } else {
        cmd = `echo "Compiler not found: ${e}"\n`;
      }
    }

    invoke('write_to_pty', { id: 1, data: cmd });
  }
</script>

<main class="flex h-screen w-screen overflow-hidden">
  <ActivityBar bind:activeTab={activeView} setActive={(v) => activeView = v} />
  
  <div class="absolute top-1 right-40 z-50 flex items-center gap-2">
    <select bind:value={$lang} class="bg-[#3c3c3c] text-white text-xs px-1 py-0.5 border border-black/50 focus:outline-none">
      <option value="en">EN</option>
      <option value="vi">VI</option>
    </select>
  </div>

  {#if activeView === 'files'}
    <Sidebar {workspacePath} />
  {/if}

  <div class="flex-1 flex flex-col">
    <div class="flex-1 flex overflow-hidden">
      <div class="flex-1 flex flex-col">
        <div class="h-9 bg-[#252526] flex items-center px-2 border-b border-black/40 justify-between">
          <div class="flex items-center gap-2 text-xs text-gray-400">
            {#if isInstalling}
              <span class="text-blue-400">{installStatus} {installProgress > 0 && installProgress < 100 ? `${installProgress}%` : ''}</span>
            {:else}
              <button class="bg-[#2d2d2d] hover:bg-[#3c3c3c] text-white px-2 py-1 rounded" on:click={() => setupCompiler('c')}>Setup C/C++</button>
              <button class="bg-[#2d2d2d] hover:bg-[#3c3c3c] text-white px-2 py-1 rounded" on:click={() => setupCompiler('rust')}>Setup Rust</button>
            {/if}
          </div>
          <button class="flex items-center gap-1 bg-[#007acc] hover:bg-[#1f8ad2] text-white px-3 py-1 text-xs rounded" on:click={runCode}>
            <Icon name="run" size={12} /> {t.run}
          </button>
        </div>
        <div class="flex-1 overflow-hidden">
          <EditorArea />
        </div>
      </div>
      
      {#if showAI}
        <div class="w-80 flex flex-col">
          <AIPanel />
        </div>
      {/if}
    </div>

    {#if showTerminal}
      <div class="h-[35%] flex flex-col border-t border-black/40 bg-[#1e1e1e]">
        <div class="h-9 flex items-center justify-between px-3 bg-[#252526] border-b border-black/40">
          <div class="flex items-center gap-4">
            <button class="text-[11px] uppercase tracking-wide text-white font-semibold flex items-center gap-2 border-b-2 border-blue-500 py-2" on:click={() => showTerminal = true}>
              <Icon name="terminal" size={14} /> {t.terminal}
            </button>
            <button class="text-[11px] uppercase tracking-wide text-[#858585] hover:text-white flex items-center gap-2 py-2" on:click={() => { showAI = !showAI; }}>
              <Icon name="ai" size={14} /> AI
            </button>
          </div>
          <button class="text-[#969696] hover:text-white p-1" on:click={() => showTerminal = false}><Icon name="close" size={14} /></button>
        </div>
        <div class="flex-1 overflow-hidden">
          <Terminal cwd={workspacePath} />
        </div>
      </div>
    {:else}
      <button class="absolute bottom-2 right-4 bg-[#252526] border border-black/40 text-[#cccccc] px-3 py-1 text-xs hover:bg-[#333333] flex items-center gap-1" on:click={() => showTerminal = true}>
        <Icon name="terminal" size={12} /> {t.terminal}
      </button>
    {/if}
  </div>
</main>
