import { createApp } from "vue"
import App from "./App.vue"
import router from "./router"
import "./style.css"
import { createAuthService } from "./plugins/AuthService"
import { createApi } from "./plugins/HttpPlugin"

const app = createApp(App)

app.use(router)

app.use(
  createAuthService({
    authority: import.meta.env.VITE_AUTH_AUTHORITY || "https://auth.haasteikko.eu",
    clientId: import.meta.env.VITE_CLIENT_ID,
    redirectUri: window.location.origin,
    audience: import.meta.env.VITE_AUTH_AUDIENCE || "https://haasteikko.eu/api",
  }),
)

app.use(
  createApi({
    apiUrl: import.meta.env.VITE_API_URL,
  }),
)

app.mount("#app")
