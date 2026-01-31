<script lang="ts" setup>

import BrandedButton from '@/components/basics/BrandedButton.vue';
import { useAuth, getPostLoginRedirect } from '@/plugins/AuthService';
import { watch, onMounted, nextTick } from 'vue';
import { useRouter, useRoute } from 'vue-router';

const { loginWithRedirect, handleCallback, isAuthenticated, isLoading } = useAuth();
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

  // Check if there's a stored redirect path from before login
  const redirectPath = getPostLoginRedirect()
  if (redirectPath) {
    await router.replace(redirectPath)
  } else {
    await router.replace({ name: 'home' })
  }
}

// Handle OAuth callback if we have code/state in URL
async function handleOAuthCallback() {
  const hasRedirectParams = Boolean(route.query.code || route.query.state)
  if (hasRedirectParams) {
    try {
      await handleCallback()
      void navigateHomeOnce()
    } catch (error) {
      console.error('OAuth callback failed:', error)
    }
  }
}

// Watch for auth state changes
watch(
  [isAuthenticated, isLoading],
  ([auth, loading]) => {
    if (!loading && auth) {
      void navigateHomeOnce()
    }
  }
)

onMounted(async () => {
  // Check for OAuth callback
  const hasRedirectParams = Boolean(route.query.code || route.query.state)
  if (hasRedirectParams) {
    await handleOAuthCallback()
  } else if (!isLoading.value && isAuthenticated.value) {
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
