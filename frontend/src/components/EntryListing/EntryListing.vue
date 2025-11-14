<script lang="ts" setup>

import { computed, ref } from "vue"
import type { LibraryItem } from "@/models/LibraryItem"
import { useLibraryApi } from '@/api/libraryApiClient';
import LibraryItemCard from "./LibraryItemCard.vue"


const libraryApi = useLibraryApi()
const items = ref<LibraryItem[]>([])

const listItems = computed(() => items.value)

async function getItems() {
  const serverItems = await libraryApi.fetchLibraryItems()

  items.value = serverItems
}

getItems()


</script>

<template>
  <div>
    <div class="grid md:grid-cols-2 w-full gap-3">
      <LibraryItemCard v-for="entry in listItems" :key="entry.id" :item="entry" @item-updated="getItems" />
    </div>
  </div>
</template>
