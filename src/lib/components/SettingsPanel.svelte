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

<!-- Đổi sang w-full h-full để chiếm chỗ Editor -->
<div class="w-full h-full bg-[#0f172a] flex flex-col overflow-y-auto">
  <div class="h-10 flex items-center px-4 text-[11px] uppercase tracking-wide text-slate-400 font-semibold border-b border-[#1e293b] bg-[#1e293b]">
    Settings
  </div>
  
  <div class="flex-1 flex flex-col items-center py-10">
    <div class="w-full max-w-2xl flex flex-col gap-6 px-6">
      
      <div class="bg-[#1e293b] p-6 rounded-2xl border border-slate-700/50 shadow-lg">
        <h3 class="text-sm font-bold text-slate-400 uppercase tracking-wider mb-4">System Info</h3>
        <div class="flex flex-col gap-3 text-[14px] text-slate-300">
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

      <div class="bg-[#1e293b] p-6 rounded-2xl border border-slate-700/50 shadow-lg">
        <h3 class="text-sm font-bold text-slate-400 uppercase tracking-wider mb-4">Updates</h3>
        <button 
          class="w-full modern-btn text-white py-2 rounded-lg text-[14px] mb-2 flex items-center justify-center gap-2 disabled:opacity-50"
          on:click={checkForUpdates}
          disabled={isChecking || updateAvailable}
        >
          {#if isChecking}<span class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>{/if}
          {isChecking ? 'Checking...' : 'Check for Updates'}
        </button>

        {#if updateAvailable}
          <button 
            class="w-full bg-emerald-500 hover:bg-emerald-600 text-white py-2 rounded-lg text-[14px] mb-2 flex items-center justify-center gap-2"
            on:click={downloadUpdate}
          >
            <Icon name="run" size={14} /> Open Download Page
          </button>
        {/if}

        <p class="text-[13px] text-slate-500 mt-2 text-center">{updateStatus}</p>
      </div>

      <div class="bg-[#1e293b] p-6 rounded-2xl border border-slate-700/50 shadow-lg">
        <h3 class="text-sm font-bold text-slate-400 uppercase tracking-wider mb-4">Editor</h3>
        <div class="flex flex-col gap-4 text-[14px] text-slate-300">
          <div class="flex items-center justify-between">
            <span>Font Size</span>
            <input type="number" value="14" class="w-20 bg-[#334155] text-white px-3 py-1.5 rounded-md border border-transparent focus:border-blue-500 focus:outline-none text-center" />
          </div>
          <div class="flex items-center justify-between">
            <span>Tab Size</span>
            <input type="number" value="2" class="w-20 bg-[#334155] text-white px-3 py-1.5 rounded-md border border-transparent focus:border-blue-500 focus:outline-none text-center" />
          </div>
          <div class="flex items-center justify-between">
            <span>Word Wrap</span>
            <label class="relative inline-flex items-center cursor-pointer">
              <input type="checkbox" class="sr-only peer" />
              <div class="w-11 h-6 bg-[#334155] rounded-full peer peer-checked:after:translate-x-full peer-checked:bg-blue-500 after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-5 after:w-5 after:transition-all"></div>
            </label>
          </div>
        </div>
      </div>

    </div>
  </div>
</div>
