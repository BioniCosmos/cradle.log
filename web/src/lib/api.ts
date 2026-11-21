import ky from 'ky'

const client = ky.create({ prefix: '/api' })

export type Log = {
  time: string
  mood: number | null
  sleepHours: number | null
  sleepQuality: number | null
  behaviors: string[]
  disease?: string
  symptoms?: string
  pain?: number | null
  medication?: string
  notes: string
}

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

export function createLog(log: Log) {
  return client.post('/log', {
    json: {
      ...log,
      time: Temporal.PlainDateTime.from(log.time).toZonedDateTime(Temporal.Now.timeZoneId()),
    },
  })
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
