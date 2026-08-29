<script lang="ts" setup>
const model = defineModel()
const { options, disabled } = defineProps<{
  options: { name: string; value: unknown }[]
  title?: string
  disabled?: boolean
}>()


</script>

<template>
  <div class="flex flex-row gap-4 items-center">
    <label v-if="title" for="select" class="font-bold">{{ title }}</label>
    <select id="select" v-model="model" :disabled="disabled" class="rounded border-brand-primary border-1 px-2 py-1 bg-light-gray disabled:opacity-50 disabled:cursor-not-allowed">
      <option v-for="option in options" :key="option.name" :value="option.value">
        {{ option.name }}
      </option>
    </select>
  </div>
</template>
