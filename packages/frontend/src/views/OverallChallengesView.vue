<script lang="ts" setup>
import { useChallengeApi } from '@/api/challengeApiClient';
import type { Challenge } from '@/models/challenge';
import { filterByStatus } from '@/composables/useChallengeProgress';
import { computed, ref } from 'vue';

const challengeApiClient = useChallengeApi()


const challenges = ref<Challenge[]>([])

async function getChallenges() {
  challenges.value = await challengeApiClient.fetchChallenges()
}

const activeChallenges = computed(() => filterByStatus(challenges.value, "active"))

const pastChallenges = computed(() => filterByStatus(challenges.value, "inactive"))

getChallenges()

</script>

<template>
  <div>
    <h2>Käynnissä olevat haasteet</h2>
    <ul class="flex flex-col gap-4">
      <li v-for="challenge in activeChallenges" :key="challenge.id" class="ml-4">
        <RouterLink :to="{ name: 'challengeSolution', params: { id: challenge.id } }">
          <span>{{ challenge.name }}</span>
        </RouterLink>


      </li>
    </ul>

    <div v-if="pastChallenges.length > 0">
      <h2>Menneet haasteet</h2>
      <ul class="flex flex-col gap-4">
        <li v-for="challenge in pastChallenges" :key="challenge.id" class="ml-4">
          <RouterLink :to="{ name: 'challengeSolution', params: { id: challenge.id } }">
            <span>{{ challenge.name }}</span>
          </RouterLink>

        </li>
      </ul>
    </div>

  </div>
</template>
