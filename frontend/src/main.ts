import { createApp } from "vue"
import App from "./App.vue"
import router from "./router"
import "./style.css"
import { createAuth0 } from "@auth0/auth0-vue"
import { createApi } from "./plugins/HttpPlugin"

const CLIENT_ID = import.meta.env.VITE_CLIENT_ID

const app = createApp(App)

app.use(router)

app.use(
  createAuth0({
    domain: "auth.haasteikko.eu",
    clientId: CLIENT_ID,
    authorizationParams: {
      redirect_uri: window.location.origin,
      audience: "https://haasteikko.eu/api",
    },
    useRefreshTokens: false
  }),
)

app.use(
  createApi({
    apiUrl: import.meta.env.VITE_API_URL,
  }),
)

app.mount("#app")
