<template>
  <div></div>
</template>
<script lang="ts" setup>
import { useRouter } from 'vue-router';
import { setRedirectHanding } from '../auth/auth';

const router = useRouter()


setRedirectHanding().then((state) => {
  if (state === "Authenticated") {

    router.push({ name: "home" })
  }
  else {
    router.push({ name: "login" })
  }
})



</script>
