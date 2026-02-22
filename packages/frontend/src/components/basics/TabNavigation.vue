<script lang="ts" setup>
import { TabGroup, TabList, Tab, TabPanels, TabPanel } from '@headlessui/vue'

defineProps<{
  tab1Label: string
  tab2Label: string
}>()

function makeTabStyle(selected: boolean) {
  return selected
    ? "bg-brand-primary text-white"
    : "bg-transparent text-text-primary hover:bg-brand-primary/10"
}
</script>

<template>
  <TabGroup as="div" class="w-full">
    <TabList as="div"
      class="p-2 bg-brand-warm-white border-b border-brand-orange/20 min-h-[50px] md:min-h-[72px]">
      <div class="flex flex-row items-center justify-between gap-4 md:gap-8">
        <!-- First Tab -->
        <div class="flex flex-1 justify-center items-center">
          <Tab v-slot="{ selected }">
            <button :class="makeTabStyle(selected)"
              class="px-3 py-1 md:px-6 md:py-3 rounded text-sm md:text-base whitespace-nowrap">
              <span>{{ tab1Label }}</span>
            </button>
          </Tab>
        </div>

        <!-- Vertical Divider -->
        <div class="h-6 md:h-8 w-px bg-brand-orange/50"></div>

        <!-- Second Tab -->
        <div class="flex flex-1 justify-center items-center">
          <Tab v-slot="{ selected }">
            <button :class="makeTabStyle(selected)"
              class="px-3 py-1 md:px-6 md:py-3 rounded text-sm md:text-base whitespace-nowrap">
              <span>{{ tab2Label }}</span>
            </button>
          </Tab>
        </div>
      </div>
    </TabList>

    <TabPanels class="bg-white mt-4">
      <TabPanel>
        <slot name="tab1"></slot>
      </TabPanel>

      <TabPanel>
        <slot name="tab2"></slot>
      </TabPanel>
    </TabPanels>
  </TabGroup>
</template>
