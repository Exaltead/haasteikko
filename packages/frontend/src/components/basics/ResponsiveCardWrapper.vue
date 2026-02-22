<script lang="ts" setup>
import { ref, onMounted, onUnmounted } from 'vue'

defineProps<{
  maxWidth?: string
}>()

const isDesktop = ref(false)

function checkScreenSize() {
  isDesktop.value = window.innerWidth >= 768 // md breakpoint
}

onMounted(() => {
  checkScreenSize()
  window.addEventListener('resize', checkScreenSize)
})

onUnmounted(() => {
  window.removeEventListener('resize', checkScreenSize)
})
</script>

<template>
  <!-- Mobile: No card wrapper, just render content directly -->
  <div v-if="!isDesktop" class="w-full" :class="maxWidth ? `max-w-[${maxWidth}]` : 'max-w-[1100px]'">
    <slot ></slot>
  </div>

  <!-- Desktop: Wrap content in card -->
  <div v-else class="card bg-brand-warm-white w-full shadow-lg" :class="maxWidth ? `max-w-[${maxWidth}]` : 'max-w-[1100px]'">
    <slot ></slot>
  </div>
</template>
