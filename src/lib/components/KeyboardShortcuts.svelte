<script>
  import { invoke } from '@tauri-apps/api/core';

  let { onAction } = $props();

  function handleKeydown(e) {
    // Ctrl/Cmd + K: Command Palette
    if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
      e.preventDefault();
      onAction?.('command-palette');
    }
    
    // Ctrl/Cmd + N: New Note
    if ((e.metaKey || e.ctrlKey) && e.key === 'n') {
      e.preventDefault();
      onAction?.('new-note');
    }
    
    // Ctrl/Cmd + S: Save
    if ((e.metaKey || e.ctrlKey) && e.key === 's') {
      e.preventDefault();
      onAction?.('save');
    }
    
    // Ctrl/Cmd + ,: Settings
    if ((e.metaKey || e.ctrlKey) && e.key === ',') {
      e.preventDefault();
      onAction?.('settings');
    }
    
    // Ctrl/Cmd + B: Toggle Sidebar
    if ((e.metaKey || e.ctrlKey) && e.key === 'b') {
      e.preventDefault();
      onAction?.('toggle-sidebar');
    }
    
    // Ctrl/Cmd + E: Toggle Edit/Preview
    if ((e.metaKey || e.ctrlKey) && e.key === 'e') {
      e.preventDefault();
      onAction?.('toggle-view');
    }
    
    // Escape: Close modals
    if (e.key === 'Escape') {
      onAction?.('escape');
    }
  }
</script>

<svelte:window onkeydown={handleKeydown} />
