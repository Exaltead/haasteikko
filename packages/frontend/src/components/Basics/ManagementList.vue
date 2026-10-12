<script lang="ts" setup>
import { computed, ref, watch } from 'vue'
import BrandedButton from './BrandedButton.vue'

type ManagementItem = {
  value: string
  display: string
}

const { items, allOptions, title } = defineProps<{
  items: ManagementItem[]
  allOptions: ManagementItem[]
  title: string
}>()

const emit = defineEmits<{
  (e: "stateChanged", currentValues: string[]): void
}>()

const currentValues = ref<string[]>(items.map((item) => item.value))
const isEditing = ref(false)

watch(
  () => items,
  (newItems) => {
    currentValues.value = newItems.map((item) => item.value)
  },
)

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

const selectedLabels = computed(() => {
  return currentValues.value
    .map((value) => allOptions.find((option) => option.value === value)?.display)
    .filter((display): display is string => display !== undefined)
})
</script>

<template>
  <div class="flex flex-col gap-2">
    <div class="flex flex-row items-center justify-between">
      <h2>{{ title }}</h2>
      <BrandedButton icon="Edit" :onClick="() => (isEditing = !isEditing)" variant="secondary" :is-pill="true" />
    </div>

    <div v-if="!isEditing" class="flex flex-row flex-wrap items-center gap-2">
      <span
        v-for="label in selectedLabels"
        :key="label"
        class="rounded-full bg-brand-primary/10 px-2 py-0.5 text-xs text-text-primary"
      >
        {{ label }}
      </span>
      <span v-if="selectedLabels.length === 0" class="text-xs text-gray-400">
        Ei valittuja haasteita
      </span>
    </div>

    <div v-else class="flex flex-col gap-2">
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
      <BrandedButton text="Valmis" icon="Check" :onClick="() => (isEditing = false)" variant="primary" class="w-fit" />
    </div>
  </div>
</template>
