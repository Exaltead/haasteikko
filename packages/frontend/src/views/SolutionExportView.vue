<script lang="ts" setup>
import { useChallengeApi } from "@/api/challengeApiClient"
import { useLibraryApi } from "@/api/libraryApiClient"
import { useSolutionsApi } from "@/api/solutionsApiClient"
import BrandedButton from "@/components/Basics/BrandedButton.vue"
import ResponsiveCardWrapper from "@/components/Basics/ResponsiveCardWrapper.vue"
import BaseIcon from "@/components/Icons/BaseIcon.vue"
import type { Question, Solution } from "@/models/challenge"
import type { LibraryItem } from "@/models/LibraryItem"
import { formatSolutionAsText } from "@/utils/solutionText"
import { onBeforeUnmount, ref, useTemplateRef, watch } from "vue"
import { useRoute } from "vue-router"

const route = useRoute()
const challengeApi = useChallengeApi()
const libraryApi = useLibraryApi()
const solutionsApi = useSolutionsApi()

const loading = ref(false)
const challengeName = ref("")
const challengeMediaType = ref<"Book" | "Game">("Book")
const questions = ref<Question[]>([])
const solutions = ref<Solution[]>([])
const items = ref<LibraryItem[]>([])

// Edited text lives only here; it is never persisted
const text = ref("")
const textArea = useTemplateRef<HTMLTextAreaElement>("textArea")

function generateText() {
  text.value = formatSolutionAsText(challengeName.value, questions.value, solutions.value, items.value)
}

async function loadData() {
  loading.value = true
  const challengeId = route.params.id as string
  const [challenges, serverSolutions, libraryItems] = await Promise.all([
    challengeApi.fetchChallenges(),
    solutionsApi.searchSolutions({ challengeId }),
    libraryApi.fetchLibraryItems(),
  ])
  const challenge = challenges.find((t) => t.id === challengeId)
  if (challenge === undefined) {
    throw new Error("Challenge not found")
  }
  challengeName.value = challenge.name
  challengeMediaType.value = challenge.targetMedia
  questions.value = challenge.questions
  solutions.value = serverSolutions
  items.value = libraryItems
  generateText()
  loading.value = false
}

watch(() => route.params.id, loadData, { immediate: true })

const copied = ref(false)
let copiedTimeout: ReturnType<typeof setTimeout> | undefined

async function copyToClipboard() {
  try {
    await navigator.clipboard.writeText(text.value)
    copied.value = true
    clearTimeout(copiedTimeout)
    copiedTimeout = setTimeout(() => (copied.value = false), 2000)
  } catch {
    // Clipboard access can be denied; leave the text selected so it can be copied by hand
    textArea.value?.focus()
    textArea.value?.select()
  }
}

onBeforeUnmount(() => clearTimeout(copiedTimeout))
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
              <BaseIcon :name="challengeMediaType" class="w-6 h-6 md:w-8 md:h-8 text-brand-orange/70" />
              <h1 class="text-lg md:text-xl font-bold whitespace-normal">{{ challengeName }}</h1>
            </div>
            <div class="w-8 md:w-10"></div> <!-- Spacer for balance -->
          </div>

          <div class="flex flex-col gap-2 w-full max-w-[84rem] mx-auto p-2 md:p-4">
            <p class="text-xs text-gray-500">
              Muokkaa tekstiä vapaasti ja kopioi se leikepöydälle. Muutoksia ei tallenneta.
            </p>
            <textarea ref="textArea" v-model="text" aria-label="Ratkaisu tekstinä" rows="24"
              class="w-full p-2 border border-brand-black rounded font-mono text-sm bg-white" />
            <div class="flex justify-end gap-2">
              <BrandedButton variant="secondary" text="Palauta" :onClick="generateText" />
              <BrandedButton variant="primary" :text="copied ? 'Kopioitu!' : 'Kopioi leikepöydälle'"
                :onClick="copyToClipboard" />
            </div>
          </div>
        </div>
      </ResponsiveCardWrapper>
    </div>
  </div>
</template>
