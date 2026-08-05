<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import * as monaco from 'monaco-editor';
  import { openTabs, activeTabPath } from '../stores';
  import Icon from './icons.svelte';
  import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';

  let editorContainer: HTMLDivElement;
  let editorInstance: monaco.editor.IStandaloneCodeEditor | null = null;

  onMount(() => {
    self.MonacoEnvironment = { getWorker: () => new editorWorker() };
    editorInstance = monaco.editor.create(editorContainer, {
      theme: 'vs-dark', automaticLayout: true, fontSize: 14,
      fontFamily: 'Consolas, "Courier New", monospace', minimap: { enabled: true },
      scrollBeyondLastLine: false, scrollbar: { verticalScrollbarSize: 10, horizontalScrollbarSize: 10 }
    });
  });

  $: if (editorInstance && $activeTabPath) {
    const tab = $openTabs.find(t => t.path === $activeTabPath);
    if (tab) {
      let model = monaco.editor.getModel(monaco.Uri.parse(`file://${tab.path}`));
      if (!model) model = monaco.editor.createModel(tab.content, tab.language, monaco.Uri.parse(`file://${tab.path}`));
      editorInstance.setModel(model);
    }
  }

  function closeTab(path: string) {
    openTabs.update(tabs => {
      const idx = tabs.findIndex(t => t.path === path); 
      tabs.splice(idx, 1);
      if ($activeTabPath === path) activeTabPath.set(tabs.length > 0 ? tabs[Math.max(0, idx - 1)].path : null);
      return tabs;
    });
  }
  
  onDestroy(() => editorInstance?.dispose());
</script>

<div class="flex flex-col h-full bg-[#1e1e1e]">
  <div class="h-9 flex items-center bg-[#252526] border-b border-black/40 overflow-x-auto">
    {#each $openTabs as tab (tab.path)}
      <!-- Đổi div thành button để fix lỗi A11y -->
      <button 
        class="h-full flex items-center px-3 gap-2 border-r border-black/40 cursor-pointer text-[13px] {$activeTabPath === tab.path ? 'bg-[#1e1e1e] text-white' : 'bg-[#2d2d2d] text-[#969696] hover:bg-[#252526]'}" 
        on:click={() => activeTabPath.set(tab.path)}
      >
        <Icon name="file" size={14} /><span>{tab.name}</span>
        <span class="hover:bg-white/20 rounded p-0.5" on:click|stopPropagation={() => closeTab(tab.path)}><Icon name="close" size={12} /></span>
      </button>
    {/each}
  </div>
  <div class="flex-1" bind:this={editorContainer}></div>
</div>
