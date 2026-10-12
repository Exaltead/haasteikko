import type { Answer, Challenge, Question, Solution } from "@/models/challenge"
import type { LibraryItem } from "@/models/LibraryItem"
import type { Event } from "@/models/Event"

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

function isQualifyingAnswer(question: Question, answer: Answer): boolean {
  if (!answer.answered || answer.questionId !== question.id) return false
  return question.kind === "Boolean" ? answer.answer === "yes" : answer.answer.trim() !== ""
}

/**
 * Questions whose chosen solution is complete: every slot has an item, and every chosen item still
 * has a qualifying answer (`answers` come from the challenge-wide search, which already excludes
 * items no event links to the challenge, and deleted items cascade away). Multi-part TextInput
 * solutions additionally need all chosen items to share the same answer.
 */
export function getSolvedCount(
  challenge: Challenge,
  solutions: Solution[],
  answers: Answer[],
): number {
  return challenge.questions.filter((question) => {
    const solution = solutions.find((s) => s.questionId === question.id)
    if (!solution) return false

    const itemIds =
      solution.kind === "SinglePartSolution"
        ? [solution.singleAnswerItemId].filter((id): id is string => !!id)
        : solution.multipleAnswerItemIds.filter((id) => id !== "")
    if (itemIds.length < Math.max(1, question.questionClusterSize)) return false

    const chosenAnswers = itemIds.map((itemId) =>
      answers.find((a) => a.itemId === itemId && isQualifyingAnswer(question, a)),
    )
    if (chosenAnswers.some((a) => a === undefined)) return false

    return question.kind !== "TextInput" || new Set(chosenAnswers.map((a) => a!.answer)).size === 1
  }).length
}

export function getAnsweredCount(challenge: Challenge, answers: Answer[]): number {
  const questionIds = new Set(challenge.questions.map((q) => q.id))
  return new Set(
    answers.filter((a) => a.answered && questionIds.has(a.questionId)).map((a) => a.questionId),
  ).size
}

export function getTotalCount(challenge: Challenge): number {
  return challenge.questions.length
}

export function getAttachedItemCount(
  challenge: Challenge,
  libraryItems: LibraryItem[],
  events: Event[],
): number {
  const matchingItemIds = new Set(
    events.filter((e) => e.activatedChallengeIds.includes(challenge.id)).map((e) => e.itemId),
  )
  return libraryItems.filter(
    (item) => item.kind === challenge.targetMedia && matchingItemIds.has(item.id),
  ).length
}

export function filterByStatus(
  challenges: Challenge[],
  status: Challenge["status"],
): Challenge[] {
  return challenges.filter((challenge) => challenge.status === status)
}
