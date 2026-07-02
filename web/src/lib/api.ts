import ky from 'ky'

const client = ky.create({ prefix: '/api' })

export interface Disease {
  id: number
  diseases: string[]
  symptoms: string[]
  creation_time: string
}

export interface JournalUpdateParams {
  diseases?: string[]
  symptoms?: string[]
}

export function createJournal(journal: string) {
  return client.post('/journal', { json: journal })
}

export function queryJournals() {
  return client.get('/journal').json<Disease[]>()
}

export function updateJournal(id: number, params: JournalUpdateParams) {
  return client.patch(`/journal/${id}`, { json: params })
}

export function deleteJournal(id: number) {
  return client.delete(`/journal/${id}`)
}
