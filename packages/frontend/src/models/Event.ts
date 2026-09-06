import { z } from "zod"

export const eventSchema = z.object({
  id: z.string(),
  itemId: z.string(),
  completedAt: z.string(),
  activatedChallengeIds: z.string().array(),
})

export type Event = z.infer<typeof eventSchema>
