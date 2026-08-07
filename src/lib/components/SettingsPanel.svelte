<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-shell';
  import Icon from './icons.svelte';

  let version = '0.0.0';
  let updateStatus = 'Idle';
  let isChecking = false;
  let updateAvailable = false;
  let latestVersion = '';

  onMount(async () => {
    try {
      version = await invoke('get_app_version');
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

<!-- Đổi nền thành #252526 phẳng như VS Code Sidebar -->
<aside class="w-72 bg-[#252526] flex flex-col border-r border-black/40 overflow-y-auto">
  <div class="h-9 flex items-center px-4 text-[11px] uppercase tracking-wide text-[#bbbbbb] font-semibold">
    Settings
  </div>
  
  <div class="flex-1 flex flex-col">
    
    <!-- Phần About (Phẳng, kẻ viền dưới) -->
    <div class="px-4 py-3 border-b border-black/30">
      <h3 class="text-[11px] font-bold text-[#888] uppercase tracking-wider mb-2">About</h3>
      <div class="flex justify-between items-center text-[13px] text-[#cccccc]">
        <span>Version</span>
        <span class="font-mono text-[#4ec9b0]">v{version}</span>
      </div>
    </div>

    <!-- Phần Updates -->
    <div class="px-4 py-3 border-b border-black/30">
      <h3 class="text-[11px] font-bold text-[#888] uppercase tracking-wider mb-2">Updates</h3>
      <button 
        class="w-full bg-[#0e639c] hover:bg-[#1177bb] text-white py-1.5 rounded text-[13px] mb-2 transition-colors flex items-center justify-center gap-2 disabled:opacity-50"
        on:click={checkForUpdates}
        disabled={isChecking || updateAvailable}
      >
        {#if isChecking}<span class="w-3 h-3 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>{/if}
        {isChecking ? 'Checking...' : 'Check for Updates'}
      </button>

      {#if updateAvailable}
        <button 
          class="w-full bg-[#2d8848] hover:bg-[#359a54] text-white py-1.5 rounded text-[13px] mb-2 transition-colors flex items-center justify-center gap-2"
          on:click={downloadUpdate}
        >
          <Icon name="run" size={12} /> Open Download Page
        </button>
      {/if}

      <p class="text-[12px] text-[#888] mt-1">{updateStatus}</p>
    </div>

    <!-- Phần Editor -->
    <div class="px-4 py-3">
      <h3 class="text-[11px] font-bold text-[#888] uppercase tracking-wider mb-2">Editor</h3>
      <div class="flex flex-col gap-3 text-[13px] text-[#cccccc]">
        <div class="flex items-center justify-between">
          <span>Font Size</span>
          <input type="number" value="14" class="w-16 bg-[#3c3c3c] text-white px-2 py-1 rounded border border-transparent focus:border-[#007acc] focus:outline-none text-center" />
        </div>
        <div class="flex items-center justify-between">
          <span>Tab Size</span>
          <input type="number" value="2" class="w-16 bg-[#3c3c3c] text-white px-2 py-1 rounded border border-transparent focus:border-[#007acc] focus:outline-none text-center" />
        </div>
        <div class="flex items-center justify-between">
          <span>Word Wrap</span>
          <label class="relative inline-flex items-center cursor-pointer">
            <input type="checkbox" class="sr-only peer" />
            <div class="w-9 h-5 bg-[#3c3c3c] rounded-full peer peer-checked:after:translate-x-full peer-checked:bg-[#0e639c] after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all"></div>
          </label>
        </div>
      </div>
    </div>

  </div>
</aside>
