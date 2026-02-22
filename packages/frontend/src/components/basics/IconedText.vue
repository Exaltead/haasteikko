<script setup lang="ts">
import type { IconName } from '@/models/iconName';
import CustomIcon from './CustomIcon.vue';


defineProps<{ text: string, iconName: IconName, heading: boolean }>();

</script>

<template>

  <div class="flex flex-row items-center justify-start gap-2">
    <CustomIcon :name="iconName" class="text-brand-primary"/>
    <div v-if="heading" class="text-brand-rock text-lg font-bold wrap-anywhere">
      {{ text }}
    </div>
    <div v-else class="text-brand-rock text-sm font-normal wrap-anywhere">
      {{ text }}
    </div>
  </div>

</template>
