<script lang="ts" setup>
import { computed, ref } from 'vue';
import BrandedButton from './BrandedButton.vue';


type ManagementItem = {
  value: string
  display: string
}

const { items, allOptions } = defineProps<{
  items: ManagementItem[]
  allOptions: ManagementItem[]


}>();

const emit = defineEmits<{
  (e: "stateChanged", currentValues: string[]): void
}>()

const currentState = ref<ManagementItem[]>([...items])

function onDelete(value: string) {
  const index = currentState.value.findIndex(t => t.value === value)
  if (index !== -1) {
    currentState.value.splice(index, 1)
  }
  emit("stateChanged", currentState.value.map(t => t.value))
}

function onAdd(value: string) {
  const toBeAdded = allOptions.find(t => t.value === value)
  if (toBeAdded) {
    currentState.value.push(toBeAdded)
    emit("stateChanged", currentState.value.map(t => t.value))
    newSelection.value = ''
  }
}

const newSelection = ref<string>('')

const currentOptions = computed(() => {
  return allOptions.filter(t => !currentState.value.some(t2 => t2.value === t.value))
})
</script>

<template>
  <div>
    <ul class="list-disc">
      <li v-for="item in currentState" :key="item.value">
        <div class="flex flex-row gap-2">
          <span class="">
            {{ item.display }}
          </span>
          <BrandedButton icon="Delete" :onClick="() => onDelete(item.value)" variant="secondary" />
        </div>
      </li>
    </ul>
    <div class="flex flex-row gap-2" v-if="currentOptions.length > 0">
      <select v-model="newSelection" class="rounded border-brand-primary border-1 px-2 py-1 bg-light-gray">
        <option v-for="option in currentOptions" :key="option.value" :value="option.value">
          {{ option.display }}
        </option>
      </select>
      <BrandedButton text="Lisää" :onClick="() => onAdd(newSelection)" variant="primary" />
    </div>
  </div>


</template>
