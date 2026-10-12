<script lang="ts" setup>
import { useChallengeApi } from '@/api/challengeApiClient'
import { useAnswerApi } from '@/api/answerApiClient'
import { useLibraryApi } from '@/api/libraryApiClient'
import { useEventApi } from '@/api/eventApiClient'
import { useSolutionsApi } from '@/api/solutionsApiClient'
import type { Challenge, Answer, Solution } from '@/models/challenge'
import type { Event } from '@/models/Event'
import { computed, ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import ChallengeCard from '@/components/Challenge/ChallengeCard.vue'
import LoadingSpinner from '@/components/Basics/LoadingSpinner.vue'
import EntryListing from '@/components/EntryListing/EntryListing.vue'
import BrandedButton from '@/components/Basics/BrandedButton.vue'
import BaseIcon from '@/components/Icons/BaseIcon.vue'
import {
  filterByStatus,
  getAttachedItemCount,
  getCompletedCount,
  getSolvedCount,
  getTotalCount,
} from '@/composables/useChallengeProgress'
import { withLastCompletedAt, type LibraryItemWithCompletion } from '@/composables/useEventSummary'

const router = useRouter()
const challengeApiClient = useChallengeApi()
const answerApiClient = useAnswerApi()
const libraryApiClient = useLibraryApi()
const eventApiClient = useEventApi()
const solutionsApiClient = useSolutionsApi()

const challenges = ref<Challenge[]>([])
const answersByChallenge = ref<Map<string, Answer[]>>(new Map())
const solutionsByChallenge = ref<Map<string, Solution[]>>(new Map())
const libraryItems = ref<LibraryItemWithCompletion[]>([])
const events = ref<Event[]>([])
const loading = ref(true)

const activeChallenges = computed(() => filterByStatus(challenges.value, "active"))

const recentLibraryItems = computed(() => {
  return [...libraryItems.value]
    .sort((a, b) => new Date(b.addedAt).getTime() - new Date(a.addedAt).getTime())
    .slice(0, 5)
})

async function loadData() {
  loading.value = true

  const [challengesResult, itemsResult, eventsResult] = await Promise.all([
    challengeApiClient.fetchChallenges(),
    libraryApiClient.fetchLibraryItems(),
    eventApiClient.fetchEvents(),
  ])

  challenges.value = challengesResult
  events.value = eventsResult
  libraryItems.value = withLastCompletedAt(itemsResult, eventsResult)

  const active = filterByStatus(challenges.value, "active")
  const answerPromises = active.map(async (challenge) => {
    const [answers, solutions] = await Promise.all([
      answerApiClient.searchAnswers({ challengeId: challenge.id }),
      solutionsApiClient.searchSolutions({ challengeId: challenge.id }),
    ])
    return { challengeId: challenge.id, answers, solutions }
  })

  const results = await Promise.all(answerPromises)
  answersByChallenge.value = new Map(results.map((r) => [r.challengeId, r.answers]))
  solutionsByChallenge.value = new Map(results.map((r) => [r.challengeId, r.solutions]))

  loading.value = false
}

onMounted(loadData)

function openNewItemPage() {
  router.push({ name: 'newLibraryItem' })
}

async function onItemUpdated() {
  libraryItems.value = withLastCompletedAt(await libraryApiClient.fetchLibraryItems(), events.value)
}
</script>

<template>
  <div class="p-4 md:p-8">
    <div class="flex flex-col lg:flex-row gap-4 lg:gap-0 bg-brand-warm-white rounded-lg">
      <!-- Challenges section -->
      <div class="p-4 lg:flex-1">
        <RouterLink :to="{ name: 'challenges' }" class="flex flex-row items-center gap-1 mb-2 w-fit">
          <h2 class="text-lg font-bold">Käynnissä olevat haasteet</h2>
          <BaseIcon name="ChevronRight" class="h-5 w-5 text-brand-orange" />
        </RouterLink>

        <div v-if="loading" class="flex justify-center py-8">
          <LoadingSpinner />
        </div>

        <div v-else-if="activeChallenges.length === 0" class="text-center py-4">
          <p class="text-gray-500">Ei aktiivisia haasteita</p>
        </div>

        <div v-else class="flex flex-wrap gap-3 justify-center lg:justify-start">
          <ChallengeCard
            v-for="challenge in activeChallenges"
            :key="challenge.id"
            :challenge="challenge"
            :completed-count="getCompletedCount(challenge, answersByChallenge.get(challenge.id) ?? [])"
            :solved-count="
              getSolvedCount(
                challenge,
                solutionsByChallenge.get(challenge.id) ?? [],
                answersByChallenge.get(challenge.id) ?? [],
              )
            "
            :total-count="getTotalCount(challenge)"
            :attached-item-count="getAttachedItemCount(challenge, libraryItems, events)"
          />
        </div>
      </div>

      <!-- Vertical divider -->
      <div class="hidden lg:block w-px bg-gray-300 my-6" />

      <!-- Horizontal divider for mobile -->
      <div class="lg:hidden h-px bg-gray-300 mx-4" />

      <!-- Library section -->
      <div class="p-4 lg:flex-1">
        <div class="flex flex-row justify-between items-center mb-2">
          <RouterLink :to="{ name: 'library' }" class="flex flex-row items-center gap-1 group">
            <h2 class="text-lg font-bold">Kirjasto</h2>
            <BaseIcon name="ChevronRight" class="h-5 w-5 text-brand-orange" />
          </RouterLink>
          <BrandedButton
            text="Lisää uusi"
            :onClick="openNewItemPage"
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
          :singleColumn="true"
          @itemUpdated="onItemUpdated"
        />
      </div>
    </div>
  </div>
</template>
