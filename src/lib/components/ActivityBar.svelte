<script lang="ts">
  import Icon from './icons.svelte';
  export let activeTab: string;
  export let setActive: (tab: string) => void;
</script>

<nav class="w-12 bg-[#333333] flex flex-col items-center py-2 gap-1 border-r border-black/40">
  <button class="w-12 h-12 flex items-center justify-center text-[#858585] hover:text-white relative {activeTab === 'files' ? 'text-white' : ''}" on:click={() => setActive('files')}>
    {#if activeTab === 'files'}<div class="absolute left-0 top-0 bottom-0 w-[2px] bg-white"></div>{/if}
    <Icon name="files" size={24} />
  </button>
  <button class="w-12 h-12 flex items-center justify-center text-[#858585] hover:text-white relative {activeTab === 'search' ? 'text-white' : ''}" on:click={() => setActive('search')}>
    {#if activeTab === 'search'}<div class="absolute left-0 top-0 bottom-0 w-[2px] bg-white"></div>{/if}
    <Icon name="search" size={24} />
  </button>
  <button class="w-12 h-12 flex items-center justify-center text-[#858585] hover:text-white relative {activeTab === 'git' ? 'text-white' : ''}" on:click={() => setActive('git')}>
    {#if activeTab === 'git'}<div class="absolute left-0 top-0 bottom-0 w-[2px] bg-white"></div>{/if}
    <Icon name="git" size={24} />
  </button>
  
  <div class="flex-1"></div>
  
  <button class="w-12 h-12 flex items-center justify-center text-[#858585] hover:text-white relative {activeTab === 'settings' ? 'text-white' : ''}" on:click={() => setActive('settings')}>
    {#if activeTab === 'settings'}<div class="absolute left-0 top-0 bottom-0 w-[2px] bg-white"></div>{/if}
    <Icon name="settings" size={24} />
  </button>
</nav>
