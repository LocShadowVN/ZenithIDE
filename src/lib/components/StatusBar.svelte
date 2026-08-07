<script lang="ts">
  import { activeTabPath, openTabs, cursorPos } from '../stores';
  import { lang } from '../i18n';
  import Icon from './icons.svelte';
  
  let line = 1; let col = 1;
  cursorPos.subscribe(v => { line = v.line; col = v.col; });
  let currentLang = 'plaintext';
  
  $: if ($activeTabPath) {
    const tab = $openTabs.find(t => t.path === $activeTabPath);
    if (tab) currentLang = tab.language;
  }
</script>

<div class="h-6 bg-[#007acc] text-white flex items-center justify-between px-4 text-xs font-medium">
  <div class="flex items-center gap-4">
    <span class="flex items-center gap-1.5"><Icon name="git" size={12} /> main</span>
    <span class="flex items-center gap-1.5"><Icon name="close" size={12} /> 0 errors</span>
  </div>
  <div class="flex items-center gap-4">
    <span>Ln {line}, Col {col}</span>
    <span class="uppercase">{currentLang}</span>
    <select bind:value={$lang} class="bg-[#007acc] text-white text-xs border-none focus:outline-none cursor-pointer">
      <option value="en">EN</option>
      <option value="vi">VI</option>
    </select>
  </div>
</div>
