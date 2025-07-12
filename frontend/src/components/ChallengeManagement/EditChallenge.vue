<script lang="ts" setup>
import type { Challenge, Question } from '@/models/challenge';
import { computed, ref } from 'vue';
import TextInput from '../basics/TextInput.vue';
import { v4 } from 'uuid';
import IconPlus from "@/components/icons/IconPlus.vue"
import IconBack from '@/components/icons/IconBack.vue';
import BrandedButton from '../basics/BrandedButton.vue';
import { challengeApiClient } from '@/api/challengeApiClient';
import ChallengeQuestionCard from './ChallengeQuestionCard.vue';
const { target } = defineProps<{ target: Challenge | undefined }>()

function createEditTarget(target: Challenge | undefined): Challenge {
  if (target === undefined) {
    return {
      id: "",
      name: "",
      questions: [],
      status: "active",
      targetMedia: "Book",
      kind: "shared"
    }
  } else {
    return {
      ...target,
      questions: target.questions.map((question) => ({
        ...question
      })).sort((a, b) => a.number - b.number)
    }

  }
}
const emit = defineEmits<{
  (e: "submitComplete"): void
  (e: "close"): void
}>()

const isSubmitting = ref(false)

const editTarget = ref<Challenge>(createEditTarget(target))
const activeCheckBox = ref(editTarget.value.status === "active")



function addNewQuestion() {
  const newQuestion: Question = {
    kind: "Boolean",
    id: v4(),
    question: "",
    number: editTarget.value.questions.length,
    questionClusterSize: 1,
  }
  editTarget.value.questions.push(newQuestion)
  editTarget.value.questions.sort((a, b) => a.number - b.number)
}

function removeQuestion(id: string) {
  if (editTarget.value.questions.length <= 1) {
    return
  }
  const index = editTarget.value.questions.findIndex((question) => question.id === id)
  if (index === -1) {
    return
  }
  editTarget.value.questions.splice(index, 1)
  editTarget.value.questions.forEach((question, i) => {
    question.number = i
  })

}

async function submit() {
  if (editTarget.value === undefined) {
    return
  }

  const toSend: Challenge = {
    ...editTarget.value,
    status: activeCheckBox.value ? "active" : "inactive"
  }

  isSubmitting.value = true
  if (toSend.id === "") {
    await challengeApiClient.addChallenge(toSend)
  }
  else {
    await challengeApiClient.updateChallenge(toSend)
  }

  isSubmitting.value = false

  emit("submitComplete")
}

const canSubmit = computed(() => {
  return isSubmitting && editTarget.value.name.length > 0 && editTarget.value.questions.length > 0
})

</script>

<template>
  <div class="p-2">
    <button :onClick="() => emit('close')" class="top-0 left-0 relative">
      <IconBack class="text-brand-primary w-8 h-fit cursor-pointer" />
    </button>
    <div class="px-10 bg-white shadow-lg rounded px-4 py-2 bg-light-gray">
      <div class="flex flex-col gap-4">
        <div class="flex flex-row gap-4 justify-between items-center">
          <TextInput class="basis-4/5" v-model="editTarget.name" name="Nimi" label="Haasteen nimi" />
          <BrandedButton text="Tallenna" :onClick="submit" class="h-fit" :isSubmitting="isSubmitting"
            :disabled="!canSubmit" />
        </div>
        <div class="flex flex-row gap-4">
          <label for="kind"> Haasteeseen soveltuvat: </label>
          <select name="kind" v-model="editTarget.targetMedia">
            <option value="Book">Kirja</option>
            <option value="Game">Peli</option>
          </select>
        </div>

        <div class="flex flex-row gap-4">
          <input type="checkbox" v-model="activeCheckBox" name="active" />
          <label for="active">Aktiivinen</label>
        </div>

        <h2>Kysymykset</h2>
        <ul class="flex flex-col gap-2">
          <li v-for="question, i in editTarget.questions" :key="question.id">
            <ChallengeQuestionCard :questionCount="editTarget.questions.length" :removeQuestion="removeQuestion"
              v-model="editTarget.questions[i]" />
          </li>
          <button class="mt-4 ml-4 w-full rounded h-full flex justify-center cursor-pointer bg-brand-primary"
            :onClick="addNewQuestion">
            <IconPlus class="w-10 text-white"></IconPlus>
          </button>
        </ul>
      </div>
    </div>

  </div>

</template>
