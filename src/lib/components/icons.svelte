<script lang="ts">
  export let name: string; export let size: number = 16;
</script>
{#if name === 'files'}
  <svg width={size} height={size} viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.2"><path d="M13.5 2.5H6.5L4 5H2.5v8.5h11V2.5z"/></svg>
{:else if name === 'search'}
  <svg width={size} height={size} viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="7" cy="7" r="4.5"/><path d="M10.5 10.5l3 3"/></svg>
{:else if name === 'git'}
  <svg width={size} height={size} viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><circle cx="4" cy="4" r="2"/><circle cx="12" cy="12" r="2"/><circle cx="12" cy="4" r="2"/><path d="M4 6v6M6 4h4M10 12H6"/></svg>
{:else if name === 'settings'}
  <!-- Icon bánh răng cưa chuẩn VS Code (Codicon Gear) -->
  <svg width={size} height={size} viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M9.1 4.4L8.6 2H7.4l-.5 2.4-.7.3-2-1.3-.9.8 1.3 2-.2.7-2.4.5v1.2l2.4.5.3.8-1.3 2 .8.8 2-1.3.8.3.4 2.4h1.2l.5-2.4.8-.3 2 1.3.8-.8-1.3-2 .3-.8 2.4-.5V6.9l-2.4-.5-.3-.8 1.3-2-.8-.8-2 1.3-.8-.3zM9.4 1l.5 2.4L12 2l2 2-1.4 2.1 2.4.4v3l-2.4.5L14 12l-2 2-2.1-1.4-.5 2.4h-3l-.5-2.4L4 14l-2-2 1.4-2.1L1 9.4v-3l2.4-.5L2 4l2-2 2.1 1.4L6.6 1h3zM8 5.5a2.5 2.5 0 1 0 0 5 2.5 2.5 0 0 0 0-5z"></path></svg>
{:else if name === 'folder'}
  <svg width={size} height={size} viewBox="0 0 16 16" fill="currentColor" class="text-[#c5c5c5]"><path d="M1 3.5A1.5 1.5 0 0 1 2.5 2h3.379a1.5 1.5 0 0 1 1.06.44L8 3.5h5.5A1.5 1.5 0 0 1 15 5v8a1.5 1.5 0 0 1-1.5 1.5h-11A1.5 1.5 0 0 1 1 13V3.5z"/></svg>
{:else if name === 'file'}
  <svg width={size} height={size} viewBox="0 0 16 16" fill="currentColor" class="text-[#888]"><path d="M9.5 1H3v14h10V4.5L9.5 1zm0 1.5L11.5 4.5H9.5V2.5z"/></svg>
{:else if name === 'close'}
  <svg width={size} height={size} viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M4 4l8 8M12 4l-8 8"/></svg>
{:else if name === 'terminal'}
  <svg width={size} height={size} viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.2"><path d="M2 4h12v8H2z"/><path d="M4 7l2 1.5L4 10M8 10h3"/></svg>
{:else if name === 'run'}
  <svg width={size} height={size} viewBox="0 0 16 16" fill="currentColor"><path d="M4 2v12l10-6z"/></svg>
{:else if name === 'ai'}
  <svg width={size} height={size} viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.2"><path d="M8 1l2 5h5l-4 3 2 6-5-4-5 4 2-6-4-3h5z"/></svg>
{:else if name === 'new-file'}
  <svg width={size} height={size} viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.2"><path d="M9 1H3v14h10V5L9 1zm0 1.5L11.5 4H9V2.5zM8 7v4M6 9h4"/></svg>
{:else if name === 'new-folder'}
  <svg width={size} height={size} viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.2"><path d="M1 3.5A1.5 1.5 0 0 1 2.5 2h3.379a1.5 1.5 0 0 1 1.06.44L8 3.5h5.5A1.5 1.5 0 0 1 15 5v2H1V3.5zM1 8h14v5a1.5 1.5 0 0 1-1.5 1.5h-11A1.5 1.5 0 0 1 1 13V8zM8 9v4M6 11h4"/></svg>
{:else if name === 'save'}
  <svg width={size} height={size} viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="1.2"><path d="M2 2h10l2 2v10H2V2zM4 2v5h6V2M4 9h8v5H4z"/></svg>
{/if}
