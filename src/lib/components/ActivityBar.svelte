<script lang="ts">
  import Icon from './icons.svelte';
  export let activeTab: string;
  export let setActive: (tab: string) => void;
</script>

<nav class="w-14 bg-[#0f172a] flex flex-col items-center py-3 gap-2 border-r border-[#1e293b]">
  <button class="w-10 h-10 flex items-center justify-center rounded-xl hover:bg-[#1e293b] transition-all {activeTab === 'files' ? 'bg-[#1e293b] text-blue-400' : 'text-slate-400'}" on:click={() => setActive('files')}>
    <Icon name="files" size={22} />
  </button>
  <button class="w-10 h-10 flex items-center justify-center rounded-xl hover:bg-[#1e293b] transition-all text-slate-400" on:click={() => setActive('search')}>
    <Icon name="search" size={22} />
  </button>
  <button class="w-10 h-10 flex items-center justify-center rounded-xl hover:bg-[#1e293b] transition-all text-slate-400" on:click={() => setActive('git')}>
    <Icon name="git" size={22} />
  </button>
  
  <div class="flex-1"></div>
  
  <button class="w-10 h-10 flex items-center justify-center rounded-xl hover:bg-[#1e293b] transition-all {activeTab === 'settings' ? 'bg-[#1e293b] text-blue-400' : 'text-slate-400'}" on:click={() => setActive('settings')}>
    <Icon name="settings" size={22} />
  </button>
</nav>
