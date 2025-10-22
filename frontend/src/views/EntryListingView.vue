<script setup lang="ts">
import EntryListing from '../components/EntryListing/EntryListing.vue'
import BrandedButton from '@/components/basics/BrandedButton.vue';
import NewItemModal from "@/components/EntryListing/NewItemModal.vue"
import { useAuth0 } from '@auth0/auth0-vue';
import { ref } from 'vue';
import { useRouter } from 'vue-router';


const router = useRouter()
const {getAccessTokenSilently} = useAuth0()

const accessToken =await getAccessTokenSilently()
console.log("Access token:", accessToken)

function createNew() {
  showDialog.value = true
}

const showDialog = ref(false)

function onNewItemSubmitComplete(id: string): void {
  showDialog.value = false

  router.push({ name: "libraryItem", params: { id } })
}

function closeModal(): void {
  showDialog.value = false
}

</script>

<template>
  <main>
    <div>

    </div>
    <div class="flex flex-col gap-3 w-full h-full md:px-15">
      <div class="flex flex-row justify-start items-center py-2 px-1.5 mt-2">
        <BrandedButton text="Lisää uusi" :onClick="createNew" icon="Plus"
          :styling="{ isPill: true, backgroundColor: 'warm-white', bold: true }" />
      </div>

      <EntryListing class="px-1 " />
      <NewItemModal :is-modal-open="showDialog" @submitComplete="onNewItemSubmitComplete" @close="closeModal" />
    </div>


  </main>
</template>
