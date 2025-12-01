<script setup lang="ts">
import Modal from '@/components/basics/PopupModal.vue';
import BrandedButton from '@/components/basics/BrandedButton.vue';
import { computed, ref } from 'vue'
import TextInput from "@/components/basics/TextInput.vue"
import type { Book, Game, LibraryItem } from '@/models/LibraryItem';
import { useLibraryApi } from '@/api/libraryApiClient';
import BrandedSelect from '@/components/basics/BrandedSelect.vue';


const libraryApi = useLibraryApi()
const { isModalOpen } = defineProps<{
  isModalOpen: boolean
}>()

const emit = defineEmits<{
  (e: "submitComplete", id: string): void
  (e: "close"): void
}>()

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

const isSubmitting = ref(false)

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

function resetState() {
  bookModel.value = { ...newBookBase }
  gameModel.value = { ...newGameBase }
}

function closeModal(): void {
  resetState()
  emit("close")
}

async function submitModal(): Promise<void> {
  function toItem(): Omit<LibraryItem, "id"|"addedAt"> {
    if (kind.value === 'Book') {
      const newBook: Omit<Book, "id"| "addedAt"> = {
        kind: "Book",
        title: bookModel.value.name,
        author: bookModel.value.author,
        translator: bookModel.value.translator,
        activatedChallengeIds: [],
        favorite: false,
        completedAt: Date.now().toLocaleString()
      }
      return newBook
    }
    if (kind.value === 'Game') {
      const newGame: Omit<Game, "id"| "addedAt"> = {
        kind: 'Game',
        title: gameModel.value.title,
        creator: gameModel.value.creator,
        activatedChallengeIds: [],
        favorite: false,
        completedAt: Date.now().toLocaleString()
      }

      return newGame
    }

    throw new Error("Invalid state, kind unknown")
  }


  const item = toItem()


  isSubmitting.value = true
  const newId = await libraryApi.addLibraryItem(item)
  if (newId === undefined) {

    //TODO: show error to user
    isSubmitting.value = false
    return
  }
  isSubmitting.value = false
  resetState()
  emit("submitComplete", newId)
}

const kinds = [
  { name: "Kirja", value: "Book" },
  { name: "Peli", value: "Game" },
]

</script>

<template>
  <Modal :show-modal="isModalOpen">
    <form>
      <div class="flex flex-col gap-4 p-4">
        <BrandedSelect v-model="kind" :options="kinds" title="Tyyppi" />
        <div v-if="kind === 'Book'">
          <TextInput name="name" label="Nimi" v-model="bookModel.name" icon="Book" />
          <TextInput name="author" label="Kirjailija" :required="true" v-model="bookModel.author" icon="Author" />
          <TextInput name="translator" label="Kääntäjä" :required="false" v-model="bookModel.translator"
            icon="Translator" />
        </div>
        <div v-if="kind === 'Game'">
          <TextInput name="name" label="Nimi" v-model="gameModel.title" icon="Game" />
          <TextInput name="author" label="Tekijä" :required="true" v-model="gameModel.creator" icon="Author" />
        </div>
        <div class="flex flex-row justify-between py-2 gap-3">
          <BrandedButton :onClick="closeModal" text="Peru" icon="Cross" :styling="{
            isPill: true,
            bold: true,
            iconColor: 'text-white'
          }" />
          <BrandedButton :onClick="submitModal" :disabled="isInvalidValid" text="Tallenna" icon="Check"
            :is-submitting="isSubmitting" :styling="{
              isPill: true,
              bold: true,
              iconColor: 'text-white'
            }" />
        </div>
      </div>
    </form>
  </Modal>
</template>
