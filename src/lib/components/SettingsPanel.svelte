<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Icon from './icons.svelte';

  let version = '0.0.0';
  
  onMount(async () => {
    try {
      version = await invoke('get_app_version');
    } catch (e) {
      console.error(e);
    }
  });
</script>

<aside class="w-80 bg-[#1e1f22] flex flex-col border-r border-white/5 overflow-y-auto">
  <div class="h-9 flex items-center px-4 text-[11px] uppercase tracking-wide text-[#8a8a8a] font-semibold border-b border-white/5">
    Settings
  </div>
  
  <div class="p-4 flex flex-col gap-6">
    
    <!-- About Section -->
    <div>
      <h3 class="text-sm font-semibold text-white mb-3">About</h3>
      <div class="text-xs text-gray-400 flex justify-between mb-1">
        <span>Version</span>
        <span class="font-mono text-blue-400">v{version}</span>
      </div>
    </div>

    <!-- Editor Settings (Mock UI cho bản sau) -->
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
