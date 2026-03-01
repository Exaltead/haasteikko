<script setup lang="ts">

import type { Answer, Question, QuestionType } from "@/models/challenge";
import { RadioGroup, RadioGroupLabel, RadioGroupOption } from "@headlessui/vue"
import { v4 } from "uuid";
import { computed, ref, watch } from "vue";

import { useChallengeApi } from "@/api/challengeApiClient";
import { useAnswerApi } from "@/api/answerApiClient";
import TextInput from "../basics/TextInput.vue";
import AnswerStatus from "./AnswerStatus.vue";
import { useAnswerSaving } from "@/composables/useAnswerSaving";

const challengeApiClient = useChallengeApi()
const answerApiClient = useAnswerApi()

const props = defineProps<{ itemId: string, challengeId: string }>()
const emits = defineEmits<{
  (e: 'submit', answers: Answer[]): void
}>()

const answersMap = ref<Map<string, Answer>>(new Map([]))

// Answer saving composable
const { 
  answerSaveStates, 
  errorMessage, 
  isAnswerDisabled, 
  saveAnswers, 
  updateAnswerStates 
} = useAnswerSaving(answersMap, props.challengeId, props.itemId, emits, answerApiClient)
const questions = ref<Question[]>([])
const challengeName = ref<string>("Haaste")
async function refreshData(): Promise<void> {
  const answers = await answerApiClient.searchAnswers({ challengeId: props.challengeId, itemId: props.itemId })
  const challenge = (await challengeApiClient.fetchChallenges())
    .find(t => t.id === props.challengeId)
  if (challenge) {
    challengeName.value = challenge.name
  }

  const newQuestions = challenge?.questions ?? []
  const newMap = new Map<string, Answer>(answers.map((answer) => [answer.id, { ...answer }]))
  for (const question of newQuestions) {
    if (answers.find(t => t.questionId === question.id)) {
      continue
    }
    const id = v4()
    if (question.kind === "TextInput") {
      newMap.set(id, {
        kind: "TextInput",
        id: id,
        questionId: question.id,
        answered: false,
        answer: "",
        itemId: props.itemId,
      })
    }
    else if (question.kind === "Boolean") {
      newMap.set(id, {
        kind: "Boolean",
        id: id,
        questionId: question.id,
        answered: false,
        answer: "no",
        itemId: props.itemId,
      })
    }
    else {
      throw new Error("Unknown question kind")
    }
  }

  questions.value = newQuestions
  answersMap.value = newMap
}


refreshData()

type DisplayAnswer = {
  kind: QuestionType
  id: string
  number: number
  question: string
  answer: string
  answerId: string
}

const displayAnswers = computed(() => {
  return [...answersMap.value].map(([id, answer]) => {
    const question = questions.value.find((q) => q.id === answer.questionId)
    if (question) {

      return {
        kind: question.kind,
        id: question.id,
        answerId: id,
        number: question.number,
        question: question.question,
        answer: answer.answer
      }

    }
    return null
  }).filter((item): item is DisplayAnswer => item !== null)
    .sort((a, b) => a.number - b.number)
})

function makeRadioStyle(checked: boolean) {
  const adds = checked ? 'bg-brand-primary text-white' : 'border border-brand-primary'

  return "rounded flex items-center w-full justify-center h-full p-1 cursor-pointer" + " " + adds
}

let previousAnswers: string = ""

watch(answersMap, async (newValue) => {
  const newAnswersStr = JSON.stringify([...newValue.values()])

  // Only detect changes if we're not already saving
  const hasSavingAnswers = Object.values(answerSaveStates.value).some(state => state === 'saving')
  
  if (previousAnswers && !hasSavingAnswers) {
    const oldAnswers: Answer[] = JSON.parse(previousAnswers)
    const changedAnswer = [...newValue.values()].find((answer, i) => {
      const oldAnswer = oldAnswers[i]
      return JSON.stringify(answer) !== JSON.stringify(oldAnswer)
    })
    
    if (changedAnswer) {
      updateAnswerStates([changedAnswer.id], 'saving')
      errorMessage.value = null
    }
  }
  previousAnswers = newAnswersStr

  await saveAnswers()
}, { deep: true })

</script>


<template>
  <div>

    <div class="flex flex-row gap-2 justify-between items-center bg-light-gray p-2">
      <h1>{{ challengeName }}</h1>
    </div>
    <div v-if="errorMessage" class="bg-red-100 border border-red-400 text-red-700 px-4 py-2 rounded mb-2">
      {{ errorMessage }}
    </div>

    <div v-for="(displayAnswer, index) in displayAnswers" :key="index" class="bg-light-gray p-2">

      <RadioGroup v-if="displayAnswer.kind === 'Boolean'" v-model="answersMap.get(displayAnswer.answerId)!.answer"
        class="flex flex-col gap-1">
        <div class="flex items-center gap-2">
          <RadioGroupLabel>{{ displayAnswer.question }}</RadioGroupLabel>
          <AnswerStatus :state="answerSaveStates[displayAnswer.answerId] || 'idle'" />
        </div>
        <div class="flex flex-row gap-12 pl-8 justify-end pr-4 items-center">
          <AnswerStatus :state="answerSaveStates[displayAnswer.answerId] || 'idle'" class="mr-4" />
          <RadioGroupOption value="no" v-slot="{ checked }" class="min-w-12 flex" :disabled="isAnswerDisabled(displayAnswer.answerId)">
            <span :class="makeRadioStyle(checked)">Ei</span>
          </RadioGroupOption>
          <RadioGroupOption v-slot="{ checked }" class="min-w-12 flex" value="yes" :disabled="isAnswerDisabled(displayAnswer.answerId)">
            <span :class="makeRadioStyle(checked)">Kyllä</span>
          </RadioGroupOption>
        </div>

      </RadioGroup>
      <div v-else-if="displayAnswer.kind === 'TextInput'" class="flex flex-col gap-1">
        <div class="flex items-center gap-2">
          <span>{{ displayAnswer.question }}</span>
          <AnswerStatus :state="answerSaveStates[displayAnswer.answerId] || 'idle'" />
        </div>
        <TextInput :name="displayAnswer.id" :label="displayAnswer.question"
          :required="true" v-model="answersMap.get(displayAnswer.answerId)!.answer" 
          :disabled="isAnswerDisabled(displayAnswer.answerId)" />
      </div>
    </div>

  </div>

</template>
