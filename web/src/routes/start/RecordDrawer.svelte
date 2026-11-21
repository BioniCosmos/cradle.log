<script lang="ts">
  import { goto } from '$app/navigation'
  import { ClipboardList } from '@lucide/svelte'
  import { Checkbox as CheckboxPrimitive } from 'bits-ui'
  import type { Attachment } from 'svelte/attachments'
  import type { EventHandler } from 'svelte/elements'
  import { createLog, type Log } from '#lib/api.ts'
  import Rating from '#lib/components/Rating.svelte'
  import { Button } from '#lib/components/ui/button/index.ts'
  import { Checkbox } from '#lib/components/ui/checkbox/index.ts'
  import * as Drawer from '#lib/components/ui/drawer/index.ts'
  import * as Field from '#lib/components/ui/field/index.ts'
  import { Input } from '#lib/components/ui/input/index.ts'
  import { Textarea } from '#lib/components/ui/textarea/index.ts'

  const behaviors = [
    { value: 'late-night', label: 'Late night' },
    { value: 'alcohol', label: 'Alcohol' },
    { value: 'caffeine', label: 'Caffeine' },
    { value: 'smoking', label: 'Smoking' },
    { value: 'sedentary', label: 'Sedentary' },
    { value: 'exercise', label: 'Exercise' },
    { value: 'irregular-meals', label: 'Irregular meals' },
    { value: 'stress', label: 'Stress' },
  ]
  let log = $state<Log>({
    time: '',
    mood: null,
    sleepHours: null,
    sleepQuality: null,
    behaviors: [],
    // disease: '',
    // symptoms: '',
    pain: null,
    // medication: '',
    notes: '',
  })
  let saving = $state(false)

  const save: EventHandler<SubmitEvent, HTMLFormElement> = async function save(e) {
    e.preventDefault()
    saving = true
    try {
      await createLog(log)
      await goto('/journal')
    } finally {
      saving = false
    }
  }

  const now: Attachment = () => {
    const time = Temporal.Now.plainDateTimeISO().toString({ smallestUnit: 'second' })
    log.time = time
  }
</script>

<Drawer.Root>
  <Drawer.Trigger>
    {#snippet child({ props })}
      <Button
        variant="ghost"
        size="icon-lg"
        aria-label="Log body data"
        title="Log body data"
        {...props}
      >
        <ClipboardList class="size-5" />
      </Button>
    {/snippet}
  </Drawer.Trigger>
  <Drawer.Content>
    <Drawer.Header>
      <Drawer.Title>Body record</Drawer.Title>
      <Drawer.Description>Log today's sleep, mood and how your body feels</Drawer.Description>
    </Drawer.Header>
    <form id="record" class="flex-1 overflow-y-auto p-4" onsubmit={save}>
      <Field.Group>
        <Field.Field>
          <Field.Label for="time">Record time</Field.Label>
          <Input
            id="time"
            type="datetime-local"
            step="1"
            required
            bind:value={log.time}
            {@attach now}
          />
        </Field.Field>
        <Field.Field>
          <Field.Label for="mood">Mood</Field.Label>
          <Rating id="mood" bind:value={log.mood} max={5} required />
        </Field.Field>
        <Field.Field>
          <Field.Label for="sleep-hours">Sleep hours</Field.Label>
          <Input
            id="sleep-hours"
            type="number"
            min="0"
            max="24"
            step="0.5"
            bind:value={log.sleepHours}
          />
        </Field.Field>
        <Field.Field>
          <Field.Label for="sleep-quality">Sleep quality</Field.Label>
          <Rating id="sleep-quality" bind:value={log.sleepQuality} max={5} />
        </Field.Field>
        <Field.Set>
          <Field.Legend variant="label">Behaviors</Field.Legend>
          <CheckboxPrimitive.Group bind:value={log.behaviors}>
            {#snippet child({ props })}
              <Field.Group class="flex-row" {...props}>
                {#each behaviors as behavior}
                  <Field.Field orientation="horizontal" class="w-fit">
                    <Checkbox id={behavior.value} value={behavior.value} />
                    <Field.Label for={behavior.value} class="font-normal">
                      {behavior.label}
                    </Field.Label>
                  </Field.Field>
                {/each}
              </Field.Group>
            {/snippet}
          </CheckboxPrimitive.Group>
        </Field.Set>
        <Field.Separator />
        <Field.Set>
          <Field.Legend>Illness</Field.Legend>
          <Field.Description>Any noticeable discomfort?</Field.Description>
          <Field.Group>
            <Field.Field>
              <Field.Label for="disease">Known disease</Field.Label>
              <Input id="disease" bind:value={log.disease} />
              <Field.Description>
                Only if you already know what it is — e.g. hypertension, upper respiratory
                infection. Leave blank if you're not sure.
              </Field.Description>
            </Field.Field>
            <Field.Field>
              <Field.Label for="symptoms">Symptoms</Field.Label>
              <Input id="symptoms" bind:value={log.symptoms} />
              <Field.Description>
                What you actually feel — e.g. headache, nausea, sore throat, fever.
              </Field.Description>
            </Field.Field>
            <Field.Field>
              <Field.Label for="medication">Medication</Field.Label>
              <Input id="medication" bind:value={log.medication} />
            </Field.Field>
            <Field.Field>
              <Field.Label for="pain">Pain level</Field.Label>
              <Rating id="pain" bind:value={log.pain} max={10} />
            </Field.Field>
          </Field.Group>
        </Field.Set>
        <Field.Separator />
        <Field.Field>
          <Field.Label for="notes">Notes</Field.Label>
          <Textarea id="notes" placeholder="Anything else worth noting?" bind:value={log.notes} />
        </Field.Field>
      </Field.Group>
    </form>
    <Drawer.Footer>
      <Button type="submit" form="record" disabled={saving}>Save record</Button>
    </Drawer.Footer>
  </Drawer.Content>
</Drawer.Root>
