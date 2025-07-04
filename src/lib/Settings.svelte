<div class="settings-container">
  <div class="settings-header">
    <button class="back-button" on:click={() => dispatch('back')}> ← Back to Menu </button>
    <h1>App Settings</h1>
    <div class="header-actions">
      <button class="save-button" class:loading={saving} on:click={saveSettings} disabled={saving}>
        {saving ? 'Saving...' : '💾 Save'}
      </button>
      <button class="reset-button" on:click={resetSettings}> 🔄 Reset to Defaults </button>
    </div>
  </div>

  {#if lastSaved}
    <div class="save-status">
      Last saved: {lastSaved}
    </div>
  {/if}

  {#if loading}
    <div class="loading-overlay">
      <div class="loading-content">
        <div class="loading-spinner"></div>
        <p>Loading settings...</p>
      </div>
    </div>
  {/if}

  <div class="settings-content">
    <form on:submit|preventDefault>
      <!-- Display Settings -->
      <fieldset>
        <legend>Display Settings</legend>
        <div class="settings-grid">
          <div class="setting-item">
            <span class="setting-label">Enable FPS Limiter:</span>
            <input
              type="checkbox"
              bind:checked={settings.default_fps_limit_enabled}
              on:change={() => scheduleAutoSave()}
            />
          </div>
          <div class="setting-item">
            <span class="setting-label">Limit:</span>
            <NumberDragBox
              bind:value={settings.default_fps_limit}
              min={1}
              max={1200}
              step={1}
              precision={0}
              on:change={() => scheduleAutoSave()}
            />
          </div>
          <div class="setting-item">
            <span class="setting-label">UI Scale:</span>
            <NumberDragBox
              bind:value={settings.ui_scale}
              min={0.5}
              max={2.0}
              step={0.1}
              precision={1}
              on:change={() => {
                applyUIScale();
                scheduleAutoSave();
              }}
            />
          </div>
        </div>
      </fieldset>

      <!-- Window Settings -->
      <fieldset>
        <legend>Window Settings</legend>
        <div class="settings-grid">
          <div class="setting-item">
            <span class="setting-label">Default Window Width:</span>
            <NumberDragBox
              bind:value={settings.window_width}
              min={800}
              max={3840}
              step={50}
              precision={0}
              on:change={() => {
                applyWindowSettings();
                scheduleAutoSave();
              }}
            />
          </div>
          <div class="setting-item">
            <span class="setting-label">Default Window Height:</span>
            <NumberDragBox
              bind:value={settings.window_height}
              min={600}
              max={2160}
              step={50}
              precision={0}
              on:change={() => {
                applyWindowSettings();
                scheduleAutoSave();
              }}
            />
          </div>
          <div class="setting-item">
            <span class="setting-label">Start Maximized:</span>
            <input
              type="checkbox"
              bind:checked={settings.window_maximized}
              on:change={() => {
                // Only save the setting, don't apply it to current window
                // This setting only affects future app launches
                scheduleAutoSave();
              }}
            />
          </div>
          <div class="setting-item">
            <span class="setting-label">Current Size:</span>
            <button
              class="capture-size-button"
              on:click={captureCurrentWindowSize}
              title="Set default window size to current window size"
            >
              📏 Use Current Size
            </button>
          </div>
        </div>
      </fieldset>

      <!-- UI Behavior Settings -->
      <fieldset>
        <legend>UI Behavior</legend>
        <div class="settings-grid">
          <div class="setting-item">
            <span class="setting-label">Auto-hide UI:</span>
            <input
              type="checkbox"
              bind:checked={settings.auto_hide_ui}
              on:change={() => scheduleAutoSave()}
            />
          </div>
          <div class="setting-item">
            <span class="setting-label">Auto-hide Delay (ms):</span>
            <NumberDragBox
              bind:value={settings.auto_hide_delay}
              min={1000}
              max={10000}
              step={500}
              precision={0}
              on:change={() => scheduleAutoSave()}
            />
          </div>
          <div class="setting-item">
            <span class="setting-label">Menu Position:</span>
            <select bind:value={settings.menu_position} on:change={() => scheduleAutoSave()}>
              <option value="left">Left</option>
              <option value="middle">Middle</option>
              <option value="right">Right</option>
            </select>
          </div>
        </div>
      </fieldset>

      <!-- Camera Settings -->
      <fieldset>
        <legend>Camera Settings</legend>
        <div class="settings-grid">
          <div class="setting-item">
            <span class="setting-label">Camera Sensitivity:</span>
            <NumberDragBox
              bind:value={settings.default_camera_sensitivity}
              min={0.1}
              max={5.0}
              step={0.1}
              precision={1}
              on:change={() => scheduleAutoSave()}
            />
          </div>
        </div>
      </fieldset>
    </form>
  </div>
</div>

<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import NumberDragBox from './components/inputs/NumberDragBox.svelte';
  import './shared-theme.css';

  const dispatch = createEventDispatcher();

  // App-wide settings
  let settings = {
    // Display Settings
    default_fps_limit: 60,
    default_fps_limit_enabled: false,

    // Window Settings
    window_width: 1200,
    window_height: 800,
    window_maximized: false,

    // UI Settings
    ui_scale: 1.0,
    auto_hide_ui: true,
    auto_hide_delay: 3000,
    menu_position: 'middle',

    // Camera Settings
    default_camera_sensitivity: 1.0,
  };

  // Loading and saving state
  let loading = false;
  let saving = false;
  let lastSaved = '';

  // Load settings from backend
  async function loadSettings() {
    loading = true;
    try {
      const loadedSettings = await invoke('get_app_settings');
      if (loadedSettings) {
        settings = { ...settings, ...loadedSettings };
      }
      console.log('Settings loaded successfully');
    } catch (e) {
      console.error('Failed to load settings:', e);
    } finally {
      loading = false;
    }
  }

  // Save settings to backend
  async function saveSettings() {
    saving = true;
    try {
      await invoke('save_app_settings', { settings });
      lastSaved = new Date().toLocaleTimeString();
      console.log('Settings saved successfully');
      // Dispatch event to notify parent component of settings change
      dispatch('settingsChanged', settings);
    } catch (e) {
      console.error('Failed to save settings:', e);
    } finally {
      saving = false;
    }
  }

  // Reset settings to defaults
  async function resetSettings() {
    if (
      confirm('Are you sure you want to reset all settings to defaults? This cannot be undone.')
    ) {
      try {
        await invoke('reset_app_settings');
        await loadSettings(); // Reload the default settings
        console.log('Settings reset to defaults');
      } catch (e) {
        console.error('Failed to reset settings:', e);
      }
    }
  }

  // Auto-save when settings change
  let saveTimeout: number | null = null;
  function scheduleAutoSave() {
    if (saveTimeout) {
      clearTimeout(saveTimeout);
    }
    saveTimeout = window.setTimeout(() => {
      saveSettings();
    }, 1000); // Auto-save after 1 second of no changes
  }

  // Apply window settings immediately when changed
  async function applyWindowSettings() {
    try {
      await invoke('apply_window_settings');
      console.log('Window settings applied immediately');
    } catch (e) {
      console.error('Failed to apply window settings:', e);
    }
  }

  // Capture current window size and set as default
  async function captureCurrentWindowSize() {
    try {
      const currentSize = await invoke<{ width: number; height: number; maximized: boolean }>(
        'get_current_window_size'
      );
      settings.window_width = currentSize.width;
      settings.window_height = currentSize.height;
      // Don't change the maximized setting - only capture the size
      // settings.window_maximized = currentSize.maximized;

      // Save the settings immediately
      await saveSettings();
      console.log('Current window size captured and saved');
    } catch (e) {
      console.error('Failed to capture current window size:', e);
    }
  }

  // Apply UI scale immediately when changed
  async function applyUIScale() {
    try {
      await invoke('set_webview_zoom', { zoomFactor: settings.ui_scale });
      console.log('UI scale applied immediately:', settings.ui_scale);
    } catch (e) {
      console.error('Failed to apply UI scale:', e);
    }
  }

  onMount(() => {
    loadSettings();
  });
</script>

<style>
  .settings-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: rgba(0, 0, 0, 0.8);
    color: rgba(255, 255, 255, 0.87);
  }

  .settings-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    background-color: rgb(255 255 255 / 30%);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .settings-header h1 {
    margin: 0;
    font-size: 1.5rem;
    color: rgba(255, 255, 255, 0.9);
  }

  .header-actions {
    display: flex;
    gap: 0.5rem;
  }

  .back-button {
    padding: 0.5rem 1rem;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    color: rgba(255, 255, 255, 0.9);
    cursor: pointer;
    font-family: inherit;
    transition: all 0.3s ease;
    font-size: 1em;
  }

  .back-button:hover {
    background: rgba(255, 255, 255, 0.2);
    border-color: rgba(255, 255, 255, 0.4);
  }

  .save-button {
    padding: 0.5rem 1rem;
    background: rgba(81, 207, 102, 0.2);
    border: 1px solid rgba(81, 207, 102, 0.4);
    border-radius: 4px;
    color: #51cf66;
    cursor: pointer;
    font-family: inherit;
    transition: all 0.3s ease;
    font-size: 1em;
  }

  .save-button:hover:not(.loading) {
    background: rgba(81, 207, 102, 0.3);
    border-color: rgba(81, 207, 102, 0.6);
  }

  .save-button.loading {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .reset-button {
    padding: 0.5rem 1rem;
    background: rgba(255, 107, 107, 0.2);
    border: 1px solid rgba(255, 107, 107, 0.4);
    border-radius: 4px;
    color: #ff6b6b;
    cursor: pointer;
    font-family: inherit;
    transition: all 0.3s ease;
    font-size: 1em;
  }

  .reset-button:hover {
    background: rgba(255, 107, 107, 0.3);
    border-color: rgba(255, 107, 107, 0.6);
  }

  .save-status {
    padding: 0.5rem 1rem;
    background: rgba(81, 207, 102, 0.1);
    border-bottom: 1px solid rgba(81, 207, 102, 0.2);
    color: #51cf66;
    font-size: 0.875rem;
    text-align: center;
  }

  .loading-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.8);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .loading-content {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    padding: 2rem;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 8px;
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .loading-spinner {
    width: 40px;
    height: 40px;
    border: 4px solid rgba(255, 255, 255, 0.3);
    border-top: 4px solid #646cff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  .settings-content {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
  }

  fieldset {
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    padding: 1rem;
    margin-bottom: 1rem;
    background: rgba(255, 255, 255, 0.05);
  }

  legend {
    font-weight: bold;
    padding: 0 0.5rem;
    color: rgba(255, 255, 255, 0.9);
    font-size: 1em;
  }

  .settings-grid {
    display: grid;
    grid-template-columns: 250px 150px;
    gap: 0.5rem 1rem;
    align-items: center;
  }

  .setting-item {
    display: contents;
  }

  .setting-label {
    font-weight: 500;
    color: rgba(255, 255, 255, 0.9);
    padding: 0.5rem 0;
    font-size: 1em;
  }

  .setting-item input[type='checkbox'] {
    width: auto;
    margin: 0;
  }

  .setting-item select {
    padding: 0.5rem;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    color: rgba(255, 255, 255, 0.9);
    font-family: inherit;
    font-size: 0.875rem;
    cursor: pointer;
    appearance: none;
    -webkit-appearance: none;
    -moz-appearance: none;
    background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='rgba(255,255,255,0.6)' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6,9 12,15 18,9'%3e%3c/polyline%3e%3c/svg%3e");
    background-repeat: no-repeat;
    background-position: right 0.5rem center;
    background-size: 1rem;
    padding-right: 2rem;
  }

  .setting-item select:focus {
    outline: none;
    border-color: rgba(255, 255, 255, 0.5);
    background-color: rgba(255, 255, 255, 0.15);
  }

  .setting-item select:hover {
    background-color: rgba(255, 255, 255, 0.15);
  }

  .setting-item select option {
    background: #2a2a2a;
    color: rgba(255, 255, 255, 0.9);
  }

  .capture-size-button {
    padding: 0.5rem 1rem;
    background: rgba(100, 149, 237, 0.2);
    border: 1px solid rgba(100, 149, 237, 0.4);
    border-radius: 4px;
    color: #6495ed;
    cursor: pointer;
    font-family: inherit;
    transition: all 0.3s ease;
    font-size: 0.875rem;
    white-space: nowrap;
  }

  .capture-size-button:hover {
    background: rgba(100, 149, 237, 0.3);
    border-color: rgba(100, 149, 237, 0.6);
  }

  /* Responsive design */
  @media (max-width: 768px) {
    .settings-grid {
      grid-template-columns: 1fr;
      gap: 0.25rem;
    }

    .setting-item {
      display: flex;
      flex-direction: column;
      gap: 0.25rem;
      padding: 0.5rem 0;
    }

    .setting-label {
      padding: 0;
    }
  }
</style>
