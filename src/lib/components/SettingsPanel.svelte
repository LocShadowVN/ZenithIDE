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
    updateStatus = 'Checking for updates...';
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
        await update.downloadAndInstall((event) => {
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

<aside class="w-80 bg-[#1e1f22] flex flex-col border-r border-white/5 overflow-y-auto">
  <div class="h-9 flex items-center px-4 text-[11px] uppercase tracking-wide text-[#8a8a8a] font-semibold border-b border-white/5">
    Settings
  </div>
  
  <div class="p-4 flex flex-col gap-6">
    
    <div>
      <h3 class="text-sm font-semibold text-white mb-3">About</h3>
      <div class="text-xs text-gray-400 flex justify-between mb-1">
        <span>Version</span>
        <span class="font-mono text-blue-400">v{version}</span>
      </div>
    </div>

    <div>
      <h3 class="text-sm font-semibold text-white mb-3">Updates</h3>
      <button 
        class="w-full bg-[#0d6efd] hover:bg-[#0b5ed7] text-white py-2 rounded text-xs font-semibold mb-2 flex items-center justify-center gap-2 transition-colors"
        on:click={checkForUpdates}
        disabled={isChecking || updateAvailable}
      >
        {#if isChecking}Checking...{:else}Check for Updates{/if}
      </button>

      {#if updateAvailable}
        <button 
          class="w-full bg-green-600 hover:bg-green-700 text-white py-2 rounded text-xs font-semibold mb-2 transition-colors"
          on:click={installUpdate}
        >
          Download & Install
        </button>
      {/if}

      <p class="text-xs text-gray-500 mt-1">{updateStatus}</p>
      {#if updateProgress > 0 && updateProgress < 100}
        <div class="w-full bg-[#3c3c3c] h-1 rounded-full mt-2">
          <div class="bg-blue-500 h-full transition-all" style="width: {updateProgress}%"></div>
        </div>
      {/if}
    </div>

    <div>
      <h3 class="text-sm font-semibold text-white mb-3">Editor</h3>
      <div class="flex flex-col gap-3 text-xs text-gray-400">
        <label class="flex items-center justify-between">
          <span>Font Size</span>
          <input type="number" value="14" class="w-12 bg-[#1e1e1e] text-white px-2 py-1 rounded border border-white/10 focus:outline-none focus:border-blue-500" />
        </label>
        <label class="flex items-center justify-between">
          <span>Tab Size</span>
          <input type="number" value="2" class="w-12 bg-[#1e1e1e] text-white px-2 py-1 rounded border border-white/10 focus:outline-none focus:border-blue-500" />
        </label>
        <label class="flex items-center justify-between">
          <span>Word Wrap</span>
          <input type="checkbox" class="accent-blue-500 h-4 w-4" />
        </label>
      </div>
    </div>

  </div>
</aside>
