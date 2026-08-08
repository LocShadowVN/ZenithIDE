<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-shell';
  import Icon from './icons.svelte';

  let version = '0.0.0';
  let sysInfo = 'Loading...';
  let updateStatus = 'Idle';
  let isChecking = false;
  let updateAvailable = false;
  let latestVersion = '';

  onMount(async () => {
    try {
      version = await invoke('get_app_version');
      sysInfo = await invoke('get_system_info');
    } catch (e) {
      console.error(e);
    }
  });

  async function checkForUpdates() {
    isChecking = true;
    updateStatus = 'Checking...';
    try {
      const res = await fetch('https://api.github.com/repos/LocShadowVN/ZenithIDE/releases/latest');
      const data = await res.json();
      latestVersion = data.tag_name.replace('v', '');
      
      if (latestVersion !== version) {
        updateStatus = `Update v${latestVersion} found!`;
        updateAvailable = true;
      } else {
        updateStatus = 'You are using the latest version.';
      }
    } catch (e) {
      updateStatus = `Error: ${e}`;
    }
    isChecking = false;
  }

  async function downloadUpdate() {
    await open('https://github.com/LocShadowVN/ZenithIDE/releases/latest');
  }
</script>

<aside class="w-72 bg-[#1e293b] flex flex-col border-r border-[#0f172a] overflow-y-auto">
  <div class="h-10 flex items-center px-4 text-[11px] uppercase tracking-wide text-slate-400 font-semibold border-b border-[#0f172a]">
    Settings
  </div>
  
  <div class="p-4 flex flex-col gap-4">
    
    <div class="bg-[#0f172a] p-4 rounded-xl border border-slate-700/50">
      <h3 class="text-[11px] font-bold text-slate-500 uppercase tracking-wider mb-3">System Info</h3>
      <div class="flex flex-col gap-2 text-[13px] text-slate-300">
        <div class="flex justify-between">
          <span>Version</span>
          <span class="font-mono text-blue-400">v{version}</span>
        </div>
        <div class="flex justify-between">
          <span>OS</span>
          <span class="font-mono text-purple-400">{sysInfo}</span>
        </div>
      </div>
    </div>

    <div class="bg-[#0f172a] p-4 rounded-xl border border-slate-700/50">
      <h3 class="text-[11px] font-bold text-slate-500 uppercase tracking-wider mb-3">Updates</h3>
      <button 
        class="w-full modern-btn text-white py-2 rounded-lg text-[13px] mb-2 flex items-center justify-center gap-2 disabled:opacity-50"
        on:click={checkForUpdates}
        disabled={isChecking || updateAvailable}
      >
        {#if isChecking}<span class="w-3 h-3 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>{/if}
        {isChecking ? 'Checking...' : 'Check for Updates'}
      </button>

      {#if updateAvailable}
        <button 
          class="w-full bg-emerald-500 hover:bg-emerald-600 text-white py-2 rounded-lg text-[13px] mb-2 flex items-center justify-center gap-2"
          on:click={downloadUpdate}
        >
          <Icon name="run" size={12} /> Open Download Page
        </button>
      {/if}

      <p class="text-[12px] text-slate-500 mt-1 text-center">{updateStatus}</p>
    </div>

    <div class="bg-[#0f172a] p-4 rounded-xl border border-slate-700/50">
      <h3 class="text-[11px] font-bold text-slate-500 uppercase tracking-wider mb-3">Editor</h3>
      <div class="flex flex-col gap-3 text-[13px] text-slate-300">
        <div class="flex items-center justify-between">
          <span>Font Size</span>
          <input type="number" value="14" class="w-16 bg-[#334155] text-white px-2 py-1 rounded-md border border-transparent focus:border-blue-500 focus:outline-none text-center" />
        </div>
        <div class="flex items-center justify-between">
          <span>Tab Size</span>
          <input type="number" value="2" class="w-16 bg-[#334155] text-white px-2 py-1 rounded-md border border-transparent focus:border-blue-500 focus:outline-none text-center" />
        </div>
        <div class="flex items-center justify-between">
          <span>Word Wrap</span>
          <label class="relative inline-flex items-center cursor-pointer">
            <input type="checkbox" class="sr-only peer" />
            <div class="w-9 h-5 bg-[#334155] rounded-full peer peer-checked:after:translate-x-full peer-checked:bg-blue-500 after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all"></div>
          </label>
        </div>
      </div>
    </div>

  </div>
</aside>
