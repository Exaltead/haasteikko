<script setup lang="ts">
import IconedText from '@/components/basics/IconedText.vue';
import type { LibraryItem } from '@/models/LibraryItem';
import IconChevronRight from '../icons/IconChevronRight.vue';
import CustomIcon from '../basics/CustomIcon.vue';
import { ref } from 'vue';
import { useLibraryApi } from '@/api/libraryApiClient';

const libraryApi = useLibraryApi()
const props = defineProps<{
  item: LibraryItem
  orderingNumber: number
}>()

const favorite = ref<boolean>(props.item.favorite)

const emit = defineEmits<{
  (e: "itemUpdated"): void
}>()

async function updateItemFavorite(): Promise<void> {
  favorite.value = !favorite.value
  const newItem = { ...props.item }
  newItem.favorite = !props.item.favorite


  await libraryApi.updateLibraryItem(newItem)
  emit("itemUpdated")
}

</script>

<template>
  <div class="card flex flex-col items-center justify-start md:max-w-[400px] min-w-[250px] w-full p-2.5 pt-2 gap-2">
    <div class="flex flex-row justify-between items-center w-full px-2.5 py-1 border-b border-brand-orange">
      <div class="flex flex-row justify-between gap-2 items-center">
        <span class="font-bold">{{`#${orderingNumber}`}}</span>
        <IconedText v-if="item.kind === 'Book'" :text="` ${item.title}`" :icon-name="'Book'" :heading="true" />
        <IconedText v-if="item.kind === 'Game'" :text="item.title" :icon-name="'Game'" :heading="true" />
      </div>

      <button @click="updateItemFavorite" class="cursor-pointer">
        <CustomIcon v-if="favorite" name="HeartFull" class="text-brand-orange" />
        <CustomIcon v-else name="HeartEmpty" class="text-brand-orange" />
      </button>

    </div>
    <RouterLink :to="{ name: 'libraryItem', params: { id: item.id } }" class="w-full">
      <div class="flex flex-row justify-between items-center w-full px-2.5">

        <div v-if="item.kind === 'Book'" class="flex flex-col justify-start w-full h-fit gap-2">
          <IconedText :text="item.author" icon-name="Author" :heading="false" />
          <IconedText v-if="item.translator" :text="item.translator" :icon-name="'Translator'" :heading="false" />
        </div>
        <div v-if="item.kind === 'Game'">
          <IconedText :text="item.creator" icon-name="Author" :heading="false" />
        </div>

        <IconChevronRight class="text-brand-orange h-[30px] w-[30px]" />

      </div>
    </RouterLink>

  </div>

</template>
