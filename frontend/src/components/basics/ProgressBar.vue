<script lang="ts" setup>
defineProps<{
  progress: number // 0-100
}>()
</script>

<template>
  <div class="w-full h-3 bg-gray-200 rounded-full overflow-hidden">
    <div
      class="h-full bg-brand-orange rounded-full transition-all duration-300"
      :style="{ width: `${Math.min(100, Math.max(0, progress))}%` }"
    />
  </div>
</template>
