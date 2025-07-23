import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import type { User, WalletState } from '../types/index'
import { STORAGE_KEYS } from '../constants/index'

// 认证状态接口
interface AuthState {
  // 用户信息
  user: User | null
  // 钱包状态
  wallet: WalletState
  // 认证状态
  isAuthenticated: boolean
  isLoading: boolean
  // 权限列表
  permissions: string[]
  // JWT令牌
  accessToken: string | null
  refreshToken: string | null
  
  // 操作方法
  setUser: (user: User | null) => void
  setWallet: (wallet: Partial<WalletState>) => void
  setTokens: (accessToken: string, refreshToken: string) => void
  setLoading: (loading: boolean) => void
  setPermissions: (permissions: string[]) => void
  login: (user: User, accessToken: string, refreshToken: string, permissions: string[]) => void
  logout: () => void
  updateProfile: (profile: Partial<User['profile']>) => void
  
  // 权限检查
  hasPermission: (permission: string) => boolean
  hasRole: (role: string) => boolean
  isAdmin: () => boolean
  isProducer: () => boolean
  isConsumer: () => boolean
  isEvaluator: () => boolean
}

// 初始钱包状态
const initialWalletState: WalletState = {
  connected: false,
  address: null,
  balance: 0,
  chainId: 0,
  provider: null,
}

// 创建认证store
export const useAuthStore = create<AuthState>()(
  persist(
    (set, get) => ({
      // 初始状态
      user: null,
      wallet: initialWalletState,
      isAuthenticated: false,
      isLoading: false,
      permissions: [],
      accessToken: null,
      refreshToken: null,

      // 设置用户信息
      setUser: (user) => {
        set({ 
          user, 
          isAuthenticated: !!user 
        })
      },

      // 设置钱包状态
      setWallet: (walletUpdate) => {
        set((state) => ({
          wallet: { ...state.wallet, ...walletUpdate }
        }))
      },

      // 设置令牌
      setTokens: (accessToken, refreshToken) => {
        set({ accessToken, refreshToken })
      },

      // 设置加载状态
      setLoading: (loading) => {
        set({ isLoading: loading })
      },

      // 设置权限
      setPermissions: (permissions) => {
        set({ permissions })
      },

      // 登录
      login: (user, accessToken, refreshToken, permissions) => {
        set({
          user,
          accessToken,
          refreshToken,
          permissions,
          isAuthenticated: true,
          isLoading: false,
        })
      },

      // 登出
      logout: () => {
        set({
          user: null,
          accessToken: null,
          refreshToken: null,
          permissions: [],
          isAuthenticated: false,
          isLoading: false,
          wallet: initialWalletState,
        })
        
        // 清除本地存储
        localStorage.removeItem(STORAGE_KEYS.ACCESS_TOKEN)
        localStorage.removeItem(STORAGE_KEYS.REFRESH_TOKEN)
        localStorage.removeItem(STORAGE_KEYS.USER_PROFILE)
        localStorage.removeItem(STORAGE_KEYS.WALLET_ADDRESS)
      },

      // 更新用户资料
      updateProfile: (profileUpdate) => {
        const { user } = get()
        if (user) {
          set({
            user: {
              ...user,
              profile: { ...user.profile, ...profileUpdate }
            }
          })
        }
      },

      // 权限检查
      hasPermission: (permission) => {
        const { permissions } = get()
        return permissions.includes(permission) || permissions.includes('*')
      },

      // 角色检查
      hasRole: (role) => {
        const { user } = get()
        return user?.userType === role
      },

      // 管理员检查
      isAdmin: () => {
        const { user } = get()
        return user?.userType === 'admin'
      },

      // 生产者检查
      isProducer: () => {
        const { user } = get()
        return user?.userType === 'producer'
      },

      // 消费者检查
      isConsumer: () => {
        const { user } = get()
        return user?.userType === 'consumer'
      },

      // 评估员检查
      isEvaluator: () => {
        const { user } = get()
        return user?.userType === 'evaluator'
      },
    }),
    {
      name: 'auth-storage',
      partialize: (state) => ({
        user: state.user,
        accessToken: state.accessToken,
        refreshToken: state.refreshToken,
        permissions: state.permissions,
        wallet: {
          address: state.wallet.address,
          connected: state.wallet.connected,
        },
      }),
    }
  )
)

// 导出选择器
export const useAuth = () => {
  const store = useAuthStore()
  return {
    user: store.user,
    wallet: store.wallet,
    isAuthenticated: store.isAuthenticated,
    isLoading: store.isLoading,
    permissions: store.permissions,
    hasPermission: store.hasPermission,
    hasRole: store.hasRole,
    isAdmin: store.isAdmin,
    isProducer: store.isProducer,
    isConsumer: store.isConsumer,
    isEvaluator: store.isEvaluator,
  }
}

// 导出操作方法
export const useAuthActions = () => {
  const store = useAuthStore()
  return {
    setUser: store.setUser,
    setWallet: store.setWallet,
    setTokens: store.setTokens,
    setLoading: store.setLoading,
    setPermissions: store.setPermissions,
    login: store.login,
    logout: store.logout,
    updateProfile: store.updateProfile,
  }
} 