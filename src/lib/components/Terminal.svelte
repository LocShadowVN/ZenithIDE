<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen, type UnlistenFn } from '@tauri-apps/api/event';
  import { Terminal } from 'xterm';
  import { FitAddon } from 'xterm-addon-fit';
  import 'xterm/css/xterm.css';

  let termContainer: HTMLDivElement;
  let terminal: Terminal; 
  let unlisten: UnlistenFn;
  const ptyId = 1;

  // Khai báo type string rõ ràng
  export let cwd: string;

  onMount(async () => {
    terminal = new Terminal({ 
      theme: { background: '#1e1e1e', foreground: '#cccccc', cursor: '#ffffff' }, 
      fontFamily: 'Consolas, "Courier New", monospace', 
      fontSize: 13, 
      cursorBlink: true 
    });
    const fitAddon = new FitAddon(); 
    terminal.loadAddon(fitAddon); 
    terminal.open(termContainer); 
    fitAddon.fit();
    
    unlisten = await listen(`pty-${ptyId}`, (event) => terminal.write(event.payload as string));
    terminal.onData((data) => invoke('write_to_pty', { id: ptyId, data }));
    
    await invoke('start_pty', { id: ptyId, cwd });
  });

  onDestroy(() => { 
    unlisten?.(); 
    terminal?.dispose(); 
  });
</script>

<div bind:this={termContainer} class="w-full h-full p-1"></div>
