<script lang="ts">
  import { invalidateAll } from '$app/navigation'
  import { Pencil, Trash } from '@lucide/svelte'
  import { deleteJournal, updateJournal, type Disease } from '$lib/api'
  import * as AlertDialog from '$lib/components/ui/alert-dialog'
  import { Badge } from '$lib/components/ui/badge'
  import { Button } from '$lib/components/ui/button'
  import * as Card from '$lib/components/ui/card'
  import * as Dialog from '$lib/components/ui/dialog'
  import { Input } from '$lib/components/ui/input'
  import { Label } from '$lib/components/ui/label'
  import type { PageProps } from './$types'

  const { data }: PageProps = $props()

  let deletingId = $state(0)
  let deleting = $derived(deletingId !== 0)

  let updatingId = $state(0)
  let updating = $derived(updatingId !== 0)
  const defaultForm = $state({ diseases: '', symptoms: '' })

  async function deleteRecord() {
    await deleteJournal(deletingId)
    await invalidateAll()
    deletingId = 0
  }

  function openUpdateDialog(disease: Disease) {
    defaultForm.diseases = disease.diseases.join(', ')
    defaultForm.symptoms = disease.symptoms.join(', ')
    updatingId = disease.id
  }

  async function update(e: SubmitEvent & { currentTarget: EventTarget & HTMLFormElement }) {
    e.preventDefault()
    const formData = new FormData(e.currentTarget)
    const toArray = (key: string) =>
      (formData.get(key) as string)
        .split(',')
        .map((x) => x.trim())
        .filter((x) => x !== '')

    await updateJournal(updatingId, {
      diseases: toArray('diseases'),
      symptoms: toArray('symptoms'),
    })
    await invalidateAll()
    updatingId = 0
  }
</script>

<div class="space-y-4 px-6 py-4">
  {#if data.diseases.length === 0}
    <p class="text-muted-foreground">No records yet.</p>
  {:else}
    {#each data.diseases as disease}
      <Card.Root>
        <Card.Header>
          <Card.Description
            >{Temporal.Instant.from(disease.creation_time)
              .toZonedDateTimeISO(Temporal.Now.timeZoneId())
              .toString()
              .slice(0, 19)
              .replace('T', ' ')}</Card.Description
          >
          <Card.Action>
            <Button variant="secondary" size="icon" onclick={() => openUpdateDialog(disease)}
              ><Pencil /></Button
            >
            <Button variant="destructive" size="icon" onclick={() => (deletingId = disease.id)}
              ><Trash /></Button
            >
          </Card.Action>
        </Card.Header>
        <Card.Content class="space-y-3">
          <div class="space-y-1">
            <h3 class="text-base">Diseases</h3>
            <div class="space-x-1">
              {#each disease.diseases as x}
                <Badge>{x}</Badge>
              {/each}
            </div>
          </div>
          <div class="space-y-1">
            <h3 class="text-base">Symptoms</h3>
            <div class="space-x-1">
              {#each disease.symptoms as symptom}
                <Badge>{symptom}</Badge>
              {/each}
            </div>
          </div>
        </Card.Content>
      </Card.Root>
    {/each}
  {/if}
</div>

<AlertDialog.Root bind:open={deleting}>
  <AlertDialog.Content>
    <AlertDialog.Header>
      <AlertDialog.Title>Are you sure to delete the record?</AlertDialog.Title>
      <AlertDialog.Description>This action cannot be undone.</AlertDialog.Description>
    </AlertDialog.Header>
    <AlertDialog.Footer>
      <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
      <AlertDialog.Action onclick={deleteRecord}>Delete</AlertDialog.Action>
    </AlertDialog.Footer>
  </AlertDialog.Content>
</AlertDialog.Root>

<Dialog.Root bind:open={updating}>
  <form onsubmit={update}>
    <Dialog.Content portalProps={{ disabled: true }}>
      <div class="grid gap-4">
        <div class="grid gap-3">
          <Label for="diseases">Diseases</Label>
          <Input id="diseases" name="diseases" defaultValue={defaultForm.diseases} />
        </div>
        <div class="grid gap-3">
          <Label for="symptoms">Symptoms</Label>
          <Input id="symptoms" name="symptoms" defaultValue={defaultForm.symptoms} />
        </div>
      </div>
      <Dialog.Footer>
        <Button type="submit">Save changes</Button>
      </Dialog.Footer>
    </Dialog.Content>
  </form>
</Dialog.Root>
