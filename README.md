# Haasteikko
A hobby website project, created for family use + trying new tech. Designed to be tool for maintaining reading/gaming challenges and tracking progress. Fairly simple to self-host.


## Architecture

The application follows a modern full-stack architecture:

- **Frontend**: Vue 3 + TypeScript SPA with Tailwind CSS  
- **Backend**: Rust API server using Axum framework

## Monorepo Structure

This project is organized as a monorepo with both frontend and backend in the `packages/` directory:

```
haasteikko/
├── packages/
│   ├── backend/     # Rust backend (Axum)
│   └── frontend/    # Vue.js frontend  
├── package.json     # Root npm workspace
├── Cargo.toml       # Root Rust workspace
└── README.md        # Project documentation
```

## Development Setup

### Prerequisites

- Node.js (v18+)
- npm (v9+)
- Rust (stable toolchain)
- Cargo

### Installation

```bash
# Install dependencies for both frontend and backend
npm run setup
```

### Running the Application

```bash
# Start both frontend and backend simultaneously
npm run start:all

# Or start individually
npm run start:frontend  # Vue.js dev server
npm run start:backend   # Rust API server
```

### Building

```bash
# Build both frontend and backend
npm run build
```

## Package Management

### Frontend (npm workspace)

```bash
# Add a dependency to frontend
npm install --workspace=frontend package-name
```

### Backend (Cargo workspace)

```bash
# Run cargo commands from root
cargo build
cargo test
```
- **Database**: SQLite with structured migrations
- **Authentication**: Auth0 for user management and JWT validation


## Developing

### Installation
Install node and rust.
Run the commands in the related folders, should install and work from that.
```sh
npm run install
cargo build
```

For testing playwright is used, follow it's installation instructions.

### Deployment

This project uses automated deployment via github CM to a Ubuntu VM.

### Starting local server

Just run `npm run dev` and `cargo run` in corresponding folders and it should work. 

NOTE: Auth0 is used as Oauth provider so get your own service. Theoretically is works with any oauth but needs bit more work to setup.

## Licenses

- **Icons**: Material Design Icons - Apache License Version 2.0
- **Frontend**: Various open source licenses (see package.json)
- **Backend**: Rust and crate licenses

No commercial license is give for using this application, you may freely self host this application. No guarantees of stability or braking changes is given.

## Contributing

See `AGENTS.md` for AI agent guidance and `CLAUDE.md` for Claude-specific instructions.

## Support

For issues and feature requests, please open a GitHub issue.
