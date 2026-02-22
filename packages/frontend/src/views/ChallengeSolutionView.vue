<script lang="ts" setup>
import { useAnswerApi } from '@/api/answerApiClient';
import { useChallengeApi } from '@/api/challengeApiClient';
import { useLibraryApi } from '@/api/libraryApiClient';

import { useSolutionsApi } from '@/api/solutionsApiClient';
import BrandedButton from '@/components/basics/BrandedButton.vue';
import LibraryItemCard from '@/components/EntryListing/LibraryItemCard.vue';
import TabNavigation from '@/components/basics/TabNavigation.vue'
import ResponsiveCardWrapper from '@/components/basics/ResponsiveCardWrapper.vue'
import CustomIcon from '@/components/basics/CustomIcon.vue';
import EmptyState from '@/components/basics/EmptyState.vue';
import QuestionSolutionItem from '@/components/Challenge/QuestionSolutionItem.vue';
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
const challengeMediaType = ref<"Book" | "Game">("Book")

const solution = ref<Solution[]>([])
async function getSolution(): Promise<Solution[]> {
  const challengeId = route.params.id as string
  const serverSolutions = await solutionsApiClient.searchSolutions({ challengeId: challengeId })

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
    challengeMediaType.value = challenge.targetMedia
  }

  // Loads answers
  const loadAnswers = async () => {
    const answers = await answerApiClient.searchAnswers({ challengeId })
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

    const questionSolution = getSolutionForQuestion(question.id)
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

  const questionSolution = getSolutionForQuestion(question.id)
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

// Filter library items for the current challenge
const challengeLibraryItems = computed(() => {
  const challengeId = route.params.id as string
  return allItems.value.filter(item =>
    item.activatedChallengeIds.includes(challengeId)
  )
})

function getSolutionForQuestion(questionId: string) {
  return solution.value.find(t => t.questionId === questionId)
}

function getSolutionValueForDisplay(questionId: string): string | string[] {
  const solutionItem = getSolutionForQuestion(questionId)
  if (!solutionItem) return []

  return solutionItem.kind === "SinglePartSolution"
    ? solutionItem.singleAnswerItemId || ""
    : solutionItem.multipleAnswerItemIds
}

function updateSolutionForQuestion(questionId: string, newValue: string | string[]) {
  const solutionIndex = solution.value.findIndex(t => t.questionId === questionId)
  if (solutionIndex !== -1) {
    const currentSolution = solution.value[solutionIndex]

    if (currentSolution?.kind === "SinglePartSolution") {
      solution.value[solutionIndex] = {
        kind: "SinglePartSolution",
        questionId: questionId,
        singleAnswerItemId: newValue as string,
        multipleAnswerItemIds: currentSolution.multipleAnswerItemIds
      }
    } else {
      solution.value[solutionIndex] = {
        kind: "MultiPartSolution",
        questionId: questionId,
        singleAnswerItemId: currentSolution?.singleAnswerItemId,
        multipleAnswerItemIds: newValue as string[]
      }
    }
  }
}

</script>


<template>
  <div>
    <div v-if="loading">
      <h1>Ladataan...</h1>
    </div>
    <div v-else class="flex flex-col items-center px-0 md:px-4">
      <ResponsiveCardWrapper>
        <div class="flex flex-col w-full">
          <div
            class="flex items-center justify-between w-full p-2 md:px-6 md:py-4 border-b border-brand-orange bg-white sticky top-0 z-10">
            <BrandedButton @click="$router.back()" icon="Back" variant="primary" class="ml-2 md:ml-0" />
            <div class="flex items-center gap-2">
              <CustomIcon :name="challengeMediaType" class="w-6 h-6 md:w-8 md:h-8 text-brand-orange/70" />
              <h1 class="text-lg md:text-xl font-bold whitespace-normal">{{ challengeName }}</h1>
            </div>
            <div class="w-8 md:w-10"></div> <!-- Spacer for balance -->
          </div>

          <TabNavigation
            tab1Label="Ratkaisu"
            :tab2Label="`Kirjastoni (${challengeLibraryItems.length})`">
            <template #tab1>
              <div class="overflow-hidden w-full max-w-[84rem] mx-auto px-2 md:px-0 py-4">
                <ul class="flex flex-col w-full">
                  <QuestionSolutionItem
                    v-for="{ question, options, status }, i in questionToAnswersMap"
                    :key="question.id"
                    :question="question"
                    :options="options"
                    :status="status"
                    :model-value="getSolutionValueForDisplay(question.id)"
                    :saving="savingQuestionId === question.id"
                    :question-index="i"
                    @update:model-value="updateSolutionForQuestion(question.id, $event)"
                  />
                </ul>
                </div>
              </template>

              <template #tab2>
                <div class="flex flex-col items-center w-full">
                  <div v-if="challengeLibraryItems.length === 0" class="max-w-[84rem] mx-auto">
                    <EmptyState message="Ei kirjastotietueita tähän haasteeseen" />
                  </div>
                  <div v-else
                    class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-2 md:gap-4 w-full max-w-[84rem] mx-auto px-2 md:px-0">
                    <LibraryItemCard v-for="(item, index) in challengeLibraryItems" :key="item.id" :item="item"
                      :orderingNumber="index + 1" @itemUpdated="loadData" />
                  </div>
                </div>
              </template>
          </TabNavigation>
        </div>
      </ResponsiveCardWrapper>
    </div>
  </div>

</template>
