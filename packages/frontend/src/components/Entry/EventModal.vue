<script lang="ts" setup>
import Modal from '@/components/Basics/PopupModal.vue'
import BrandedButton from '@/components/Basics/BrandedButton.vue'
import EventFields from '@/components/Entry/EventFields.vue'
import { useEventApi } from '@/api/eventApiClient'
import type { LibraryItem } from '@/models/LibraryItem'
import type { Challenge } from '@/models/challenge'
import type { Event } from '@/models/Event'
import { computed, ref, watch } from 'vue'

const eventApi = useEventApi()

// Creates a new event for the item, or edits `event` when given
const { isModalOpen, item, challenges, event } = defineProps<{
  isModalOpen: boolean
  item: LibraryItem
  challenges: Challenge[]
  event?: Event
}>()

const emit = defineEmits<{
  (e: "submitComplete", eventId: string): void
  (e: "close"): void
}>()

const completedAt = ref<Date>(new Date())
const activatedChallengeIds = ref<string[]>([])

function resetState() {
  if (event) {
    completedAt.value = new Date(event.completedAt)
    activatedChallengeIds.value = [...event.activatedChallengeIds]
    return
  }
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

const challengeOptions = computed(() => {
  return challenges.filter((challenge) => challenge.targetMedia === item.kind)
})

const isSubmitting = ref(false)
async function submitModal(): Promise<void> {
  isSubmitting.value = true
  const fields = {
    itemId: item.id,
    completedAt: completedAt.value.toISOString(),
    activatedChallengeIds: activatedChallengeIds.value,
  }
  let eventId: string
  if (event) {
    await eventApi.updateEvent({ id: event.id, ...fields })
    eventId = event.id
  } else {
    eventId = await eventApi.addEvent(fields)
  }
  isSubmitting.value = false
  emit("submitComplete", eventId)
}

function closeModal(): void {
  emit("close")
}
</script>

<template>
  <Modal :show-modal="isModalOpen">
    <div class="flex flex-col gap-4 p-4">
      <EventFields
        v-model:completed-at="completedAt"
        v-model:activated-challenge-ids="activatedChallengeIds"
        :challenges="challengeOptions"
      />

      <div class="flex flex-row justify-between py-2 gap-3">
        <BrandedButton :onClick="closeModal" text="Peru" icon="Cross" :is-pill="true" variant="secondary" />
        <BrandedButton :onClick="submitModal" text="Tallenna" icon="Check" :is-submitting="isSubmitting"
          :is-pill="true" :bold="true" variant="primary" />
      </div>
    </div>
  </Modal>
</template>
