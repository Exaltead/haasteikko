<script setup lang="ts">
import EntryFilter from "@/components/EntryListing/EntryFilter.vue"
import EntryListing from "../components/EntryListing/EntryListing.vue"
import BrandedButton from "@/components/basics/BrandedButton.vue"
import NewItemModal from "@/components/EntryListing/NewItemModal.vue"
import { ref, watch, onMounted, computed } from "vue"
import { useRouter } from "vue-router"
import {
  usePreferencesApi,
  type YearFilterOption,
  type EntryTypeFilter,
} from "@/api/preferencesApiClient"
import { useLibraryApi } from "@/api/libraryApiClient"
import type { LibraryItem } from "@/models/LibraryItem"

const router = useRouter()
const preferencesApi = usePreferencesApi()
const libraryApi = useLibraryApi()

const showFilter = ref(false)
const yearFilter = ref<YearFilterOption>("all")
const typeFilter = ref<EntryTypeFilter[]>(["Book", "Game"])
const items = ref<LibraryItem[]>([])
const savedYearPreference = ref<string | null | undefined>(undefined)
const savedTypePreference = ref<string[] | null | undefined>(undefined)

const currentYear = new Date().getFullYear()

const availableYears = computed(() => {
  const years = new Set<number>()
  for (const item of items.value) {
    years.add(new Date(item.addedAt).getFullYear())
  }
  return Array.from(years).sort((a, b) => b - a)
})

async function fetchItems() {
  items.value = await libraryApi.fetchLibraryItems()
}

onMounted(async () => {
  await fetchItems()

  try {
    const preferences = await preferencesApi.getPreferences()
    savedYearPreference.value = preferences.libraryYearFilter
    savedTypePreference.value = preferences.libraryTypeFilter
    applyPreferences()
  } catch {
    // Use default if preferences can't be loaded
  }
})

function applyPreferences() {
  // Apply year filter preference
  if (savedYearPreference.value !== undefined) {
    if (savedYearPreference.value === "all" || savedYearPreference.value === null) {
      yearFilter.value = "all"
    } else {
      const year = parseInt(savedYearPreference.value, 10)
      if (!isNaN(year) && (year === currentYear || availableYears.value.includes(year))) {
        yearFilter.value = year
      } else {
        yearFilter.value = "all"
      }
    }
  }

  // Apply type filter preference
  if (savedTypePreference.value !== undefined && savedTypePreference.value !== null) {
    const validTypes = savedTypePreference.value.filter(
      (t): t is EntryTypeFilter => t === "Book" || t === "Game",
    )
    if (validTypes.length > 0) {
      typeFilter.value = validTypes
    }
  }
}

watch(yearFilter, async (newValue) => {
  try {
    await preferencesApi.updatePreferences({ libraryYearFilter: String(newValue) })
  } catch {
    // Silently fail on preference save
  }
})

watch(typeFilter, async (newValue) => {
  try {
    await preferencesApi.updatePreferences({ libraryTypeFilter: newValue })
  } catch {
    // Silently fail on preference save
  }
})

function createNew() {
  showDialog.value = true
}

const showDialog = ref(false)

function onNewItemSubmitComplete(id: string): void {
  showDialog.value = false

  router.push({ name: "libraryItem", params: { id } })
}

function closeModal(): void {
  showDialog.value = false
}

function toggleFilter(): void {
  showFilter.value = !showFilter.value
}
</script>

<template>
  <main>
    <div></div>
    <div class="flex flex-col gap-3 w-full h-full md:px-15">
      <div class="flex flex-col">
        <div
          class="flex flex-row justify-between md:justify-start items-center py-2 px-4 gap-10 bg-brand-warm-white"
        >
          <BrandedButton icon="Filter" :onClick="toggleFilter" :isPill="true" variant="secondary" />
          <BrandedButton
            text="Lisää uusi"
            :onClick="createNew"
            icon="Plus"
            :isPill="true"
            variant="primary"
            :bold="true"
          />
        </div>
        <EntryFilter
          v-if="showFilter"
          v-model:yearFilter="yearFilter"
          v-model:typeFilter="typeFilter"
          :availableYears="availableYears"
        />
      </div>

      <EntryListing
        class="px-1"
        :items="items"
        :yearFilter="yearFilter"
        :typeFilter="typeFilter"
        @itemUpdated="fetchItems"
      />
      <NewItemModal
        :is-modal-open="showDialog"
        @submitComplete="onNewItemSubmitComplete"
        @close="closeModal"
      />
    </div>
  </main>
</template>
