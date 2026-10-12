<script lang="ts" setup>
defineProps<{
  progress: number // 0-100
  solvedProgress?: number // 0-100, drawn in green over `progress`
}>()

function clamp(value: number): number {
  return Math.min(100, Math.max(0, value))
}
</script>

<template>
  <div class="relative w-full h-3 bg-gray-200 rounded-full overflow-hidden">
    <div
      class="absolute inset-y-0 left-0 bg-brand-orange rounded-full transition-all duration-300"
      :style="{ width: `${clamp(progress)}%` }"
    />
    <div
      v-if="solvedProgress !== undefined"
      class="absolute inset-y-0 left-0 bg-brand-green rounded-full transition-all duration-300"
      :style="{ width: `${clamp(solvedProgress)}%` }"
    />
  </div>
</template>
