<script lang="ts" setup>
import { useChallengeApi } from '@/api/challengeApiClient';
import { useSolutionsApi } from '@/api/solutionsApiClient';
import type { Challenge, SolutionSet } from '@/models/challenge';
import { computed, ref } from 'vue';

const challengeApiClient = useChallengeApi()
const solutionsApiClient = useSolutionsApi()

type ChallengeAndSolution = {
  challenge: Challenge
  solutionSet: SolutionSet | undefined
}

const challengeAndSolutions = ref<ChallengeAndSolution[]>([])
async function getChallengesAndSolutions() {
  const challenges = await challengeApiClient.fetchChallenges()
  const solutions = await solutionsApiClient.fetchSolutionSets()

  challengeAndSolutions.value = challenges.map(challenge => {
    const solutionSet = solutions.find(solution => solution.challengeId === challenge.id)
    return { challenge, solutionSet }
  })
}

const activeChallenges = computed(() => {
  return challengeAndSolutions.value.filter(challengeAndSolution => {
    return challengeAndSolution.challenge.status === "active"
  })
})

const pastChallenges = computed(() => {
  return challengeAndSolutions.value.filter(challengeAndSolution => {
    return challengeAndSolution.challenge.status === "inactive" && challengeAndSolution.solutionSet !== undefined
  })
})

getChallengesAndSolutions()

</script>

<template>
  <div>
    <h2>Käynnissä olevat haasteet</h2>
    <ul class="flex flex-col gap-4">
      <li v-for="challengeAndSolution in activeChallenges" :key="challengeAndSolution.challenge.id" class="ml-4">
        <RouterLink :to="{ name: 'challengeSolution', params: { id: challengeAndSolution.challenge.id } }">
          <span>{{ challengeAndSolution.challenge.name }}</span>
        </RouterLink>


      </li>
    </ul>

    <div v-if="pastChallenges.length > 0">
      <h2>Menneet haasteet</h2>
      <ul class="flex flex-col gap-4">
        <li v-for="challengeAndSolution in pastChallenges" :key="challengeAndSolution.challenge.id" class="ml-4">
          <RouterLink :to="{ name: 'challengeSolution', params: { id: challengeAndSolution.challenge.id } }">
            <span>{{ challengeAndSolution.challenge.name }}</span>
          </RouterLink>

        </li>
      </ul>
    </div>

  </div>
</template>
