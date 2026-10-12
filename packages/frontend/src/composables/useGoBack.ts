import { useRouter, type RouteLocationRaw } from "vue-router"

/**
 * Back navigation that stays inside the app. Vue Router records the previous in-app path in
 * `history.state.back`; when it's missing (deep link, new tab, shared URL) `router.back()` would
 * leave the app, so navigate to `fallback` instead.
 */
export function useGoBack() {
  const router = useRouter()

  return function goBack(fallback: RouteLocationRaw): void {
    const state = history.state as { back?: string | null } | null
    if (state?.back) {
      router.back()
    } else {
      router.push(fallback)
    }
  }
}
