<script setup lang="ts">
import BrandedButton from '@/components/Basics/BrandedButton.vue';
import ResponsiveCardWrapper from '@/components/Basics/ResponsiveCardWrapper.vue';
import { computed, onMounted, ref, watch } from 'vue'
import TextInput from "@/components/Basics/TextInput.vue"
import type { Book, Game, LibraryItem } from '@/models/LibraryItem';
import type { Challenge } from '@/models/challenge';
import { useLibraryApi } from '@/api/libraryApiClient';
import { useEventApi } from '@/api/eventApiClient';
import { useChallengeApi } from '@/api/challengeApiClient';
import BrandedSelect from '@/components/Basics/BrandedSelect.vue';
import DatePicker from '@/components/Basics/DatePicker.vue';
import ManagementList from '@/components/Basics/ManagementList.vue';
import { useRouter } from 'vue-router';

const router = useRouter()
const libraryApi = useLibraryApi()
const eventApi = useEventApi()
const challengeApi = useChallengeApi()

type NewBook = {
  name: string,
  author: string,
  translator: string
}

type NewGame = {
  title: string,
  creator: string
}

const newBookBase: NewBook = {
  name: "",
  author: "",
  translator: ""
}

const newGameBase: NewGame = {
  title: "",
  creator: ""
}

type ValidKinds = "Book" | "Game"

const bookModel = ref<NewBook>({ ...newBookBase })
const gameModel = ref<NewGame>({ ...newGameBase })
const kind = ref<ValidKinds>("Book")
const completedAt = ref<Date>(new Date())

const challenges = ref<Challenge[]>([])
const activatedChallengeIds = ref<string[]>([])

function defaultActivatedChallengeIds(): string[] {
  return challenges.value
    .filter((challenge) => challenge.status === "active" && challenge.targetMedia === kind.value)
    .map((challenge) => challenge.id)
}

onMounted(async () => {
  challenges.value = await challengeApi.fetchChallenges()
  activatedChallengeIds.value = defaultActivatedChallengeIds()
})

watch(kind, () => {
  activatedChallengeIds.value = defaultActivatedChallengeIds()
})

const selectedChallengeItems = computed(() => {
  return activatedChallengeIds.value
    .map((id) => challenges.value.find((challenge) => challenge.id === id))
    .filter((challenge): challenge is Challenge => challenge !== undefined)
    .map((challenge) => ({ value: challenge.id, display: challenge.name }))
})

const allChallengeOptions = computed(() => {
  return challenges.value
    .filter((challenge) => challenge.targetMedia === kind.value)
    .map((challenge) => ({ value: challenge.id, display: challenge.name }))
})

function onChallengeStateChanged(challengeIds: string[]) {
  activatedChallengeIds.value = challengeIds
}

const isSubmitting = ref(false)
const errorMessage = ref<string | null>(null)

const isInvalidValid = computed(() => {
  if (kind.value === "Book") {
    const hasName = bookModel.value.name.length > 0
    const hasAuthor = bookModel.value.author.length > 0

    return !(hasName && hasAuthor) || isSubmitting.value

  } else if (kind.value === "Game") {
    const hasTitle = gameModel.value.title.length > 0
    const hasAuthor = gameModel.value.creator.length > 0
    return !(hasTitle && hasAuthor) || isSubmitting.value
  }

  return true

})

function cancel(): void {
  router.back()
}

async function submitForm(): Promise<void> {
  function toItem(): Omit<LibraryItem, "id" | "addedAt"> {
    if (kind.value === 'Book') {
      const newBook: Omit<Book, "id" | "addedAt"> = {
        kind: "Book",
        title: bookModel.value.name,
        author: bookModel.value.author,
        translator: bookModel.value.translator,
        favorite: false,
      }
      return newBook
    }
    if (kind.value === 'Game') {
      const newGame: Omit<Game, "id" | "addedAt"> = {
        kind: 'Game',
        title: gameModel.value.title,
        creator: gameModel.value.creator,
        favorite: false,
      }

      return newGame
    }

    throw new Error("Invalid state, kind unknown")
  }


  const item = toItem()


  isSubmitting.value = true
  errorMessage.value = null
  let newId: string | undefined
  try {
    newId = await libraryApi.addLibraryItem(item)
    if (newId === undefined) {
      errorMessage.value = "Tallentaminen epäonnistui. Yritä uudelleen."
      return
    }

    await eventApi.addEvent({
      itemId: newId,
      completedAt: completedAt.value.toISOString(),
      activatedChallengeIds: activatedChallengeIds.value,
    })
  } catch (error) {
    console.error("Failed to save item:", error)
    errorMessage.value = "Tallentaminen epäonnistui. Yritä uudelleen."
    return
  } finally {
    isSubmitting.value = false
  }

  // Replace the form's history entry, so going back from the new item doesn't reopen the form
  router.replace({ name: "libraryItem", params: { id: newId }, state: { entryTab: 1 } })
}

const kinds = [
  { name: "Kirja", value: "Book" },
  { name: "Peli", value: "Game" },
]

</script>

<template>
  <div class="flex flex-col items-center px-0 md:px-4">
    <ResponsiveCardWrapper>
      <div class="flex flex-col w-full">
        <div
          class="flex items-center justify-between w-full p-2 md:px-6 md:py-4 border-b border-brand-orange bg-white sticky top-0 z-10">
          <BrandedButton :onClick="cancel" icon="Back" variant="primary" class="ml-2 md:ml-0"
            data-testid="back-button" />
          <h1 class="text-lg md:text-xl font-bold whitespace-normal">Lisää uusi</h1>
          <div class="w-8 md:w-10"></div> <!-- Spacer for balance -->
        </div>

        <form class="w-full max-w-[84rem] mx-auto px-2 md:px-6 py-4">
          <div class="flex flex-col gap-4">
            <BrandedSelect v-model="kind" :options="kinds" title="Tyyppi" />
            <div v-if="kind === 'Book'">
              <TextInput name="name" label="Nimi" v-model="bookModel.name" icon="Book" />
              <TextInput name="author" label="Kirjailija" :required="true" v-model="bookModel.author"
                icon="Author" />
              <TextInput name="translator" label="Kääntäjä" :required="false" v-model="bookModel.translator"
                icon="Translator" />
            </div>
            <div v-if="kind === 'Game'">
              <TextInput name="name" label="Nimi" v-model="gameModel.title" icon="Game" />
              <TextInput name="author" label="Tekijä" :required="true" v-model="gameModel.creator" icon="Author" />
            </div>
            <DatePicker v-model="completedAt" label="Päiväys" icon="Calendar" />
            <ManagementList
              :key="kind"
              title="Haasteet"
              :items="selectedChallengeItems"
              :allOptions="allChallengeOptions"
              @stateChanged="onChallengeStateChanged"
            />
            <div v-if="errorMessage" class="bg-red-100 border border-red-400 text-red-700 px-4 py-2 rounded">
              {{ errorMessage }}
            </div>
            <div class="flex flex-row justify-between py-2 gap-3">
              <BrandedButton :onClick="cancel" text="Peru" icon="Cross" :is-pill="true" variant="secondary" />
              <BrandedButton :onClick="submitForm" :disabled="isInvalidValid" text="Tallenna" icon="Check"
                :is-submitting="isSubmitting" :is-pill="true" :bold="true" variant="primary" />
            </div>
          </div>
        </form>
      </div>
    </ResponsiveCardWrapper>
  </div>
</template>
