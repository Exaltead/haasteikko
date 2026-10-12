<script lang="ts" setup>
import { computed } from 'vue'
import type { Event } from '@/models/Event'
import type { LibraryItem } from '@/models/LibraryItem'
import type { Answer, Challenge } from '@/models/challenge'
import BrandedButton from '@/components/Basics/BrandedButton.vue'
import ChallengeAnswerCard from '@/components/Entry/ChallengeAnswerCard.vue'

const { event, item, challenges, answers } = defineProps<{
  event: Event
  item: LibraryItem
  challenges: Challenge[]
  answers: Answer[]
}>()

const emit = defineEmits<{
  (e: "edit"): void
}>()

const linkedChallenges = computed(() => {
  return event.activatedChallengeIds
    .map((id) => challenges.find((challenge) => challenge.id === id))
    .filter((challenge): challenge is Challenge => challenge !== undefined)
})

const formatDate = (date: Date): string => date.toLocaleDateString("fi-FI")
</script>

<template>
  <div class="flex flex-col gap-3 rounded border border-brand-orange bg-white p-3">
    <div class="flex flex-row items-center justify-between gap-2">
      <span class="font-medium">{{ formatDate(new Date(event.completedAt)) }}</span>
      <BrandedButton
        icon="Edit"
        :onClick="() => emit('edit')"
        variant="secondary"
        :is-pill="true"
        aria-label="Muokkaa tapahtumaa"
      />
    </div>

    <div v-if="linkedChallenges.length > 0" class="grid grid-cols-1 gap-2 md:grid-cols-2">
      <ChallengeAnswerCard
        v-for="challenge in linkedChallenges"
        :key="challenge.id"
        :challenge="challenge"
        :item-id="item.id"
        :answers="answers"
      />
    </div>
    <span v-else class="text-xs text-gray-400">Ei liitettyjä haasteita</span>
  </div>
</template>
