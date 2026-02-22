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
      class="bg-brand-warm-white border-b border-brand-orange/20 h-[50px] md:h-[72px] flex flex-row items-center justify-between gap-4 md:gap-8">
        <!-- First Tab -->
        <div class="flex flex-1 justify-center items-center h-full">
          <Tab v-slot="{ selected }">
            <button :class="makeTabStyle(selected)"
              class="px-3 py-1 md:px-6 md:py-3 rounded text-sm md:text-base whitespace-nowrap">
              <span>{{ tab1Label }}</span>
            </button>
          </Tab>
        </div>

        <!-- Vertical Divider -->
        <div class="h-full w-px bg-brand-orange/50"></div>

        <!-- Second Tab -->
        <div class="flex flex-1 justify-center items-center h-full">
          <Tab v-slot="{ selected }">
            <button :class="makeTabStyle(selected)"
              class="px-3 py-1 md:px-6 md:py-3 rounded text-sm md:text-base whitespace-nowrap">
              <span>{{ tab2Label }}</span>
            </button>
          </Tab>
        </div>
    </TabList>

    <TabPanels class="bg-white">
      <TabPanel>
        <slot name="tab1"></slot>
      </TabPanel>

      <TabPanel>
        <slot name="tab2"></slot>
      </TabPanel>
    </TabPanels>
  </TabGroup>
</template>
