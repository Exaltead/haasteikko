<script lang="ts" setup>
import { computed } from "vue"
import type { IconName } from "@/models/iconName"
import { icons } from "./icons"

const props = defineProps<{ name: IconName }>()
const icon = computed(() => icons[props.name])
</script>

<template>
  <svg
    xmlns="http://www.w3.org/2000/svg"
    :viewBox="icon.viewBox"
    :width="icon.sized ? '24px' : undefined"
    :height="icon.sized ? '24px' : undefined"
    fill="currentColor"
  >
    <title v-if="icon.title">{{ icon.title }}</title>
    <path :d="icon.path" />
  </svg>
</template>
