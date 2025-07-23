import React from 'react'

export const SystemManagementPage: React.FC = () => {
  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 py-8">
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-4">⚙️ 系统管理</h1>
          <p className="text-gray-600">系统配置和管理功能</p>
        </div>
        
        <div className="bg-white rounded-xl shadow-lg p-8">
          <div className="text-center py-12">
            <div className="text-6xl mb-4">🚧</div>
            <h2 className="text-2xl font-bold text-gray-900 mb-4">功能开发中</h2>
            <p className="text-gray-600 mb-8">
              系统管理功能正在开发中，敬请期待！
            </p>
            <button
              onClick={() => window.history.back()}
              className="px-6 py-3 bg-red-500 text-white rounded-lg hover:bg-red-600 transition-colors"
            >
              返回上一页
            </button>
          </div>
        </div>
      </div>
    </div>
  )
} 