<script lang="ts" setup>
import { ref } from 'vue'

type ManagementItem = {
  value: string
  display: string
}

const { items, allOptions } = defineProps<{
  items: ManagementItem[]
  allOptions: ManagementItem[]
}>()

const emit = defineEmits<{
  (e: "stateChanged", currentValues: string[]): void
}>()

const currentValues = ref<string[]>(items.map((item) => item.value))

function toggle(value: string, checked: boolean) {
  if (checked) {
    if (!currentValues.value.includes(value)) {
      currentValues.value = [...currentValues.value, value]
    }
  } else {
    currentValues.value = currentValues.value.filter((v) => v !== value)
  }
  emit("stateChanged", currentValues.value)
}
</script>

<template>
  <ul class="flex flex-col gap-1">
    <li v-for="option in allOptions" :key="option.value">
      <label class="flex flex-row items-center gap-2 cursor-pointer">
        <input
          type="checkbox"
          class="w-4 h-4 accent-brand-primary cursor-pointer"
          :checked="currentValues.includes(option.value)"
          @change="toggle(option.value, ($event.target as HTMLInputElement).checked)"
        />
        <span>{{ option.display }}</span>
      </label>
    </li>
  </ul>
</template>
