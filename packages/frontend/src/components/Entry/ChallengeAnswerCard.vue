<script lang="ts" setup>
import { computed } from 'vue'
import type { Answer, Challenge } from '@/models/challenge'
import { getAnsweredCount, getTotalCount } from '@/composables/useChallengeProgress'
import BaseIcon from '@/components/Icons/BaseIcon.vue'
import ProgressBar from '@/components/Basics/ProgressBar.vue'

const { challenge, itemId, answers } = defineProps<{
  challenge: Challenge
  itemId: string
  answers: Answer[]
}>()

const answeredCount = computed(() => getAnsweredCount(challenge, answers))
const totalCount = computed(() => getTotalCount(challenge))
</script>

<template>
  <RouterLink
    :to="{ name: 'itemChallengeAnswers', params: { id: itemId, challengeId: challenge.id } }"
    class="flex flex-col gap-2 rounded border border-brand-orange/60 bg-white p-3 transition-colors hover:border-brand-orange hover:bg-brand-orange/5"
    :data-testid="`challenge-card-${challenge.id}`"
  >
    <div class="flex flex-row items-center justify-between gap-2">
      <span class="font-medium">{{ challenge.name }}</span>
      <BaseIcon name="ChevronRight" class="h-5 w-5 text-brand-orange" />
    </div>
    <div class="flex flex-row items-center gap-3">
      <div class="flex-1">
        <ProgressBar :progress="totalCount > 0 ? (answeredCount / totalCount) * 100 : 0" />
      </div>
      <span class="text-sm whitespace-nowrap">{{ answeredCount }} / {{ totalCount }} vastattu</span>
    </div>
  </RouterLink>
</template>
