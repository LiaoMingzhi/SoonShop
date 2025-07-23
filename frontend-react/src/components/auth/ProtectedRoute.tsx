import React, { ReactNode } from 'react'
import { Navigate, useLocation } from 'react-router-dom'
import { useAuthStore } from '../../stores/authStore'
import { LoadingSpinner } from '../common/LoadingSpinner'

interface ProtectedRouteProps {
  children: ReactNode
  requiredRole?: string
  fallback?: ReactNode
}

export const ProtectedRoute: React.FC<ProtectedRouteProps> = ({
  children,
  requiredRole,
  fallback,
}) => {
  const location = useLocation()
  const { isAuthenticated, isLoading, user } = useAuthStore()

  // 如果正在加载，显示加载状态
  if (isLoading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <LoadingSpinner size="large" text="验证登录状态..." />
      </div>
    )
  }

  // 如果用户未认证，重定向到登录页面
  if (!isAuthenticated) {
    return <Navigate to="/login" state={{ from: location }} replace />
  }

  // 如果需要特定角色但用户角色不符合，显示无权限页面
  if (requiredRole && user?.userType !== requiredRole) {
    if (fallback) {
      return <>{fallback}</>
    }
    
    return (
      <div className="min-h-screen flex items-center justify-center bg-gray-50">
        <div className="max-w-md w-full bg-white rounded-lg shadow-lg p-6">
          <div className="text-center">
            <div className="text-6xl mb-4">🔒</div>
            <h1 className="text-2xl font-bold text-gray-900 mb-2">
              权限不足
            </h1>
            <p className="text-gray-600 mb-6">
              抱歉，您没有权限访问此页面。请联系管理员获取相应权限。
            </p>
            
            <div className="space-y-4">
              <button
                onClick={() => window.history.back()}
                className="w-full px-4 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300 transition-colors"
              >
                返回上一页
              </button>
              
              <button
                onClick={() => window.location.href = '/'}
                className="w-full px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600 transition-colors"
              >
                返回首页
              </button>
            </div>
            
            <div className="mt-6 text-sm text-gray-500">
              <p>需要权限: {requiredRole}</p>
              <p>当前权限: {user?.userType || '无'}</p>
            </div>
          </div>
        </div>
      </div>
    )
  }

  // 权限验证通过，渲染子组件
  return <>{children}</>
}

// 高阶组件版本
export const withAuth = <P extends object>(
  Component: React.ComponentType<P>,
  requiredRole?: string
) => {
  return (props: P) => (
    <ProtectedRoute requiredRole={requiredRole}>
      <Component {...props} />
    </ProtectedRoute>
  )
}

// 权限检查 Hook
export const usePermission = (requiredRole?: string) => {
  const { isAuthenticated, user } = useAuthStore()
  
  const hasPermission = React.useMemo(() => {
    if (!isAuthenticated) return false
    if (!requiredRole) return true
    return user?.userType === requiredRole
  }, [isAuthenticated, user?.userType, requiredRole])
  
  return {
    hasPermission,
    isAuthenticated,
    userRole: user?.userType,
  }
}

// 条件渲染组件
export const PermissionGate: React.FC<{
  children: ReactNode
  requiredRole?: string
  fallback?: ReactNode
}> = ({ children, requiredRole, fallback }) => {
  const { hasPermission } = usePermission(requiredRole)
  
  if (!hasPermission) {
    return fallback ? <>{fallback}</> : null
  }
  
  return <>{children}</>
} 