<script lang="ts" setup>

import { computed, ref } from "vue"
import type { LibraryItem } from "@/models/LibraryItem"
import { useLibraryApi } from '@/api/libraryApiClient';
import LibraryItemCard from "./LibraryItemCard.vue"


const libraryApi = useLibraryApi()
const items = ref<LibraryItem[]>([])

const listItems = computed(() => {
  const sortedItems = [...items.value].sort((a, b) => {
    const dateA = new Date(a.addedAt).getTime()
    const dateB = new Date(b.addedAt).getTime()
    return dateA - dateB
  })

  const books = sortedItems.filter(item => item.kind === "Book").map((item, index) => {
    return { ...item, orderingNumber: index + 1 }
  })
  const games = sortedItems.filter(item => item.kind === "Game").map((item, index) => {
    return { ...item, orderingNumber: index + 1 }
  })

  const combined = [...books, ...games]

  const resorted = combined.sort((a, b) => {
    const dateA = new Date(a.addedAt).getTime()
    const dateB = new Date(b.addedAt).getTime()
    return dateB - dateA
  })


  return resorted
})

async function getItems() {
  const serverItems = await libraryApi.fetchLibraryItems()

  items.value = serverItems
}

getItems()


</script>

<template>
  <div>
    <div class="grid md:grid-cols-2 w-full gap-3">
      <LibraryItemCard v-for="entry in listItems" :key="entry.id"
        :item="entry" @item-updated="getItems" :ordering-number="entry.orderingNumber" />
    </div>
  </div>
</template>
