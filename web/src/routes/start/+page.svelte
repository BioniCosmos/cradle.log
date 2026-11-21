<script lang="ts">
  import { goto } from '$app/navigation'
  import { SendHorizontal } from '@lucide/svelte'
  import { createJournal } from '#lib/api.ts'
  import Logo from '#lib/components/Logo.svelte'
  import { Button } from '#lib/components/ui/button/index.ts'
  import Wave from '#lib/components/Wave.svelte'
  import RecordDrawer from './RecordDrawer.svelte'
  import './page.css'

  const prompt = 'How do you feeling today?'

  let journal = $state('')

  async function create() {
    await createJournal(journal)
    await goto('/journal')
  }
</script>

<div class="flex h-[calc(100dvh-3.25rem)] flex-col">
  <div class="flex flex-1 flex-col items-center justify-center gap-25 pb-8">
    <div aria-hidden="true">
      <div
        class="absolute size-26 animate-[ripple] rounded-full border border-primary/40 ease-out animation-duration-[3s] repeat-infinite"
      ></div>
      <div
        class="absolute size-26 animate-[ripple] rounded-full border border-primary/30 delay-[1s] ease-out animation-duration-[3s] repeat-infinite"
      ></div>
      <div
        class="absolute size-26 animate-[ripple] rounded-full border border-primary/20 delay-[2s] ease-out animation-duration-[3s] repeat-infinite"
      ></div>
      <Logo
        class="size-26 animate-[orb-pulse] rounded-full ease-in-out animation-duration-[3s] repeat-infinite"
      />
    </div>
    <p class="max-w-4/5 text-center font-heading text-3xl font-light text-balance">
      {prompt}
    </p>
  </div>
  <div class="fixed bottom-0 left-0 -z-10 w-full" aria-hidden="true">
    <Wave
      class="absolute bottom-0 left-0 h-55 w-[200%] animate-[wave] opacity-18 ease-linear animation-duration-[13s] repeat-infinite"
      fill="oklch(0.625 0.135 42)"
    />
    <Wave
      class="absolute bottom-0 left-0 h-65 w-[200%] animate-[wave] opacity-13 ease-linear animation-duration-[19s] direction-reverse repeat-infinite"
      fill="oklch(0.72 0.105 46)"
    />
    <Wave
      class="absolute bottom-0 left-0 h-75 w-[200%] animate-[wave] opacity-10 delay-[-5s] ease-linear animation-duration-[27s] repeat-infinite"
      fill="oklch(0.625 0.135 42)"
    />
  </div>
  <div class="mx-auto mb-6 flex w-3/5 items-center gap-2 rounded-xl bg-card px-3 py-1.5 shadow">
    <textarea
      name="journal"
      class="field-sizing-content max-h-[60dvh] grow resize-none outline-none"
      bind:value={journal}></textarea>
    <div class="flex gap-1">
      <RecordDrawer />
      <Button size="icon-lg" onclick={create}>
        <SendHorizontal class="size-5" />
      </Button>
    </div>
  </div>
</div>
