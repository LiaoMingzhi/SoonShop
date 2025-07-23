import React from 'react';
import { Link } from 'react-router-dom';

export const Footer: React.FC = () => {
  return (
    <footer className="bg-gray-800 text-white py-12 mt-16">
      <div className="max-w-7xl mx-auto px-4 tablet:px-6 desktop:px-8">
        <div className="grid grid-cols-1 tablet:grid-cols-2 desktop:grid-cols-4 gap-8">
          {/* 品牌区域 */}
          <div>
            <div className="flex items-center mb-4">
              <div className="bg-gradient-to-br from-blue-500 to-purple-600 w-10 h-10 rounded-lg flex items-center justify-center mr-3">
                <svg className="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M20 7h-3V6a4 4 0 00-8 0v1H6a1 1 0 00-1 1v11a3 3 0 003 3h8a3 3 0 003-3V8a1 1 0 00-1-1zM11 6a2 2 0 014 0v1h-4V6zm7 13a1 1 0 01-1 1H8a1 1 0 01-1-1V9h2v1a1 1 0 002 0V9h4v1a1 1 0 002 0V9h2v10z"/>
                </svg>
              </div>
              <span className="text-xl font-bold">SoonShop</span>
            </div>
            <p className="text-gray-400 text-sm">
              基于共产主义经济原理的商业平台，致力于构建公平、公正、共同富裕的经济体系。
            </p>
          </div>
          
          {/* 核心功能 */}
          <div>
            <h4 className="font-bold mb-4">核心功能</h4>
            <div className="space-y-2 text-sm text-gray-400">
              <div><Link to="/vouchers" className="hover:text-white transition-colors duration-200">提货券管理</Link></div>
              <div><Link to="/consumption" className="hover:text-white transition-colors duration-200">消费与倍增</Link></div>
              <div><Link to="/b2c" className="hover:text-white transition-colors duration-200">B2C电商平台</Link></div>
              <div><Link to="/evaluation" className="hover:text-white transition-colors duration-200">企业评估</Link></div>
            </div>
          </div>
          
          {/* 应用场景 */}
          <div>
            <h4 className="font-bold mb-4">应用场景</h4>
            <div className="space-y-2 text-sm text-gray-400">
              <div>餐饮服务</div>
              <div>医疗健康</div>
              <div>住房服务</div>
              <div>教育培训</div>
            </div>
          </div>
          
          {/* 联系我们 */}
          <div>
            <h4 className="font-bold mb-4">联系我们</h4>
            <div className="space-y-2 text-sm text-gray-400">
              <div>邮箱：admin@soonshop.com</div>
              <div>电话：400-888-0000</div>
              <div>地址：北京市朝阳区</div>
            </div>
          </div>
        </div>
        
        {/* 版权信息 */}
        <div className="border-t border-gray-700 mt-8 pt-8 text-center text-sm text-gray-400">
          <p>&copy; {new Date().getFullYear()} SoonShop. 保留所有权利。基于共产主义经济原理构建。</p>
        </div>
      </div>
    </footer>
  );
};

export default Footer; 