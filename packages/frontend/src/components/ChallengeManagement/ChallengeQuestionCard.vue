<script lang="ts" setup>
import TextInput from '../basics/TextInput.vue';
import BrandedButton from '../basics/BrandedButton.vue';

import type { Question } from '@/models/challenge';
import { computed } from 'vue';
import BrandedSelect from '../basics/BrandedSelect.vue';
const model = defineModel<Question>()

const props = defineProps<{
  questionCount: number,
  removeQuestion: (id: string) => void,
}>()

const selectOptions = computed(() => {
  return [...Array(props.questionCount).keys()].map((i) => ({
    name: (i + 1).toString(),
    value: i,
  }))
})

function internalRemoveQuestion() {
  const id = model.value?.id
  if (!!id) {
    props.removeQuestion(id)
  }
}

const kinds = [
  { name: "Kyllä/Ei", value: "Boolean" },
  { name: "Tekstisyöte", value: "TextInput" },
]

const questionClusterSizeOptions = [
  { name: "1", value: 1 },
  { name: "2", value: 2 },
  { name: "3", value: 3 },
  { name: "4", value: 4 },
  { name: "5", value: 5 },
]

</script>

<template>
  <div v-if="!model"></div>
  <div v-else class="bg-white shadow-lg rounded px-4 py-2 flex flex-col gap-2">
    <TextInput v-model="model.question" name="kysymys" label="Kysymys" />
    <BrandedSelect v-model="model.kind" :options="kinds" title="Kysymyksen tyyppi" />
    <div v-if="model.kind === 'TextInput'">
      <BrandedSelect v-model="model.questionClusterSize" :options="questionClusterSizeOptions"
        title="Montako samaa vastausta tarvitaan?" />
    </div>

    <div class="flex flex-row gap-4">
      <BrandedSelect v-model="model.number" :options="selectOptions" title="Järjestysnumero" />

    </div>
    <div class="flex flex-row justify-end">
      <BrandedButton icon="Delete" class="h-fit" :onClick="() => internalRemoveQuestion()" variant="secondary" />
    </div>
  </div>


</template>
