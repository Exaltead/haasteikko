<script lang="ts" setup>
import DatePicker from '@/components/Basics/DatePicker.vue'
import type { Challenge } from '@/models/challenge'

// Form fields of an event: completion date and the linked challenges
const { challenges } = defineProps<{
  challenges: Challenge[] // the challenges selectable for the item's kind
}>()

const completedAt = defineModel<Date>("completedAt", { required: true })
const activatedChallengeIds = defineModel<string[]>("activatedChallengeIds", { required: true })

function toggleChallenge(challengeId: string, checked: boolean) {
  activatedChallengeIds.value = checked
    ? [...activatedChallengeIds.value, challengeId]
    : activatedChallengeIds.value.filter((id) => id !== challengeId)
}
</script>

<template>
  <div class="flex flex-col gap-4">
    <DatePicker v-model="completedAt" label="Päiväys" icon="Calendar" />

    <fieldset class="flex flex-col gap-2">
      <legend class="mb-2">Haasteet</legend>
      <label
        v-for="challenge in challenges"
        :key="challenge.id"
        class="flex flex-row items-center gap-2 cursor-pointer"
      >
        <input
          type="checkbox"
          class="w-4 h-4 accent-brand-primary cursor-pointer"
          :checked="activatedChallengeIds.includes(challenge.id)"
          @change="toggleChallenge(challenge.id, ($event.target as HTMLInputElement).checked)"
        />
        <span>{{ challenge.name }}</span>
      </label>
      <span v-if="challenges.length === 0" class="text-xs text-gray-400">Ei haasteita</span>
    </fieldset>
  </div>
</template>
