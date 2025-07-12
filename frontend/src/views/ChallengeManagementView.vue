<script lang="ts" setup>
import { challengeApiClient } from '@/api/challengeApiClient';
import EditChallenge from '@/components/ChallengeManagement/EditChallenge.vue';
import ManageChallenges from '@/components/ChallengeManagement/ManageChallenges.vue';
import type { Challenge } from '@/models/challenge';
import { ref } from 'vue';


const data = ref<Challenge[]>([]);

async function getChallenges(): Promise<void> {
  data.value = await challengeApiClient.fetchChallenges()
  console.log("Fetched challenges", data.value)
}

getChallenges()

const isEditing = ref(false);
const selectedItem = ref<Challenge | undefined>(undefined);

function handleEdit(challenge: Challenge): void {
  selectedItem.value = challenge;
  isEditing.value = true;
}
function handleCreate(): void {
  selectedItem.value = undefined;
  isEditing.value = true;
}
function handleClose(): void {
  isEditing.value = false;
  selectedItem.value = undefined;
}
function handleSubmitComplete(): void {
  isEditing.value = false;
  selectedItem.value = undefined;
  getChallenges()
}
</script>

<template>
  <div class="">
    <EditChallenge v-if="isEditing" :target="selectedItem" @submitComplete="() => { handleSubmitComplete() }"
      @close="handleClose" />
    <ManageChallenges v-else :challenges="data" @edit-challenge="(challenge) => handleEdit(challenge)"
      @create-challenge="handleCreate" />

  </div>
</template>
