<script lang="ts" setup>
import Modal from '@/components/Basics/PopupModal.vue'
import BrandedButton from '@/components/Basics/BrandedButton.vue'
import DatePicker from '@/components/Basics/DatePicker.vue'
import ManagementList from '@/components/Basics/ManagementList.vue'
import { useEventApi } from '@/api/eventApiClient'
import type { LibraryItem } from '@/models/LibraryItem'
import type { Challenge } from '@/models/challenge'
import { computed, ref, watch } from 'vue'

const eventApi = useEventApi()

const { isModalOpen, item, challenges } = defineProps<{
  isModalOpen: boolean
  item: LibraryItem
  challenges: Challenge[]
}>()

const emit = defineEmits<{
  (e: "submitComplete", eventId: string): void
  (e: "close"): void
}>()

const completedAt = ref<Date>(new Date())
const activatedChallengeIds = ref<string[]>([])

function resetState() {
  completedAt.value = new Date()
  activatedChallengeIds.value = challenges
    .filter((challenge) => challenge.status === "active" && challenge.targetMedia === item.kind)
    .map((challenge) => challenge.id)
}

watch(
  () => isModalOpen,
  (open) => {
    if (open) {
      resetState()
    }
  },
)

const selectedChallengeItems = computed(() => {
  return activatedChallengeIds.value
    .map((id) => challenges.find((challenge) => challenge.id === id))
    .filter((challenge): challenge is Challenge => challenge !== undefined)
    .map((challenge) => ({ value: challenge.id, display: challenge.name }))
})

const allChallengeOptions = computed(() => {
  return challenges
    .filter((challenge) => challenge.targetMedia === item.kind)
    .map((challenge) => ({ value: challenge.id, display: challenge.name }))
})

function onChallengeStateChanged(challengeIds: string[]) {
  activatedChallengeIds.value = challengeIds
}

const isSubmitting = ref(false)
async function submitModal(): Promise<void> {
  isSubmitting.value = true
  const newEventId = await eventApi.addEvent({
    itemId: item.id,
    completedAt: completedAt.value.toISOString(),
    activatedChallengeIds: activatedChallengeIds.value,
  })
  isSubmitting.value = false
  emit("submitComplete", newEventId)
}

function closeModal(): void {
  emit("close")
}
</script>

<template>
  <Modal :show-modal="isModalOpen">
    <div class="flex flex-col gap-4 p-4">
      <DatePicker v-model="completedAt" label="Päiväys" icon="Calendar" />

      <ManagementList
        title="Haasteet"
        :items="selectedChallengeItems"
        :allOptions="allChallengeOptions"
        @stateChanged="onChallengeStateChanged"
      />

      <div class="flex flex-row justify-between py-2 gap-3">
        <BrandedButton :onClick="closeModal" text="Peru" icon="Cross" :is-pill="true" variant="secondary" />
        <BrandedButton :onClick="submitModal" text="Tallenna" icon="Check" :is-submitting="isSubmitting"
          :is-pill="true" :bold="true" variant="primary" />
      </div>
    </div>
  </Modal>
</template>
