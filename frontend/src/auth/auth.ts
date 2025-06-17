import { createStandardPublicClientApplication, InteractionRequiredAuthError, type AccountInfo, type IPublicClientApplication } from "@azure/msal-browser"
import { tr } from "zod/v4/locales"

const clientId = import.meta.env.VITE_CLIENT_ID
const tenantId = import.meta.env.VITE_TENANT_ID
const API_URL = import.meta.env.VITE_API_URL



console.log(clientId, tenantId, API_URL)
let sharedAuthInstance: IPublicClientApplication | undefined = undefined


export async function getAuthInstance(): Promise<IPublicClientApplication> {
  if (!sharedAuthInstance) {
    sharedAuthInstance = await createStandardPublicClientApplication({
      auth: {
        clientId: clientId,
        authority: `https://${"haasteikko"}.ciamlogin.com/`,
        redirectUri: `http://localhost:5173/auth/callback`,
        navigateToLoginRequestUrl: false

      }
    })
  }

  return sharedAuthInstance
}

export async function getAccessToken({ routeOnLoginFail } = { routeOnLoginFail: true }): Promise<string | undefined> {
  const authInstance = await getAuthInstance()
  try {
    console.log("Getting token")
    const token = await authInstance.acquireTokenSilent({ scopes: ["user.read"] })
    return token.accessToken
  }
  catch (error) {
    console.log(error)
    if (error instanceof InteractionRequiredAuthError && routeOnLoginFail) {
      await authInstance.acquireTokenRedirect({ scopes: ["user.read"] })
    }

  }

}

export async function setRedirectHanding(): Promise<"Authenticated" | "Unauthenticated"> {
  const authInstance = await getAuthInstance()
  try {
    const tokens = await authInstance.handleRedirectPromise()
    if (tokens) {
      authInstance.setActiveAccount(tokens.account)
      return "Authenticated"
    }
    else {
      return "Unauthenticated"
    }
  } catch (error) {
    console.error("Error setting redirect handling:", error)
    return "Unauthenticated"
  }
}

export async function redirectLogin() {
  const authInstance = await getAuthInstance()
  await authInstance.loginRedirect({
    scopes: ["user.read"],
    prompt: "login"
  })


  /*await authInstance.acquireTokenRedirect({
        scopes: ["user.read"]
      })*/
  /*try {
    const token = await authInstance.acquireTokenSilent({ scopes: ["user.read"]})
    return token.accessToken
  }
  catch(error) {
    console.log(error)
    if (error instanceof InteractionRequiredAuthError) {
      const accessToken =  await authInstance.acquireTokenRedirect({
        scopes: ["user.read"]
      })

      return accessToken
    }
  }*/


  /*  await authInstance.loginRedirect({
    scopes: ["user.read"],
    prompt: "login"
  })*/
}
