<script lang="ts" setup>
import type { Challenge } from '@/models/challenge';
import BrandedButton from '../basics/BrandedButton.vue';

const { challenges } = defineProps<{ challenges: Challenge[] }>();
const emit = defineEmits<{
  (e: 'edit-challenge', challenge: Challenge): void;
  (e: 'create-challenge'): void;
}>();

function handleEdit(challenge: Challenge): void {
  emit('edit-challenge', challenge);
}
</script>

<template>
  <div class="p-10">
    <BrandedButton text="Luo uusi haaste" icon="Plus" :onClick="() => emit('create-challenge')" variant="primary"/>
    <div class="flex items-center justify-center">
      <ul class="flex flex-col gap-4">
        <li v-for="challenge in challenges" :key="challenge.id">
          <div class="flex items-center bg-white shadow-lg rounded px-4 py-2 flex flex-col gap-2 w-100">
            <div>
              <span class="font-bold text-xl">{{ challenge.name }}</span>
            </div>
            <div class="flex flex-row justify-end w-full">
              <BrandedButton text="Muokkaa" :onClick="() => { handleEdit(challenge) }" variant="primary"/>

            </div>

          </div>

        </li>
      </ul>
    </div>
  </div>
</template>
