<script lang="ts" setup>
import type { Challenge } from '@/models/challenge'
import BaseIcon from '@/components/Icons/BaseIcon.vue'
import ProgressBar from '@/components/Basics/ProgressBar.vue'

defineProps<{
  challenge: Challenge
  completedCount: number
  totalCount: number
  attachedItemCount: number
}>()

</script>

<template>
  <RouterLink :to="{ name: 'challengeSolution', params: { id: challenge.id } }" class="w-full">
    <div class="card flex flex-col md:max-w-[400px] min-w-[250px] w-full p-4 gap-3">
      <div class="flex flex-row justify-between items-center w-full border-b border-brand-orange pb-2">
        <div class="flex flex-row items-center gap-2">
          <BaseIcon :name="challenge.targetMedia" class="text-brand-orange h-6 w-6" />
          <h3 class="font-bold text-lg">{{ challenge.name }}</h3>
        </div>
        <BaseIcon name="ChevronRight" class="text-brand-orange h-6 w-6" />
      </div>

      <div class="flex flex-row items-center gap-3">
        <div class="flex-1">
          <ProgressBar :progress="totalCount > 0 ? (completedCount / totalCount) * 100 : 0" />
        </div>
        <span class="text-sm font-medium whitespace-nowrap">
          {{ completedCount }} / {{ totalCount }}
        </span>
      </div>

      <div class="flex flex-row items-center gap-2 text-sm text-gray-600">
        <BaseIcon :name="challenge.targetMedia" class="h-4 w-4 text-brand-orange" />
        <span>{{ attachedItemCount }} {{ challenge.targetMedia === 'Book' ? 'kirjaa' : 'peliä' }}</span>
      </div>
    </div>
  </RouterLink>
</template>
