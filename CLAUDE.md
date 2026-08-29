# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Haasteikko is a full-stack application for tracking reading, gaming, and watching challenges. It's a monorepo with:
- **packages/frontend/**: Vue 3 + TypeScript SPA
- **packages/backend/**: Rust API server using Axum
- **scripts/**: Python migration utilities

## Common Commands

### Root workspace (from repo root)
```bash
npm run setup         # npm install + cargo fetch
npm run start:all     # Frontend dev server + backend, concurrently
npm run build         # Frontend build + cargo build --release
npm run test:e2e      # Delegates to the frontend Playwright suite
```

### Frontend (from `packages/frontend/` directory)
```bash
npm run dev           # Start dev server (localhost:5173)
npm run build         # Production build (runs type-check + vite build)
npm run type-check    # TypeScript validation (vue-tsc --build)
npm run lint          # ESLint with auto-fix
npm run format        # Prettier formatting (src/ only)
npm run storybook     # Component development (port 6006)
npm run test:e2e      # Playwright E2E tests (Desktop Firefox project)
PLAYWRIGHT_HTML_OPEN=never npx playwright test   # Same, suppressing the HTML report popup
npx playwright test e2e/challenge-flow.spec.ts   # Run a single E2E spec
```
`playwright.config.ts` auto-starts the full stack needed for E2E tests via its `webServer`
entries: the mock OIDC server (`npm run mock-oidc`, port 9000), the backend against a scratch
`test-e2e.sqlite` (port 3000), and the frontend dev server in `test` mode (port 5173). No manual
setup is required beyond having `cargo` on PATH — just run the Playwright command.

Component/story tests run through Vitest's browser mode via `@storybook/addon-vitest`
(configured in `vite.config.ts`); there is no separate `test` script yet — run with
`npx vitest`.

### Backend (from `packages/backend/` directory)
```bash
cargo build --release                                    # Local build
cargo build --release --target x86_64-unknown-linux-gnu  # Production build
cargo test                                               # Run tests
cargo test <test_name>                                   # Run a single test
```

## Architecture

### Frontend
- **api/**: API clients extending `BaseApiClient`, each backed by a Zod schema pair
  (entity schema + "new entity" schema) and an injected `HttpProxy` for the actual HTTP calls
- **api/HttpProxy.ts**: interface for the get/put/post/delete proxy; `plugins/HttpPlugin.ts`'s
  `useHttpApi()` composable is the concrete implementation - it attaches the bearer token
  (via `AuthService`) and validates responses against the Zod schema passed in
- **components/**: Reusable Vue components (basics/, Entry/, EntryListing/, Challenge/,
  ChallengeManagement/, icons/, etc.)
- **views/**: Page-level components
- **models/**: TypeScript types (challenge.ts, LibraryItem.ts)
- **plugins/AuthService.ts**: the actual auth implementation, see Authentication below

Uses Zod for runtime API response validation. Tailwind CSS v4 for styling.

### Backend
Feature-based module structure where each feature (library, challenge, solution, etc.) has:
- `mod.rs` - Route definitions
- `domain.rs` - Business logic
- `api.rs` - HTTP handlers
- `repository.rs` - Database operations

Key files:
- `main.rs` - Server setup, CORS, routing
- `auth.rs` - JWT validation via Auth0 JWKS, `User` extractor
- `database.rs` - SQLite connection, `Repository` trait
- `migrations.rs` - SQL migration runner

### Database
SQLite with migrations in `packages/backend/migrations/`. Foreign keys enforced. User-scoped data access pattern.

## Authentication

Auth0 is used as the OAuth provider, but the frontend does NOT use the `@auth0/auth0-vue` SDK:
- Domain: auth.haasteikko.eu
- Frontend: a hand-rolled Vue plugin (`plugins/AuthService.ts`) built directly on `oidc-client-ts`'s
  `UserManager` (authorization-code flow, tokens in `localStorage`). `useAuth()` exposes
  `isAuthenticated`/`user`/`getAccessTokenSilently`/`loginWithRedirect`/`logout`/`handleCallback`;
  `authGuard` is the router navigation guard. `src/modules/auth-store.ts` is an older
  sessionStorage-based helper - check whether it's still wired up before extending it.
- E2E tests run against `e2e/mock-oidc/server.ts`, a local mock OIDC provider, instead of real Auth0
- Backend: JWT validation via JWKS endpoint

## Environment Variables

**Frontend** (via Vite):
- `VITE_API_URL` - Backend API URL
- `VITE_CLIENT_ID` - Auth0 client ID

**Backend**:
- `JWKS_URL` - Auth0 JWKS endpoint
- `REQUIRED_AUDIENCE` - JWT audience (https://haasteikko.eu/api)
- `MIGRATIONS_PATH` - Path to SQL migrations
- `DATABASE_PATH` - SQLite file path (defaults to database.sqlite)

## Code Style

- 2 space indentation, LF line endings, 100 char max line width
- Frontend: No semicolons, double quotes (Prettier)
- Icons: Material Design icons (Apache 2.0 licensed)

## Deployment

Automated via GitHub Actions on push to main. Deploys to Ubuntu VM via rsync, restarts systemd service `haasteikko-backend`.
