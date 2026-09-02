<script>
  import Sidebar from './lib/components/Sidebar.svelte';
  import Editor from './lib/components/Editor.svelte';
  import Settings from './lib/components/Settings.svelte';
  import Greetings from './lib/components/Greetings.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let currentView = $state('greetings');
  let selectedNote = $state(null);
  let isSettingsOpen = $state(false);
  let vaults = $state([]);
  let notes = $state([]);

  async function loadVaults() {
    try {
      vaults = await invoke('get_vaults');
    } catch (e) {
      console.error('Failed to load vaults:', e);
    }
  }

  async function loadNotes() {
    try {
      notes = await invoke('get_notes', { vaultId: null });
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

  async function handleNoteSaved(newNote) {
    if (newNote) {
      selectedNote = newNote;
      currentView = 'editor';
    }
    await loadNotes();
  }

  $effect(() => {
    loadVaults();
    loadNotes();
  });
</script>

<div class="app">
  <Sidebar 
    {currentView} 
    {vaults}
    {notes}
    onViewChange={handleViewChange} 
    onNoteSelect={handleNoteSelect}
    onSettingsToggle={handleSettingsToggle}
    onVaultCreated={handleVaultCreated}
    onNoteSaved={handleNoteSaved}
  />
  
  <main class="content">
    {#if currentView === 'greetings'}
      <Greetings onNoteSelect={handleNoteSelect} />
    {:else if currentView === 'editor'}
      <Editor note={selectedNote} onNoteSaved={handleNoteSaved} />
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
