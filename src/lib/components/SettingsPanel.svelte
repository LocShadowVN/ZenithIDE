<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { check } from '@tauri-apps/plugin-updater';
  import { relaunch } from '@tauri-apps/plugin-process';
  import Icon from './icons.svelte';

  let version = '0.0.0';
  let updateStatus = 'Idle';
  let isChecking = false;
  let updateAvailable = false;
  let updateProgress = 0;

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
      const update = await check();
      if (update) {
        updateStatus = `Update v${update.version} found!`;
        updateAvailable = true;
      } else {
        updateStatus = 'You are using the latest version.';
      }
    } catch (e) {
      updateStatus = `Error: ${e}`;
    }
    isChecking = false;
  }

  async function installUpdate() {
    updateStatus = 'Downloading...';
    try {
      const update = await check();
      if (update) {
        let downloaded = 0;
        let total = 0;
        await update.downloadAndInstall((event: any) => {
          switch (event.event) {
            case 'Started':
              total = event.data.contentLength || 0;
              break;
            case 'Progress':
              downloaded += event.data.chunkLength;
              updateProgress = Math.round((downloaded / total) * 100);
              updateStatus = `Downloading... ${updateProgress}%`;
              break;
            case 'Finished':
              updateStatus = 'Installing...';
              break;
          }
        });
        await relaunch();
      }
    } catch (e) {
      updateStatus = `Error: ${e}`;
    }
  }
</script>

<aside class="w-80 bg-[#1e1e1e] flex flex-col border-r border-black/40 overflow-y-auto">
  <div class="h-10 flex items-center px-4 text-[11px] uppercase tracking-wide text-[#bbbbbb] font-semibold border-b border-black/40 bg-[#252526]">
    Settings
  </div>
  
  <div class="p-4 flex flex-col gap-6">
    
    <!-- About Section -->
    <div class="bg-[#252526] p-4 rounded-lg border border-white/5">
      <h3 class="text-xs font-bold text-[#8a8a8a] uppercase tracking-wider mb-3">About</h3>
      <div class="text-xs text-gray-300 flex justify-between items-center">
        <span>Version</span>
        <span class="font-mono text-blue-400 bg-[#1e1e1e] px-2 py-1 rounded">v{version}</span>
      </div>
    </div>

    <!-- Update Section -->
    <div class="bg-[#252526] p-4 rounded-lg border border-white/5">
      <h3 class="text-xs font-bold text-[#8a8a8a] uppercase tracking-wider mb-3">Updates</h3>
      <button 
        class="w-full bg-[#0d6efd] hover:bg-[#0b5ed7] text-white py-2 rounded-md text-xs font-semibold mb-2 flex items-center justify-center gap-2 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
        on:click={checkForUpdates}
        disabled={isChecking || updateAvailable}
      >
        {#if isChecking}<span class="w-3 h-3 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>{/if}
        {#if isChecking}Checking...{:else}Check for Updates{/if}
      </button>

      {#if updateAvailable}
        <button 
          class="w-full bg-green-600 hover:bg-green-700 text-white py-2 rounded-md text-xs font-semibold mb-2 flex items-center justify-center gap-2 transition-colors"
          on:click={installUpdate}
        >
          <Icon name="run" size={14} /> Download & Install
        </button>
      {/if}

      <p class="text-xs text-gray-500 mt-2 text-center">{updateStatus}</p>
      {#if updateProgress > 0 && updateProgress < 100}
        <div class="w-full bg-[#1e1e1e] h-1.5 rounded-full mt-2 overflow-hidden">
          <div class="bg-blue-500 h-full transition-all" style="width: {updateProgress}%"></div>
        </div>
      {/if}
    </div>

    <!-- Editor Settings -->
    <div class="bg-[#252526] p-4 rounded-lg border border-white/5">
      <h3 class="text-xs font-bold text-[#8a8a8a] uppercase tracking-wider mb-3">Editor</h3>
      <div class="flex flex-col gap-4 text-xs text-gray-300">
        <div class="flex items-center justify-between">
          <span>Font Size</span>
          <input type="number" value="14" class="w-16 bg-[#1e1e1e] text-white px-2 py-1.5 rounded border border-white/10 focus:outline-none focus:border-blue-500 text-center" />
        </div>
        <div class="flex items-center justify-between">
          <span>Tab Size</span>
          <input type="number" value="2" class="w-16 bg-[#1e1e1e] text-white px-2 py-1.5 rounded border border-white/10 focus:outline-none focus:border-blue-500 text-center" />
        </div>
        <div class="flex items-center justify-between">
          <span>Word Wrap</span>
          <label class="relative inline-flex items-center cursor-pointer">
            <input type="checkbox" class="sr-only peer" />
            <div class="w-9 h-5 bg-[#1e1e1e] rounded-full peer peer-checked:after:translate-x-full peer-checked:bg-blue-600 after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:rounded-full after:h-4 after:w-4 after:transition-all"></div>
          </label>
        </div>
      </div>
    </div>

  </div>
</aside>
