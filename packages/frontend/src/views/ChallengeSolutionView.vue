<script lang="ts" setup>
import { useAnswerApi } from '@/api/answerApiClient';
import { useChallengeApi } from '@/api/challengeApiClient';
import { useLibraryApi } from '@/api/libraryApiClient';

import { useSolutionsApi } from '@/api/solutionsApiClient';
import BrandedButton from '@/components/basics/BrandedButton.vue';
import BrandedSelect from '@/components/basics/BrandedSelect.vue';
import IconCheck from '@/components/icons/IconCheck.vue';
import IconDoubleCheck from '@/components/icons/IconDoubleCheck.vue';
import IconWarning from '@/components/icons/IconWarning.vue';
import type { Answer, Question, Solution } from '@/models/challenge';
import type { LibraryItem } from '@/models/LibraryItem';
import { computed, ref, watch } from 'vue';
import { useRoute } from 'vue-router';

const route = useRoute()
const libraryApi = useLibraryApi()
const challengeApiClient = useChallengeApi()
const answerApiClient = useAnswerApi()
const solutionsApiClient = useSolutionsApi()


const allItems = ref<LibraryItem[]>([])
const questions = ref<Question[]>([])
const allAnswers = ref<Answer[]>([])
const challengeName = ref("")

const solution = ref<Solution[]>([])
async function getSolution(): Promise<Solution[]> {
  const challengeId = route.params.id as string
  const serverSolutions = await solutionsApiClient.searchSolutions({challengeId: challengeId })

  // Build solution array for all questions, using server data where available
  const result: Solution[] = []
  for (const question of questions.value) {
    const existingSolution = serverSolutions.find(t => t.questionId === question.id)
    if (existingSolution === undefined) {
      result.push({
        kind: question.questionClusterSize > 1 ? "MultiPartSolution" : "SinglePartSolution",
        questionId: question.id,
        singleAnswerItemId: "",
        multipleAnswerItemIds: [...Array(question.questionClusterSize).keys()].map(() => "")
      })
    } else {
      if (question.kind === "TextInput" && existingSolution.multipleAnswerItemIds.length !== question.questionClusterSize) {
        existingSolution.multipleAnswerItemIds = [...Array(question.questionClusterSize).keys()].map(() => "")
      }
      result.push(existingSolution)
    }
  }
  return result
}

const loading = ref(false)
async function loadData() {
  loading.value = true
  const challengeId = route.params.id as string
  // Loads library items
  const loadItems = async () => {
    const items = await libraryApi.fetchLibraryItems()
    // TODO: filter on server side
    items.filter((item) => {
      return item.activatedChallengeIds.includes(challengeId)
    })

    allItems.value = items
  }

  // Loads questions in the challenges
  const loadChallenges = async () => {
    const challenges = await challengeApiClient.fetchChallenges()
    const challenge = challenges.find(t => t.id === challengeId)
    if (challenge === undefined) {
      throw new Error("Challenge not found")
    }
    challenge.questions.sort((a, b) => a.number - b.number)
    questions.value = challenge.questions
    challengeName.value = challenge.name
  }

  // Loads answers
  const loadAnswers = async () => {
    const answers = await answerApiClient.searchAnswers({ challengeId})
    allAnswers.value = answers.filter(t => {
      return t.answered === true
    })
  }

  await Promise.all([loadItems(), loadChallenges(), loadAnswers()])

  solution.value = await getSolution()
  previousSolution = JSON.stringify(solution.value)
  loading.value = false
}

watch(() => route.params.id, loadData, { immediate: true })

const savingQuestionId = ref<string | null>(null)

let previousSolution: string = ""

watch(solution, async (newValue) => {
  if (loading.value) return

  const newSolutionStr = JSON.stringify(newValue)

  if (previousSolution) {
    const oldSolution: Solution[] = JSON.parse(previousSolution)
    const changedQuestion = newValue.find((s, i) =>
      JSON.stringify(s) !== JSON.stringify(oldSolution[i])
    )
    if (changedQuestion) {
      savingQuestionId.value = changedQuestion.questionId
    }
  }
  previousSolution = newSolutionStr

  // Filter out solutions with empty item IDs and convert empty strings to undefined
  const solutionsToSave = newValue
    .filter(s => {
      if (s.kind === "SinglePartSolution") {
        return s.singleAnswerItemId && s.singleAnswerItemId !== ""
      } else {
        return s.multipleAnswerItemIds.some(id => id && id !== "")
      }
    })
    .map(s => ({
      ...s,
      singleAnswerItemId: s.singleAnswerItemId && s.singleAnswerItemId !== "" ? s.singleAnswerItemId : undefined,
      multipleAnswerItemIds: s.multipleAnswerItemIds.filter(id => id && id !== "")
    }))

  const challengeId = route.params.id as string
  await solutionsApiClient.upsertSolutions(solutionsToSave, challengeId)

  savingQuestionId.value = null
}, { deep: true })



function getQuestionAnswers(question: Question): Answer[] {
  if (question.kind === "Boolean") {
    return allAnswers.value.filter((answer) => answer.questionId === question.id
      && answer.kind === "Boolean" && answer.answer === "yes")

  }
  if (question.kind === "TextInput") {
    const options = allAnswers.value.filter((answer) => answer.questionId === question.id
      && answer.kind === "TextInput" && answer.answer !== "")

    const questionSolution = solution.value.find(t => t.questionId === question.id)
    if (questionSolution === undefined) {
      return []
    }


    const clusterSize = question.questionClusterSize

    const uniqueAnswers = [... new Set<string>(options.map(t => t.answer))]
      .filter(t => {
        const count = options.filter(t2 => t2.answer === t).length
        return count >= clusterSize
      })


    const trueOptions = options.filter(t => uniqueAnswers.includes(t.answer))

    if (questionSolution.multipleAnswerItemIds[0] !== "") {
      const answered = allAnswers.value
        .find(t => t.itemId === questionSolution.multipleAnswerItemIds[0]
          && t.questionId === question.id)

      if (answered === undefined) {
        return []
      }
      return trueOptions.filter(t => t.answer === answered.answer)
    }

    return trueOptions
  }

  return []
}

type QuestionStatus = 'warning' | 'selected' | 'unique' | null

function getQuestionStatus(question: Question, options: { name: string, value: string }[]): QuestionStatus {
  if (options.length === 0) {
    return null
  }

  const questionSolution = solution.value.find(t => t.questionId === question.id)
  if (!questionSolution) {
    return 'warning'
  }

  // Get the selected item ID(s) for this question
  let selectedItemId: string | null = null
  if (question.kind === 'Boolean') {
    selectedItemId = questionSolution.singleAnswerItemId || null
  } else if (question.kind === 'TextInput') {
    selectedItemId = questionSolution.multipleAnswerItemIds[0] || null
  }

  if (!selectedItemId) {
    return 'warning'
  }

  // Check if this item is used in other questions
  const otherSolutionsUsingItem = solution.value.filter(s => {
    if (s.questionId === question.id) return false
    if (s.singleAnswerItemId === selectedItemId) return true
    if (s.multipleAnswerItemIds.includes(selectedItemId!)) return true
    return false
  })

  if (otherSolutionsUsingItem.length === 0) {
    return 'unique'
  }

  return 'selected'
}

const questionToAnswersMap = computed(() => {

  const mapping = questions.value.map((question) => {
    const answers = getQuestionAnswers(question)
      .map(answer => {
        const item = allItems.value.find(t2 => t2.id === answer.itemId)
        if (item === undefined) {
          return undefined
        }
        return {
          name: item.title,
          value: item.id
        }
      })
      .filter((t): t is { name: string, value: string } => t !== undefined)
    return {
      question,
      options: answers,
      status: getQuestionStatus(question, answers),
    }
  })



  const sorted = mapping.sort((a, b) => a.question.number - b.question.number)

  return sorted
})


</script>


<template>
  <div>
    <BrandedButton @click="$router.back()" class="mt-4 ml-4 md:mt-14 md:ml-14 w-fit" icon="Back" variant="primary" />
    <div v-if="loading">
      <h1>Ladataan...</h1>
    </div>
    <div v-else class="flex flex-col items-center px-4">
      <div class="flex flex-col gap-4 mt-4 w-full max-w-2xl">
        <div class="card overflow-hidden">
          <h1 class="p-4 text-xl font-bold border-b border-brand-orange">{{ challengeName }}</h1>
          <ul class="flex flex-col w-full">
            <li v-for="{ question, options, status }, i in questionToAnswersMap" :key="question.id"
              class="w-full p-4" :class="i % 2 === 0 ? 'bg-white' : 'bg-light-gray'">
              <div v-if="question.kind === 'Boolean'" class="flex flex-col gap-2 w-full">
                <h2>{{ question.question }}</h2>
                <div class="flex justify-between items-center gap-2">
                  <div class="w-5 h-5 flex-shrink-0">
                    <IconWarning v-if="status === 'warning'" class="w-5 h-5 text-yellow-500" />
                    <IconCheck v-else-if="status === 'selected'" class="w-5 h-5 text-green-500" />
                    <IconDoubleCheck v-else-if="status === 'unique'" class="w-5 h-5 text-green-500" />
                  </div>
                  <div class="flex items-center gap-2">
                    <span v-if="savingQuestionId === question.id" class="animate-spin inline-block w-4 h-4 border-2 border-brand-orange border-t-transparent rounded-full"></span>
                    <BrandedSelect v-if="options.length > 0" :options="options" v-model="solution.find(t => t.questionId === question.id)!.singleAnswerItemId" :disabled="savingQuestionId === question.id" />
                    <span v-else class="text-text-primary">Ei vastauksia</span>
                  </div>
                </div>
              </div>
              <div v-else-if="question.kind === 'TextInput'" class="flex flex-col gap-2 w-full">
                <h2>{{ question.question }}</h2>
                <div v-if="options.length > 0" class="flex justify-between items-start gap-2">
                  <div class="w-5 h-5 flex-shrink-0 mt-1">
                    <IconWarning v-if="status === 'warning'" class="w-5 h-5 text-yellow-500" />
                    <IconCheck v-else-if="status === 'selected'" class="w-5 h-5 text-green-500" />
                    <IconDoubleCheck v-else-if="status === 'unique'" class="w-5 h-5 text-green-500" />
                  </div>
                  <div class="flex flex-col gap-2 items-end">
                    <div v-for="_, index in solution.find(t => t.questionId === question.id)!.multipleAnswerItemIds" :key="index">
                      <div v-if="index === 0 || solution[i]!.multipleAnswerItemIds[0] !== ''" class="flex items-center gap-2">
                        <span v-if="savingQuestionId === question.id" class="animate-spin inline-block w-4 h-4 border-2 border-brand-orange border-t-transparent rounded-full"></span>
                        <BrandedSelect v-if="options.length > 0" :options="options"
                          v-model="solution[i]!.multipleAnswerItemIds[index]" :title="`Osa ${index + 1}`" :disabled="savingQuestionId === question.id" />
                      </div>
                    </div>
                  </div>
                </div>
                <div v-else class="flex justify-between items-center gap-2">
                  <div class="w-5 h-5 flex-shrink-0"></div>
                  <span class="text-text-primary">Ei vastauksia</span>
                </div>
              </div>
            </li>
          </ul>
        </div>
      </div>
    </div>

  </div>

</template>
