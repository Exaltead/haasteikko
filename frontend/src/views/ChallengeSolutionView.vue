<script lang="ts" setup>
import { useAnswerApi } from '@/api/answerApiClient';
import { useChallengeApi } from '@/api/challengeApiClient';
import { useLibraryApi } from '@/api/libraryApiClient';

import { useSolutionsApi } from '@/api/solutionsApiClient';
import BrandedButton from '@/components/basics/BrandedButton.vue';
import BrandedSelect from '@/components/basics/BrandedSelect.vue';
import type { Answer, Question, Solution, SolutionSet } from '@/models/challenge';
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

function getEmptySolution(): Solution[] {
  return questions.value.map(t => {
    return {
      questionId: t.id,
      singleAnswerItemId: "",
      multipleAnswerItemIds: [...Array(t.questionClusterSize).keys()].map(() => "")
    }
  })
}

const solution = ref<Solution[]>([])
const solutionId = ref<string | undefined>(undefined)
async function getSolution(): Promise<Solution[]> {
  const challengeId = route.params.id as string
  const solutionSet = await solutionsApiClient.getSolutionSetByChallengeId(challengeId)
  if (solutionSet === undefined) {
    const solution = getEmptySolution()

    return solution
  } else {
    solutionId.value = solutionSet.id
    const solutions = []

    for (const question of questions.value) {
      const solutionItem = solutionSet.solutions.find(t => t.questionId === question.id)
      if (solutionItem === undefined) {
        solutions.push({
          questionId: question.id,
          singleAnswerItemId: "",
          multipleAnswerItemIds: [...Array(question.questionClusterSize).keys()].map(() => "")
        })
      } else {
        if (question.kind === "TextInput" && solutionItem.multipleAnswerItemIds.length !== question.questionClusterSize) {
          solutionItem.multipleAnswerItemIds = [...Array(question.questionClusterSize).keys()].map(() => "")
        }
        solutions.push(solutionItem)
      }
    }

    return solutions
  }


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
  }

  // Loads answers
  const loadAnswers = async () => {
    const answers = await answerApiClient.getChallengeAnswers(challengeId)
    allAnswers.value = answers.filter(t => {
      return t.answered === true
    })
  }

  await Promise.all([loadItems(), loadChallenges(), loadAnswers()])

  solution.value = await getSolution()
  loading.value = false
}

const isSubmitting = ref(false)
async function submitSolution() {
  isSubmitting.value = true
  const challengeId = route.params.id as string
  if (solutionId.value === undefined) {
    const solutionSet: Omit<SolutionSet, "id"> = {
      challengeId: challengeId,
      solutions: solution.value
    }
    await solutionsApiClient.addSolutionSet(solutionSet)
  } else {
    const solutionSet: SolutionSet = {
      id: solutionId.value,
      challengeId: challengeId,
      solutions: solution.value
    }
    await solutionsApiClient.updateSolutionSet(solutionSet)
  }

  isSubmitting.value = false

  solution.value = await getSolution()
}

watch(() => route.params.id, loadData, { immediate: true })



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
    }
  })

  return mapping.sort((a, b) => a.question.number - b.question.number)
})





</script>


<template>
  <div>
    <BrandedButton @click="$router.back()" class="mt-4 ml-4 md:mt-14 md:ml-14 w-fit" icon="Back" />
    <div v-if="loading">
      <h1>Ladataan...</h1>
    </div>
    <div v-else class="flex flex-col items-center">
      <div class="flex flex-col gap-4 mt-10 items-center w-fit">
        <ul class="flex flex-col gap-4 items-center">
          <li v-for="{ question, options }, i in questionToAnswersMap" :key="question.id" class="w-full">
            <div v-if="question.kind === 'Boolean'" class="flex flex-col gap-2 justify-start">
              <h2>{{ question.question }}</h2>
              <div class="flex flex-row justify-end w-full">
                <BrandedSelect v-if="options.length > 0" :options="options" v-model="solution[i].singleAnswerItemId" />
                <span v-else class="text-text-primary">Ei vastauksia</span>
              </div>

            </div>
            <div v-else-if="question.kind === 'TextInput'" class="flex flex-col gap-2">
              <h2>{{ question.question }}</h2>
              <div v-if="options.length > 0">
                <div v-for="_, index in solution[i].multipleAnswerItemIds" :key="index">
                  <div v-if="index === 0 || solution[i].multipleAnswerItemIds[0] !== ''">
                    <BrandedSelect v-if="options.length > 0" :options="options"
                      v-model="solution[i].multipleAnswerItemIds[index]" :title="`Osa ${index + 1}`" />
                  </div>
                </div>
              </div>
              <span v-else class="text-text-primary">Ei vastauksia</span>
            </div>
          </li>
        </ul>
        <div class="flex flex-row justify-start w-full">
          <BrandedButton @click="() => submitSolution()" class="mb-4 w-fit" text="Tallenna"
            :isSubmitting="isSubmitting" />
        </div>

      </div>
    </div>

  </div>

</template>
