<script lang="ts" setup>
import CustomIcon from '@/components/basics/CustomIcon.vue'
import type { IconName } from '@/models/iconName'

type QuestionStatus = 'warning' | 'selected' | 'unique' | null

withDefaults(defineProps<{
  status: QuestionStatus
  size?: 'sm' | 'md' | 'lg'
}>(), {
  size: 'md'
})

const getIconName = (status: QuestionStatus): IconName | null => {
  switch (status) {
    case 'warning': return 'Warning'
    case 'selected': return 'Check'
    case 'unique': return 'DoubleCheck'
    default: return null
  }
}

const getIconColor = (status: QuestionStatus): string => {
  switch (status) {
    case 'warning': return 'text-yellow-500'
    case 'selected': return 'text-green-500'
    case 'unique': return 'text-green-500'
    default: return 'text-transparent'
  }
}

const getIconSize = (size: 'sm' | 'md' | 'lg'): string => {
  switch (size) {
    case 'sm': return 'w-4 h-4'
    case 'md': return 'w-5 h-5'
    case 'lg': return 'w-6 h-6'
  }
}
</script>

<template>
  <div class="flex-shrink-0" :class="getIconSize(size)">
    <CustomIcon 
      v-if="getIconName(status)" 
      :name="getIconName(status)!" 
      :class="getIconColor(status)"
    />
  </div>
</template>