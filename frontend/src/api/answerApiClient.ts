
import { answerSchema, type Answer } from "@/models/challenge"
import { z } from "zod"
import { BaseApiClient } from "./baseApiClient"


const answerSetSchema = z.object({
  id: z.string().uuid(),
  challengeId: z.string(),
  itemId: z.string(),
  answers: answerSchema.array(),
})

const newAnswerSetSchema = z.object({
  challengeId: z.string(),
  itemId: z.string(),
  answers: answerSchema.array(),
})

type AnswerSet = z.infer<typeof answerSetSchema>

class AnswerApiClient extends BaseApiClient<
  typeof answerSetSchema,
  typeof newAnswerSetSchema
> {
  constructor() {
    super(answerSetSchema, newAnswerSetSchema, "answer")
  }


  async getAnswer(challengeId: string, itemId: string): Promise<{ id?: string | undefined; answers: Answer[] }> {
    const results = await this.fetchEntities(new URLSearchParams({ challengeId, itemId }))
    if (results.length === 0) {
      return { answers: [] }
    }
    if (results.length !== 1) {
      throw new Error("Invalid answer data")
    }
    const { id, answers } = results[0]
    return { id, answers }
  }

  async getChallengeAnswers(challengeId: string): Promise<Answer[]> {
    const results = await this.fetchEntities(new URLSearchParams({ challengeId }))
    return results.flatMap((t) => t.answers)
  }


  async addAnswer(answers: Answer[], challengeId: string, itemId: string): Promise<string> {
    return this.addEntity({ challengeId, itemId, answers })
  }

  async updateAnswer(id: string, answers: Answer[], challengeId: string, itemId: string): Promise<void> {
    return this.updateEntity(id, { id, challengeId, itemId, answers })
  }
}

export const answerApiClient = new AnswerApiClient()
