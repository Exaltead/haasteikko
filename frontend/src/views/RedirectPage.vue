<template>
  <div></div>
</template>
<script lang="ts" setup>
import { useRouter } from 'vue-router';
import { authInstance } from '../auth/auth';

const router = useRouter()

authInstance.handleRedirectPromise()
  .then((tokens) => {
    if (tokens) {
      authInstance.setActiveAccount(tokens.account)
      router.push({ name: "home" })
    }
    else {
      router.push({ name: "login" })
    }


  })
  .catch((error) => {
    console.log(error)
  })


</script>
