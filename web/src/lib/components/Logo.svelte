<script lang="ts">
  import type { SvelteHTMLElements } from 'svelte/elements'

  const props: SvelteHTMLElements['svg'] = $props()
</script>

<svg viewBox="0 0 20 20" width="20" height="20" aria-hidden="true" {...props}>
  <defs>
    <radialGradient id="radial-glow" cx="35%" cy="35%" fx="35%" fy="35%">
      <stop offset="0%" stop-color="oklch(0.72 0.105 46)" />
      <stop offset="70%" stop-color="oklch(0.625 0.135 42)" />
    </radialGradient>
  </defs>
  <circle cx="10" cy="10" r="10" fill="url(#radial-glow)" />
</svg>
