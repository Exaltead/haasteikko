import { createApp } from "vue"
import App from "./App.vue"
import router from "./router"
import "./style.css"
import { createAuth0 } from "@auth0/auth0-vue"
import { createApi } from "./plugins/HttpPlugin"

const app = createApp(App)

app.use(router)

app.use(
  createAuth0({
    domain: "auth.haasteikko.eu",
    clientId: "XaIT7NyA5QbLhHtKuKexYO36srSZRW9M",
    authorizationParams: {
      redirect_uri: window.location.origin,
      audience: "https://haasteikko.eu/api",
    },
  }),
)

app.use(
  createApi({
    apiUrl: import.meta.env.VITE_API_URL,
  }),
)

app.mount("#app")
