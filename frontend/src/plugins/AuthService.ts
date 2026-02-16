import { UserManager, WebStorageStateStore, type UserManagerSettings, type User } from "oidc-client-ts"
import { ref, readonly, type Ref, type DeepReadonly } from "vue"
import type { Plugin, App, InjectionKey } from "vue"
import type { NavigationGuardWithThis } from "vue-router"

export interface AuthConfig {
  authority: string
  clientId: string
  redirectUri: string
  audience?: string
  scope?: string
}

export interface AuthState {
  isAuthenticated: DeepReadonly<Ref<boolean>>
  isLoading: DeepReadonly<Ref<boolean>>
  user: DeepReadonly<Ref<User | null>>
}

export interface AuthService extends AuthState {
  loginWithRedirect: () => Promise<void>
  logout: () => Promise<void>
  getAccessTokenSilently: () => Promise<string>
  handleCallback: () => Promise<void>
}

const AUTH_SERVICE_KEY: InjectionKey<AuthService> = Symbol("auth-service")

// Global state shared across the app
const isAuthenticated = ref(false)
const isLoading = ref(true)
const user = ref<User | null>(null)

let userManager: UserManager | null = null

function createUserManager(config: AuthConfig): UserManager {
  const settings: UserManagerSettings = {
    authority: config.authority,
    client_id: config.clientId,
    redirect_uri: config.redirectUri,
    post_logout_redirect_uri: config.redirectUri,
    response_type: "code",
    scope: config.scope || "openid profile email",
    userStore: new WebStorageStateStore({ store: window.localStorage }),
    automaticSilentRenew: true,
    extraQueryParams: config.audience ? { audience: config.audience } : undefined,
  }

  return new UserManager(settings)
}

async function initializeAuth(): Promise<void> {
  if (!userManager) return

  isLoading.value = true

  try {
    // Check if we're handling a callback
    if (window.location.search.includes("code=") || window.location.search.includes("error=")) {
      // Don't try to get user here, let handleCallback do it
      return
    }

    // Try to get existing user from storage
    const existingUser = await userManager.getUser()
    if (existingUser && !existingUser.expired) {
      user.value = existingUser
      isAuthenticated.value = true
    }
  } catch (error) {
    console.error("[Auth] Failed to initialize:", error)
  } finally {
    isLoading.value = false
  }
}

export function createAuthService(config: AuthConfig): Plugin {
  return {
    install(app: App) {
      userManager = createUserManager(config)

      // Set up event listeners
      userManager.events.addUserLoaded((loadedUser) => {
        user.value = loadedUser
        isAuthenticated.value = true
        isLoading.value = false
      })

      userManager.events.addUserUnloaded(() => {
        user.value = null
        isAuthenticated.value = false
      })

      userManager.events.addSilentRenewError((error) => {
        console.error("[Auth] Silent renew error:", error)
      })

      const authService: AuthService = {
        isAuthenticated: readonly(isAuthenticated),
        isLoading: readonly(isLoading),
        user: readonly(user),

        async loginWithRedirect(): Promise<void> {
          if (!userManager) throw new Error("Auth not initialized")
          await userManager.signinRedirect()
        },

        async logout(): Promise<void> {
          if (!userManager) throw new Error("Auth not initialized")
          await userManager.signoutRedirect()
        },

        async getAccessTokenSilently(): Promise<string> {
          if (!userManager) throw new Error("Auth not initialized")

          let currentUser = await userManager.getUser()

          if (!currentUser || currentUser.expired) {
            // Try silent renew
            try {
              currentUser = await userManager.signinSilent()
            } catch {
              throw new Error("Unable to get access token - user not authenticated")
            }
          }

          if (!currentUser?.access_token) {
            throw new Error("No access token available")
          }

          return currentUser.access_token
        },

        async handleCallback(): Promise<void> {
          if (!userManager) throw new Error("Auth not initialized")

          isLoading.value = true
          try {
            const callbackUser = await userManager.signinRedirectCallback()
            user.value = callbackUser
            isAuthenticated.value = true

            // Clean up the URL
            window.history.replaceState({}, document.title, window.location.pathname)
          } catch (error) {
            console.error("[Auth] Callback error:", error)
            throw error
          } finally {
            isLoading.value = false
          }
        },
      }

      app.provide(AUTH_SERVICE_KEY, authService)

      // Initialize auth state
      initializeAuth()
    },
  }
}

export function useAuth(): AuthService {
  const auth = inject(AUTH_SERVICE_KEY)
  if (!auth) {
    throw new Error("Auth service not provided. Did you forget to install the auth plugin?")
  }
  return auth
}

// Need to import inject
import { inject } from "vue"

/**
 * Navigation guard that requires authentication.
 * Redirects to login if user is not authenticated.
 */
export const authGuard: NavigationGuardWithThis<undefined> = async (to, from, next) => {
  // Wait for auth to initialize
  const maxWait = 5000
  const startTime = Date.now()

  while (isLoading.value && Date.now() - startTime < maxWait) {
    await new Promise((resolve) => setTimeout(resolve, 50))
  }

  if (isAuthenticated.value) {
    next()
  } else {
    // Store the intended destination
    sessionStorage.setItem("auth_redirect", to.fullPath)

    // Redirect to login
    if (userManager) {
      await userManager.signinRedirect()
    } else {
      next("/")
    }
  }
}

/**
 * Get the stored redirect path after login
 */
export function getPostLoginRedirect(): string | null {
  const redirect = sessionStorage.getItem("auth_redirect")
  sessionStorage.removeItem("auth_redirect")
  return redirect
}
