<script lang="ts" setup>
import { computed, ref, watch } from 'vue'
import type { Event } from '@/models/Event'
import type { LibraryItem } from '@/models/LibraryItem'
import type { Answer, Challenge } from '@/models/challenge'
import { useEventApi } from '@/api/eventApiClient'
import DatePicker from '@/components/Basics/DatePicker.vue'
import ManagementList from '@/components/Basics/ManagementList.vue'
import ChallengeAnswerCard from '@/components/Entry/ChallengeAnswerCard.vue'

const eventApi = useEventApi()

const { event, item, challenges, answers, initiallyExpanded } = defineProps<{
  event: Event
  item: LibraryItem
  challenges: Challenge[]
  answers: Answer[]
  initiallyExpanded?: boolean
}>()

const emit = defineEmits<{
  (e: "eventUpdated"): void
  (e: "expandedChanged", expanded: boolean): void
}>()

const isExpanded = ref(initiallyExpanded ?? false)

function toggleExpanded() {
  isExpanded.value = !isExpanded.value
  emit("expandedChanged", isExpanded.value)
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

      <div v-if="linkedChallenges.length > 0" class="grid grid-cols-1 gap-2 md:grid-cols-2">
        <ChallengeAnswerCard
          v-for="challenge in linkedChallenges"
          :key="challenge.id"
          :challenge="challenge"
          :item-id="item.id"
          :answers="answers"
        />
      </div>
    </div>
  </div>
</template>
