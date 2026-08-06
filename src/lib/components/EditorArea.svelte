<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import * as monaco from 'monaco-editor';
  import { openTabs, activeTabPath, cursorPos } from '../stores';
  import Icon from './icons.svelte';
  import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
  import { invoke } from '@tauri-apps/api/core';

  let editorContainer: HTMLDivElement;
  let editorInstance: monaco.editor.IStandaloneCodeEditor | null = null;
  let isSaving = false;

  onMount(() => {
    self.MonacoEnvironment = { getWorker: () => new editorWorker() };
    editorInstance = monaco.editor.create(editorContainer, {
      theme: 'vs-dark', automaticLayout: true, fontSize: 14,
      fontFamily: 'Consolas, "Courier New", monospace', minimap: { enabled: true },
      scrollBeyondLastLine: false, scrollbar: { verticalScrollbarSize: 10, horizontalScrollbarSize: 10 }
    });

    editorInstance.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, saveFile);
    editorInstance.onDidChangeCursorPosition(e => cursorPos.set({ line: e.position.lineNumber, col: e.position.column }));
  });

  async function saveFile() {
    const path = $activeTabPath;
    if (!path || !editorInstance) return;
    isSaving = true;
    const model = editorInstance.getModel();
    if (model) {
      try { await invoke('save_file', { path, content: model.getValue() }); } catch (e) { console.error(e); }
    }
    setTimeout(() => isSaving = false, 1000);
  }

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
  <div class="h-9 flex items-center bg-[#252526] border-b border-black/40 justify-between">
    <div class="h-full flex items-center overflow-x-auto">
      {#each $openTabs as tab (tab.path)}
        <button class="h-full flex items-center px-3 gap-2 border-r border-black/40 cursor-pointer text-[13px] {$activeTabPath === tab.path ? 'bg-[#1e1e1e] text-white' : 'bg-[#2d2d2d] text-[#969696] hover:bg-[#252526]'}" on:click={() => activeTabPath.set(tab.path)}>
          <Icon name="file" size={14} /><span>{tab.name}</span>
          <span class="hover:bg-white/20 rounded p-0.5" on:click|stopPropagation={() => closeTab(tab.path)}><Icon name="close" size={12} /></span>
        </button>
      {/each}
    </div>
    <button class="flex items-center gap-1 bg-[#2d2d2d] hover:bg-[#3c3c3c] text-white px-3 py-1 text-xs rounded mr-2 transition-colors" on:click={saveFile}>
      {#if isSaving}<span class="text-green-400">Saved!</span>{:else}<Icon name="save" size={14} /> Save{/if}
    </button>
  </div>
  <div class="flex-1" bind:this={editorContainer}></div>
</div>
