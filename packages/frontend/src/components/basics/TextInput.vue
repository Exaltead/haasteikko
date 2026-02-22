<script lang="ts" setup>
import type { IconName } from '@/models/iconName';
import CustomIcon from './CustomIcon.vue';

defineProps<{
  name: string,
  label: string,
  icon?: IconName
}>()

const model = defineModel()
</script>

<template>
  <div class="flex flex-col gap-2">
    <div class="flex flex-row items-center gap-2">
      <CustomIcon v-if="icon" :name="icon" class="text-brand-primary" />
      <label :for="name"> {{ label }}</label>
    </div>
    <input :id="name" type="text" v-model="model" class="rounded border border-brand-primary bg-light-gray" />
  </div>
</template>
