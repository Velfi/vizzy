{#if showUI}
  <div class={containerClass}>
    <div class="simulation-controls">
      <slot />
    </div>
  </div>
{/if}

<script lang="ts">
  import { onMount, onDestroy } from 'svelte';

  export let position: string = 'middle'; // 'left', 'middle', 'right'
  export let showUI: boolean = true;

  $: containerClass = `simulation-menu-container ${position}`;

  // Apply body classes for left/right positioning
  onMount(() => {
    updateBodyClass();
  });

  onDestroy(() => {
    // Clean up body classes
    document.body.classList.remove('menu-left', 'menu-right');
  });

  $: {
    // Update body class when position changes
    if (typeof document !== 'undefined') {
      updateBodyClass();
    }
  }

  function updateBodyClass() {
    // Remove existing menu classes
    document.body.classList.remove('menu-left', 'menu-right');

    // Add new class based on position
    if (position === 'left') {
      document.body.classList.add('menu-left');
    } else if (position === 'right') {
      document.body.classList.add('menu-right');
    }
  }
</script>

<style>
  .simulation-menu-container {
    position: relative;
    padding: 0.5rem;
    background: rgba(0, 0, 0, 0.8);
    backdrop-filter: blur(10px);
  }

  .simulation-menu-container.left {
    position: fixed;
    left: 0;
    top: 60px; /* Below the fixed control bar */
    bottom: 0;
    max-width: 40%;
    min-width: 600px;
    overflow-y: auto;
  }

  .simulation-menu-container.middle {
    max-width: 40%;
    overflow-y: auto;
    margin: 60px auto 0 auto; /* Account for fixed control bar */
  }

  .simulation-menu-container.right {
    position: fixed;
    right: 0;
    top: 60px; /* Below the fixed control bar */
    bottom: 0;
    max-width: 40%;
    min-width: 600px;
    overflow-y: auto;
  }

  .simulation-controls {
    width: 100%;
  }

  /* Responsive adjustments */
  @media (max-width: 1024px) {
    .simulation-menu-container.left,
    .simulation-menu-container.right {
      position: relative;
      width: 100%;
      max-width: 800px;
      margin: 60px auto 0 auto; /* Account for fixed control bar */
      top: auto;
      bottom: auto;
    }
  }

  @media (max-width: 600px) {
    .simulation-menu-container.left,
    .simulation-menu-container.right,
    .simulation-menu-container.middle {
      margin: 50px auto 0 auto; /* Account for smaller fixed control bar on mobile */
    }
  }

  /* Ensure proper spacing for left/right positioned menus */
  .simulation-menu-container.left .simulation-controls,
  .simulation-menu-container.right .simulation-controls {
    padding: 0;
  }

  /* Adjust fieldset styling for side panels */
  .simulation-menu-container.left :global(fieldset),
  .simulation-menu-container.right :global(fieldset) {
    margin-bottom: 0.5rem;
  }

  /* Make sure content doesn't get too cramped in side panels */
  .simulation-menu-container.left :global(.control-group),
  .simulation-menu-container.right :global(.control-group) {
    flex-direction: column;
    align-items: stretch;
    gap: 0.25rem;
  }
</style>
