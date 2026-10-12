<script lang="ts" setup>
import { ref, computed, useId } from "vue"
import type { IconName } from "@/models/iconName"
import BaseIcon from "@/components/Icons/BaseIcon.vue"

defineProps<{
  label: string
  icon?: IconName
}>()

const model = defineModel<Date>({ required: true })

// There is no native input to attach a <label> to, so the label names the whole picker as a group
const labelId = useId()

const viewDate = ref(new Date(model.value))

const weekDays = ["Ma", "Ti", "Ke", "To", "Pe", "La", "Su"]

const currentMonthName = computed(() => {
  return viewDate.value.toLocaleDateString("fi-FI", { month: "long", year: "numeric" })
})

const calendarDays = computed(() => {
  const year = viewDate.value.getFullYear()
  const month = viewDate.value.getMonth()

  const firstDay = new Date(year, month, 1)
  const lastDay = new Date(year, month + 1, 0)

  // Monday = 0, Sunday = 6
  let startDayOfWeek = firstDay.getDay() - 1
  if (startDayOfWeek < 0) startDayOfWeek = 6

  const days: { date: Date; isCurrentMonth: boolean }[] = []

  // Previous month days
  for (let i = startDayOfWeek - 1; i >= 0; i--) {
    const date = new Date(year, month, -i)
    days.push({ date, isCurrentMonth: false })
  }

  // Current month days
  for (let i = 1; i <= lastDay.getDate(); i++) {
    const date = new Date(year, month, i)
    days.push({ date, isCurrentMonth: true })
  }

  // Next month days to fill the grid
  const remaining = 42 - days.length
  for (let i = 1; i <= remaining; i++) {
    const date = new Date(year, month + 1, i)
    days.push({ date, isCurrentMonth: false })
  }

  return days
})

function previousMonth() {
  viewDate.value = new Date(viewDate.value.getFullYear(), viewDate.value.getMonth() - 1, 1)
}

function nextMonth() {
  viewDate.value = new Date(viewDate.value.getFullYear(), viewDate.value.getMonth() + 1, 1)
}

function selectDate(date: Date) {
  model.value = date
  viewDate.value = new Date(date)
}

function formatFullDate(date: Date): string {
  return date.toLocaleDateString("fi-FI", {
    weekday: "long",
    day: "numeric",
    month: "long",
    year: "numeric",
  })
}

function isSelected(date: Date): boolean {
  return (
    date.getDate() === model.value.getDate() &&
    date.getMonth() === model.value.getMonth() &&
    date.getFullYear() === model.value.getFullYear()
  )
}

function isToday(date: Date): boolean {
  const today = new Date()
  return (
    date.getDate() === today.getDate() &&
    date.getMonth() === today.getMonth() &&
    date.getFullYear() === today.getFullYear()
  )
}
</script>

<template>
  <div class="flex flex-col gap-1" role="group" :aria-labelledby="labelId">
    <div class="flex flex-row items-center gap-2">
      <BaseIcon v-if="icon" :name="icon" class="text-brand-primary" aria-hidden="true" />
      <span :id="labelId">{{ label }}</span>
    </div>

    <div class="rounded border border-brand-primary bg-light-gray p-2">
      <!-- Month navigation -->
      <div class="mb-1 flex items-center justify-between">
        <button
          type="button"
          @click="previousMonth"
          class="rounded p-0.5 hover:bg-brand-primary/10"
          aria-label="Edellinen kuukausi"
        >
          <BaseIcon name="ChevronLeft" class="h-4 w-4" />
        </button>
        <span class="text-sm font-medium capitalize" aria-live="polite">{{ currentMonthName }}</span>
        <button
          type="button"
          @click="nextMonth"
          class="rounded p-0.5 hover:bg-brand-primary/10"
          aria-label="Seuraava kuukausi"
        >
          <BaseIcon name="ChevronRight" class="h-4 w-4" />
        </button>
      </div>

      <!-- Weekday headers -->
      <div class="mb-1 grid grid-cols-7 text-center text-xs text-gray-500" aria-hidden="true">
        <div v-for="day in weekDays" :key="day">{{ day }}</div>
      </div>

      <!-- Calendar grid -->
      <div class="grid grid-cols-7">
        <button
          v-for="(day, index) in calendarDays"
          :key="index"
          type="button"
          @click="selectDate(day.date)"
          :aria-label="formatFullDate(day.date)"
          :aria-pressed="isSelected(day.date)"
          :aria-current="isToday(day.date) ? 'date' : undefined"
          class="rounded py-1 text-xs transition-colors"
          :class="{
            'text-gray-400': !day.isCurrentMonth,
            'bg-brand-primary text-white': isSelected(day.date),
            'ring-1 ring-brand-primary': isToday(day.date) && !isSelected(day.date),
            'hover:bg-brand-primary/10': !isSelected(day.date),
          }"
        >
          {{ day.date.getDate() }}
        </button>
      </div>
    </div>
  </div>
</template>
