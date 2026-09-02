<script>
  import { invoke } from '@tauri-apps/api/core';

  let activeSection = $state('appearance');
  let settings = $state({
    theme: 'light',
    fontSize: 'medium',
    showPreview: true,
    autoSave: true,
    autoSaveInterval: 30,
  });

  const sections = [
    { id: 'appearance', name: 'Appearance', icon: '🎨' },
    { id: 'editor', name: 'Editor', icon: '📝' },
    { id: 'ai', name: 'AI Models', icon: '🤖' },
    { id: 'audio', name: 'Audio', icon: '🔊' },
    { id: 'plugins', name: 'Plugins', icon: '🧩' },
    { id: 'about', name: 'About', icon: 'ℹ️' },
  ];

  async function loadSettings() {
    try {
      const allSettings = await invoke('get_all_settings');
      for (const s of allSettings) {
        if (s.key in settings) {
          try {
            settings[s.key] = JSON.parse(s.value);
          } catch {
            settings[s.key] = s.value;
          }
        }
      }
    } catch (e) {
      console.error('Failed to load settings:', e);
    }
  }

  async function handleSave() {
    try {
      for (const [key, value] of Object.entries(settings)) {
        await invoke('set_setting', { key, value: JSON.stringify(value) });
      }
    } catch (e) {
      console.error('Failed to save settings:', e);
    }
  }

  $effect(() => {
    loadSettings();
  });
</script>

<div class="settings">
  <div class="settings-header">
    <h2>Settings</h2>
  </div>

  <div class="settings-content">
    <nav class="settings-nav">
      {#each sections as section}
        <button 
          class="nav-item"
          class:active={activeSection === section.id}
          onclick={() => activeSection = section.id}
        >
          <span class="icon">{section.icon}</span>
          <span>{section.name}</span>
        </button>
      {/each}
    </nav>

    <div class="settings-panel">
      {#if activeSection === 'appearance'}
        <div class="section">
          <h3>Appearance</h3>
          
          <div class="setting">
            <label for="theme-select">Theme</label>
            <select id="theme-select" bind:value={settings.theme}>
              <option value="light">Light</option>
              <option value="dark">Dark</option>
              <option value="system">System</option>
            </select>
          </div>

          <div class="setting">
            <label for="font-size-select">Font Size</label>
            <select id="font-size-select" bind:value={settings.fontSize}>
              <option value="small">Small</option>
              <option value="medium">Medium</option>
              <option value="large">Large</option>
            </select>
          </div>
        </div>
      {:else if activeSection === 'editor'}
        <div class="section">
          <h3>Editor</h3>
          
          <div class="setting">
            <label>
              <input type="checkbox" bind:checked={settings.showPreview} />
              Show live preview
            </label>
          </div>

          <div class="setting">
            <label>
              <input type="checkbox" bind:checked={settings.autoSave} />
              Auto-save
            </label>
          </div>

          {#if settings.autoSave}
            <div class="setting">
              <label for="autosave-interval">Auto-save interval (seconds)</label>
              <input id="autosave-interval" type="number" bind:value={settings.autoSaveInterval} min="5" max="300" />
            </div>
          {/if}
        </div>
      {:else if activeSection === 'ai'}
        <div class="section">
          <h3>AI Models</h3>
          <p class="description">Configure your local and cloud AI models.</p>
          
          <button class="btn-primary">+ Add Cloud Backend</button>
          
          <div class="models-list">
            <p class="placeholder">No models configured yet.</p>
          </div>
        </div>
      {:else if activeSection === 'audio'}
        <div class="section">
          <h3>Audio</h3>
          <p class="description">Configure speech-to-text and text-to-speech settings.</p>
          
          <div class="setting">
            <label for="tts-voice">Default TTS Voice</label>
            <select id="tts-voice">
              <option>Default</option>
            </select>
          </div>

          <div class="setting">
            <label for="stt-lang">Speech Recognition Language</label>
            <select id="stt-lang">
              <option>Auto-detect</option>
              <option>English</option>
              <option>German</option>
              <option>Spanish</option>
            </select>
          </div>
        </div>
      {:else if activeSection === 'plugins'}
        <div class="section">
          <h3>Plugins</h3>
          <p class="description">Manage installed plugins and browse the marketplace.</p>
          
          <button class="btn-primary">Browse Marketplace</button>
          
          <div class="plugins-list">
            <p class="placeholder">No plugins installed.</p>
          </div>
        </div>
      {:else if activeSection === 'about'}
        <div class="section">
          <h3>About Taker</h3>
          <p class="description">Local AI Note Taker & Study Companion</p>
          
          <div class="info">
            <p><strong>Version:</strong> 0.1.0</p>
            <p><strong>License:</strong> MIT</p>
            <p><strong>Repository:</strong> <a href="https://github.com/taker-app/taker">GitHub</a></p>
          </div>
        </div>
      {/if}

      <div class="settings-footer">
        <button class="btn-primary" onclick={handleSave}>Save Settings</button>
      </div>
    </div>
  </div>
</div>

<style>
  .settings {
    display: flex;
    flex-direction: column;
    height: 100%;
    background-color: var(--color-canvas);
  }

  .settings-header {
    padding: var(--space-md) var(--space-lg);
    border-bottom: 1px solid var(--color-border);
  }

  .settings-header h2 {
    font-size: 1.25rem;
    font-weight: 600;
  }

  .settings-content {
    display: flex;
    flex: 1;
    overflow: hidden;
  }

  .settings-nav {
    width: 200px;
    padding: var(--space-sm);
    border-right: 1px solid var(--color-border);
    background-color: var(--color-surface);
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

  .settings-panel {
    flex: 1;
    padding: var(--space-lg);
    overflow-y: auto;
  }

  .section {
    margin-bottom: var(--space-xl);
  }

  .section h3 {
    font-size: 1rem;
    font-weight: 600;
    margin-bottom: var(--space-md);
  }

  .description {
    color: var(--color-text-secondary);
    margin-bottom: var(--space-md);
  }

  .setting {
    margin-bottom: var(--space-md);
  }

  .setting label {
    display: block;
    margin-bottom: var(--space-xs);
    font-weight: 500;
  }

  .setting select,
  .setting input[type="number"] {
    width: 100%;
    max-width: 300px;
    padding: var(--space-sm) var(--space-md);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    background-color: var(--color-canvas);
    color: var(--color-text);
  }

  .setting input[type="checkbox"] {
    margin-right: var(--space-sm);
  }

  .btn-primary {
    padding: var(--space-sm) var(--space-md);
    background-color: var(--color-accent);
    color: white;
    border-radius: var(--radius-md);
    font-weight: 500;
    transition: background-color var(--transition-fast);
  }

  .btn-primary:hover {
    background-color: var(--color-accent-hover);
  }

  .models-list,
  .plugins-list {
    margin-top: var(--space-md);
    padding: var(--space-md);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
  }

  .placeholder {
    color: var(--color-text-secondary);
    font-style: italic;
  }

  .info {
    padding: var(--space-md);
    background-color: var(--color-surface);
    border-radius: var(--radius-md);
  }

  .info p {
    margin-bottom: var(--space-sm);
  }

  .settings-footer {
    margin-top: var(--space-xl);
    padding-top: var(--space-md);
    border-top: 1px solid var(--color-border);
  }
</style>
