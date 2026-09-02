<script>
  import { invoke } from '@tauri-apps/api/core';

  let { audioSrc = null, title = '', onClose } = $props();

  let isPlaying = $state(false);
  let currentTime = $state(0);
  let duration = $state(0);
  let volume = $state(1);
  let playbackRate = $state(1);

  let audioElement;

  $effect(() => {
    if (audioElement && audioSrc) {
      audioElement.src = audioSrc;
      audioElement.load();
    }
  });

  function togglePlay() {
    if (!audioElement) return;
    
    if (isPlaying) {
      audioElement.pause();
    } else {
      audioElement.play();
    }
    isPlaying = !isPlaying;
  }

  function handleTimeUpdate() {
    if (audioElement) {
      currentTime = audioElement.currentTime;
    }
  }

  function handleLoadedMetadata() {
    if (audioElement) {
      duration = audioElement.duration;
    }
  }

  function handleEnded() {
    isPlaying = false;
    currentTime = 0;
  }

  function handleSeek(e) {
    const rect = e.target.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const percent = x / rect.width;
    if (audioElement) {
      audioElement.currentTime = percent * duration;
    }
  }

  function handleVolumeChange(e) {
    volume = parseFloat(e.target.value);
    if (audioElement) {
      audioElement.volume = volume;
    }
  }

  function changePlaybackRate(rate) {
    playbackRate = rate;
    if (audioElement) {
      audioElement.playbackRate = rate;
    }
  }

  function formatTime(secs) {
    const mins = Math.floor(secs / 60);
    const remaining = Math.floor(secs % 60);
    return `${mins}:${remaining.toString().padStart(2, '0')}`;
  }

  function skipBackward() {
    if (audioElement) {
      audioElement.currentTime = Math.max(0, currentTime - 10);
    }
  }

  function skipForward() {
    if (audioElement) {
      audioElement.currentTime = Math.min(duration, currentTime + 10);
    }
  }
</script>

<div class="audio-player">
  <audio 
    bind:this={audioElement}
    ontimeupdate={handleTimeUpdate}
    onloadedmetadata={handleLoadedMetadata}
    onended={handleEnded}
  ></audio>

  <div class="player-header">
    <span class="title">{title || 'Audio Player'}</span>
    {#if onClose}
      <button class="close-btn" onclick={onClose}>×</button>
    {/if}
  </div>

  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <div class="player-progress" onclick={handleSeek}>
    <div class="progress-bar">
      <div class="progress-fill" style="width: {duration ? (currentTime / duration) * 100 : 0}%"></div>
    </div>
    <div class="time-display">
      <span>{formatTime(currentTime)}</span>
      <span>{formatTime(duration)}</span>
    </div>
  </div>

  <div class="player-controls">
    <div class="controls-left">
      <button class="control-btn" onclick={skipBackward} title="Skip back 10s">
        ⏪
      </button>
      <button class="play-btn" onclick={togglePlay}>
        {isPlaying ? '⏸️' : '▶️'}
      </button>
      <button class="control-btn" onclick={skipForward} title="Skip forward 10s">
        ⏩
      </button>
    </div>

    <div class="controls-center">
      <div class="speed-controls">
        <button 
          class="speed-btn" 
          class:active={playbackRate === 0.5}
          onclick={() => changePlaybackRate(0.5)}
        >
          0.5x
        </button>
        <button 
          class="speed-btn" 
          class:active={playbackRate === 1}
          onclick={() => changePlaybackRate(1)}
        >
          1x
        </button>
        <button 
          class="speed-btn" 
          class:active={playbackRate === 1.5}
          onclick={() => changePlaybackRate(1.5)}
        >
          1.5x
        </button>
        <button 
          class="speed-btn" 
          class:active={playbackRate === 2}
          onclick={() => changePlaybackRate(2)}
        >
          2x
        </button>
      </div>
    </div>

    <div class="controls-right">
      <span class="volume-icon">🔊</span>
      <input 
        type="range" 
        class="volume-slider"
        min="0" 
        max="1" 
        step="0.1" 
        value={volume}
        oninput={handleVolumeChange}
      />
    </div>
  </div>
</div>

<style>
  .audio-player {
    background-color: var(--color-surface);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: var(--space-md);
  }

  .player-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: var(--space-md);
  }

  .title {
    font-weight: 600;
    color: var(--color-text);
  }

  .close-btn {
    width: 28px;
    height: 28px;
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

  .player-progress {
    cursor: pointer;
    margin-bottom: var(--space-md);
  }

  .progress-bar {
    height: 6px;
    background-color: var(--color-border);
    border-radius: var(--radius-sm);
    overflow: hidden;
    margin-bottom: var(--space-xs);
  }

  .progress-fill {
    height: 100%;
    background-color: var(--color-accent);
    transition: width 0.1s linear;
  }

  .time-display {
    display: flex;
    justify-content: space-between;
    font-size: 0.75rem;
    color: var(--color-text-secondary);
  }

  .player-controls {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .controls-left, .controls-right {
    display: flex;
    align-items: center;
    gap: var(--space-sm);
  }

  .control-btn {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: var(--radius-md);
    font-size: 1rem;
    transition: background-color var(--transition-fast);
  }

  .control-btn:hover {
    background-color: var(--color-surface-hover);
  }

  .play-btn {
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    background-color: var(--color-accent);
    color: white;
    border-radius: 50%;
    font-size: 1.25rem;
    transition: background-color var(--transition-fast);
  }

  .play-btn:hover {
    background-color: var(--color-accent-hover);
  }

  .speed-controls {
    display: flex;
    gap: var(--space-xs);
    background-color: var(--color-canvas);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: 2px;
  }

  .speed-btn {
    padding: var(--space-xs) var(--space-sm);
    border-radius: var(--radius-sm);
    font-size: 0.75rem;
    font-weight: 500;
    color: var(--color-text-secondary);
    transition: all var(--transition-fast);
  }

  .speed-btn:hover {
    background-color: var(--color-surface-hover);
  }

  .speed-btn.active {
    background-color: var(--color-accent);
    color: white;
  }

  .volume-icon {
    font-size: 0.875rem;
  }

  .volume-slider {
    width: 80px;
    height: 4px;
    -webkit-appearance: none;
    background-color: var(--color-border);
    border-radius: var(--radius-sm);
    outline: none;
  }

  .volume-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 12px;
    height: 12px;
    background-color: var(--color-accent);
    border-radius: 50%;
    cursor: pointer;
  }
</style>
