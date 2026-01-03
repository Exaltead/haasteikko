<script lang="ts" setup>

import BrandedButton from '@/components/basics/BrandedButton.vue';
import { useAuth0 } from '@auth0/auth0-vue';
import { watch, onMounted, nextTick } from 'vue';
import { useRouter, useRoute } from 'vue-router';



const { loginWithRedirect, isAuthenticated, isLoading } = useAuth0();
const router = useRouter()
const route = useRoute()

// Prevent multiple navigation attempts (auth SDK may toggle isLoading briefly).
let navigated = false

async function navigateHomeOnce() {
  if (navigated) return
  navigated = true

  // Wait for router to be ready and a microtask to let any synchronous
  // SDK work complete. This makes navigation deterministic when the SDK
  // transitions isLoading from true -> false.
  await router.isReady()
  await nextTick()

  await router.replace({ name: 'home' })

}

// If auth0 is already processing (redirect callback) there will be a short loading
// period. Wait until loading finishes and the user is authenticated before
// navigating to the protected 'home' route. Use replace so we don't leave the
// login page in history.
watch(
  [isAuthenticated, isLoading],
  ([auth, loading]) => {
    // If we are in the OAuth redirect URL (contains code/state) the auth
    // plugin or router may perform a cleanup navigation; wait until that is
    // finished to avoid racing and causing our navigation to be cancelled.
    if (!loading && auth) {
      const hasRedirectParams = Boolean(route.query.code || route.query.state)
      if (hasRedirectParams) {
        console.log('Detected redirect params in URL; waiting for router cleanup')
        // watch the route and navigate once query params are gone
        const stop = watch(
          () => route.fullPath,
          () => {
            const stillHas = Boolean(route.query.code || route.query.state)
            if (!stillHas) {
              stop()
              void navigateHomeOnce()
            }
          }
        )
      } else {
        void navigateHomeOnce()
      }
    }
  }
)

onMounted(() => {
  if (!isLoading.value && isAuthenticated.value) {
    void navigateHomeOnce()
  }
})

function login() {
  loginWithRedirect()
}


</script>
<template>
    <div class="p-10">
      <BrandedButton :onClick="login" text="Kirjaudu sisään" variant="primary"></BrandedButton>
  </div>
</template>
