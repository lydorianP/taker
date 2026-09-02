<script>
  import { invoke } from '@tauri-apps/api/core';

  let { note = null, onNoteSaved } = $props();

  let title = $state(note?.title || '');
  let content = $state(note?.content || '');
  let isPreview = $state(false);
  let isSaving = $state(false);

  $effect(() => {
    if (note) {
      title = note.title || '';
      content = note.content || '';
    }
  });

  async function handleSave() {
    if (isSaving) return;
    isSaving = true;
    
    try {
      if (note?.id) {
        await invoke('update_note', { 
          id: note.id, 
          title: title || null, 
          content: content || null 
        });
      } else {
        const newNote = await invoke('create_note', { 
          title: title || 'Untitled', 
          content, 
          vaultId: null 
        });
        note = newNote;
      }
      onNoteSaved?.();
    } catch (e) {
      console.error('Failed to save note:', e);
    } finally {
      isSaving = false;
    }
  }

  function togglePreview() {
    isPreview = !isPreview;
  }
</script>

<div class="editor">
  <div class="editor-header">
    <input 
      type="text" 
      class="title-input" 
      placeholder="Note title..." 
      bind:value={title}
    />
    <div class="actions">
      <button class="btn-secondary" onclick={togglePreview}>
        {isPreview ? 'Edit' : 'Preview'}
      </button>
      <button class="btn-primary" onclick={handleSave} disabled={isSaving}>
        {isSaving ? 'Saving...' : 'Save'}
      </button>
    </div>
  </div>

  <div class="editor-content">
    {#if isPreview}
      <div class="preview">
        {@html (content || '').replace(/\n/g, '<br/>')}
      </div>
    {:else}
      <textarea 
        class="content-input" 
        placeholder="Start writing your note..." 
        bind:value={content}
      ></textarea>
    {/if}
  </div>

  <div class="editor-footer">
    <div class="ai-actions">
      <button class="ai-btn">✨ Summarize</button>
      <button class="ai-btn">🃏 Generate Flashcards</button>
      <button class="ai-btn">📊 Create Slideshow</button>
      <button class="ai-btn">🎙️ Generate Podcast</button>
    </div>
  </div>
</div>

<style>
  .editor {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: var(--space-lg);
    gap: var(--space-md);
  }

  .editor-header {
    display: flex;
    align-items: center;
    gap: var(--space-md);
  }

  .title-input {
    flex: 1;
    padding: var(--space-sm) var(--space-md);
    font-size: 1.5rem;
    font-weight: 600;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background-color: var(--color-canvas);
    color: var(--color-text);
  }

  .title-input:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  .actions {
    display: flex;
    gap: var(--space-sm);
  }

  .btn-primary {
    padding: var(--space-sm) var(--space-md);
    background-color: var(--color-accent);
    color: white;
    border-radius: var(--radius-md);
    font-weight: 500;
    transition: background-color var(--transition-fast);
  }

  .btn-primary:hover:not(:disabled) {
    background-color: var(--color-accent-hover);
  }

  .btn-primary:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .btn-secondary {
    padding: var(--space-sm) var(--space-md);
    background-color: var(--color-surface);
    color: var(--color-text);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    font-weight: 500;
    transition: background-color var(--transition-fast);
  }

  .btn-secondary:hover {
    background-color: var(--color-surface-hover);
  }

  .editor-content {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .content-input {
    flex: 1;
    padding: var(--space-md);
    font-family: var(--font-mono);
    font-size: 0.875rem;
    line-height: 1.6;
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background-color: var(--color-canvas);
    color: var(--color-text);
    resize: none;
  }

  .content-input:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  .preview {
    flex: 1;
    padding: var(--space-md);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background-color: var(--color-canvas);
    line-height: 1.6;
  }

  .editor-footer {
    border-top: 1px solid var(--color-border);
    padding-top: var(--space-md);
  }

  .ai-actions {
    display: flex;
    gap: var(--space-sm);
    flex-wrap: wrap;
  }

  .ai-btn {
    padding: var(--space-sm) var(--space-md);
    background-color: var(--color-surface);
    color: var(--color-text);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    font-size: 0.875rem;
    transition: all var(--transition-fast);
  }

  .ai-btn:hover {
    background-color: var(--color-accent);
    color: white;
    border-color: var(--color-accent);
  }
</style>
