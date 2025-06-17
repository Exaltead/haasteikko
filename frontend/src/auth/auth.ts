import { createStandardPublicClientApplication, InteractionRequiredAuthError } from "@azure/msal-browser"

const clientId = import.meta.env.VITE_CLIENT_ID
const tenantId = import.meta.env.VITE_TENANT_ID
const API_URL = import.meta.env.VITE_API_URL



console.log(clientId, tenantId, API_URL)
export const authInstance = await createStandardPublicClientApplication({
  auth: {
    clientId: clientId,
    authority: `https://${"haasteikko"}.ciamlogin.com/`,
    redirectUri: `http://localhost:5173/auth/callback`,
    navigateToLoginRequestUrl: false

  }
})


export async function getAccessToken({routeOnLoginFail} ={routeOnLoginFail:true} ): Promise<string| undefined> {
  try {
    console.log("Gettiung tokken")
    const token = await authInstance.acquireTokenSilent({ scopes: ["user.read"]})
    return token.accessToken
  }
  catch(error) {
    console.log(error)
    if (error instanceof InteractionRequiredAuthError && routeOnLoginFail) {
      await authInstance.acquireTokenRedirect({scopes: ["user.read"]})
    }

  }

}

export async function redirectLogin() {
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
