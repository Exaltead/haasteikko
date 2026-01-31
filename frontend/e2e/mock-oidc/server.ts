/**
 * Mock OIDC Server for E2E Testing
 *
 * This provides a minimal OIDC-compatible server that:
 * 1. Serves JWKS for backend JWT validation
 * 2. Provides direct token generation for tests (bypassing full OAuth flow)
 * 3. Handles basic OIDC discovery
 */

import express from "express"
import { randomBytes } from "crypto"
import { SignJWT, exportJWK, generateKeyPair } from "jose"

const PORT = parseInt(process.env.OIDC_PORT || "9000")
const ISSUER = `http://localhost:${PORT}`

interface KeyPair {
  publicKey: Awaited<ReturnType<typeof generateKeyPair>>["publicKey"]
  privateKey: Awaited<ReturnType<typeof generateKeyPair>>["privateKey"]
  kid: string
}

let keyPair: KeyPair | null = null
let jwks: object | null = null

async function initializeKeys(): Promise<void> {
  const { publicKey, privateKey } = await generateKeyPair("RS256")
  const kid = "test-key-" + Date.now()

  keyPair = {
    publicKey,
    privateKey,
    kid,
  }

  const publicJwk = await exportJWK(publicKey)
  jwks = {
    keys: [
      {
        ...publicJwk,
        kid,
        use: "sig",
        alg: "RS256",
      },
    ],
  }

  console.log("[Mock OIDC] Keys initialized with kid:", kid)
}

async function createToken(userId: string, audience: string): Promise<string> {
  if (!keyPair) {
    throw new Error("Keys not initialized")
  }

  const token = await new SignJWT({
    sub: userId,
    email: `${userId}@test.local`,
    email_verified: true,
  })
    .setProtectedHeader({ alg: "RS256", kid: keyPair.kid })
    .setIssuer(ISSUER)
    .setAudience(audience)
    .setIssuedAt()
    .setExpirationTime("1h")
    .sign(keyPair.privateKey)

  return token
}

async function startServer(): Promise<void> {
  await initializeKeys()

  const app = express()
  app.use(express.json())

  // CORS for frontend
  app.use((_req, res, next) => {
    res.header("Access-Control-Allow-Origin", "*")
    res.header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
    res.header("Access-Control-Allow-Headers", "Content-Type, Authorization")
    next()
  })

  // OpenID Configuration
  app.get("/.well-known/openid-configuration", (_req, res) => {
    res.json({
      issuer: ISSUER,
      authorization_endpoint: `${ISSUER}/authorize`,
      token_endpoint: `${ISSUER}/token`,
      userinfo_endpoint: `${ISSUER}/userinfo`,
      jwks_uri: `${ISSUER}/.well-known/jwks.json`,
      response_types_supported: ["code"],
      subject_types_supported: ["public"],
      id_token_signing_alg_values_supported: ["RS256"],
      scopes_supported: ["openid", "profile", "email"],
      token_endpoint_auth_methods_supported: ["none"],
      code_challenge_methods_supported: ["S256"],
    })
  })

  // JWKS endpoint
  app.get("/.well-known/jwks.json", (_req, res) => {
    res.json(jwks)
  })

  // In-memory authorization codes
  const authCodes = new Map<string, { userId: string; redirectUri: string; codeChallenge?: string }>()

  // Authorization endpoint - auto-approve and redirect
  app.get("/authorize", (req, res) => {
    const { redirect_uri, state, login_hint, code_challenge } = req.query

    const userId = (login_hint as string) || `test-user-${Date.now()}`
    const code = randomBytes(32).toString("hex")

    authCodes.set(code, {
      userId,
      redirectUri: redirect_uri as string,
      codeChallenge: code_challenge as string | undefined,
    })

    // Auto-redirect with code
    const redirectUrl = new URL(redirect_uri as string)
    redirectUrl.searchParams.set("code", code)
    if (state) {
      redirectUrl.searchParams.set("state", state as string)
    }

    res.redirect(redirectUrl.toString())
  })

  // Token endpoint
  app.post("/token", async (req, res) => {
    const { code, grant_type } = req.body

    if (grant_type !== "authorization_code") {
      res.status(400).json({ error: "unsupported_grant_type" })
      return
    }

    const authCode = authCodes.get(code)
    if (!authCode) {
      res.status(400).json({ error: "invalid_grant" })
      return
    }

    authCodes.delete(code)

    try {
      const accessToken = await createToken(authCode.userId, "https://haasteikko.eu/api")
      const idToken = await createToken(authCode.userId, ISSUER)

      res.json({
        access_token: accessToken,
        id_token: idToken,
        token_type: "Bearer",
        expires_in: 3600,
      })
    } catch (error) {
      console.error("Token creation error:", error)
      res.status(500).json({ error: "server_error" })
    }
  })

  // Userinfo endpoint
  app.get("/userinfo", (req, res) => {
    // In a real implementation, we'd validate the token
    // For testing, just return mock data
    res.json({
      sub: "test-user",
      email: "test@test.local",
      email_verified: true,
    })
  })

  // Direct token generation endpoint for E2E tests
  // This bypasses the OAuth flow entirely
  app.post("/test/token", async (req, res) => {
    const { userId, audience } = req.body

    if (!userId) {
      res.status(400).json({ error: "userId is required" })
      return
    }

    try {
      const token = await createToken(userId, audience || "https://haasteikko.eu/api")
      res.json({ token })
    } catch (error) {
      console.error("Test token creation error:", error)
      res.status(500).json({ error: "Failed to create token" })
    }
  })

  // Health check
  app.get("/health", (_req, res) => {
    res.json({ status: "ok", issuer: ISSUER })
  })

  app.listen(PORT, () => {
    console.log(`Mock OIDC server running at ${ISSUER}`)
    console.log(``)
    console.log(`Endpoints:`)
    console.log(`  Discovery: ${ISSUER}/.well-known/openid-configuration`)
    console.log(`  JWKS:      ${ISSUER}/.well-known/jwks.json`)
    console.log(`  Authorize: ${ISSUER}/authorize`)
    console.log(`  Token:     ${ISSUER}/token`)
    console.log(`  Test:      ${ISSUER}/test/token (POST with {userId})`)
  })
}

startServer().catch(console.error)
