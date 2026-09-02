<script>
  import { invoke } from '@tauri-apps/api/core';

  let { show = false, onClose } = $props();

  let marketplacePlugins = $state([]);
  let installedPlugins = $state([]);
  let activeTab = $state('marketplace');
  let isLoading = $state(false);
  let installingPlugin = $state(null);

  async function loadMarketplacePlugins() {
    isLoading = true;
    try {
      marketplacePlugins = await invoke('get_marketplace_plugins');
    } catch (e) {
      console.error('Failed to load marketplace:', e);
    } finally {
      isLoading = false;
    }
  }

  async function loadInstalledPlugins() {
    try {
      installedPlugins = await invoke('get_plugins');
    } catch (e) {
      console.error('Failed to load installed plugins:', e);
    }
  }

  async function handleInstallPlugin(plugin) {
    installingPlugin = plugin.id;
    try {
      await invoke('install_plugin', {
        pluginId: plugin.id,
        name: plugin.name,
        version: plugin.version,
        description: plugin.description,
        author: plugin.author,
        pluginType: plugin.plugin_type,
      });
      await loadInstalledPlugins();
    } catch (e) {
      console.error('Failed to install plugin:', e);
    } finally {
      installingPlugin = null;
    }
  }

  async function handleUninstallPlugin(plugin) {
    if (!confirm(`Uninstall ${plugin.name}?`)) return;
    
    try {
      await invoke('uninstall_plugin', { id: plugin.id });
      await loadInstalledPlugins();
    } catch (e) {
      console.error('Failed to uninstall plugin:', e);
    }
  }

  async function handleTogglePlugin(plugin) {
    try {
      if (plugin.is_enabled) {
        await invoke('disable_plugin', { id: plugin.id });
      } else {
        await invoke('enable_plugin', { id: plugin.id });
      }
      await loadInstalledPlugins();
    } catch (e) {
      console.error('Failed to toggle plugin:', e);
    }
  }

  $effect(() => {
    if (show) {
      loadMarketplacePlugins();
      loadInstalledPlugins();
    }
  });
</script>

{#if show}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="modal-overlay" onclick={onClose}>
    <!-- svelte-ignore a11y_click_events_have_key_events -->
    <!-- svelte-ignore a11y_no_static_element_interactions -->
    <div class="modal" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h2>Plugin Marketplace</h2>
        <button class="close-btn" onclick={onClose}>×</button>
      </div>

      <div class="tabs">
        <button 
          class="tab" 
          class:active={activeTab === 'marketplace'}
          onclick={() => activeTab = 'marketplace'}
        >
          Marketplace
        </button>
        <button 
          class="tab" 
          class:active={activeTab === 'installed'}
          onclick={() => activeTab = 'installed'}
        >
          Installed ({installedPlugins.length})
        </button>
      </div>

      <div class="modal-body">
        {#if activeTab === 'marketplace'}
          {#if isLoading}
            <div class="loading">Loading plugins...</div>
          {:else}
            <div class="plugins-list">
              {#each marketplacePlugins as plugin}
                <div class="plugin-card">
                  <div class="plugin-info">
                    <div class="plugin-header">
                      <h3>{plugin.name}</h3>
                      <span class="plugin-type">{plugin.plugin_type}</span>
                    </div>
                    <p class="description">{plugin.description}</p>
                    <div class="plugin-meta">
                      <span class="author">by {plugin.author}</span>
                      <span class="stats">↓ {plugin.downloads.toLocaleString()}</span>
                      <span class="rating">★ {plugin.rating}</span>
                    </div>
                  </div>
                  <button 
                    class="btn-install"
                    onclick={() => handleInstallPlugin(plugin)}
                    disabled={installingPlugin === plugin.id}
                  >
                    {installingPlugin === plugin.id ? 'Installing...' : 'Install'}
                  </button>
                </div>
              {/each}
            </div>
          {/if}
        {:else}
          <div class="plugins-list">
            {#if installedPlugins.length === 0}
              <div class="empty-state">
                <p>No plugins installed yet.</p>
                <p>Browse the marketplace to find plugins.</p>
              </div>
            {:else}
              {#each installedPlugins as plugin}
                <div class="plugin-card installed">
                  <div class="plugin-info">
                    <div class="plugin-header">
                      <h3>{plugin.name}</h3>
                      <span class="version">v{plugin.version}</span>
                    </div>
                    <p class="manifest-path">{plugin.manifest_path}</p>
                  </div>
                  <div class="plugin-actions">
                    <button 
                      class="btn-toggle"
                      class:enabled={plugin.is_enabled}
                      onclick={() => handleTogglePlugin(plugin)}
                    >
                      {plugin.is_enabled ? 'Enabled' : 'Disabled'}
                    </button>
                    <button 
                      class="btn-uninstall"
                      onclick={() => handleUninstallPlugin(plugin)}
                    >
                      Uninstall
                    </button>
                  </div>
                </div>
              {/each}
            {/if}
          </div>
        {/if}
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
    max-width: 700px;
    max-height: 80vh;
    background-color: var(--color-surface);
    border-radius: var(--radius-lg);
    box-shadow: 0 4px 24px rgba(0, 0, 0, 0.15);
    display: flex;
    flex-direction: column;
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

  .tabs {
    display: flex;
    border-bottom: 1px solid var(--color-border);
    padding: 0 var(--space-lg);
  }

  .tab {
    padding: var(--space-md);
    font-weight: 500;
    color: var(--color-text-secondary);
    transition: all var(--transition-fast);
    border-bottom: 2px solid transparent;
  }

  .tab:hover {
    color: var(--color-text);
  }

  .tab.active {
    color: var(--color-accent);
    border-bottom-color: var(--color-accent);
  }

  .modal-body {
    flex: 1;
    overflow-y: auto;
    padding: var(--space-lg);
  }

  .loading {
    text-align: center;
    padding: var(--space-xl);
    color: var(--color-text-secondary);
  }

  .plugins-list {
    display: flex;
    flex-direction: column;
    gap: var(--space-md);
  }

  .plugin-card {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: var(--space-md);
    background-color: var(--color-canvas);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }

  .plugin-card.installed {
    background-color: var(--color-surface);
  }

  .plugin-info {
    flex: 1;
  }

  .plugin-header {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
    margin-bottom: var(--space-xs);
  }

  .plugin-header h3 {
    font-size: 1rem;
    font-weight: 600;
    color: var(--color-text);
  }

  .plugin-type, .version {
    padding: 2px var(--space-xs);
    background-color: var(--color-accent);
    color: white;
    font-size: 0.75rem;
    border-radius: var(--radius-sm);
  }

  .version {
    background-color: var(--color-text-secondary);
  }

  .description {
    font-size: 0.875rem;
    color: var(--color-text-secondary);
    margin-bottom: var(--space-sm);
  }

  .plugin-meta {
    display: flex;
    gap: var(--space-md);
    font-size: 0.75rem;
    color: var(--color-text-secondary);
  }

  .manifest-path {
    font-size: 0.75rem;
    color: var(--color-text-secondary);
    font-family: var(--font-mono);
  }

  .btn-install {
    padding: var(--space-sm) var(--space-md);
    background-color: var(--color-accent);
    color: white;
    border-radius: var(--radius-md);
    font-weight: 500;
    transition: background-color var(--transition-fast);
  }

  .btn-install:hover:not(:disabled) {
    background-color: var(--color-accent-hover);
  }

  .btn-install:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .plugin-actions {
    display: flex;
    gap: var(--space-sm);
  }

  .btn-toggle {
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-sm);
    font-size: 0.75rem;
    font-weight: 500;
    background-color: var(--color-border);
    color: var(--color-text-secondary);
    transition: all var(--transition-fast);
  }

  .btn-toggle.enabled {
    background-color: var(--color-success);
    color: white;
  }

  .btn-uninstall {
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-sm);
    font-size: 0.75rem;
    font-weight: 500;
    background-color: var(--color-warning);
    color: white;
    transition: background-color var(--transition-fast);
  }

  .btn-uninstall:hover {
    opacity: 0.8;
  }

  .empty-state {
    text-align: center;
    padding: var(--space-xl);
    color: var(--color-text-secondary);
  }

  .empty-state p {
    margin-bottom: var(--space-sm);
  }
</style>
