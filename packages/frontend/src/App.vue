<script setup lang="ts">
import { RouterView } from 'vue-router'

import NavBar from '@/components/NavBar.vue';
</script>

<template>
  <div class="flex min-h-screen w-screen flex-col">
    <NavBar />
    <RouterView class="flex-1 bg-brand-very-light-orange"/>
  </div>



</template>
