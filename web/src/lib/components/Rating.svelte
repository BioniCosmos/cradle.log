<script lang="ts">
  import { cn } from '#lib/utils.ts'

  let {
    value = $bindable(null),
    max,
    id,
    required = false,
  }: { value?: number | null; max: number; id?: string; required?: boolean } = $props()

  let input: HTMLInputElement
  $effect(() =>
    input.setCustomValidity(required && value === null ? 'Please select a rating.' : ''),
  )
</script>

<div class="group/dots relative flex flex-wrap gap-1 max-inline-fit" role="group">
  {#each Array.from({ length: max }, (_, i) => i + 1) as i}
    {const selected = $derived(value === null ? false : value >= i)}
    <button
      type="button"
      aria-label={`${i}/${max}`}
      aria-pressed={selected}
      class={cn(
        'inline-flex size-5 items-center justify-center rounded-full border transition-colors focus-visible:ring-3 focus-visible:ring-ring/50 focus-visible:outline-none',
        selected ? 'border-primary bg-primary' : 'border-input hover:bg-muted',
      )}
      onclick={() => (value = i)}
    >
      <span
        class={cn(
          'text-[10px] leading-none opacity-0 transition-opacity duration-150 group-hover/dots:opacity-100 group-has-focus-visible/dots:opacity-100',
          selected ? 'text-primary-foreground' : 'text-muted-foreground',
        )}
      >
        {i}
      </span>
    </button>
  {/each}
  <input
    type="range"
    bind:this={input}
    {id}
    class="pointer-events-none absolute inset-0 opacity-0"
    bind:value={() => value ?? 0, (x) => (value = x)}
    {max}
    step={1}
  />
</div>
