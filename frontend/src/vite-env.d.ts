/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_API_URL: string
  readonly VITE_CLIENT_ID: string
  readonly VITE_TENANT_ID: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}
