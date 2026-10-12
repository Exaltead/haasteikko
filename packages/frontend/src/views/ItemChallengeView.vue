<script lang="ts" setup>
import { useAnswerApi } from '@/api/answerApiClient';
import { useChallengeApi } from '@/api/challengeApiClient';
import { useEventApi } from '@/api/eventApiClient';
import { useLibraryApi } from '@/api/libraryApiClient';
import BrandedButton from '@/components/Basics/BrandedButton.vue';
import ProgressBar from '@/components/Basics/ProgressBar.vue';
import ResponsiveCardWrapper from '@/components/Basics/ResponsiveCardWrapper.vue';
import EntryChallenge from '@/components/Entry/EntryChallenge.vue';
import BaseIcon from '@/components/Icons/BaseIcon.vue';
import { getAnsweredCount, getTotalCount } from '@/composables/useChallengeProgress';
import { useGoBack } from '@/composables/useGoBack';
import type { Answer, Challenge } from '@/models/challenge';
import type { Event } from '@/models/Event';
import type { LibraryItem } from '@/models/LibraryItem';
import { computed, ref, watch } from 'vue';
import { useRoute } from 'vue-router';
import { z } from 'zod';

const libraryApi = useLibraryApi()
const challengeApi = useChallengeApi()
const eventApi = useEventApi()
const answerApi = useAnswerApi()

const route = useRoute()
const goBack = useGoBack()

const item = ref<LibraryItem | undefined>(undefined)
const challenge = ref<Challenge | undefined>(undefined)
const events = ref<Event[]>([])
const answers = ref<Answer[]>([])

const itemId = computed(() => z.string().parse(route.params.id))
const challengeId = computed(() => z.string().parse(route.params.challengeId))

async function getAnswers() {
  answers.value = await answerApi.searchAnswers({
    itemId: itemId.value,
    challengeId: challengeId.value,
  })
}

async function fetchData() {
  const [foundItem, challenges, foundEvents] = await Promise.all([
    libraryApi.getLibraryItem(itemId.value),
    challengeApi.fetchChallenges(),
    eventApi.fetchEventsForItem(itemId.value),
    getAnswers(),
  ])
  item.value = foundItem
  challenge.value = challenges.find((c) => c.id === challengeId.value)
  events.value = foundEvents
}

watch(() => [route.params.id, route.params.challengeId], fetchData, { immediate: true })

// Answers are shared by every event of the item, but only count toward the challenge while at
// least one event links it.
const isLinked = computed(() =>
  events.value.some((e) => e.activatedChallengeIds.includes(challengeId.value)),
)

const answeredCount = computed(() =>
  challenge.value ? getAnsweredCount(challenge.value, answers.value) : 0,
)
const totalCount = computed(() => (challenge.value ? getTotalCount(challenge.value) : 0))
</script>

<template>
  <div>
    <div v-if="!item || !challenge">
      <h1>Ladataan...</h1>
    </div>
    <div v-else class="flex flex-col items-center px-0 md:px-4">
      <ResponsiveCardWrapper>
        <div class="flex flex-col w-full">
          <div
            class="flex items-center justify-between w-full p-2 md:px-6 md:py-4 border-b border-brand-orange bg-white sticky top-0 z-10">
            <BrandedButton :onClick="() => goBack({ name: 'libraryItem', params: { id: itemId } })" icon="Back" variant="primary" class="ml-2 md:ml-0"
              data-testid="back-button" />
            <div class="flex flex-col items-center">
              <div class="flex items-center gap-2">
                <BaseIcon :name="item.kind" class="w-6 h-6 md:w-8 md:h-8 text-brand-orange/70" />
                <h1 class="text-lg md:text-xl font-bold whitespace-normal">{{ item.title }}</h1>
              </div>
              <span class="text-sm text-gray-600">{{ challenge.name }}</span>
            </div>
            <div class="w-8 md:w-10"></div> <!-- Spacer for balance -->
          </div>

          <div class="flex flex-col gap-4 w-full max-w-[84rem] mx-auto px-2 md:px-6 py-4">
            <div class="flex flex-row items-center gap-3">
              <div class="flex-1">
                <ProgressBar :progress="totalCount > 0 ? (answeredCount / totalCount) * 100 : 0" />
              </div>
              <span class="text-sm whitespace-nowrap">{{ answeredCount }} / {{ totalCount }} vastattu</span>
            </div>

            <div v-if="!isLinked"
              class="flex flex-row items-center gap-2 rounded border border-brand-orange bg-brand-orange/10 p-2 text-sm">
              <BaseIcon name="Warning" class="h-5 w-5 shrink-0 text-brand-orange" />
              <span>Vastauksia ei lasketa haasteeseen – liitä haaste johonkin tapahtumaan.</span>
            </div>

            <EntryChallenge :item-id="item.id" :challenge="challenge" @submit="getAnswers" />
          </div>
        </div>
      </ResponsiveCardWrapper>
    </div>
  </div>
</template>
