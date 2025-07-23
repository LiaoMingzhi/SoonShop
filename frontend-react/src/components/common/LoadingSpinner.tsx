import React from 'react'

interface LoadingSpinnerProps {
  size?: 'small' | 'medium' | 'large'
  color?: 'primary' | 'secondary' | 'white'
  text?: string
  className?: string
}

export const LoadingSpinner: React.FC<LoadingSpinnerProps> = ({
  size = 'medium',
  color = 'primary',
  text,
  className = '',
}) => {
  // 尺寸类名
  const sizeClasses = {
    small: 'w-4 h-4',
    medium: 'w-8 h-8',
    large: 'w-12 h-12',
  }

  // 颜色类名
  const colorClasses = {
    primary: 'border-red-500',
    secondary: 'border-gray-500',
    white: 'border-white',
  }

  // 文本尺寸
  const textSizeClasses = {
    small: 'text-sm',
    medium: 'text-base',
    large: 'text-lg',
  }

  return (
    <div className={`flex flex-col items-center justify-center ${className}`}>
      {/* 旋转加载器 */}
      <div
        className={`
          ${sizeClasses[size]}
          ${colorClasses[color]}
          border-2 border-t-transparent rounded-full animate-spin
        `}
      />
      
      {/* 加载文本 */}
      {text && (
        <p className={`mt-2 text-gray-600 ${textSizeClasses[size]}`}>
          {text}
        </p>
      )}
    </div>
  )
}

// 页面级别的加载组件
export const PageLoading: React.FC<{ text?: string }> = ({ text = '加载中...' }) => {
  return (
    <div className="min-h-screen flex items-center justify-center bg-gray-50">
      <div className="text-center">
        <LoadingSpinner size="large" text={text} />
      </div>
    </div>
  )
}

// 内容区域的加载组件
export const ContentLoading: React.FC<{ text?: string }> = ({ text = '加载中...' }) => {
  return (
    <div className="flex items-center justify-center py-12">
      <LoadingSpinner size="medium" text={text} />
    </div>
  )
}

// 按钮内的加载组件
export const ButtonLoading: React.FC = () => {
  return (
    <LoadingSpinner size="small" color="white" className="mr-2" />
  )
}

// 骨架屏组件
export const SkeletonLoading: React.FC<{ 
  lines?: number
  className?: string
}> = ({ lines = 3, className = '' }) => {
  return (
    <div className={`animate-pulse ${className}`}>
      {Array.from({ length: lines }).map((_, index) => (
        <div
          key={index}
          className={`bg-gray-200 rounded h-4 mb-2 ${
            index === lines - 1 ? 'w-3/4' : 'w-full'
          }`}
        />
      ))}
    </div>
  )
}

// 卡片骨架屏
export const CardSkeleton: React.FC<{ className?: string }> = ({ className = '' }) => {
  return (
    <div className={`bg-white rounded-lg shadow-md p-6 ${className}`}>
      <div className="animate-pulse">
        {/* 头部 */}
        <div className="flex items-center space-x-4 mb-4">
          <div className="w-10 h-10 bg-gray-200 rounded-full" />
          <div className="flex-1">
            <div className="h-4 bg-gray-200 rounded w-1/2 mb-2" />
            <div className="h-3 bg-gray-200 rounded w-1/3" />
          </div>
        </div>
        
        {/* 内容 */}
        <div className="space-y-3">
          <div className="h-4 bg-gray-200 rounded" />
          <div className="h-4 bg-gray-200 rounded" />
          <div className="h-4 bg-gray-200 rounded w-3/4" />
        </div>
        
        {/* 底部 */}
        <div className="flex justify-between items-center mt-6">
          <div className="h-8 bg-gray-200 rounded w-20" />
          <div className="h-8 bg-gray-200 rounded w-16" />
        </div>
      </div>
    </div>
  )
}

// 表格骨架屏
export const TableSkeleton: React.FC<{ 
  rows?: number
  columns?: number
  className?: string
}> = ({ rows = 5, columns = 4, className = '' }) => {
  return (
    <div className={`bg-white rounded-lg shadow-md overflow-hidden ${className}`}>
      <div className="animate-pulse">
        {/* 表头 */}
        <div className="bg-gray-50 px-6 py-4 border-b">
          <div className="grid grid-cols-4 gap-4">
            {Array.from({ length: columns }).map((_, index) => (
              <div key={index} className="h-4 bg-gray-200 rounded" />
            ))}
          </div>
        </div>
        
        {/* 表格行 */}
        {Array.from({ length: rows }).map((_, rowIndex) => (
          <div key={rowIndex} className="px-6 py-4 border-b last:border-b-0">
            <div className="grid grid-cols-4 gap-4">
              {Array.from({ length: columns }).map((_, colIndex) => (
                <div key={colIndex} className="h-4 bg-gray-200 rounded" />
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  )
} 