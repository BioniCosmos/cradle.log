<script lang="ts">
  import { ArrowRight } from '@lucide/svelte'
  import Logo from '#lib/components/Logo.svelte'
  import { Button } from '#lib/components/ui/button/index.ts'

  const features = [
    {
      title: 'Write freely',
      details:
        'No forms. No dropdowns. Describe how you feel in plain words — Cradle.log understands the rest.',
    },
    {
      title: 'See patterns',
      details:
        'Symptoms, medications, mood, and sleep surface as a quiet timeline. Patterns emerge over weeks, not pop-ups.',
    },
    {
      title: 'Stay private',
      details:
        'Your record lives with you. No ads, no data brokers. Export or delete everything, any time.',
    },
  ]
</script>

<div class="min-h-dvh antialiased">
  <div class="mx-auto max-w-300 px-5.5 md:px-10">
    <a href="/" class="flex items-center gap-2.5 pt-7 font-heading tracking-tight">
      <Logo />
      <div class="text-2xl">Cradle.log</div>
    </a>
    <div class="pt-20">
      <div class="font-mono text-xs tracking-widest text-muted-foreground uppercase">
        ⸺ A personal log for your body, spirit and soul
      </div>
      <div
        class="mt-12 w-fit bg-[linear-gradient(135deg,oklch(0.32_0.20_28)_0%,oklch(0.58_0.22_40)_50%,oklch(0.80_0.16_62)_100%)] bg-clip-text font-heading text-[clamp(50px,8vw,84px)]/[1.2] font-medium text-transparent"
      >
        Track
        <br />
        Accompany
        <br />
        Guard
      </div>
      <p class="mt-13 max-w-[50ch] text-lg text-secondary-foreground">
        Write how you feel — symptoms, sleep, mood, whatever's on your mind. Cradle.log reads it,
        finds the patterns, and keeps a quiet record you can look back along.
      </p>
      <Button href="/start" size="lg" class="mt-8 mb-14 rounded-full px-4 py-6 text-base">
        Start your log
        <ArrowRight data-icon="inline-end" />
      </Button>
    </div>
    <div class="flex flex-col border-y md:flex-row">
      {#each features as { title, details }}
        <div
          class="flex flex-1 flex-col gap-4 border-b px-9 py-10 last:border-b-0 md:border-r md:border-b-0 md:last:border-r-0"
        >
          <h3 class="font-heading text-2xl">{title}</h3>
          <p class="text-sm text-muted-foreground">{details}</p>
        </div>
      {/each}
    </div>
    <div class="h-8"></div>
  </div>
</div>
