import type { Event } from "@/models/Event"
import type { LibraryItem } from "@/models/LibraryItem"

export function getLastCompletedAt(itemId: string, events: Event[]): string | undefined {
  const matches = events.filter((e) => e.itemId === itemId).map((e) => e.completedAt)
  return matches.length ? matches.sort()[matches.length - 1] : undefined
}

export type LibraryItemWithCompletion = LibraryItem & { lastCompletedAt?: string }

export function withLastCompletedAt(
  items: LibraryItem[],
  events: Event[],
): LibraryItemWithCompletion[] {
  return items.map((item) => ({ ...item, lastCompletedAt: getLastCompletedAt(item.id, events) }))
}
