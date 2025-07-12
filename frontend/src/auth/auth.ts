import { createStandardPublicClientApplication, InteractionRequiredAuthError, type AccountInfo, type IPublicClientApplication } from "@azure/msal-browser"

const clientId = import.meta.env.VITE_CLIENT_ID
const tenantId = import.meta.env.VITE_TENANT_ID
const API_URL = import.meta.env.VITE_API_URL
const REDIRECT_URI = import.meta.env.VITE_REDIRECT_URI
const POST_LOGOUT_REDIRECT_URI = import.meta.env.VITE_POST_LOGOUT_REDIRECT_URI
const APP_CIAM_NAME = import.meta.env.VITE_APP_CIAM_NAME
const requestedScopes = ["api://7026c030-9feb-4589-b17f-cc27c5cacb6d/haasteikko/haasteikko.call"]



console.log(clientId, tenantId, API_URL)
let sharedAuthInstance: IPublicClientApplication | undefined = undefined


export async function getAuthInstance(): Promise<IPublicClientApplication> {
  if (!sharedAuthInstance) {
    sharedAuthInstance = await createStandardPublicClientApplication({
      auth: {
        clientId: clientId,
        authority: `https://${APP_CIAM_NAME}.ciamlogin.com/`,
        redirectUri: REDIRECT_URI,
        navigateToLoginRequestUrl: false,
        postLogoutRedirectUri: POST_LOGOUT_REDIRECT_URI
      },
      telemetry: undefined

    })
  }

  return sharedAuthInstance
}

export async function getAccessToken({ routeOnLoginFail } = { routeOnLoginFail: true }): Promise<string | undefined> {
  const authInstance = await getAuthInstance()
  try {
    console.log("Getting token")
    const token = await authInstance.acquireTokenSilent({ scopes: requestedScopes })
    return token.accessToken
  }
  catch (error) {
    if (error instanceof InteractionRequiredAuthError && routeOnLoginFail) {
      await authInstance.acquireTokenRedirect({ scopes: requestedScopes })
    }
    console.error("Error acquiring token silently:", error)
  }

  return undefined
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

export async function redirectLogout() {
  const authInstance = await getAuthInstance()
  await authInstance.logoutRedirect({})

}

export async function clear() {
  const authInstance = await getAuthInstance()
  await authInstance.clearCache()
  authInstance.setActiveAccount(null)
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
