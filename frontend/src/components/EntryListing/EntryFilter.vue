<script lang="ts" setup>
import { computed } from "vue"
import BrandedSelect from "@/components/basics/BrandedSelect.vue"
import type { YearFilterOption } from "@/api/preferencesApiClient"

const props = defineProps<{
  availableYears: number[]
}>()

const yearFilter = defineModel<YearFilterOption>("yearFilter", { required: true })

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
</script>

<template>
  <div class="flex flex-row gap-4 px-4 py-2 bg-brand-warm-white">
    <BrandedSelect v-model="yearFilter" :options="yearFilterOptions" title="Vuosi" />
  </div>
</template>
