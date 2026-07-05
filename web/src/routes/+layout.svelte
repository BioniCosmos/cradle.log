<script lang="ts">
  import { page } from '$app/state'
  import Logo from '$lib/components/Logo.svelte'
  import { Button } from '$lib/components/ui/button'
  import './layout.css'

  const { children } = $props()

  const nav = [
    { path: '/start', title: 'Start' },
    { path: '/journal', title: 'Journal' },
  ]
</script>

{#if page.route.id !== '/'}
  <header class="flex items-center justify-between px-4 py-3">
    <a href="/" class="flex items-center gap-2.5 font-heading text-xl tracking-tight">
      <Logo />
      <div>Cradle.log</div>
    </a>
    <nav>
      {#each nav as { path, title }}
        <Button
          href={path}
          variant="ghost"
          size="sm"
          class={{
            'bg-primary text-primary-foreground hover:bg-primary hover:text-primary-foreground':
              page.route.id === path,
          }}>{title}</Button
        >
      {/each}
    </nav>
  </header>
{/if}
<main>
  {@render children()}
</main>
