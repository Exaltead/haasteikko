import { jwtDecode } from "jwt-decode"

export function saveTokens(accessToken: string): void {
  sessionStorage.setItem("accessToken", accessToken)
}

export function getAccessToken(): string | null {
  return sessionStorage.getItem("accessToken")
}

export function getRefreshToken(): string | null {
  return sessionStorage.getItem("refreshToken")
}

export function clearTokens(): void {
  sessionStorage.removeItem("accessToken")
}

export function isLoggedIn(): boolean {

  const accessToken = getAccessToken()
  if (!accessToken) {
    return false
  }

  const decoded = jwtDecode(accessToken) as { exp: number }
  const currentTime = Math.floor(Date.now() / 1000) // Current time in seconds
  const hasValidToken = decoded.exp > currentTime

  // Bit hacky, but clearing the token here if expired sets the sate to more "correct" state
  if (!hasValidToken) {
    clearTokens()
  }

  return hasValidToken
}
