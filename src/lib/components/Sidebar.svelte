<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Icon from './icons.svelte';
  import { openTabs, activeTabPath } from '../stores';
  import { lang } from '../i18n';

  interface FileNode { name: string; path: string; is_dir: boolean; }
  
  // Đổi tên prop cho khớp với +page.svelte
  export let workspacePath: string;
  
  let nodes: FileNode[] = [];
  let currentLang: 'en' | 'vi' = 'en';
  lang.subscribe((v: 'en' | 'vi') => currentLang = v);

  async function loadDir(path: string) { 
    nodes = await invoke<FileNode[]>('list_directory', { path }); 
  }
  
  $: if (workspacePath) loadDir(workspacePath);

  async function openFile(node: FileNode) {
    const content = await invoke<string>('read_file', { path: node.path });
    const langMap: Record<string, string> = { '.rs': 'rust', '.js': 'javascript', '.ts': 'typescript', '.c': 'c', '.cpp': 'cpp', '.html': 'html' };
    const ext = node.path.substring(node.path.lastIndexOf('.'));
    const language = langMap[ext] || 'plaintext';
    
    openTabs.update(tabs => { 
      if (!tabs.find(t => t.path === node.path)) tabs.push({ ...node, content, language }); 
      return tabs; 
    });
    activeTabPath.set(node.path);
  }
</script>

<aside class="w-60 bg-[#252526] flex flex-col border-r border-black/40">
  <div class="h-9 flex items-center justify-between px-4 text-[11px] uppercase tracking-wide text-[#bbbbbb] font-semibold">
    {currentLang === 'vi' ? 'Trình duyệt' : 'Explorer'}
  </div>
  <div class="flex-1 overflow-y-auto py-1">
    {#each nodes as node}
      <button class="w-full text-left pl-2 pr-3 py-[3px] flex items-center gap-1.5 hover:bg-white/5 text-[#cccccc]" on:click={() => !node.is_dir && openFile(node)}>
        {#if node.is_dir}<span class="text-[#c5c5c5]"><Icon name="folder" size={16} /></span>{:else}<span class="text-[#888]"><Icon name="file" size={16} /></span>{/if}
        <span class="truncate">{node.name}</span>
      </button>
    {/each}
  </div>
</aside>
