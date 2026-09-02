<script>
  import { invoke } from '@tauri-apps/api/core';
  import { marked } from 'marked';

  let { note = null, vaultId = null, onNoteSaved } = $props();

  let title = $state('');
  let content = $state('');
  let viewMode = $state('edit'); // 'edit', 'preview', 'split'
  let isSaving = $state(false);
  let isProcessing = $state(false);
  let flashcards = $state([]);
  let summary = $state(null);

  $effect(() => {
    if (note) {
      title = note.title || '';
      content = note.content || '';
      flashcards = [];
      summary = null;
    }
  });

  let renderedContent = $derived(marked(content || ''));

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
          vaultId 
        });
        onNoteSaved?.(newNote);
        return;
      }
      onNoteSaved?.();
    } catch (e) {
      console.error('Failed to save note:', e);
    } finally {
      isSaving = false;
    }
  }

  async function handleSummarize() {
    if (!note?.id || isProcessing) return;
    isProcessing = true;
    try {
      summary = await invoke('summarize_note', { noteId: note.id });
    } catch (e) {
      console.error('Failed to summarize:', e);
    } finally {
      isProcessing = false;
    }
  }

  async function handleGenerateFlashcards() {
    if (!note?.id || isProcessing) return;
    isProcessing = true;
    try {
      flashcards = await invoke('generate_flashcards', { noteId: note.id });
    } catch (e) {
      console.error('Failed to generate flashcards:', e);
    } finally {
      isProcessing = false;
    }
  }

  async function handleGenerateSlideshow() {
    if (!note?.id || isProcessing) return;
    isProcessing = true;
    try {
      await invoke('generate_slideshow', { noteId: note.id });
      alert('Slideshow generated! Check the slideshows section.');
    } catch (e) {
      console.error('Failed to generate slideshow:', e);
    } finally {
      isProcessing = false;
    }
  }

  function handleKeydown(e) {
    if ((e.metaKey || e.ctrlKey) && e.key === 's') {
      e.preventDefault();
      handleSave();
    }
  }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div class="editor" onkeydown={handleKeydown}>
  <div class="editor-header">
    <input 
      type="text" 
      class="title-input" 
      placeholder="Note title..." 
      bind:value={title}
    />
    <div class="actions">
      <div class="view-toggle">
        <button 
          class="toggle-btn" 
          class:active={viewMode === 'edit'}
          onclick={() => viewMode = 'edit'}
          title="Edit mode"
        >
          ✏️
        </button>
        <button 
          class="toggle-btn" 
          class:active={viewMode === 'split'}
          onclick={() => viewMode = 'split'}
          title="Split view"
        >
          📐
        </button>
        <button 
          class="toggle-btn" 
          class:active={viewMode === 'preview'}
          onclick={() => viewMode = 'preview'}
          title="Preview mode"
        >
          👁️
        </button>
      </div>
      <button class="btn-primary" onclick={handleSave} disabled={isSaving}>
        {isSaving ? 'Saving...' : 'Save'}
      </button>
    </div>
  </div>

  <div class="editor-content" class:split={viewMode === 'split'}>
    {#if viewMode === 'edit' || viewMode === 'split'}
      <div class="editor-pane">
        <textarea 
          class="content-input" 
          placeholder="Start writing your note... (Markdown supported)" 
          bind:value={content}
        ></textarea>
      </div>
    {/if}

    {#if viewMode === 'preview' || viewMode === 'split'}
      <div class="preview-pane">
        <div class="preview markdown-body">
          {@html renderedContent}
        </div>
      </div>
    {/if}
  </div>

  {#if summary}
    <div class="summary-panel">
      <h3>Summary</h3>
      <p>{summary.summary}</p>
      {#if summary.key_points.length > 0}
        <div class="key-points">
          <h4>Key Points</h4>
          <ul>
            {#each summary.key_points as point}
              <li>{point}</li>
            {/each}
          </ul>
        </div>
      {/if}
    </div>
  {/if}

  {#if flashcards.length > 0}
    <div class="flashcards-panel">
      <h3>Generated Flashcards ({flashcards.length})</h3>
      <div class="flashcards-grid">
        {#each flashcards as card}
          <div class="flashcard">
            <div class="question">
              <strong>Q:</strong> {card.question}
            </div>
            <div class="answer">
              <strong>A:</strong> {card.answer}
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}

  <div class="editor-footer">
    <div class="ai-actions">
      <button class="ai-btn" onclick={handleSummarize} disabled={isProcessing || !note?.id}>
        {isProcessing ? 'Processing...' : '✨ Summarize'}
      </button>
      <button class="ai-btn" onclick={handleGenerateFlashcards} disabled={isProcessing || !note?.id}>
        {isProcessing ? 'Processing...' : '🃏 Generate Flashcards'}
      </button>
      <button class="ai-btn" onclick={handleGenerateSlideshow} disabled={isProcessing || !note?.id}>
        {isProcessing ? 'Processing...' : '📊 Create Slideshow'}
      </button>
      <button class="ai-btn" disabled={isProcessing || !note?.id}>
        🎙️ Generate Podcast
      </button>
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
    align-items: center;
    gap: var(--space-sm);
  }

  .view-toggle {
    display: flex;
    background-color: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: 2px;
  }

  .toggle-btn {
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-sm);
    font-size: 0.875rem;
    transition: all var(--transition-fast);
  }

  .toggle-btn:hover {
    background-color: var(--color-surface-hover);
  }

  .toggle-btn.active {
    background-color: var(--color-accent);
    color: white;
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

  .editor-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
    min-height: 0;
  }

  .editor-content.split {
    flex-direction: row;
  }

  .editor-pane, .preview-pane {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-height: 0;
  }

  .editor-content.split .editor-pane,
  .editor-content.split .preview-pane {
    flex: 1;
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
    width: 100%;
    box-sizing: border-box;
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
    overflow-y: auto;
  }

  .summary-panel {
    background-color: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: var(--space-md);
    margin-top: var(--space-md);
  }

  .summary-panel h3 {
    font-size: 1rem;
    font-weight: 600;
    margin-bottom: var(--space-sm);
    color: var(--color-accent);
  }

  .summary-panel p {
    color: var(--color-text);
    line-height: 1.6;
  }

  .key-points {
    margin-top: var(--space-md);
  }

  .key-points h4 {
    font-size: 0.875rem;
    font-weight: 600;
    margin-bottom: var(--space-sm);
  }

  .key-points ul {
    list-style-type: disc;
    padding-left: var(--space-lg);
  }

  .key-points li {
    margin-bottom: var(--space-xs);
    color: var(--color-text-secondary);
  }

  .flashcards-panel {
    background-color: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: var(--space-md);
    margin-top: var(--space-md);
  }

  .flashcards-panel h3 {
    font-size: 1rem;
    font-weight: 600;
    margin-bottom: var(--space-md);
    color: var(--color-accent);
  }

  .flashcards-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
    gap: var(--space-md);
  }

  .flashcard {
    background-color: var(--color-canvas);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: var(--space-md);
  }

  .flashcard .question {
    font-weight: 500;
    margin-bottom: var(--space-sm);
    color: var(--color-text);
  }

  .flashcard .answer {
    color: var(--color-text-secondary);
    font-size: 0.875rem;
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

  .ai-btn:hover:not(:disabled) {
    background-color: var(--color-accent);
    color: white;
    border-color: var(--color-accent);
  }

  .ai-btn:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  /* Markdown styles */
  .markdown-body :global(h1) {
    font-size: 2rem;
    font-weight: 700;
    margin-bottom: var(--space-md);
    color: var(--color-text);
  }

  .markdown-body :global(h2) {
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: var(--space-sm);
    color: var(--color-text);
  }

  .markdown-body :global(h3) {
    font-size: 1.25rem;
    font-weight: 600;
    margin-bottom: var(--space-sm);
    color: var(--color-text);
  }

  .markdown-body :global(p) {
    margin-bottom: var(--space-md);
    color: var(--color-text);
  }

  .markdown-body :global(ul), .markdown-body :global(ol) {
    margin-bottom: var(--space-md);
    padding-left: var(--space-lg);
  }

  .markdown-body :global(li) {
    margin-bottom: var(--space-xs);
  }

  .markdown-body :global(code) {
    font-family: var(--font-mono);
    background-color: var(--color-surface);
    padding: 2px var(--space-xs);
    border-radius: var(--radius-sm);
    font-size: 0.875em;
  }

  .markdown-body :global(pre) {
    background-color: var(--color-surface);
    padding: var(--space-md);
    border-radius: var(--radius-md);
    overflow-x: auto;
    margin-bottom: var(--space-md);
  }

  .markdown-body :global(pre code) {
    background: none;
    padding: 0;
  }

  .markdown-body :global(blockquote) {
    border-left: 4px solid var(--color-accent);
    padding-left: var(--space-md);
    margin-bottom: var(--space-md);
    color: var(--color-text-secondary);
  }

  .markdown-body :global(a) {
    color: var(--color-accent);
    text-decoration: underline;
  }

  .markdown-body :global(strong) {
    font-weight: 600;
  }

  .markdown-body :global(em) {
    font-style: italic;
  }

  .markdown-body :global(hr) {
    border: none;
    border-top: 1px solid var(--color-border);
    margin: var(--space-lg) 0;
  }

  .markdown-body :global(table) {
    width: 100%;
    border-collapse: collapse;
    margin-bottom: var(--space-md);
  }

  .markdown-body :global(th), .markdown-body :global(td) {
    border: 1px solid var(--color-border);
    padding: var(--space-sm);
    text-align: left;
  }

  .markdown-body :global(th) {
    background-color: var(--color-surface);
    font-weight: 600;
  }
</style>
