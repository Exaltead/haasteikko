import type { Question, Solution } from "@/models/challenge"
import type { LibraryItem } from "@/models/LibraryItem"

const INDENT = "   "
const EMPTY_PLACEHOLDER = "–"

function describeItem(item: LibraryItem): string {
  const maker = item.kind === "Book" ? item.author : item.creator
  return maker ? `${item.title} – ${maker}` : item.title
}

function pickedItemIds(question: Question, solution: Solution | undefined): string[] {
  if (solution === undefined) return []
  if (question.kind === "Boolean") {
    return solution.singleAnswerItemId ? [solution.singleAnswerItemId] : []
  }
  return solution.multipleAnswerItemIds.filter((id) => id !== "")
}

/**
 * Renders a (partially) filled solution as a numbered plain-text list. Unsolved questions are
 * kept with a placeholder so they can be filled in by hand.
 */
export function formatSolutionAsText(
  challengeName: string,
  questions: Question[],
  solutions: Solution[],
  items: LibraryItem[],
): string {
  const itemsById = new Map(items.map((item) => [item.id, item]))

  const blocks = [...questions]
    .sort((a, b) => a.number - b.number)
    .map((question) => {
      const parts = question.questionClusterSize > 1 ? ` (${question.questionClusterSize} osaa)` : ""
      const solution = solutions.find((s) => s.questionId === question.id)
      const itemLines = pickedItemIds(question, solution)
        .map((id) => itemsById.get(id))
        .filter((item): item is LibraryItem => item !== undefined)
        .map((item) => INDENT + describeItem(item))
      const lines = itemLines.length > 0 ? itemLines : [INDENT + EMPTY_PLACEHOLDER]
      return [`${question.number}. ${question.question}${parts}`, ...lines].join("\n")
    })

  return [challengeName, ...blocks].join("\n\n")
}
