<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import * as monaco from 'monaco-editor';
  import { openTabs, activeTabPath, cursorPos, editorContent } from '../stores';
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
      scrollBeyondLastLine: false, scrollbar: { verticalScrollbarSize: 8, horizontalScrollbarSize: 8 },
      padding: { top: 10 }
    });

    editorInstance.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, saveFile);
    editorInstance.onDidChangeCursorPosition(e => cursorPos.set({ line: e.position.lineNumber, col: e.position.column }));
    
    // Cập nhật nội dung realtime cho AI
    editorInstance.onDidChangeModelContent(() => {
      const model = editorInstance.getModel();
      if (model) editorContent.set(model.getValue());
    });
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
      editorContent.set(model.getValue());
    }
  }

  function closeTab(path: string) {
    openTabs.update(tabs => {
      const idx = tabs.findIndex(t => t.path === path); 
      tabs.splice(idx, 1);
      
      // DỌN DẸP RAM: Xóa model của file đã đóng
      const model = monaco.editor.getModel(monaco.Uri.parse(`file://${path}`));
      if (model) model.dispose();

      if ($activeTabPath === path) activeTabPath.set(tabs.length > 0 ? tabs[Math.max(0, idx - 1)].path : null);
      return tabs;
    });
  }
  
  onDestroy(() => {
    // Dọn dẹp toàn bộ khi tắt app
    monaco.editor.getModels().forEach(m => m.dispose());
    editorInstance?.dispose();
  });
</script>

<div class="flex flex-col h-full bg-[#0f172a]">
  <div class="h-10 flex items-center bg-[#1e293b] border-b border-[#0f172a] justify-between">
    <div class="h-full flex items-center overflow-x-auto">
      {#if $openTabs.length === 0}
        <div class="px-4 text-xs text-slate-500">No file open.</div>
      {/if}
      {#each $openTabs as tab (tab.path)}
        <div 
          class="h-full flex items-center px-4 gap-2 cursor-pointer text-[13px] group {$activeTabPath === tab.path ? 'bg-[#0f172a] text-white' : 'bg-[#1e293b] text-slate-400 hover:bg-[#334155]'}" 
          role="button" tabindex="0"
          on:click={() => activeTabPath.set(tab.path)}
          on:keydown={(e) => e.key === 'Enter' && activeTabPath.set(tab.path)}
        >
          <Icon name="file" size={14} />
          <span>{tab.name}</span>
          <button class="hover:bg-white/20 rounded p-0.5 ml-1 opacity-0 group-hover:opacity-100" on:click|stopPropagation={() => closeTab(tab.path)}>
            <Icon name="close" size={12} />
          </button>
        </div>
      {/each}
    </div>
    <button class="flex items-center gap-1 modern-btn text-white px-4 py-1.5 text-xs rounded-lg mr-3 disabled:opacity-50" on:click={saveFile} disabled={!$activeTabPath}>
      {#if isSaving}<span class="text-green-300 flex items-center gap-1"><Icon name="save" size={14}/> Saved!</span>{:else}<Icon name="save" size={14} /> Save{/if}
    </button>
  </div>
  <div class="flex-1" bind:this={editorContainer}></div>
</div>
