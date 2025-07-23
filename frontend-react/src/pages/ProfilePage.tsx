import React from 'react';

const ProfilePage: React.FC = () => {
  return (
    <div className="min-h-screen bg-gray-50">
      {/* 页面头部 */}
      <div className="bg-white shadow-sm border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="text-2xl font-bold text-gray-800 flex items-center">
                <i className="fas fa-user text-indigo-600 mr-3"></i>
                个人中心
              </h1>
              <p className="text-gray-600 mt-1">管理您的个人资料和账户设置</p>
            </div>
            <button className="btn-primary text-white px-4 py-2 rounded-lg font-medium">
              <i className="fas fa-edit mr-2"></i>
              编辑资料
            </button>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <div className="text-center py-16">
          <i className="fas fa-user-circle text-6xl text-gray-400 mb-4"></i>
          <h2 className="text-2xl font-bold text-gray-800 mb-4">个人中心开发中</h2>
          <p className="text-gray-600 mb-8">
            用户个人中心功能正在开发中，敬请期待！
          </p>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 max-w-4xl mx-auto">
            <div className="p-6 bg-white rounded-lg shadow-sm">
              <i className="fas fa-id-card text-blue-600 text-3xl mb-3"></i>
              <h3 className="font-bold text-gray-800 mb-2">个人资料</h3>
              <p className="text-sm text-gray-600">管理基本信息</p>
            </div>
            <div className="p-6 bg-white rounded-lg shadow-sm">
              <i className="fas fa-history text-green-600 text-3xl mb-3"></i>
              <h3 className="font-bold text-gray-800 mb-2">历史记录</h3>
              <p className="text-sm text-gray-600">查看使用记录</p>
            </div>
            <div className="p-6 bg-white rounded-lg shadow-sm">
              <i className="fas fa-cog text-purple-600 text-3xl mb-3"></i>
              <h3 className="font-bold text-gray-800 mb-2">账户设置</h3>
              <p className="text-sm text-gray-600">安全和隐私设置</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ProfilePage; 