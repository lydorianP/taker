<script>
  import { invoke } from '@tauri-apps/api/core';

  let { show = false, onClose, onAction } = $props();

  let query = $state('');
  let selectedIndex = $state(0);
  let results = $state([]);

  const commands = [
    { id: 'new-note', name: 'New Note', icon: '📝', category: 'Notes' },
    { id: 'new-vault', name: 'New Vault', icon: '📁', category: 'Vaults' },
    { id: 'save', name: 'Save Current Note', icon: '💾', category: 'Notes' },
    { id: 'toggle-sidebar', name: 'Toggle Sidebar', icon: '↔️', category: 'View' },
    { id: 'settings', name: 'Open Settings', icon: '⚙️', category: 'App' },
    { id: 'summarize', name: 'Summarize Note', icon: '✨', category: 'AI' },
    { id: 'generate-flashcards', name: 'Generate Flashcards', icon: '🃏', category: 'AI' },
    { id: 'generate-slideshow', name: 'Create Slideshow', icon: '📊', category: 'AI' },
    { id: 'generate-podcast', name: 'Generate Podcast', icon: '🎙️', category: 'AI' },
    { id: 'read-aloud', name: 'Read Note Aloud', icon: '🔊', category: 'Audio' },
    { id: 'search', name: 'Search Notes', icon: '🔍', category: 'Notes' },
    { id: 'plugins', name: 'Open Plugin Marketplace', icon: '🧩', category: 'App' },
    { id: 'about', name: 'About Taker', icon: 'ℹ️', category: 'App' },
  ];

  $effect(() => {
    if (query) {
      results = commands.filter(cmd => 
        cmd.name.toLowerCase().includes(query.toLowerCase()) ||
        cmd.category.toLowerCase().includes(query.toLowerCase())
      );
    } else {
      results = commands;
    }
    selectedIndex = 0;
  });

  function handleKeydown(e) {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, 0);
    } else if (e.key === 'Enter') {
      e.preventDefault();
      handleSelect(results[selectedIndex]);
    } else if (e.key === 'Escape') {
      onClose?.();
    }
  }

  function handleSelect(command) {
    if (command) {
      onAction?.(command.id);
      onClose?.();
    }
  }
</script>

{#if show}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="palette-overlay" onclick={onClose}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="palette" onclick={(e) => e.stopPropagation()}>
      <div class="search-box">
        <span class="search-icon">🔍</span>
        <input 
          type="text" 
          placeholder="Type a command..." 
          bind:value={query}
          onkeydown={handleKeydown}
        />
        <kbd class="shortcut">ESC</kbd>
      </div>
      
      <div class="results">
        {#if results.length === 0}
          <div class="no-results">No commands found</div>
        {:else}
          {#each results as command, i}
            <button 
              class="result-item"
              class:selected={i === selectedIndex}
              onclick={() => handleSelect(command)}
              onmouseenter={() => selectedIndex = i}
            >
              <span class="icon">{command.icon}</span>
              <span class="name">{command.name}</span>
              <span class="category">{command.category}</span>
            </button>
          {/each}
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .palette-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: flex-start;
    justify-content: center;
    padding-top: 20vh;
    z-index: 2000;
  }

  .palette {
    width: 100%;
    max-width: 560px;
    background-color: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
    overflow: hidden;
  }

  .search-box {
    display: flex;
    align-items: center;
    padding: var(--space-md);
    border-bottom: 1px solid var(--color-border);
    gap: var(--space-sm);
  }

  .search-icon {
    font-size: 1.125rem;
    color: var(--color-text-secondary);
  }

  .search-box input {
    flex: 1;
    padding: 0;
    border: none;
    background: none;
    font-size: 1rem;
    color: var(--color-text);
    outline: none;
  }

  .search-box input::placeholder {
    color: var(--color-text-secondary);
  }

  .shortcut {
    padding: 2px 6px;
    background-color: var(--color-border);
    border-radius: var(--radius-sm);
    font-size: 0.75rem;
    font-family: var(--font-mono);
    color: var(--color-text-secondary);
  }

  .results {
    max-height: 320px;
    overflow-y: auto;
    padding: var(--space-sm);
  }

  .no-results {
    padding: var(--space-lg);
    text-align: center;
    color: var(--color-text-secondary);
  }

  .result-item {
    width: 100%;
    display: flex;
    align-items: center;
    gap: var(--space-md);
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    color: var(--color-text);
    text-align: left;
    transition: background-color var(--transition-fast);
  }

  .result-item:hover, .result-item.selected {
    background-color: var(--color-surface-hover);
  }

  .result-item.selected {
    background-color: var(--color-accent);
    color: white;
  }

  .icon {
    font-size: 1.125rem;
    width: 24px;
    text-align: center;
  }

  .name {
    flex: 1;
    font-weight: 500;
  }

  .category {
    font-size: 0.75rem;
    color: var(--color-text-secondary);
    padding: 2px 8px;
    background-color: var(--color-border);
    border-radius: var(--radius-sm);
  }

  .result-item.selected .category {
    background-color: rgba(255, 255, 255, 0.2);
    color: white;
  }
</style>
