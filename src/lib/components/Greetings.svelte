<script>
  let { onNoteSelect } = $props();

  let searchQuery = $state('');
  let activeTab = $state('recommended');

  const recommendedModels = [
    {
      id: 1,
      name: 'Phi-4-mini',
      description: 'Fast and efficient for quick tasks',
      size: '3.8B',
      quantization: 'Q4_K_M',
      downloadSize: '2.5 GB',
      useCase: 'Quick summaries, flashcards',
    },
    {
      id: 2,
      name: 'Qwen2.5-3B',
      description: 'Great balance of speed and quality',
      size: '3B',
      quantization: 'Q4_K_M',
      downloadSize: '2.0 GB',
      useCase: 'General study tasks',
    },
    {
      id: 3,
      name: 'Llama 3.3 8B',
      description: 'High quality for complex tasks',
      size: '8B',
      quantization: 'Q4_K_M',
      downloadSize: '5.0 GB',
      useCase: 'Study Q&A, note expansion',
    },
    {
      id: 4,
      name: 'Qwen2.5-7B',
      description: 'Excellent for multilingual tasks',
      size: '7B',
      quantization: 'Q4_K_M',
      downloadSize: '4.5 GB',
      useCase: 'Multilingual study notes',
    },
  ];

  let cloudBackends = $state([]);

  function handleModelSelect(model) {
    console.log('Selected model:', model);
  }

  function handleAddCloudBackend() {
    console.log('Add cloud backend');
  }

  function handleNewNote() {
    onNoteSelect?.({ id: null, title: '', content: '' });
  }
</script>

<div class="greetings">
  <div class="greetings-header">
    <h1>Welcome to Taker</h1>
    <p>Your local AI study companion</p>
  </div>

  <div class="greetings-content">
    <div class="quick-actions">
      <button class="action-card" onclick={handleNewNote}>
        <span class="icon">📝</span>
        <span class="title">New Note</span>
        <span class="description">Start writing</span>
      </button>

      <button class="action-card" onclick={handleAddCloudBackend}>
        <span class="icon">☁️</span>
        <span class="title">Cloud Backend</span>
        <span class="description">Add OpenAI compatible API</span>
      </button>

      <button class="action-card">
        <span class="icon">📁</span>
        <span class="title">New Vault</span>
        <span class="description">Organize notes</span>
      </button>
    </div>

    <div class="models-section">
      <div class="tabs">
        <button 
          class="tab" 
          class:active={activeTab === 'recommended'}
          onclick={() => activeTab = 'recommended'}
        >
          Recommended Models
        </button>
        <button 
          class="tab" 
          class:active={activeTab === 'search'}
          onclick={() => activeTab = 'search'}
        >
          Search HuggingFace
        </button>
        <button 
          class="tab" 
          class:active={activeTab === 'cloud'}
          onclick={() => activeTab = 'cloud'}
        >
          Cloud Backends
        </button>
      </div>

      {#if activeTab === 'recommended'}
        <div class="models-grid">
          {#each recommendedModels as model}
            <div class="model-card">
              <div class="model-header">
                <h3>{model.name}</h3>
                <span class="badge">{model.size}</span>
              </div>
              <p class="description">{model.description}</p>
              <div class="model-meta">
                <span>Quantization: {model.quantization}</span>
                <span>Download: {model.downloadSize}</span>
              </div>
              <div class="use-case">
                <strong>Best for:</strong> {model.useCase}
              </div>
              <button class="btn-download" onclick={() => handleModelSelect(model)}>
                Download
              </button>
            </div>
          {/each}
        </div>
      {:else if activeTab === 'search'}
        <div class="search-section">
          <div class="search-box">
            <input 
              type="text" 
              placeholder="Search for GGUF models on HuggingFace..." 
              bind:value={searchQuery}
            />
            <button class="btn-search">Search</button>
          </div>
          <div class="search-results">
            <p class="placeholder">Enter a search query to find models.</p>
          </div>
        </div>
      {:else if activeTab === 'cloud'}
        <div class="cloud-section">
          <button class="btn-primary" onclick={handleAddCloudBackend}>
            + Add OpenAI Compatible Backend
          </button>
          <div class="cloud-list">
            {#if cloudBackends.length === 0}
              <p class="placeholder">No cloud backends configured.</p>
            {/if}
          </div>
        </div>
      {/if}
    </div>
  </div>
</div>

<style>
  .greetings {
    display: flex;
    flex-direction: column;
    height: 100%;
    padding: var(--space-xl);
    overflow-y: auto;
  }

  .greetings-header {
    text-align: center;
    margin-bottom: var(--space-xl);
  }

  .greetings-header h1 {
    font-size: 2rem;
    font-weight: 700;
    color: var(--color-accent);
    margin-bottom: var(--space-sm);
  }

  .greetings-header p {
    color: var(--color-text-secondary);
    font-size: 1.125rem;
  }

  .greetings-content {
    flex: 1;
    max-width: 1000px;
    margin: 0 auto;
    width: 100%;
  }

  .quick-actions {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: var(--space-md);
    margin-bottom: var(--space-xl);
  }

  .action-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: var(--space-sm);
    padding: var(--space-lg);
    background-color: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    transition: all var(--transition-fast);
    text-align: center;
  }

  .action-card:hover {
    border-color: var(--color-accent);
    transform: translateY(-2px);
  }

  .action-card .icon {
    font-size: 2rem;
  }

  .action-card .title {
    font-weight: 600;
    color: var(--color-text);
  }

  .action-card .description {
    font-size: 0.875rem;
    color: var(--color-text-secondary);
  }

  .models-section {
    background-color: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
  }

  .tabs {
    display: flex;
    border-bottom: 1px solid var(--color-border);
  }

  .tab {
    flex: 1;
    padding: var(--space-md);
    font-weight: 500;
    color: var(--color-text-secondary);
    transition: all var(--transition-fast);
  }

  .tab:hover {
    background-color: var(--color-surface-hover);
  }

  .tab.active {
    color: var(--color-accent);
    border-bottom: 2px solid var(--color-accent);
  }

  .models-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: var(--space-md);
    padding: var(--space-md);
  }

  .model-card {
    padding: var(--space-md);
    background-color: var(--color-canvas);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }

  .model-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-sm);
  }

  .model-header h3 {
    font-size: 1rem;
    font-weight: 600;
  }

  .badge {
    padding: var(--space-xs) var(--space-sm);
    background-color: var(--color-accent);
    color: white;
    font-size: 0.75rem;
    font-weight: 600;
    border-radius: var(--radius-sm);
  }

  .model-card .description {
    color: var(--color-text-secondary);
    font-size: 0.875rem;
    margin-bottom: var(--space-sm);
  }

  .model-meta {
    display: flex;
    gap: var(--space-md);
    font-size: 0.75rem;
    color: var(--color-text-secondary);
    margin-bottom: var(--space-sm);
  }

  .use-case {
    font-size: 0.875rem;
    margin-bottom: var(--space-md);
  }

  .btn-download {
    width: 100%;
    padding: var(--space-sm) var(--space-md);
    background-color: var(--color-accent);
    color: white;
    border-radius: var(--radius-md);
    font-weight: 500;
    transition: background-color var(--transition-fast);
  }

  .btn-download:hover {
    background-color: var(--color-accent-hover);
  }

  .search-section,
  .cloud-section {
    padding: var(--space-md);
  }

  .search-box {
    display: flex;
    gap: var(--space-sm);
    margin-bottom: var(--space-md);
  }

  .search-box input {
    flex: 1;
    padding: var(--space-sm) var(--space-md);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background-color: var(--color-canvas);
    color: var(--color-text);
  }

  .search-box input:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  .btn-search {
    padding: var(--space-sm) var(--space-md);
    background-color: var(--color-accent);
    color: white;
    border-radius: var(--radius-md);
    font-weight: 500;
  }

  .btn-primary {
    padding: var(--space-sm) var(--space-md);
    background-color: var(--color-accent);
    color: white;
    border-radius: var(--radius-md);
    font-weight: 500;
    transition: background-color var(--transition-fast);
    margin-bottom: var(--space-md);
  }

  .btn-primary:hover {
    background-color: var(--color-accent-hover);
  }

  .search-results,
  .cloud-list {
    min-height: 200px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .placeholder {
    color: var(--color-text-secondary);
    font-style: italic;
  }
</style>
