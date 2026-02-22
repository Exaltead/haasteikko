<script lang="ts" setup>
import { computed } from "vue"
import BrandedSelect from "@/components/basics/BrandedSelect.vue"
import CustomIcon from "@/components/basics/CustomIcon.vue"
import type { YearFilterOption, EntryTypeFilter } from "@/api/preferencesApiClient"

const props = defineProps<{
  availableYears: number[]
}>()

const yearFilter = defineModel<YearFilterOption>("yearFilter", { required: true })
const typeFilter = defineModel<EntryTypeFilter[]>("typeFilter", { required: true })

const currentYear = new Date().getFullYear()

const yearFilterOptions = computed(() => {
  const options: { name: string; value: YearFilterOption }[] = [{ name: "Kaikki", value: "all" }]

  // Always include current year first
  options.push({ name: `Nykyinen(${currentYear})`, value: currentYear })

  // Add other years from library (excluding current year to avoid duplicate)
  for (const year of props.availableYears) {
    if (year !== currentYear) {
      options.push({ name: String(year), value: year })
    }
  }

  return options
})

function isTypeSelected(type: EntryTypeFilter): boolean {
  return typeFilter.value.includes(type)
}

function toggleType(type: EntryTypeFilter): void {
  if (isTypeSelected(type)) {
    typeFilter.value = typeFilter.value.filter((t) => t !== type)
  } else {
    typeFilter.value = [...typeFilter.value, type]
  }
}
</script>

<template>
  <div class="flex flex-col gap-2 px-4 md:px-15 py-2 bg-brand-warm-white">
    <BrandedSelect v-model="yearFilter" :options="yearFilterOptions" title="Vuosi" />

    <div class="flex flex-row gap-4">
      <label class="flex items-center gap-2 cursor-pointer">
        <input
          type="checkbox"
          :checked="isTypeSelected('Book')"
          @change="toggleType('Book')"
          class="w-5 h-5 accent-brand-orange"
        />
        <CustomIcon name="Book" class="w-5 h-5 text-brand-orange" />
      </label>

      <label class="flex items-center gap-2 cursor-pointer">
        <input
          type="checkbox"
          :checked="isTypeSelected('Game')"
          @change="toggleType('Game')"
          class="w-5 h-5 accent-brand-orange"
        />
        <CustomIcon name="Game" class="w-5 h-5 text-brand-orange" />
      </label>
    </div>
  </div>
</template>
