<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import Icon from './icons.svelte';
  import { openTabs, activeTabPath } from '../stores';

  interface FileNode { name: string; path: string; is_dir: boolean; }
  export let workspacePath: string;
  export let refreshKey: number;
  let nodes: FileNode[] = [];

  async function loadDir(path: string) { 
    nodes = await invoke<FileNode[]>('list_directory', { path }); 
  }
  $: if (workspacePath && refreshKey) loadDir(workspacePath);

  async function openFile(node: FileNode) {
    const content = await invoke<string>('read_file', { path: node.path });
    const langMap: Record<string, string> = { '.rs': 'rust', '.js': 'javascript', '.ts': 'typescript', '.c': 'c', '.cpp': 'cpp', '.html': 'html' };
    const ext = node.path.substring(node.path.lastIndexOf('.'));
    const language = langMap[ext] || 'plaintext';
    
    openTabs.update(tabs => { if (!tabs.find(t => t.path === node.path)) tabs.push({ ...node, content, language }); return tabs; });
    activeTabPath.set(node.path);
  }

  async function newFile() {
    const path = await invoke<string>('create_new_file', { lang: 'txt' });
    loadDir(workspacePath);
    openFile({ name: path.split(/[\\/]/).pop() || 'untitled.txt', path, is_dir: false });
  }

  async function newFolder() {
    await invoke<string>('create_new_folder', { name: `new_folder_${Date.now()}` });
    loadDir(workspacePath);
  }
</script>

<aside class="w-60 bg-[#252526] flex flex-col border-r border-black/40">
  <div class="h-9 flex items-center justify-between px-4 text-[11px] font-bold uppercase text-[#bbbbbb]">
    Explorer
    <div class="flex gap-3">
      <button class="hover:text-white transition" on:click={newFile} title="New File"><Icon name="new-file" size={14} /></button>
      <button class="hover:text-white transition" on:click={newFolder} title="New Folder"><Icon name="new-folder" size={14} /></button>
    </div>
  </div>
  <div class="h-6 flex items-center px-4 text-[11px] font-bold uppercase text-[#bbbbbb]">
    Zenith_Workspace
  </div>
  <div class="flex-1 overflow-y-auto py-1">
    {#if nodes.length === 0}
      <div class="text-center text-xs text-[#5a5a5a] mt-10 px-4">
        No files opened.<br/>Click the icon above to create one.
      </div>
    {:else}
      {#each nodes as node}
        <button class="w-full text-left pl-5 pr-3 py-[3px] flex items-center gap-1.5 hover:bg-white/5 text-[#cccccc] transition-colors" on:click={() => !node.is_dir && openFile(node)}>
          {#if node.is_dir}
            <span class="text-[#c5c5c5]"><Icon name="folder" size={16} /></span>
          {:else}
            <span class="text-[#519aba]"><Icon name="file" size={16} /></span>
          {/if}
          <span class="truncate text-[13px]">{node.name}</span>
        </button>
      {/each}
    {/if}
  </div>
</aside>
