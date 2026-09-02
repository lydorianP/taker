<script>
  import { invoke } from '@tauri-apps/api/core';

  let { show = false, onClose, onBackendAdded } = $props();

  let name = $state('');
  let endpoint = $state('');
  let apiKey = $state('');
  let modelName = $state('');
  let isSaving = $state(false);

  async function handleSave() {
    if (!name.trim() || !endpoint.trim() || !modelName.trim()) return;
    
    isSaving = true;
    try {
      await invoke('add_cloud_backend', {
        name: name.trim(),
        endpoint: endpoint.trim(),
        apiKey: apiKey.trim(),
        modelName: modelName.trim(),
      });
      
      name = '';
      endpoint = '';
      apiKey = '';
      modelName = '';
      
      onBackendAdded?.();
      onClose?.();
    } catch (e) {
      console.error('Failed to add cloud backend:', e);
    } finally {
      isSaving = false;
    }
  }

  function handleCancel() {
    name = '';
    endpoint = '';
    apiKey = '';
    modelName = '';
    onClose?.();
  }
</script>

{#if show}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-overlay" onclick={handleCancel}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h2>Add Cloud Backend</h2>
        <button class="close-btn" onclick={handleCancel}>×</button>
      </div>
      
      <div class="modal-body">
        <p class="description">
          Connect to any OpenAI-compatible API (OpenAI, Anthropic, local servers, etc.)
        </p>
        
        <div class="form-group">
          <label for="name">Name</label>
          <input 
            id="name"
            type="text" 
            placeholder="My API" 
            bind:value={name}
          />
        </div>
        
        <div class="form-group">
          <label for="endpoint">Endpoint URL</label>
          <input 
            id="endpoint"
            type="url" 
            placeholder="https://api.openai.com/v1" 
            bind:value={endpoint}
          />
        </div>
        
        <div class="form-group">
          <label for="apiKey">API Key</label>
          <input 
            id="apiKey"
            type="password" 
            placeholder="sk-..." 
            bind:value={apiKey}
          />
        </div>
        
        <div class="form-group">
          <label for="modelName">Model Name</label>
          <input 
            id="modelName"
            type="text" 
            placeholder="gpt-4" 
            bind:value={modelName}
          />
        </div>
      </div>
      
      <div class="modal-footer">
        <button class="btn-secondary" onclick={handleCancel}>Cancel</button>
        <button 
          class="btn-primary" 
          onclick={handleSave}
          disabled={isSaving || !name.trim() || !endpoint.trim() || !modelName.trim()}
        >
          {isSaving ? 'Adding...' : 'Add Backend'}
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .modal {
    width: 100%;
    max-width: 480px;
    background-color: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: 0 4px 24px rgba(0, 0, 0, 0.15);
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-lg);
    border-bottom: 1px solid var(--color-border);
  }

  .modal-header h2 {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--color-text);
  }

  .close-btn {
    width: 32px;
    height: 32px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-sm);
    font-size: 1.25rem;
    color: var(--color-text-secondary);
    transition: all var(--transition-fast);
  }

  .close-btn:hover {
    background-color: var(--color-surface-hover);
    color: var(--color-text);
  }

  .modal-body {
    padding: var(--space-lg);
  }

  .description {
    color: var(--color-text-secondary);
    font-size: 0.875rem;
    margin-bottom: var(--space-lg);
  }

  .form-group {
    margin-bottom: var(--space-md);
  }

  .form-group label {
    display: block;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--color-text);
    margin-bottom: var(--space-xs);
  }

  .form-group input {
    width: 100%;
    padding: var(--space-sm) var(--space-md);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background-color: var(--color-canvas);
    color: var(--color-text);
    font-size: 0.875rem;
  }

  .form-group input:focus {
    outline: none;
    border-color: var(--color-accent);
  }

  .form-group input::placeholder {
    color: var(--color-text-secondary);
  }

  .modal-footer {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-sm);
    padding: var(--space-lg);
    border-top: 1px solid var(--color-border);
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
</style>
