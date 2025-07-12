<script lang="ts" setup>
import { challengeApiClient } from '@/api/challengeApiClient';
import { libraryApi } from '@/api/libraryApiClient';;
import EntryBasics from '@/components/Entry/EntryBasics.vue';
import EntryChallenge from '@/components/Entry/EntryChallenge.vue';
import IconBack from '@/components/icons/IconBack.vue';
import type { Challenge } from '@/models/challenge';
import type { LibraryItem } from '@/models/LibraryItem';
import { TabGroup, TabList, TabPanel, TabPanels, Tab } from '@headlessui/vue';
import { computed, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { z } from 'zod';

const route = useRoute()

const router = useRouter()

function toLibrary() {
  router.push({ name: "home" })
}

const challenges = ref<Challenge[]>([])
const item = ref<LibraryItem | undefined>(undefined)

async function getChallenges() {
  challenges.value = await challengeApiClient.fetchChallenges()
}
async function getItem() {
  const itemId = z.string().parse(route.params.id)
  const foundItem = await libraryApi.getLibraryItem(itemId)
  item.value = foundItem
}

async function fetchData() {
  await Promise.all([getChallenges(), getItem()])
}




watch(() => route.params.id, fetchData, { immediate: true })



function makeTabStyle(selected: boolean) {
  return selected
    ? "bg-brand-primary text-white"
    : "bg-white text-brand-primary hover:bg-brand-primary hover:text-white"
}

const inEditMode = ref(false)
function setEditingMode(state: boolean) {
  inEditMode.value = state
  if (state === false) {
    // As disabling the edit mode, we need to fetch the item again
    getItem()
  }
}

const challengeKeys = computed(() => {
  if (item.value) {
    const mapped = item.value.activatedChallengeIds.map((t, i) => {
      const challenge = challenges.value.find(t2 => t2.id === t)
      return { value: t, display: challenge?.name ?? ("Haaste " + i) }
    })

    return mapped
  }
  return []
})

</script>

<template>
  <main>
    <div>
      <button :onClick="router.back" class="p-4">
        <IconBack class="text-brand-primary w-8 h-fit cursor-pointer" />
      </button>
      <div class="px-4 md:px-10 flex flex-col gap-10" v-if="item">
        <EntryBasics @objectDeleted="toLibrary" :challenges="challenges" @object-edited="getItem" :item="item"
          @edit-mode-changed="setEditingMode" />
        <TabGroup v-if="!inEditMode && challengeKeys.length > 0">
          <TabList class="flex flex-row gap-6">
            <Tab v-for="{ display, value } in challengeKeys" :key="value" v-slot="{ selected }">
              <button :class="makeTabStyle(selected)">
                <span>{{ display }}</span>
              </button>
            </Tab>
          </TabList>
          <TabPanels>
            <TabPanel v-for="{ value } in challengeKeys" :key="value">
              <EntryChallenge :itemId="item.id" :challengeId="value" />
            </TabPanel>
          </TabPanels>
        </TabGroup>
      </div>
    </div>
  </main>
</template>
