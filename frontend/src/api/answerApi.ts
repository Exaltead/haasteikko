import { answerSchema, type Answer } from "@/models/challenge"
import { getAccessToken } from "@/modules/auth-store"
import { z } from "zod"

const API_URL = import.meta.env.VITE_API_URL

function getHeaders(): HeadersInit {
  return {
    "Content-Type": "application/json",
    Authorization: `Bearer ${getAccessToken()}`,
  }
}

export async function getAnswer(
  challengeId: string,
  itemId: string,
): Promise<{ id?: string | undefined; answers: Answer[] }> {
  const resp = await fetch(`${API_URL}/answer?challengeId=${challengeId}&itemId=${itemId}`, {
    method: "GET",
    headers: getHeaders(),
  })

  if (!resp.ok) {
    throw new Error("Failed to fetch answers")
  }

  const data = await resp.json()
  const schema = z
    .object({
      id: z.string().uuid().optional(),
      answers: answerSchema.array(),
    })
    .array()

  const answers = schema.parse(data)

  if (answers.length === 0) {
    return { answers: [] }
  }
  if (answers.length !== 1) {
    throw new Error("Invalid answer data")
  }
  return answers[0]
}

export async function getChallengeAnswers(challengeId: string): Promise<Answer[]> {
  const resp = await fetch(`${API_URL}/answer?challengeId=${challengeId}`, {
    method: "GET",
    headers: getHeaders(),
  })

  if (!resp.ok) {
    throw new Error("Failed to fetch answers")
  }

  const data = await resp.json()

  const content = z
    .object({
      id: z.string().uuid(),
      answers: answerSchema.array(),
    })
    .array()
    .parse(data)

  return content.flatMap((t) => t.answers)
}

export async function addAnswer(
  answers: Answer[],
  challengeId: string,
  itemId: string,
): Promise<void> {
  const resp = await fetch(`${API_URL}/answer`, {
    method: "POST",
    headers: getHeaders(),
    body: JSON.stringify({
      challengeId,
      itemId,
      answers,
    }),
  })

  if (!resp.ok) {
    throw new Error("Failed to add answer set")
  }
}

export async function updateAnswer(
  id: string,
  answers: Answer[],
  challengeId: string,
  itemId: string,
): Promise<void> {
  const resp = await fetch(`${API_URL}/answer/${id}`, {
    method: "PUT",
    headers: getHeaders(),
    body: JSON.stringify({ id, challengeId, itemId, answers }),
  })

  if (!resp.ok) {
    throw new Error("Failed to update challenge")
  }
}
