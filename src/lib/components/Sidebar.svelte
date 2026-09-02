<script>
  let { 
    currentView = 'greetings', 
    vaults = [], 
    notes = [],
    selectedVaultId = null,
    onViewChange, 
    onNoteSelect, 
    onSettingsToggle,
    onVaultCreated,
    onVaultSelect,
    onNewNote
  } = $props();

  let isCollapsed = $state(false);
  let showNewVaultInput = $state(false);
  let newVaultName = $state('');

  function handleViewChange(view) {
    onViewChange?.(view);
  }

  function handleNoteSelect(note) {
    onNoteSelect?.(note);
  }

  function handleSettingsToggle() {
    onSettingsToggle?.();
  }

  function toggleSidebar() {
    isCollapsed = !isCollapsed;
  }

  async function createVault() {
    if (!newVaultName.trim()) return;
    try {
      await invoke('create_vault', { name: newVaultName.trim(), description: null });
      newVaultName = '';
      showNewVaultInput = false;
      onVaultCreated?.();
    } catch (e) {
      console.error('Failed to create vault:', e);
    }
  }

  function handleVaultKeydown(e) {
    if (e.key === 'Enter') createVault();
    if (e.key === 'Escape') { showNewVaultInput = false; newVaultName = ''; }
  }

  function selectVault(vaultId) {
    onVaultSelect?.(vaultId);
  }

  function handleNewNote() {
    onNewNote?.();
  }

  import { invoke } from '@tauri-apps/api/core';
</script>

<aside class="sidebar" class:collapsed={isCollapsed}>
  <div class="sidebar-header">
    <h1 class="logo">Taker</h1>
    <button class="toggle-btn" onclick={toggleSidebar}>
      {isCollapsed ? '→' : '←'}
    </button>
  </div>

  <nav class="nav">
    <button 
      class="nav-item" 
      class:active={currentView === 'greetings'}
      onclick={() => handleViewChange('greetings')}
    >
      <span class="icon">🏠</span>
      {#if !isCollapsed}<span>Home</span>{/if}
    </button>

    <button 
      class="nav-item"
      class:active={currentView === 'editor' && !selectedNote}
      onclick={handleNewNote}
    >
      <span class="icon">📝</span>
      {#if !isCollapsed}<span>New Note</span>{/if}
    </button>

    <button 
      class="nav-item"
      onclick={handleSettingsToggle}
    >
      <span class="icon">⚙️</span>
      {#if !isCollapsed}<span>Settings</span>{/if}
    </button>
  </nav>

  {#if !isCollapsed}
    <div class="vaults">
      <div class="section-header">
        <h3 class="section-title">Vaults</h3>
        <button class="add-btn" onclick={() => showNewVaultInput = true}>+</button>
      </div>
      
      {#if showNewVaultInput}
        <div class="new-vault-input">
          <input 
            type="text" 
            placeholder="Vault name..." 
            bind:value={newVaultName}
            onkeydown={handleVaultKeydown}
          />
          <button onclick={createVault}>✓</button>
          <button onclick={() => { showNewVaultInput = false; newVaultName = ''; }}>✕</button>
        </div>
      {/if}

      <button 
        class="vault-item"
        class:active={selectedVaultId === null}
        onclick={() => selectVault(null)}
      >
        📚 All Notes
      </button>

      {#each vaults as vault}
        <button 
          class="vault-item"
          class:active={selectedVaultId === vault.id}
          onclick={() => selectVault(vault.id)}
        >
          📁 {vault.name}
        </button>
      {/each}
    </div>

    <div class="notes">
      <h3 class="section-title">Notes</h3>
      {#each notes as note}
        <button 
          class="note-item"
          class:active={currentView === 'editor' && selectedNote?.id === note.id}
          onclick={() => handleNoteSelect(note)}
        >
          📄 {note.title || 'Untitled'}
        </button>
      {/each}
      {#if notes.length === 0}
        <p class="empty-state">No notes yet</p>
      {/if}
    </div>
  {/if}
</aside>

<style>
  .sidebar {
    width: 260px;
    height: 100%;
    background-color: var(--color-surface);
    border-right: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    transition: width var(--transition-normal);
    overflow: hidden;
  }

  .sidebar.collapsed {
    width: 60px;
  }

  .sidebar-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-md);
    border-bottom: 1px solid var(--color-border);
  }

  .logo {
    font-size: 1.25rem;
    font-weight: 700;
    color: var(--color-accent);
  }

  .toggle-btn {
    width: 28px;
    height: 28px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-sm);
    color: var(--color-text-secondary);
    transition: background-color var(--transition-fast);
  }

  .toggle-btn:hover {
    background-color: var(--color-surface-hover);
  }

  .nav {
    padding: var(--space-sm);
    border-bottom: 1px solid var(--color-border);
  }

  .nav-item {
    width: 100%;
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    color: var(--color-text);
    text-align: left;
    transition: background-color var(--transition-fast);
  }

  .nav-item:hover {
    background-color: var(--color-surface-hover);
  }

  .nav-item.active {
    background-color: var(--color-accent);
    color: white;
  }

  .icon {
    font-size: 1.125rem;
  }

  .vaults, .notes {
    padding: var(--space-sm);
    flex: 1;
    overflow-y: auto;
  }

  .notes {
    flex: 1;
  }

  .section-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-sm) var(--space-md);
  }

  .section-title {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-text-secondary);
    margin-bottom: var(--space-xs);
  }

  .add-btn {
    width: 20px;
    height: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-sm);
    color: var(--color-text-secondary);
    font-size: 0.875rem;
    transition: all var(--transition-fast);
  }

  .add-btn:hover {
    background-color: var(--color-accent);
    color: white;
  }

  .new-vault-input {
    display: flex;
    gap: var(--space-xs);
    padding: var(--space-sm) var(--space-md);
    margin-bottom: var(--space-sm);
  }

  .new-vault-input input {
    flex: 1;
    padding: var(--space-xs) var(--space-sm);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    font-size: 0.875rem;
    background-color: var(--color-canvas);
    color: var(--color-text);
  }

  .new-vault-input input:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  .new-vault-input button {
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-sm);
    font-size: 0.75rem;
  }

  .new-vault-input button:first-of-type {
    background-color: var(--color-success);
    color: white;
  }

  .new-vault-input button:last-of-type {
    background-color: var(--color-warning);
    color: white;
  }

  .vault-item, .note-item {
    width: 100%;
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-sm) var(--space-md);
    border-radius: var(--radius-md);
    color: var(--color-text);
    text-align: left;
    font-size: 0.875rem;
    transition: background-color var(--transition-fast);
  }

  .vault-item:hover, .note-item:hover {
    background-color: var(--color-surface-hover);
  }

  .vault-item.active, .note-item.active {
    background-color: var(--color-accent);
    color: white;
  }

  .empty-state {
    padding: var(--space-sm) var(--space-md);
    color: var(--color-text-secondary);
    font-size: 0.875rem;
    font-style: italic;
  }
</style>
