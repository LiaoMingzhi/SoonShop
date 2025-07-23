import React, { useState } from 'react';
import { StatsCard, FeatureCard, Card, ProductCard } from '../../../components/ui/Card';
import { Button } from '../../../components/ui/Button';
import { Breadcrumb } from '../../../components/layout/Navigation';

const VoucherManagementPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'overview' | 'publish' | 'consume' | 'manage'>('overview');

  const breadcrumbItems = [
    { label: '首页', href: '/' },
    { label: '提货券管理' }
  ];

  // 统计数据
  const statsData = [
    {
      title: '总发布量',
      value: '45,678',
      subtitle: '累计发布提货券数量',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M22 10V6a2 2 0 00-2-2H4a2 2 0 00-2 2v4c1.1 0 2 .9 2 2s-.9 2-2 2v4a2 2 0 002 2h16a2 2 0 002-2v-4c-1.1 0-2-.9-2-2s.9-2 2-2z"/>
        </svg>
      ),
      gradient: 'blue' as const,
      trend: { value: 15, label: '较上月' }
    },
    {
      title: '已使用',
      value: '23,456',
      subtitle: '消费者已使用数量',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
        </svg>
      ),
      gradient: 'green' as const,
      trend: { value: 8, label: '较上月' }
    },
    {
      title: '未使用',
      value: '22,222',
      subtitle: '待使用提货券数量',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
        </svg>
      ),
      gradient: 'orange' as const,
      trend: { value: 12, label: '较上月' }
    },
    {
      title: '参与企业',
      value: '2,834',
      subtitle: '发布提货券的企业数',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 7V3H2v18h20V7H12zM6 19H4v-2h2v2zm0-4H4v-2h2v2zm0-4H4V9h2v2zm0-4H4V5h2v2zm4 12H8v-2h2v2zm0-4H8v-2h2v2zm0-4H8V9h2v2zm0-4H8V5h2v2zm10 12h-8v-2h2v-2h-2v-2h2v-2h-2V9h8v10zm-2-8h-2v2h2v-2zm0 4h-2v2h2v-2z"/>
        </svg>
      ),
      gradient: 'purple' as const,
      trend: { value: 6, label: '较上月' }
    },
    {
      title: '成功率',
      value: '95.8%',
      subtitle: '提货券使用成功率',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M16 6l2.29 2.29-4.88 4.88-4-4L2 16.59 3.41 18l6-6 4 4 6.3-6.29L22 12V6z"/>
        </svg>
      ),
      gradient: 'pink' as const,
      trend: { value: 2, label: '较上月' }
    },
    {
      title: '倍增奖励',
      value: '1,256,789',
      subtitle: '企业获得的倍增奖励',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
        </svg>
      ),
      gradient: 'teal' as const,
      trend: { value: 25, label: '较上月' }
    }
  ];

  // 标签配置
  const tabs = [
    { id: 'overview', label: '概览', icon: '📊' },
    { id: 'publish', label: '发布提货券', icon: '🎫' },
    { id: 'consume', label: '消费者获取', icon: '🛒' },
    { id: 'manage', label: '管理中心', icon: '⚙️' }
  ];

  // 发布提货券的企业功能
  const publisherFeatures = [
    {
      title: '餐饮服务券',
      description: '餐厅、咖啡厅、食堂等餐饮服务提货券，提供优质用餐体验',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M8.1 13.34l2.83-2.83L3.91 3.5c-1.56 1.56-1.56 4.09 0 5.66l4.19 4.18zm6.78-1.81c1.53.71 3.68.21 5.27-1.38 1.91-1.91 2.28-4.65.81-6.12-1.46-1.46-4.20-1.10-6.12.81-1.59 1.59-2.09 3.74-1.38 5.27L3.7 19.87l1.41 1.41L12 14.41l6.88 6.88 1.41-1.41L13.41 13l1.47-1.47z"/>
        </svg>
      ),
      gradient: 'blue' as const,
      actionText: '立即发布'
    },
    {
      title: '医疗健康券',
      description: '医院、诊所、体检中心等医疗健康服务提货券',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M19 8h-2v3h-3v2h3v3h2v-3h3v-2h-3V8zM4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6zm16-4H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 14H8V4h12v12z"/>
        </svg>
      ),
      gradient: 'green' as const,
      actionText: '立即发布'
    },
    {
      title: '教育培训券',
      description: '学校、培训机构、在线教育等教育服务提货券',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M5 13.18v4L12 21l7-3.82v-4L12 17l-7-3.82zM12 3L1 9l11 6 9-4.91V17h2V9L12 3z"/>
        </svg>
      ),
      gradient: 'purple' as const,
      actionText: '立即发布'
    },
    {
      title: '生活服务券',
      description: '家政、维修、美容、健身等生活服务提货券',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
        </svg>
      ),
      gradient: 'orange' as const,
      actionText: '立即发布'
    }
  ];

  // 消费者可获取的提货券
  const availableVouchers = [
    {
      name: '海底捞火锅双人套餐',
      description: '价值298元的双人火锅套餐，包含锅底、菜品、饮料',
      image: '/api/placeholder/300/200',
      originalPrice: '￥298',
      badge: { text: '热门', color: 'orange' as const },
      stats: { rating: 4.8, reviews: 1234, stock: 500 }
    },
    {
      name: '全面体检套餐',
      description: '三甲医院专业体检，包含血检、心电图、B超等项目',
      image: '/api/placeholder/300/200',
      originalPrice: '￥680',
      badge: { text: '推荐', color: 'green' as const },
      stats: { rating: 4.9, reviews: 856, stock: 200 }
    },
    {
      name: '英语口语培训课',
      description: '专业外教一对一口语培训，提升英语交流能力',
      image: '/api/placeholder/300/200',
      originalPrice: '￥450',
      badge: { text: '限时', color: 'blue' as const },
      stats: { rating: 4.7, reviews: 678, stock: 150 }
    },
    {
      name: '家政保洁服务',
      description: '专业保洁人员上门服务，深度清洁家居环境',
      image: '/api/placeholder/300/200',
      originalPrice: '￥180',
      badge: { text: '新品', color: 'purple' as const },
      stats: { rating: 4.6, reviews: 432, stock: 300 }
    },
    {
      name: '五星级酒店住宿',
      description: '豪华酒店标准间一晚，含早餐和WiFi',
      image: '/api/placeholder/300/200',
      originalPrice: '￥598',
      badge: { text: '精选', color: 'orange' as const },
      stats: { rating: 4.8, reviews: 934, stock: 80 }
    },
    {
      name: '高端美容护理',
      description: '专业美容师面部护理，使用进口护肤品',
      image: '/api/placeholder/300/200',
      originalPrice: '￥380',
      badge: { text: '女士', color: 'pink' as const },
      stats: { rating: 4.9, reviews: 567, stock: 120 }
    }
  ];

  // 管理中心功能
  const managementFeatures = [
    {
      title: '提货券统计',
      description: '实时查看提货券发布、使用、剩余等详细数据统计',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM9 17H7v-7h2v7zm4 0h-2V7h2v10zm4 0h-2v-4h2v4z"/>
        </svg>
      ),
      gradient: 'blue' as const,
      actionText: '查看统计'
    },
    {
      title: '用户管理',
      description: '管理平台用户，包括企业用户和消费者用户',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
        </svg>
      ),
      gradient: 'green' as const,
      actionText: '用户管理'
    },
    {
      title: '企业认证',
      description: '企业资质认证和审核，确保提货券的真实性',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5l-9-4z"/>
        </svg>
      ),
      gradient: 'purple' as const,
      actionText: '认证管理'
    },
    {
      title: '倍增奖励',
      description: '管理企业倍增奖励机制，设置奖励倍数和条件',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
        </svg>
      ),
      gradient: 'orange' as const,
      actionText: '奖励设置'
    }
  ];

  const renderContent = () => {
    switch (activeTab) {
      case 'overview':
        return (
          <div className="space-y-16">
            {/* 统计概览 */}
            <div className="grid grid-cols-2 tablet:grid-cols-3 desktop:grid-cols-6 gap-8">
              {statsData.map((stat, index) => (
                <StatsCard
                  key={stat.title}
                  title={stat.title}
                  value={stat.value}
                  subtitle={stat.subtitle}
                  icon={stat.icon}
                  gradient={stat.gradient}
                  trend={stat.trend}
                  className="stats-animation"
                  style={{ animationDelay: `${index * 0.2}s` }}
                />
              ))}
            </div>

            {/* 实时动态 */}
            <Card variant="elevated" padding="xl">
              <h3 className="text-2xl font-bold text-gray-800 mb-8">实时动态</h3>
              <div className="space-y-6">
                <div className="flex items-center p-4 bg-blue-50 rounded-xl">
                  <div className="w-12 h-12 bg-gradient-blue rounded-xl flex items-center justify-center text-white mr-4">
                    <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                      <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4z"/>
                    </svg>
                  </div>
                  <div className="flex-1">
                    <p className="font-semibold text-gray-800">新用户注册</p>
                    <p className="text-sm text-gray-600">用户"张三"刚刚注册了平台账号</p>
                  </div>
                  <span className="text-sm text-gray-500">2分钟前</span>
                </div>
                
                <div className="flex items-center p-4 bg-green-50 rounded-xl">
                  <div className="w-12 h-12 bg-gradient-green rounded-xl flex items-center justify-center text-white mr-4">
                    <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                      <path d="M22 10V6a2 2 0 00-2-2H4a2 2 0 00-2 2v4c1.1 0 2 .9 2 2s-.9 2-2 2v4a2 2 0 002 2h16a2 2 0 002-2v-4c-1.1 0-2-.9-2-2s.9-2 2-2z"/>
                    </svg>
                  </div>
                  <div className="flex-1">
                    <p className="font-semibold text-gray-800">提货券发布</p>
                    <p className="text-sm text-gray-600">"海底捞火锅"发布了500张火锅套餐提货券</p>
                  </div>
                  <span className="text-sm text-gray-500">5分钟前</span>
                </div>
                
                <div className="flex items-center p-4 bg-purple-50 rounded-xl">
                  <div className="w-12 h-12 bg-gradient-purple rounded-xl flex items-center justify-center text-white mr-4">
                    <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                      <path d="M16 6l2.29 2.29-4.88 4.88-4-4L2 16.59 3.41 18l6-6 4 4 6.3-6.29L22 12V6z"/>
                    </svg>
                  </div>
                  <div className="flex-1">
                    <p className="font-semibold text-gray-800">倍增奖励</p>
                    <p className="text-sm text-gray-600">"美容院"获得了10倍奖励，共计3,800元</p>
                  </div>
                  <span className="text-sm text-gray-500">8分钟前</span>
                </div>
              </div>
            </Card>
          </div>
        );

      case 'publish':
        return (
          <div className="space-y-16">
            <div className="text-center">
              <h2 className="text-4xl font-bold text-gray-800 mb-6">企业发布提货券</h2>
              <p className="text-xl text-gray-600">选择您的服务类型，发布提货券</p>
            </div>
            
            <div className="grid grid-cols-1 desktop:grid-cols-2 gap-12">
              {publisherFeatures.map((feature) => (
                <FeatureCard
                  key={feature.title}
                  title={feature.title}
                  description={feature.description}
                  icon={feature.icon}
                  gradient={feature.gradient}
                  actionText={feature.actionText}
                  onAction={() => {}}
                />
              ))}
            </div>
            
            <Card variant="elevated" padding="xl">
              <h3 className="text-2xl font-bold text-gray-800 mb-8">发布流程</h3>
              <div className="grid grid-cols-1 tablet:grid-cols-4 gap-8">
                <div className="text-center">
                  <div className="w-16 h-16 bg-gradient-blue rounded-full flex items-center justify-center text-white text-2xl font-bold mx-auto mb-4">1</div>
                  <h4 className="font-semibold text-gray-800 mb-2">填写服务信息</h4>
                  <p className="text-sm text-gray-600">详细描述您的服务内容</p>
                </div>
                <div className="text-center">
                  <div className="w-16 h-16 bg-gradient-green rounded-full flex items-center justify-center text-white text-2xl font-bold mx-auto mb-4">2</div>
                  <h4 className="font-semibold text-gray-800 mb-2">设置发放数量</h4>
                  <p className="text-sm text-gray-600">确定提货券的发放数量</p>
                </div>
                <div className="text-center">
                  <div className="w-16 h-16 bg-gradient-purple rounded-full flex items-center justify-center text-white text-2xl font-bold mx-auto mb-4">3</div>
                  <h4 className="font-semibold text-gray-800 mb-2">平台审核</h4>
                  <p className="text-sm text-gray-600">等待平台审核通过</p>
                </div>
                <div className="text-center">
                  <div className="w-16 h-16 bg-gradient-orange rounded-full flex items-center justify-center text-white text-2xl font-bold mx-auto mb-4">4</div>
                  <h4 className="font-semibold text-gray-800 mb-2">正式发布</h4>
                  <p className="text-sm text-gray-600">提货券正式上线供用户获取</p>
                </div>
              </div>
            </Card>
          </div>
        );

      case 'consume':
        return (
          <div className="space-y-16">
            <div className="text-center">
              <h2 className="text-4xl font-bold text-gray-800 mb-6">消费者免费获取</h2>
              <p className="text-xl text-gray-600">选择您需要的服务，免费获取提货券</p>
            </div>
            
            <div className="grid grid-cols-1 tablet:grid-cols-2 desktop:grid-cols-3 gap-8">
              {availableVouchers.map((voucher) => (
                <ProductCard
                  key={voucher.name}
                  name={voucher.name}
                  description={voucher.description}
                  image={voucher.image}
                  originalPrice={voucher.originalPrice}
                  badge={voucher.badge}
                  stats={voucher.stats}
                  onAction={() => {}}
                />
              ))}
            </div>
            
            <Card variant="gradient" gradient="blue" padding="xl" className="text-white">
              <div className="text-center">
                <h3 className="text-3xl font-bold mb-6">使用流程</h3>
                <div className="grid grid-cols-1 tablet:grid-cols-3 gap-8">
                  <div className="text-center">
                    <div className="w-16 h-16 bg-white/20 rounded-full flex items-center justify-center text-white text-2xl font-bold mx-auto mb-4">1</div>
                    <h4 className="font-semibold mb-2">选择服务</h4>
                    <p className="text-sm opacity-80">浏览并选择您需要的服务</p>
                  </div>
                  <div className="text-center">
                    <div className="w-16 h-16 bg-white/20 rounded-full flex items-center justify-center text-white text-2xl font-bold mx-auto mb-4">2</div>
                    <h4 className="font-semibold mb-2">免费获取</h4>
                    <p className="text-sm opacity-80">点击获取，提货券自动发放</p>
                  </div>
                  <div className="text-center">
                    <div className="w-16 h-16 bg-white/20 rounded-full flex items-center justify-center text-white text-2xl font-bold mx-auto mb-4">3</div>
                    <h4 className="font-semibold mb-2">使用服务</h4>
                    <p className="text-sm opacity-80">前往服务商处使用提货券</p>
                  </div>
                </div>
              </div>
            </Card>
          </div>
        );

      case 'manage':
        return (
          <div className="space-y-16">
            <div className="text-center">
              <h2 className="text-4xl font-bold text-gray-800 mb-6">管理中心</h2>
              <p className="text-xl text-gray-600">全面管理平台运营</p>
            </div>
            
            <div className="grid grid-cols-1 desktop:grid-cols-2 gap-12">
              {managementFeatures.map((feature) => (
                <FeatureCard
                  key={feature.title}
                  title={feature.title}
                  description={feature.description}
                  icon={feature.icon}
                  gradient={feature.gradient}
                  actionText={feature.actionText}
                  onAction={() => {}}
                />
              ))}
            </div>
          </div>
        );

      default:
        return null;
    }
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* 页面头部 */}
      <div className="bg-white border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-8 py-8">
          <Breadcrumb items={breadcrumbItems} className="mb-6" />
          
          <div className="flex flex-col desktop:flex-row desktop:items-center desktop:justify-between">
            <div>
              <h1 className="text-4xl desktop:text-6xl font-bold text-gray-800 mb-4">
                提货券管理
              </h1>
              <p className="text-xl text-gray-600">
                企业发布提货券，消费者免费获取，实现共赢
              </p>
            </div>
            
            <div className="flex space-x-4 mt-6 desktop:mt-0">
              <Button variant="outline" size="lg">
                导出数据
              </Button>
              <Button 
                variant="gradient" 
                gradient="blue" 
                size="lg"
                icon={
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
                  </svg>
                }
              >
                发布提货券
              </Button>
            </div>
          </div>
        </div>
      </div>

      {/* 标签导航 */}
      <div className="bg-white border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-8">
          <div className="flex space-x-8 overflow-x-auto">
            {tabs.map((tab) => (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as any)}
                className={`flex items-center space-x-2 py-4 px-6 text-sm font-medium transition-all duration-200 border-b-2 ${
                  activeTab === tab.id
                    ? 'border-primary-500 text-primary-600'
                    : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                }`}
              >
                <span className="text-lg">{tab.icon}</span>
                <span>{tab.label}</span>
              </button>
            ))}
          </div>
        </div>
      </div>

      {/* 主要内容区域 */}
      <div className="py-16">
        <div className="max-w-7xl mx-auto px-8">
          {renderContent()}
        </div>
      </div>
    </div>
  );
};

export default VoucherManagementPage; 