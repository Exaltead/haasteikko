<script lang="ts" setup>
import { computed } from "vue"
import type { LibraryItem } from "@/models/LibraryItem"
import type { YearFilterOption, EntryTypeFilter } from "@/api/preferencesApiClient"
import LibraryItemCard from "./LibraryItemCard.vue"

const props = withDefaults(
  defineProps<{
    items: LibraryItem[]
    yearFilter?: YearFilterOption
    typeFilter?: EntryTypeFilter[]
    searchQuery?: string
    singleColumn?: boolean
  }>(),
  {
    yearFilter: "all",
    typeFilter: () => ["Book", "Game"],
    searchQuery: "",
    singleColumn: false,
  },
)

const emit = defineEmits<{
  itemUpdated: []
}>()

function matchesSearch(item: LibraryItem, query: string): boolean {
  const searchLower = query.toLowerCase()
  if (item.title.toLowerCase().includes(searchLower)) return true
  if (item.kind === "Book") {
    if (item.author.toLowerCase().includes(searchLower)) return true
    if (item.translator?.toLowerCase().includes(searchLower)) return true
  }
  if (item.kind === "Game") {
    if (item.creator.toLowerCase().includes(searchLower)) return true
  }
  return false
}

const listItems = computed(() => {
  let filteredItems = [...props.items]

  if (props.searchQuery) {
    filteredItems = filteredItems.filter((item) => matchesSearch(item, props.searchQuery))
  }

  if (props.yearFilter !== "all") {
    filteredItems = filteredItems.filter(
      (item) => {
        const date = item.completedAt ? new Date(item.completedAt) : new Date(item.addedAt)
        return date.getFullYear() === props.yearFilter
      },
    )
  }

  if (props.typeFilter.length > 0) {
    filteredItems = filteredItems.filter((item) => props.typeFilter.includes(item.kind))
  }

  const sortedItems = filteredItems.sort((a, b) => {
    const dateA = a.completedAt ? new Date(a.completedAt).getTime() : 0
    const dateB = b.completedAt ? new Date(b.completedAt).getTime() : 0
    return dateA - dateB
  })

  const books = sortedItems
    .filter((item) => item.kind === "Book")
    .map((item, index) => {
      return { ...item, orderingNumber: index + 1 }
    })
  const games = sortedItems
    .filter((item) => item.kind === "Game")
    .map((item, index) => {
      return { ...item, orderingNumber: index + 1 }
    })

  const combined = [...books, ...games]

  const resorted = combined.sort((a, b) => {
    // Primary: completedAt descending (newest first)
    const dateA = a.completedAt ? new Date(a.completedAt).getTime() : 0
    const dateB = b.completedAt ? new Date(b.completedAt).getTime() : 0
    if (dateA !== dateB) return dateB - dateA

    // Secondary: addedAt descending (newer additions first)
    const addedA = new Date(a.addedAt).getTime()
    const addedB = new Date(b.addedAt).getTime()
    if (addedA !== addedB) return addedB - addedA

    // Tertiary: title ascending (alphabetical for complete consistency)
    return a.title.localeCompare(b.title)
  })

  return resorted
})

function onItemUpdated() {
  emit("itemUpdated")
}
</script>

<template>
  <div>
    <div class="grid w-full gap-3" :class="singleColumn ? '' : 'md:grid-cols-2'">
      <LibraryItemCard
        v-for="entry in listItems"
        :key="entry.id"
        :item="entry"
        @item-updated="onItemUpdated"
        :ordering-number="entry.orderingNumber"
      />
    </div>
  </div>
</template>
