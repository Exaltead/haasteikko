<script lang="ts" setup>
import type { IconName } from "@/models/iconName"
import CustomIcon from "./CustomIcon.vue"
import { computed } from "vue"
import LoadingSpinner from "./LoadingSpinner.vue"

type BrandingVariant = "primary" | "secondary"

type BrandingVariantConfig = {
  backgroundColor: string,
  textColor: string
  disabledBackgroundColor: string,
  disabledTextColor: string
  iconColor: string,
  disabledIconColor: string
}

const brandingVariantConfigs: Record<BrandingVariant, BrandingVariantConfig> = {
  primary: {
    backgroundColor: "bg-brand-primary",
    textColor: "text-black",
    disabledBackgroundColor: "bg-brand-disabled",
    disabledTextColor: "text-text-disabled",
    iconColor: "text-white",
    disabledIconColor: "text-white"
  },
  secondary: {
    backgroundColor: "bg-brand-warm-white",
    textColor: "text-black",
    disabledBackgroundColor: "bg-brand-warm-white-200",
    disabledTextColor: "text-text-disabled",
    iconColor: "text-brand-primary",
    disabledIconColor: "text-text-disabled"
  }
}

const props = defineProps<{
  onClick: () => void
  text?: string
  isSubmitting?: boolean
  icon?: IconName
  disabled?: boolean
  variant: BrandingVariant
  isPill?: boolean
  bold?: boolean
}>()

const backgroundColor = computed(() => {
  const variantConfig = brandingVariantConfigs[props.variant]
  if (props.disabled) {
    return variantConfig.disabledBackgroundColor
  }
  return variantConfig.backgroundColor
})

const textColor = computed(() => {
  const variantConfig = brandingVariantConfigs[props.variant]
  if (props.disabled) {
    return variantConfig.disabledTextColor
  }
  return variantConfig.textColor
})

const iconStyle = computed(() => {
  const variantConfig = brandingVariantConfigs[props.variant]
  const size = isIconOnly.value && props.isPill ? 'w-5 h-5' : 'w-[22px] h-fit'
  return size + ' ' + (props.disabled ? variantConfig.disabledIconColor : variantConfig.iconColor)
})

const isIconOnly = computed(() => {
  return props.icon && !props.text
})

const buttonStyle = computed(() => {
  const styles = [backgroundColor.value]
  if (props.isPill) {
    styles.push("rounded-full")
    if (isIconOnly.value) {
      styles.push("p-1")
    }
  } else {
    styles.push("rounded")
  }
  if (props.disabled) {
    styles.push("cursor-not-allowed")
  } else {
    styles.push("cursor-pointer")
  }
  if (props.bold) {
    styles.push("font-bold")
  }
  return styles.join(" ")
})

</script>

<template>
  <button
    @click="onClick"
    class="border border-brand-black"
    :class="[buttonStyle, isIconOnly && isPill ? '' : 'py-1 px-2']"
    type="button"
    :disabled="disabled"
  >
    <div class="flex flex-row gap-1 items-center justify-between">
      <div v-if="isSubmitting">
        <LoadingSpinner :background-color="backgroundColor" />
      </div>
      <CustomIcon v-if="icon && !isSubmitting" :name="icon" :class="iconStyle" />
      <span v-if="text" class="text-nowrap text-center" :class="textColor">{{ text }}</span>
    </div>
  </button>
</template>
