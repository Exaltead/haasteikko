<script setup lang="ts">

type AnswerSaveState = 'idle' | 'saving' | 'success' | 'error'

defineProps<{
  state: AnswerSaveState
}>()
</script>

<template>
  <span v-if="state === 'saving'" class="animate-spin inline-block w-4 h-4 border-2 border-brand-orange border-t-transparent rounded-full"></span>
  <span v-else-if="state === 'success'" class="text-green-500">✓</span>
  <span v-else-if="state === 'error'" class="text-red-500">✗</span>
</template>