<script lang="ts" setup>
import { computed, ref } from 'vue';
import Button from '@/components/basics/BrandedButton.vue';
import { postLogin } from '@/api/authApi';
import { saveTokens } from '@/modules/auth-store';
import { useRouter } from 'vue-router';
import { authInstance, getAccessToken, redirectLogin } from '@/auth/auth';

const router = useRouter()

const userName = ref<string>("")
const password = ref<string>("")

const loggingIn = ref(false)

const invalidLogin = computed(() => {
  return userName.value.length === 0 || password.value.length === 0
})



async function doLogin() {
  loggingIn.value = true
  const token = await postLogin(userName.value.trim(), password.value.trim())
  loggingIn.value = false
  if (token) {
    saveTokens(token)
    router.push({ name: "home" })
  }
}

const loginDisabled = computed(() => {
  return loggingIn.value || invalidLogin.value
})

const loginText = computed(() => {
  return loggingIn.value ? "Kirjaudutaan sisään..." : "Kirjaudu sisään"
})


async function routeOnUserLoggedIn() {

  const accessToken = await getAccessToken({ routeOnLoginFail: true })

  if (accessToken) {
    console.log(accessToken)
    router.push({ name: "login" })
  }
  else {
    setTimeout(routeOnUserLoggedIn, 1000)
  }
}


routeOnUserLoggedIn()
</script>

<template>
  <main>
    <div class="flex flex-row w-full justify-center pt-10">
      <div
        class="flex flex-col justify-center w-fit border border-brand-primary bg-light-gray rounded-lg shadow-lg p-10 pt-4 gap-10">

        <p class="mx-auto text-lg  font-bold">Kirjaudu sisään Haasteikkoon</p>
        <Button :onClick="redirectLogin"></Button>
        <div class="flex flex-col gap-10 max-w-md mx-auto">

          <div class="flex flex-col gap-2">
            <label for="username"> Käyttäjätunnus</label>
            <input id="username" type="text" required v-model="userName"
              class="rounded border border-brand-primary bg-white" />
          </div>
          <div class="flex flex-col gap-2">
            <label for="Password"> Salasana</label>
            <input id="password" type="password" required v-model="password"
              class="rounded border border-brand-primary bg-white" />
          </div>

        </div>

        <div class="flex w-full justify-center">
          <Button :disabled="loginDisabled" :onClick="doLogin" :text="loginText" :is-submitting="loggingIn"
            class="border border-brand-primary rounded">
            Kirjaudu sisään
          </Button>
        </div>
      </div>

    </div>

  </main>
</template>
