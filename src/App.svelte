<script>
  import Sidebar from './lib/components/Sidebar.svelte';
  import Editor from './lib/components/Editor.svelte';
  import Settings from './lib/components/Settings.svelte';
  import Greetings from './lib/components/Greetings.svelte';
  import CommandPalette from './lib/components/CommandPalette.svelte';
  import KeyboardShortcuts from './lib/components/KeyboardShortcuts.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let currentView = $state('greetings');
  let selectedNote = $state(null);
  let selectedVaultId = $state(null);
  let isSettingsOpen = $state(false);
  let showCommandPalette = $state(false);
  let vaults = $state([]);
  let notes = $state([]);

  async function loadVaults() {
    try {
      vaults = await invoke('get_vaults');
    } catch (e) {
      console.error('Failed to load vaults:', e);
    }
  }

  async function loadNotes(vaultId = null) {
    try {
      notes = await invoke('get_notes', { vaultId });
    } catch (e) {
      console.error('Failed to load notes:', e);
    }
  }

  function handleViewChange(view) {
    currentView = view;
  }

  async function handleNoteSelect(note) {
    if (note && note.id) {
      try {
        selectedNote = await invoke('get_note', { id: note.id });
      } catch (e) {
        console.error('Failed to load note:', e);
        selectedNote = note;
      }
    } else {
      selectedNote = note;
    }
    currentView = 'editor';
  }

  function handleSettingsToggle() {
    isSettingsOpen = !isSettingsOpen;
  }

  async function handleVaultCreated() {
    await loadVaults();
  }

  function handleVaultSelect(vaultId) {
    selectedVaultId = vaultId;
    loadNotes(vaultId);
  }

  async function handleNoteSaved(newNote) {
    if (newNote) {
      selectedNote = newNote;
      currentView = 'editor';
    }
    await loadNotes(selectedVaultId);
  }

  function handleNewNote() {
    selectedNote = null;
    currentView = 'editor';
  }

  function handleCommandPaletteAction(actionId) {
    switch (actionId) {
      case 'new-note':
        handleNewNote();
        break;
      case 'settings':
        isSettingsOpen = true;
        break;
      case 'toggle-sidebar':
        // TODO: Toggle sidebar
        break;
      default:
        console.log('Action:', actionId);
    }
  }

  function handleKeyboardAction(actionId) {
    switch (actionId) {
      case 'command-palette':
        showCommandPalette = !showCommandPalette;
        break;
      case 'new-note':
        handleNewNote();
        break;
      case 'settings':
        isSettingsOpen = !isSettingsOpen;
        break;
      case 'escape':
        showCommandPalette = false;
        isSettingsOpen = false;
        break;
      default:
        console.log('Keyboard action:', actionId);
    }
  }

  $effect(() => {
    loadVaults();
    loadNotes();
  });
</script>

<KeyboardShortcuts onAction={handleKeyboardAction} />

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="app">
  <Sidebar 
    {currentView} 
    {vaults}
    {notes}
    {selectedVaultId}
    onViewChange={handleViewChange} 
    onNoteSelect={handleNoteSelect}
    onSettingsToggle={handleSettingsToggle}
    onVaultCreated={handleVaultCreated}
    onVaultSelect={handleVaultSelect}
    onNewNote={handleNewNote}
  />
  
  <main class="content">
    {#if currentView === 'greetings'}
      <Greetings onNoteSelect={handleNoteSelect} />
    {:else if currentView === 'editor'}
      <Editor note={selectedNote} vaultId={selectedVaultId} onNoteSaved={handleNoteSaved} />
    {:else if currentView === 'settings'}
      <Settings />
    {/if}
  </main>

  {#if isSettingsOpen}
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="settings-overlay" onclick={handleSettingsToggle}>
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <div class="settings-panel" onclick={(e) => e.stopPropagation()}>
        <Settings />
      </div>
    </div>
  {/if}

  <CommandPalette 
    show={showCommandPalette}
    onClose={() => showCommandPalette = false}
    onAction={handleCommandPaletteAction}
  />
</div>

<style>
  .app {
    display: flex;
    height: 100vh;
    overflow: hidden;
  }

  .content {
    flex: 1;
    overflow: auto;
    background-color: var(--color-canvas);
  }

  .settings-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: flex-end;
    z-index: 100;
  }

  .settings-panel {
    width: 400px;
    height: 100%;
    background-color: var(--color-surface);
    box-shadow: -2px 0 8px rgba(0, 0, 0, 0.1);
    overflow: auto;
  }
</style>
