<script lang="ts" setup>
withDefaults(defineProps<{
  message: string
  icon?: string
  size?: 'sm' | 'md' | 'lg'
}>(), {
  size: 'md'
})

const getSizeClasses = (size: 'sm' | 'md' | 'lg'): string => {
  switch (size) {
    case 'sm': return 'text-sm'
    case 'md': return 'text-base'
    case 'lg': return 'text-lg'
  }
}
</script>

<template>
  <div class="text-center py-8">
    <p :class="['text-text-primary', getSizeClasses(size)]">
      {{ message }}
    </p>
  </div>
</template>