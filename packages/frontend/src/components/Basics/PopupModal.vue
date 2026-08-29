<!-- eslint-disable vue/multi-word-component-names -->
<script lang="ts" setup>
defineProps<{ showModal: boolean }>()

</script>

<template>

  <div v-if="showModal" class="fixed inset-0 w-full h-screen bg-[rgba(0,0,0,0.5)] flex justify-center items-center">
    <div class="w-[90vw] max-w-md rounded bg-white">
      <slot></slot>
    </div>

  </div>

</template>
