import { answerSchema, type Answer } from "@/models/challenge"
import type { HttpProxy } from "./HttpProxy"
import { useHttpApi } from "@/plugins/HttpPlugin"
import z from "zod"

//TODO: move the api url to the http proxy, maybe
const API_URL = import.meta.env.VITE_API_URL

class AnswerApiClient {
  constructor(private proxy: HttpProxy) {}

  async searchAnswers(query: { challengeId?: string; itemId?: string }): Promise<Answer[]> {
    const params = new URLSearchParams()
    if (query.challengeId) {
      params.append("challengeId", query.challengeId)
    }
    if (query.itemId) {
      params.append("itemId", query.itemId)
    }

    const res = await this.proxy.get(
      `${API_URL}/answers`,
      params,
      z.object({ answers: answerSchema.array() }),
    )

    return res.answers
  }

  async upsertAnswers(answers: Answer[], challengeId: string, itemId: string): Promise<Answer[]> {
    const result = await this.proxy.post(
      `${API_URL}/answers/${itemId}/${challengeId}`,
      { answers },
      z.object({ answers: answerSchema.array() }),
    )

    return result.answers
  }
}

export function useAnswerApi() {
  const proxy = useHttpApi()
  return new AnswerApiClient(proxy)
}
