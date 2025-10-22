<script lang="ts" setup>
console.log("LoginView")
  import { useAuth0 } from '@auth0/auth0-vue';
import { watch } from 'vue';
import { useRouter } from 'vue-router';

  const { loginWithRedirect, isAuthenticated, user, logout } = useAuth0();
  const router = useRouter()

  watch(isAuthenticated, (newValue) => {
    if(newValue){
      console.log("Now authenticated, redirecting to /home")
      router.push({ name: 'home'})
    }
  });

  function login() {
    loginWithRedirect()
  }

  function doLogout(){
    logout({
      logoutParams: {
        returnTo: window.location.origin
      }
    })
  }

</script>
<template>
    <div>

    <button @click="login">Log in</button>
    <button @click="doLogout"> Log Out</button>
    <h2>User Profile</h2>
    <pre v-if="isAuthenticated">
        <code>{{ user }}</code>
    </pre>
    <p v-else>Not authenticated</p>
  </div>
</template>
