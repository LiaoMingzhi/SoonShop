import type { AppConfig } from '../types'
import { ENVIRONMENT } from '@constants/index'

// 获取环境变量
const getEnvVar = (key: string, defaultValue?: string): string => {
  const value = import.meta.env[key] || defaultValue
  if (!value) {
    throw new Error(`Environment variable ${key} is not defined`)
  }
  return value
}

// 获取当前环境
const getCurrentEnvironment = (): string => {
  return import.meta.env.MODE || ENVIRONMENT.DEVELOPMENT
}

// 基础配置
const baseConfig: AppConfig = {
  apiUrl: getEnvVar('VITE_API_URL', 'http://localhost:8080'),
  wsUrl: getEnvVar('VITE_WS_URL', 'ws://localhost:8080'),
  solanaRpcUrl: getEnvVar('VITE_SOLANA_RPC_URL', 'https://api.mainnet-beta.solana.com'),
  environment: getCurrentEnvironment() as any,
  version: getEnvVar('VITE_APP_VERSION', '1.0.0'),
  features: {
    enableB2B: getEnvVar('VITE_ENABLE_B2B', 'true') === 'true',
    enableRestaurant: getEnvVar('VITE_ENABLE_RESTAURANT', 'true') === 'true',
    enableHealthcare: getEnvVar('VITE_ENABLE_HEALTHCARE', 'true') === 'true',
    enableHousing: getEnvVar('VITE_ENABLE_HOUSING', 'true') === 'true',
    enableAnalytics: getEnvVar('VITE_ENABLE_ANALYTICS', 'true') === 'true',
  },
}

// 环境特定配置
const environmentConfigs = {
  [ENVIRONMENT.DEVELOPMENT]: {
    ...baseConfig,
    apiUrl: 'http://localhost:8080',
    wsUrl: 'ws://localhost:8080',
    solanaRpcUrl: 'https://api.devnet.solana.com',
  },
  [ENVIRONMENT.STAGING]: {
    ...baseConfig,
    apiUrl: 'https://staging-api.soonshop.com',
    wsUrl: 'wss://staging-api.soonshop.com',
    solanaRpcUrl: 'https://api.testnet.solana.com',
  },
  [ENVIRONMENT.PRODUCTION]: {
    ...baseConfig,
    apiUrl: 'https://api.soonshop.com',
    wsUrl: 'wss://api.soonshop.com',
    solanaRpcUrl: 'https://api.mainnet-beta.solana.com',
  },
}

// 导出当前环境配置
export const config: AppConfig = environmentConfigs[getCurrentEnvironment()] || environmentConfigs[ENVIRONMENT.DEVELOPMENT]

// 调试信息
if (getCurrentEnvironment() === ENVIRONMENT.DEVELOPMENT) {
  console.log('🚀 SoonShop Application Config:', config)
}

// 导出工具函数
export const isDevelopment = () => config.environment === ENVIRONMENT.DEVELOPMENT
export const isProduction = () => config.environment === ENVIRONMENT.PRODUCTION
export const isStaging = () => config.environment === ENVIRONMENT.STAGING

// 功能开关检查
export const isFeatureEnabled = (feature: keyof AppConfig['features']): boolean => {
  return config.features[feature] ?? false
}

// API URL 构建器
export const buildApiUrl = (endpoint: string): string => {
  const baseUrl = config.apiUrl.replace(/\/$/, '') // 移除末尾斜杠
  const cleanEndpoint = endpoint.replace(/^\//, '') // 移除开头斜杠
  return `${baseUrl}/${cleanEndpoint}`
}

// WebSocket URL 构建器
export const buildWsUrl = (endpoint: string): string => {
  const baseUrl = config.wsUrl.replace(/\/$/, '')
  const cleanEndpoint = endpoint.replace(/^\//, '')
  return `${baseUrl}/${cleanEndpoint}`
}

// 默认导出配置
export default config 