/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_API_BASE_URL: string
  readonly VITE_SOLANA_RPC_URL: string
  readonly VITE_WS_URL: string
  readonly MODE: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
} 