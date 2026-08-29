import type { Answer, Challenge } from "@/models/challenge"
import type { LibraryItem } from "@/models/LibraryItem"

export function getCompletedCount(challenge: Challenge, answers: Answer[]): number {
  const questionsWithAnswers = new Set<string>()

  for (const answer of answers) {
    if (!answer.answered) continue

    const question = challenge.questions.find((q) => q.id === answer.questionId)
    if (!question) continue

    if (question.kind === "Boolean" && answer.answer === "yes") {
      questionsWithAnswers.add(answer.questionId)
    } else if (question.kind === "TextInput" && answer.answer !== "") {
      questionsWithAnswers.add(answer.questionId)
    }
  }

  return questionsWithAnswers.size
}

export function getTotalCount(challenge: Challenge): number {
  return challenge.questions.length
}

export function getAttachedItemCount(challenge: Challenge, libraryItems: LibraryItem[]): number {
  return libraryItems.filter(
    (item) => item.kind === challenge.targetMedia && item.activatedChallengeIds.includes(challenge.id),
  ).length
}

export function filterByStatus(
  challenges: Challenge[],
  status: Challenge["status"],
): Challenge[] {
  return challenges.filter((challenge) => challenge.status === status)
}
