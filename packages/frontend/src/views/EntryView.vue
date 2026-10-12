<script lang="ts" setup>
import { useChallengeApi } from '@/api/challengeApiClient';
import { useLibraryApi } from '@/api/libraryApiClient';
import { useEventApi } from '@/api/eventApiClient';
import { useAnswerApi } from '@/api/answerApiClient';
import EntryBasics from '@/components/Entry/EntryBasics.vue';
import EntryEvent from '@/components/Entry/EntryEvent.vue';
import NewEventModal from '@/components/Entry/NewEventModal.vue';
import BaseIcon from '@/components/Icons/BaseIcon.vue';
import BrandedButton from '@/components/Basics/BrandedButton.vue';
import TabNavigation from '@/components/Basics/TabNavigation.vue';
import ResponsiveCardWrapper from '@/components/Basics/ResponsiveCardWrapper.vue';
import EmptyState from '@/components/Basics/EmptyState.vue';
import { useGoBack } from '@/composables/useGoBack';
import type { Answer, Challenge } from '@/models/challenge';
import type { LibraryItem } from '@/models/LibraryItem';
import type { Event } from '@/models/Event';
import { computed, ref, watch } from 'vue';
import { useRoute, useRouter } from 'vue-router';
import { z } from 'zod';

const libraryApi = useLibraryApi()
const challengeApiClient = useChallengeApi()
const eventApi = useEventApi()
const answerApi = useAnswerApi()

const route = useRoute()

const router = useRouter()
const goBack = useGoBack()

function toLibrary() {
  router.push({ name: "home" })
}

const challenges = ref<Challenge[]>([])
const item = ref<LibraryItem | undefined>(undefined)
const events = ref<Event[]>([])
const answers = ref<Answer[]>([])

// Set by NewEntryView's redirect right after creating an item + its auto-linked event, so the
// Events tab can open pre-expanded on that event without any extra clicks. One-time navigation
// state only - not meant to survive a refresh or be shareable. Also updated after manually
// creating an event via NewEventModal, so that event opens pre-expanded too, and whenever an event
// is expanded, so navigating back from a challenge answer page reopens it.
const expandEventId = ref<string | undefined>(
  (history.state as { expandEventId?: string } | null)?.expandEventId,
)

async function getChallenges() {
  challenges.value = await challengeApiClient.fetchChallenges()
}
async function getItem() {
  const itemId = z.string().parse(route.params.id)
  const foundItem = await libraryApi.getLibraryItem(itemId)
  item.value = foundItem
}
async function getEvents() {
  const itemId = z.string().parse(route.params.id)
  events.value = await eventApi.fetchEventsForItem(itemId)
}

async function getAnswers() {
  const itemId = z.string().parse(route.params.id)
  answers.value = await answerApi.searchAnswers({ itemId })
}

async function fetchData() {
  await Promise.all([getChallenges(), getItem(), getEvents(), getAnswers()])
}


watch(() => route.params.id, fetchData, { immediate: true })



const inEditMode = ref(false)
function setEditingMode(state: boolean) {
  inEditMode.value = state
  if (state === false) {
    // As disabling the edit mode, we need to fetch the item again
    getItem()
  }
}

const initialTabIndex = expandEventId.value ? 1 : 0

const showEventModal = ref(false)
async function onEventCreated(eventId: string) {
  showEventModal.value = false
  expandEventId.value = eventId
  await getEvents()
}

function onEventExpandedChanged(eventId: string, expanded: boolean) {
  if (expanded) {
    expandEventId.value = eventId
  } else if (expandEventId.value === eventId) {
    expandEventId.value = undefined
  }
  history.replaceState({ ...history.state, expandEventId: expandEventId.value }, "")
}

const sortedEvents = computed(() => {
  return [...events.value].sort(
    (a, b) => new Date(b.completedAt).getTime() - new Date(a.completedAt).getTime(),
  )
})

</script>

<template>
  <div>
    <div v-if="!item">
      <h1>Ladataan...</h1>
    </div>
    <div v-else class="flex flex-col items-center px-0 md:px-4">
      <ResponsiveCardWrapper>
        <div class="flex flex-col w-full">
          <div
            class="flex items-center justify-between w-full p-2 md:px-6 md:py-4 border-b border-brand-orange bg-white sticky top-0 z-10">
            <BrandedButton :onClick="() => goBack({ name: 'library' })" icon="Back" variant="primary" class="ml-2 md:ml-0"
              data-testid="back-button" />
            <div class="flex items-center gap-2">
              <BaseIcon :name="item.kind" class="w-6 h-6 md:w-8 md:h-8 text-brand-orange/70" />
              <h1 class="text-lg md:text-xl font-bold whitespace-normal">{{ item.title }}</h1>
            </div>
            <div class="w-8 md:w-10"></div> <!-- Spacer for balance -->
          </div>

          <TabNavigation tab1Label="Tiedot" :tab2Label="`Tapahtumat (${sortedEvents.length})`"
            :default-index="initialTabIndex">
            <template #tab1>
              <div class="w-full max-w-[84rem] mx-auto px-2 md:px-6 py-4">
                <EntryBasics @objectDeleted="toLibrary" @object-edited="getItem" :item="item"
                  @edit-mode-changed="setEditingMode" />
              </div>
            </template>

            <template #tab2>
              <div class="flex flex-col gap-3 w-full max-w-[84rem] mx-auto px-2 md:px-6 py-4">
                <EmptyState v-if="sortedEvents.length === 0" message="Ei vielä tapahtumia" />
                <EntryEvent
                  v-for="event in sortedEvents"
                  :key="event.id"
                  :event="event"
                  :item="item"
                  :challenges="challenges"
                  :answers="answers"
                  :initially-expanded="event.id === expandEventId"
                  @event-updated="getEvents"
                  @expanded-changed="(expanded: boolean) => onEventExpandedChanged(event.id, expanded)"
                />
                <BrandedButton
                  text="Lisää uusi tapahtuma"
                  :onClick="() => (showEventModal = true)"
                  icon="Plus"
                  variant="primary"
                />
              </div>
            </template>
          </TabNavigation>
        </div>
      </ResponsiveCardWrapper>
      <NewEventModal
        :is-modal-open="showEventModal"
        :item="item"
        :challenges="challenges"
        @submit-complete="onEventCreated"
        @close="showEventModal = false"
      />
    </div>
  </div>
</template>
