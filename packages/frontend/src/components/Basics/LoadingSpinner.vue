<script lang="ts" setup>
import { computed } from 'vue';

const props = defineProps<{
  backgroundColor?: string
}>()

const spinnerClass = computed(() => {
  const bgColor = props.backgroundColor ? props.backgroundColor : 'bg-brand-primary'

  return `${bgColor} text-text-primary mr-3 size-5 animate-spin rounded-full border-4 border-white border-t-transparent`
})
</script>

<template>
  <div :class="spinnerClass"></div>

</template>
