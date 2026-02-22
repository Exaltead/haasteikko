# AGENTS.md

This file provides guidance for AI agents (Claude, Mistral Vibe, etc.) working with the Haasteikko codebase.

## Project Structure

```
haasteikko/
├── .github/              # GitHub configuration and workflows
├── backend/              # Rust API server (Axum)
├── frontend/             # Vue 3 + TypeScript SPA
├── CLAUDE.md             # Claude-specific instructions
├── AGENTS.md             # This file - general AI agent guidance
└── README.md             # Project overview
```

## Key Directories

### Backend (`backend/`)
- **Language**: Rust
- **Framework**: Axum
- **Database**: SQLite
- **Structure**: Feature-based modules with `mod.rs`, `domain.rs`, `api.rs`, `repository.rs`
- **Key files**:
  - `main.rs` - Server setup and routing
  - `auth.rs` - JWT validation (Auth0)
  - `database.rs` - SQLite connection management
  - `migrations.rs` - Database migration runner

### Frontend (`frontend/`)
- **Language**: TypeScript
- **Framework**: Vue 3
- **Styling**: Tailwind CSS v4
- **Validation**: Zod for API responses
- **Structure**:
  - `api/` - API clients with Auth0 token injection
  - `components/` - Reusable Vue components
  - `views/` - Page-level components
  - `models/` - TypeScript types
  - `plugins/` - HTTP and authentication plugins



### GitHub Configuration (`.github/`)
- **Workflows**: CI/CD in `workflows/main.yml`
- **Prompts**: AI agent instructions in `prompts/`
- **Copilot**: High-level instructions in `copilot-instructions.md`

## Development Workflow

### Common Commands

**Frontend** (from `frontend/`):
```bash
npm run dev           # Start dev server
npm run build         # Production build
npm run type-check    # TypeScript validation
npm run lint          # ESLint with auto-fix
npm run storybook     # Component development
```

**Backend** (from `backend/`):
```bash
cargo build --release                                    # Local build
cargo build --release --target x86_64-unknown-linux-gnu  # Production build
cargo test                                               # Run tests
```



## Authentication

- **Provider**: Auth0
- **Frontend**: `@auth0/auth0-vue` library
- **Backend**: JWT validation via Auth0 JWKS endpoint
- **Environment variables**:
  - `VITE_CLIENT_ID` (frontend)
  - `JWKS_URL` (backend)
  - `REQUIRED_AUDIENCE` (backend)

## Database

- **Type**: SQLite
- **Migrations**: SQL files in `backend/migrations/`
- **Access pattern**: User-scoped data with foreign key constraints
- **Connection**: Managed via `database.rs`

## Deployment

- **Target**: Ubuntu VM
- **Method**: GitHub Actions workflow
- **Process**:
  1. Build frontend (Vite)
  2. Build backend (Rust cross-compiled)
  3. Package artifacts
  4. Deploy via rsync
  5. Restart systemd service

## Code Conventions

- **Indentation**: 2 spaces
- **Line endings**: LF
- **Max line width**: 100 characters
- **Frontend**: No semicolons, double quotes (Prettier)
- **Icons**: Material Design icons (Apache 2.0)

## Agent-Specific Files

- **CLAUDE.md**: Claude-specific instructions and patterns
- **AGENTS.md**: This file - general AI agent guidance
- `.github/prompts/`:
  - `impl.prompt.md` - Implementation instructions
  - `plan.prompt.md` - Planning session guidance
- `.github/copilot-instructions.md` - High-level project overview

## Best Practices for AI Agents

1. **Read existing code**: Understand patterns before making changes
2. **Follow conventions**: Match existing style and structure
3. **Use dedicated tools**: Prefer `search_replace` over file overwrites
4. **Verify changes**: Read back files after modifications
5. **Ask questions**: Clarify requirements before implementation
6. **Break down tasks**: Use todo lists for complex changes
7. **Respect constraints**: Don't modify files without explicit permission

## Common Patterns

### Backend Module Structure
```
feature/
├── mod.rs      # Route definitions
├── domain.rs   # Business logic
├── api.rs      # HTTP handlers
└── repository.rs # Database operations
```

### Frontend API Client Pattern
```typescript
// Extends BaseApiClient
class ChallengeApiClient extends BaseApiClient {
  async getChallenges(): Promise<Challenge[]> {
    return this.get<Challenge[]>('/challenges');
  }
}
```

### Database Repository Trait
```rust
pub trait Repository<T> {
    fn create(&self, item: &T) -> Result<T, RepositoryError>;
    fn get_by_id(&self, id: i64) -> Result<Option<T>, RepositoryError>;
    // ... other CRUD operations
}
```

## Environment Variables

**Frontend**:
- `VITE_API_URL` - Backend API endpoint
- `VITE_CLIENT_ID` - Auth0 client ID

**Backend**:
- `JWKS_URL` - Auth0 JWKS endpoint
- `REQUIRED_AUDIENCE` - JWT audience
- `MIGRATIONS_PATH` - Path to SQL migrations
- `DATABASE_PATH` - SQLite file path

## Testing

- **Frontend**: Playwright E2E tests in `frontend/e2e/`
- **Backend**: Rust unit tests (run with `cargo test`)
- **CI/CD**: Automated builds and deployments on push to main

## Troubleshooting

1. **Frontend build issues**: Check Node.js version (22.x required)
2. **Backend build issues**: Verify Rust toolchain and target
3. **Database issues**: Check migration files and SQLite permissions
4. **Authentication issues**: Verify Auth0 configuration and JWT validation
