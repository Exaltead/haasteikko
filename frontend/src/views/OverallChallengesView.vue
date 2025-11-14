<script lang="ts" setup>
import { useChallengeApi } from '@/api/challengeApiClient';
import type { Challenge } from '@/models/challenge';
import { computed, ref } from 'vue';

const challengeApiClient = useChallengeApi()


const challenges = ref<Challenge[]>([])

async function getChallenges() {
  challenges.value = await challengeApiClient.fetchChallenges()
}

const activeChallenges = computed(() => {
  return challenges.value.filter(challenge => challenge.status === "active")
})

const pastChallenges = computed(() => {
  return challenges.value.filter(challenge => challenge.status === "inactive")
})

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
