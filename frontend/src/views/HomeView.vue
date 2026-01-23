<script lang="ts" setup>
import { useChallengeApi } from '@/api/challengeApiClient'
import { useAnswerApi } from '@/api/answerApiClient'
import { useLibraryApi } from '@/api/libraryApiClient'
import type { Challenge, Answer } from '@/models/challenge'
import type { LibraryItem } from '@/models/LibraryItem'
import { computed, ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import ChallengeCard from '@/components/Challenge/ChallengeCard.vue'
import LoadingSpinner from '@/components/basics/LoadingSpinner.vue'
import EntryListing from '@/components/EntryListing/EntryListing.vue'
import NewItemModal from '@/components/EntryListing/NewItemModal.vue'
import BrandedButton from '@/components/basics/BrandedButton.vue'
import IconChevronRight from '@/components/icons/IconChevronRight.vue'

const router = useRouter()
const challengeApiClient = useChallengeApi()
const answerApiClient = useAnswerApi()
const libraryApiClient = useLibraryApi()

const challenges = ref<Challenge[]>([])
const answersByChallenge = ref<Map<string, Answer[]>>(new Map())
const libraryItems = ref<LibraryItem[]>([])
const loading = ref(true)
const showNewItemModal = ref(false)

const activeChallenges = computed(() => {
  return challenges.value.filter(challenge => challenge.status === "active")
})

const recentLibraryItems = computed(() => {
  return [...libraryItems.value]
    .sort((a, b) => new Date(b.addedAt).getTime() - new Date(a.addedAt).getTime())
    .slice(0, 5)
})

function getCompletedCount(challenge: Challenge): number {
  const answers = answersByChallenge.value.get(challenge.id) ?? []

  const questionsWithAnswers = new Set<string>()

  for (const answer of answers) {
    if (!answer.answered) continue

    const question = challenge.questions.find(q => q.id === answer.questionId)
    if (!question) continue

    if (question.kind === "Boolean" && answer.answer === "yes") {
      questionsWithAnswers.add(answer.questionId)
    } else if (question.kind === "TextInput" && answer.answer !== "") {
      questionsWithAnswers.add(answer.questionId)
    }
  }

  return questionsWithAnswers.size
}

function getTotalCount(challenge: Challenge): number {
  return challenge.questions.length
}

function getAttachedItemCount(challenge: Challenge): number {
  return libraryItems.value.filter(item =>
    item.kind === challenge.targetMedia &&
    item.activatedChallengeIds.includes(challenge.id)
  ).length
}

async function loadData() {
  loading.value = true

  const [challengesResult, itemsResult] = await Promise.all([
    challengeApiClient.fetchChallenges(),
    libraryApiClient.fetchLibraryItems()
  ])

  challenges.value = challengesResult
  libraryItems.value = itemsResult

  const active = challenges.value.filter(c => c.status === "active")
  const answerPromises = active.map(async (challenge) => {
    const answers = await answerApiClient.searchAnswers({ challengeId: challenge.id })
    return { challengeId: challenge.id, answers }
  })

  const results = await Promise.all(answerPromises)
  const newMap = new Map<string, Answer[]>()
  for (const result of results) {
    newMap.set(result.challengeId, result.answers)
  }
  answersByChallenge.value = newMap

  loading.value = false
}

onMounted(loadData)

function openNewItemModal() {
  showNewItemModal.value = true
}

function closeNewItemModal() {
  showNewItemModal.value = false
}

function onNewItemSubmitComplete(id: string) {
  showNewItemModal.value = false
  router.push({ name: 'libraryItem', params: { id } })
}

async function onItemUpdated() {
  libraryItems.value = await libraryApiClient.fetchLibraryItems()
}
</script>

<template>
  <div class="p-4 md:p-8 flex flex-col gap-4">
    <div class="bg-brand-warm-white rounded-lg p-4">
      <h2 class="text-lg font-bold mb-2">Käynnissä olevat haasteet</h2>

      <div v-if="loading" class="flex justify-center py-8">
        <LoadingSpinner />
      </div>

      <div v-else-if="activeChallenges.length === 0" class="text-center py-4">
        <p class="text-gray-500">Ei aktiivisia haasteita</p>
      </div>

      <div v-else class="flex flex-wrap gap-3 justify-center md:justify-start">
        <ChallengeCard
          v-for="challenge in activeChallenges"
          :key="challenge.id"
          :challenge="challenge"
          :completed-count="getCompletedCount(challenge)"
          :total-count="getTotalCount(challenge)"
          :attached-item-count="getAttachedItemCount(challenge)"
        />
      </div>
    </div>

    <div class="bg-brand-warm-white rounded-lg p-4">
      <div class="flex flex-row justify-between items-center mb-2">
        <RouterLink :to="{ name: 'library' }" class="flex flex-row items-center gap-1 group">
          <h2 class="text-lg font-bold">Kirjasto</h2>
          <IconChevronRight class="h-5 w-5 text-brand-orange" />
        </RouterLink>
        <BrandedButton
          text="Lisää uusi"
          :onClick="openNewItemModal"
          icon="Plus"
          :isPill="true"
          variant="primary"
          :bold="true"
        />
      </div>

      <div v-if="loading" class="flex justify-center py-8">
        <LoadingSpinner />
      </div>

      <div v-else-if="recentLibraryItems.length === 0" class="text-center py-4">
        <p class="text-gray-500">Ei kirjoja tai pelejä</p>
      </div>

      <EntryListing
        v-else
        :items="recentLibraryItems"
        @itemUpdated="onItemUpdated"
      />
    </div>

    <NewItemModal
      :is-modal-open="showNewItemModal"
      @submitComplete="onNewItemSubmitComplete"
      @close="closeNewItemModal"
    />
  </div>
</template>
