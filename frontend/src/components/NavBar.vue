<script lang="ts" setup>
import { computed, ref, watch } from 'vue';
import IconMenu from './icons/IconMenu.vue';
import { useRouter } from 'vue-router';
import { useAuth0 } from '@auth0/auth0-vue';

const { logout } = useAuth0();

const navOpen = ref(false)

const navClassName = computed<string>(() => {
  return navOpen.value ? "md:block" : "hidden md:block"
})

function toggleNavOpen() {
  navOpen.value = !navOpen.value
}

const router = useRouter()

watch(router.currentRoute, () => { navOpen.value = false })

const showNavLinks = computed(() => {
  return router.currentRoute.value.name !== "login"
})

function doLogout() {

  navOpen.value = false
  logout({ logoutParams: { returnTo: window.location.origin}})
  router.push({ name: "login" })
}


</script>

<template>
  <div v-if="showNavLinks" class="md:flex bg-brand-primary p-2 md:flex justify-between md:items-center">
    <div class=" flex flex-row gap-6 md:items-center">
      <button @click="toggleNavOpen" class="cursor-pointer md:hidden">
        <IconMenu class="w-6 text-white" />
      </button>
      <h1 class="font-bold text-text-primary">Haasteikko</h1>
    </div>


    <div class="left-0 md:static absolute md:w-auto bg-brand-primary w-full" :class="navClassName">
      <div class="md:flex md:flex-row p-4 md:py-0 md:gap-10 items-center">
        <div class="h-fit">
          <RouterLink :to="{ name: 'home' }" class="text-white">
            <span class="cursor-pointer">Koti</span>
          </RouterLink>
        </div>
        <div class="h-fit">
          <RouterLink :to="{ name: 'library' }" class="text-white">
            <span class="cursor-pointer">Kirjasto</span>
          </RouterLink>
        </div>
        <div class="h-fit">
          <RouterLink :to="{ name: 'challenges' }" class="text-white">
            <span class="cursor-pointer">Haasteet</span>
          </RouterLink>
        </div>
        <div class="h-fit">
          <RouterLink :to="{ name: 'manageChallenges' }" class="text-white">
            <span class="cursor-pointer">Hallinnoi haasteita</span>
          </RouterLink>
        </div>


        <div class="mt-10 md:mt-0 md:px-4 text-text-primary border w-fit p-2 rounded-lg">
          <button :onClick="doLogout" class="cursor-pointer">Kirjaudu ulos</button>
        </div>
      </div>

    </div>

  </div>
  <div v-else class="bg-brand-primary p-2">
    <h1 class="font-bold text-text-primary">Haasteikko</h1>
  </div>
</template>
