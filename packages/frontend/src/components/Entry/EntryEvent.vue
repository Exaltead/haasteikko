<script lang="ts" setup>
import { computed, ref, watch } from 'vue'
import type { Event } from '@/models/Event'
import type { LibraryItem } from '@/models/LibraryItem'
import type { Challenge } from '@/models/challenge'
import { useEventApi } from '@/api/eventApiClient'
import DatePicker from '@/components/Basics/DatePicker.vue'
import ManagementList from '@/components/Basics/ManagementList.vue'
import EntryChallenge from '@/components/Entry/EntryChallenge.vue'
import { TabGroup, TabList, TabPanel, TabPanels, Tab } from '@headlessui/vue'

const eventApi = useEventApi()

const { event, item, challenges, initiallyExpanded } = defineProps<{
  event: Event
  item: LibraryItem
  challenges: Challenge[]
  initiallyExpanded?: boolean
}>()

const emit = defineEmits<{
  (e: "eventUpdated"): void
}>()

const isExpanded = ref(initiallyExpanded ?? false)

function toggleExpanded() {
  isExpanded.value = !isExpanded.value
}

const completedAt = ref(new Date(event.completedAt))

const linkedChallenges = computed(() => {
  return event.activatedChallengeIds
    .map((id) => challenges.find((challenge) => challenge.id === id))
    .filter((challenge): challenge is Challenge => challenge !== undefined)
})

const activatedChallengeList = computed(() => {
  return linkedChallenges.value.map((challenge) => ({
    value: challenge.id,
    display: challenge.name,
  }))
})

const allChallenges = computed(() => {
  return challenges
    .filter((challenge) => challenge.targetMedia === item.kind)
    .map((challenge) => ({
      value: challenge.id,
      display: challenge.name,
    }))
})

const formatDate = (date: Date): string => date.toLocaleDateString("fi-FI")

const isSaving = ref(false)
async function save(activatedChallengeIds: string[]): Promise<void> {
  isSaving.value = true
  await eventApi.updateEvent({
    id: event.id,
    itemId: event.itemId,
    completedAt: completedAt.value.toISOString(),
    activatedChallengeIds,
  })
  isSaving.value = false
  emit("eventUpdated")
}

function onChallengeStateChanged(challengeIds: string[]) {
  save(challengeIds)
}

watch(completedAt, () => {
  save(event.activatedChallengeIds)
})
</script>

<template>
  <div class="rounded border border-brand-orange bg-white">
    <button
      @click="toggleExpanded"
      type="button"
      class="flex w-full flex-row items-center justify-between gap-2 p-3 text-left cursor-pointer"
    >
      <span class="font-medium">{{ formatDate(new Date(event.completedAt)) }}</span>
      <div class="flex flex-row flex-wrap gap-1 justify-end">
        <span
          v-for="challenge in linkedChallenges"
          :key="challenge.id"
          class="rounded-full bg-brand-primary/10 px-2 py-0.5 text-xs text-text-primary"
        >
          {{ challenge.name }}
        </span>
        <span v-if="linkedChallenges.length === 0" class="text-xs text-gray-400">
          Ei liitettyjä haasteita
        </span>
      </div>
    </button>

    <div v-if="isExpanded" class="flex flex-col gap-4 border-t border-brand-orange/40 p-3">
      <DatePicker v-model="completedAt" label="Päiväys" icon="Calendar" />

      <ManagementList
        title="Haasteet"
        :items="activatedChallengeList"
        :allOptions="allChallenges"
        @stateChanged="onChallengeStateChanged"
      />

      <TabGroup v-if="linkedChallenges.length > 0">
        <TabList class="flex flex-row gap-6">
          <Tab v-for="challenge in linkedChallenges" :key="challenge.id" v-slot="{ selected }">
            <button
              :class="
                selected
                  ? 'bg-brand-primary text-white'
                  : 'bg-white text-brand-primary hover:bg-brand-primary hover:text-white'
              "
            >
              <span>{{ challenge.name }}</span>
            </button>
          </Tab>
        </TabList>
        <TabPanels>
          <TabPanel v-for="challenge in linkedChallenges" :key="challenge.id">
            <EntryChallenge :eventId="event.id" :challengeId="challenge.id" />
          </TabPanel>
        </TabPanels>
      </TabGroup>
    </div>
  </div>
</template>
