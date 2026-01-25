<script setup lang="ts">
import CustomIcon from "@/components/basics/CustomIcon.vue"

const searchQuery = defineModel<string>({ default: "" })
</script>

<template>
  <div
    class="flex flex-row items-center flex-1 md:flex-initial md:min-w-64 border border-brand-black rounded-full bg-white px-3 py-1"
  >
    <CustomIcon name="Search" class="w-5 h-5 text-brand-primary shrink-0" />
    <input
      v-model="searchQuery"
      type="text"
      placeholder="Hae..."
      class="flex-1 ml-2 outline-none bg-transparent text-sm"
    />
  </div>
</template>
