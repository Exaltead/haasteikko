import { z } from "zod"
import { BaseApiClient, buildSearchParams } from "./baseApiClient"
import { eventSchema, type Event } from "@/models/Event"
import type { HttpProxy } from "./HttpProxy"
import { useHttpApi } from "@/plugins/HttpPlugin"

const newEventSchema = eventSchema.omit({ id: true }).extend({
  activatedChallengeIds: z.string().array().optional(),
})
type NewEvent = z.infer<typeof newEventSchema>

class EventApiClient extends BaseApiClient<typeof eventSchema, typeof newEventSchema> {
  constructor(proxy: HttpProxy) {
    super(eventSchema, newEventSchema, "event", proxy)
  }

  async fetchEvents(): Promise<Event[]> {
    return await this.fetchEntities(new URLSearchParams())
  }

  async fetchEventsForItem(itemId: string): Promise<Event[]> {
    return await this.fetchEntities(buildSearchParams({ itemId }))
  }

  async addEvent(event: NewEvent): Promise<string> {
    return await this.addEntity(event)
  }

  async updateEvent(event: Event): Promise<void> {
    return await this.updateEntity(event.id, event)
  }

  async deleteEvent(id: string): Promise<void> {
    return await this.deleteEntity(id)
  }

  async getEvent(id: string): Promise<Event | undefined> {
    return await this.fetchEntity(id)
  }
}

export function useEventApi(): EventApiClient {
  const proxy = useHttpApi()
  return new EventApiClient(proxy)
}
