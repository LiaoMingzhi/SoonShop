import React from 'react'

export const B2BMarketplacePage: React.FC = () => {
  return (
    <div className="min-h-screen bg-gray-50">
      <div className="max-w-7xl mx-auto px-4 py-8">
        <div className="text-center mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-4">🏢 B2B市场</h1>
          <p className="text-gray-600">企业间的商业合作平台</p>
        </div>
        
        <div className="bg-white rounded-xl shadow-lg p-8">
          <div className="text-center py-12">
            <div className="text-6xl mb-4">🚧</div>
            <h2 className="text-2xl font-bold text-gray-900 mb-4">功能开发中</h2>
            <p className="text-gray-600 mb-8">
              B2B市场正在开发中，敬请期待！
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