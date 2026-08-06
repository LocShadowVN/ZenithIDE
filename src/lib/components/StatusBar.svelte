<script lang="ts">
  import { activeTabPath, openTabs, cursorPos } from '../stores';
  let line = 1; let col = 1;
  cursorPos.subscribe(v => { line = v.line; col = v.col; });
  let currentLang = 'plaintext';
  $: if ($activeTabPath) {
    const tab = $openTabs.find(t => t.path === $activeTabPath);
    if (tab) currentLang = tab.language;
  }
</script>
<div class="h-6 bg-[#007acc] text-white flex items-center justify-between px-3 text-xs">
  <div class="flex items-center gap-4">
    <span>Ready</span>
  </div>
  <div class="flex items-center gap-4">
    <span>Ln {line}, Col {col}</span>
    <span class="uppercase">{currentLang}</span>
  </div>
</div>
