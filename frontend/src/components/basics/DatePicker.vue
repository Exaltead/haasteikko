<script lang="ts" setup>
import { ref, computed } from "vue"
import type { IconName } from "@/models/iconName"
import CustomIcon from "./CustomIcon.vue"
import IconChevronLeft from "@/components/icons/IconChevronLeft.vue"
import IconChevronRight from "@/components/icons/IconChevronRight.vue"

defineProps<{
  label: string
  icon?: IconName
}>()

const model = defineModel<Date>({ required: true })

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
  <div class="flex flex-col gap-1">
    <div class="flex flex-row items-center gap-2">
      <CustomIcon v-if="icon" :name="icon" class="text-brand-primary" />
      <label>{{ label }}</label>
    </div>

    <div class="rounded border border-brand-primary bg-light-gray p-2">
      <!-- Month navigation -->
      <div class="mb-1 flex items-center justify-between">
        <button
          type="button"
          @click="previousMonth"
          class="rounded p-0.5 hover:bg-brand-primary/10"
        >
          <IconChevronLeft class="h-4 w-4" />
        </button>
        <span class="text-sm font-medium capitalize">{{ currentMonthName }}</span>
        <button
          type="button"
          @click="nextMonth"
          class="rounded p-0.5 hover:bg-brand-primary/10"
        >
          <IconChevronRight class="h-4 w-4" />
        </button>
      </div>

      <!-- Weekday headers -->
      <div class="mb-1 grid grid-cols-7 text-center text-xs text-gray-500">
        <div v-for="day in weekDays" :key="day">{{ day }}</div>
      </div>

      <!-- Calendar grid -->
      <div class="grid grid-cols-7">
        <button
          v-for="(day, index) in calendarDays"
          :key="index"
          type="button"
          @click="selectDate(day.date)"
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
