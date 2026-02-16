# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Haasteikko is a full-stack application for tracking reading, gaming, and watching challenges. It's a monorepo with:
- **frontend/**: Vue 3 + TypeScript SPA
- **backend/**: Rust API server using Axum
- **scripts/**: Python migration utilities

## Common Commands

### Frontend (from `frontend/` directory)
```bash
npm run dev           # Start dev server (localhost:5173)
npm run build         # Production build (runs type-check + vite build)
npm run type-check    # TypeScript validation
npm run lint          # ESLint with auto-fix
npm run format        # Prettier formatting
npm run storybook     # Component development (port 6006)
PLAYWRIGHT_HTML_OPEN=never npx playwright test   # E2E tests (suppress report popup)
```

### Backend (from `backend/` directory)
```bash
cargo build --release                                    # Local build
cargo build --release --target x86_64-unknown-linux-gnu  # Production build
cargo test                                               # Run tests
```

## Architecture

### Frontend
- **api/**: API clients extending `BaseApiClient` - handles Auth0 token injection
- **components/**: Reusable Vue components (basics/, Entry/, icons/, etc.)
- **views/**: Page-level components
- **models/**: TypeScript types (challenge.ts, LibraryItem.ts)
- **plugins/HttpPlugin**: Provides `$http` for API access with authentication

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
SQLite with migrations in `backend/migrations/`. Foreign keys enforced. User-scoped data access pattern.

## Authentication

Auth0 is used for authentication:
- Domain: auth.haasteikko.eu
- Frontend: `@auth0/auth0-vue` library
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
