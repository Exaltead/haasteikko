<script lang="ts" setup>
import IconBack from "@/components/icons/IconBack.vue"
import IconDelete from "@/components/icons/IconDelete.vue"
import IconPlus from "@/components/icons/IconPlus.vue"
import type { IconName } from "@/models/iconName"
import CustomIcon from "./CustomIcon.vue"
import { computed } from "vue"
import LoadingSpinner from "./LoadingSpinner.vue"


type ButtonStyle = {
  isPill?: boolean
  backgroundColor?: "primary" | "warm-white"
  bold?: boolean
  iconColor?: string
}

const props = defineProps<{
  onClick: () => void
  text?: string
  isSubmitting?: boolean
  icon?: IconName
  disabled?: boolean
  styling?: ButtonStyle
}>()

const backgroundColor = computed(() => {
  if (props.styling?.backgroundColor) {
    if (props.styling?.backgroundColor === "warm-white") {
      return "bg-brand-warm-white"
    }
    return "bg-brand-primary"
  }
  if (props.disabled) {
    return "bg-brand-disabled"
  }
  return "bg-brand-primary"
})

function createButtonStyle(disabled: boolean, style: ButtonStyle | undefined) {
  let baseStyle = "py-1 pl-2 pr-3"


  baseStyle = baseStyle + " " + backgroundColor.value

  if (style?.isPill) {
    baseStyle = baseStyle + " rounded-full border border-brand-black"
  } else {
    baseStyle = baseStyle + " rounded border border-brand-black"
  }

  if (disabled) {
    if (!style?.backgroundColor) {
      baseStyle = baseStyle + " bg-brand-disabled"
    }
    baseStyle = baseStyle + " cursor-not-allowed text-text-disabled"
  } else {
    baseStyle = baseStyle + " cursor-pointer text-black"
  }
  if (style?.bold) {
    baseStyle = baseStyle + " font-bold"
  }

  return baseStyle
}

const iconStyle = computed(() => {
  const baseStyle = 'w-[22px] h-fit'
  if (props.styling?.iconColor) {
    return baseStyle + ' ' + props.styling.iconColor
  }
  return baseStyle + " text-brand-primary"
})



</script>

<template>
  <button @click="onClick" :class="createButtonStyle(disabled ?? false, styling)" type="button" :disabled="disabled">
    <div class="flex flex-row gap-2 items-center justify-start">
      <div v-if="isSubmitting">
        <LoadingSpinner :background-color="backgroundColor" />
      </div>
      <CustomIcon v-if="icon && !isSubmitting" :name="icon" :class="iconStyle" />
      <span v-if="text" class="text-black text-nowrap text-center">{{ text }}</span>
    </div>
  </button>
</template>
