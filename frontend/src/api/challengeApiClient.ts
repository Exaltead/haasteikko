
import { challengeSchema, type Challenge } from "@/models/challenge"
import { z } from "zod"
import { BaseApiClient } from "./baseApiClient"

const newChallengeSchema = challengeSchema.omit({ id: true })

class ChallengeApiClient extends BaseApiClient<
  typeof challengeSchema,
  typeof newChallengeSchema
> {
  constructor() {
    super(challengeSchema, newChallengeSchema, "challenge")
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

export const challengeApiClient = new ChallengeApiClient()
