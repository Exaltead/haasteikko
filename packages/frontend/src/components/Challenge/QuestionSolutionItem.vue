<script lang="ts" setup>
import BrandedSelect from '@/components/basics/BrandedSelect.vue'
import StatusIcon from './StatusIcon.vue'
import EmptyState from '@/components/basics/EmptyState.vue'
import type { Question } from '@/models/challenge'

type QuestionStatus = 'warning' | 'selected' | 'unique' | null

interface QuestionSolutionItemProps {
  question: Question
  options: { name: string, value: string }[]
  status: QuestionStatus
  modelValue: string | string[]
  saving: boolean
  questionIndex: number
}

defineProps<QuestionSolutionItemProps>()
const emit = defineEmits(['update:modelValue'])

function handleSelectUpdate(value: string | string[]) {
  emit('update:modelValue', value)
}
</script>

<template>
  <li class="w-full p-4" :class="questionIndex % 2 === 0 ? 'bg-white' : 'bg-light-gray'">
    <div v-if="question.kind === 'Boolean'" class="flex flex-col gap-2 w-full">
      <h2>{{ question.question }}</h2>
      <div class="flex justify-between items-center gap-2">
        <StatusIcon :status="status" />
        <div class="flex items-center gap-2">
          <span v-if="saving" class="loading-spinner"></span>
          <BrandedSelect v-if="options.length > 0" :options="options"
            :model-value="modelValue" 
            @update:model-value="handleSelectUpdate"
            :disabled="saving" />
          <EmptyState v-else message="Ei vastauksia" size="sm" />
        </div>
      </div>
    </div>
    <div v-else-if="question.kind === 'TextInput'" class="flex flex-col gap-2 w-full">
      <h2>{{ question.question }}</h2>
      <div v-if="options.length > 0" class="flex justify-between items-start gap-2">
        <div class="mt-1">
          <StatusIcon :status="status" />
        </div>
        <div class="flex flex-col gap-2 items-end">
          <div v-for="(_, index) in (modelValue as string[])" :key="index">
            <div v-if="index === 0 || (modelValue as string[])[0] !== ''"
              class="flex items-center gap-2">
              <span v-if="saving" class="loading-spinner"></span>
              <BrandedSelect :options="options"
                :model-value="(modelValue as string[])[index]"
                @update:model-value="(val) => handleSelectUpdate((modelValue as string[]).map((v, i) => i === index ? val : v))"
                :title="`Osa ${index + 1}`"
                :disabled="saving" />
            </div>
          </div>
        </div>
      </div>
      <div v-else class="flex justify-between items-center gap-2">
        <div class="w-5 h-5 flex-shrink-0"></div>
        <EmptyState message="Ei vastauksia" size="sm" />
      </div>
    </div>
  </li>
</template>