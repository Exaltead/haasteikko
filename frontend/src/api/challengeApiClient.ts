import { challengeSchema, type Challenge } from "@/models/challenge"
import { BaseApiClient } from "./baseApiClient"
import type { HttpProxy } from "./HttpProxy"
import { useHttpApi } from "@/plugins/HttpPlugin"

const newChallengeSchema = challengeSchema.omit({ id: true })

class ChallengeApiClient extends BaseApiClient<typeof challengeSchema, typeof newChallengeSchema> {
  constructor(proxy: HttpProxy) {
    super(challengeSchema, newChallengeSchema, "challenge", proxy)
  }

  async fetchChallenges(): Promise<Challenge[]> {
    return this.fetchEntities(new URLSearchParams({}))
  }

  async addChallenge(challenge: Omit<Challenge, "id">): Promise<string> {
    return this.addEntity(challenge)
  }

  async updateChallenge(challenge: Challenge): Promise<void> {
    return this.updateEntity(challenge.id, challenge)
  }
}

export function useChallengeApi(): ChallengeApiClient {
  const proxy = useHttpApi()
  return new ChallengeApiClient(proxy)
}
