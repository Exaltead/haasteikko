<script setup lang="ts">
import EntryFilter from '@/components/EntryListing/EntryFilter.vue';
import EntryListing from '../components/EntryListing/EntryListing.vue'
import BrandedButton from '@/components/basics/BrandedButton.vue';
import NewItemModal from "@/components/EntryListing/NewItemModal.vue"
import { ref } from 'vue';
import { useRouter } from 'vue-router';


const router = useRouter()

const showFilter = ref(false)


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

function toggleFilter(): void {
  showFilter.value = !showFilter.value
}

</script>

<template>
  <main>
    <div>

    </div>
    <div class="flex flex-col gap-3 w-full h-full md:px-15">
      <div class="flex flex-col gap-2">
        <div class="flex flex-row justify-between md:justify-start items-center py-2 px-4 gap-10 bg-brand-warm-white">
          <!--<BrandedButton icon="Filter" :onClick="toggleFilter" :isPill="true" variant="secondary" />-->
          <BrandedButton text="Lisää uusi" :onClick="createNew" icon="Plus" :isPill="true" variant="primary"
            :bold="true" />

        </div>
        <EntryFilter v-if="showFilter" />
      </div>


      <EntryListing class="px-1 " />
      <NewItemModal :is-modal-open="showDialog" @submitComplete="onNewItemSubmitComplete" @close="closeModal" />
    </div>


  </main>
</template>
