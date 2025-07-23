import React, { useState } from 'react';
import { StatsCard, FeatureCard, Card, ProductCard } from '../../../components/ui/Card';
import { Button } from '../../../components/ui/Button';
import { Breadcrumb } from '../../../components/layout/Navigation';

const B2CMarketplacePage: React.FC = () => {
  const [selectedCategory, setSelectedCategory] = useState<string>('all');

  const breadcrumbItems = [
    { label: '首页', href: '/' },
    { label: 'B2C电商平台' }
  ];

  // 统计数据
  const statsData = [
    {
      title: '在线商品',
      value: '15,678',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M7 18c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2zM1 2v2h2l3.6 7.59-1.35 2.45c-.16.28-.25.61-.25.96 0 1.1.9 2 2 2h12v-2H7.42c-.14 0-.25-.11-.25-.25l.03-.12L8.1 13h7.45c.75 0 1.41-.41 1.75-1.03L21.7 4H5.21l-.94-2H1zm16 16c-1.1 0-2 .9-2 2s.9 2 2 2 2-.9 2-2-.9-2-2-2z"/>
        </svg>
      ),
      gradient: 'blue' as const
    },
    {
      title: '注册用户',
      value: '12,456',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 12c2.21 0 4-1.79 4-4s-1.79-4-4-4-4 1.79-4 4 1.79 4 4 4zm0 2c-2.67 0-8 1.34-8 4v2h16v-2c0-2.66-5.33-4-8-4z"/>
        </svg>
      ),
      gradient: 'green' as const
    },
    {
      title: '成功订单',
      value: '8,945',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
        </svg>
      ),
      gradient: 'purple' as const
    },
    {
      title: '合作商家',
      value: '1,234',
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 7V3H2v18h20V7H12zM6 19H4v-2h2v2zm0-4H4v-2h2v2zm0-4H4V9h2v2zm0-4H4V5h2v2zm4 12H8v-2h2v2zm0-4H8v-2h2v2zm0-4H8V9h2v2zm0-4H8V5h2v2zm10 12h-8v-2h2v-2h-2v-2h2v-2h-2V9h8v10zm-2-8h-2v2h2v-2zm0 4h-2v2h2v-2z"/>
        </svg>
      ),
      gradient: 'orange' as const
    }
  ];

  // 商品分类
  const categories = [
    {
      title: '餐饮美食',
      description: '精选餐厅提货券，免费享用美食',
      count: 156,
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M8.1 13.34l2.83-2.83L3.91 3.5c-1.56 1.56-1.56 4.09 0 5.66l4.19 4.18zm6.78-1.81c1.53.71 3.68.21 5.27-1.38 1.91-1.91 2.28-4.65.81-6.12-1.46-1.46-4.20-1.10-6.12.81-1.59 1.59-2.09 3.74-1.38 5.27L3.7 19.87l1.41 1.41L12 14.41l6.88 6.88 1.41-1.41L13.41 13l1.47-1.47z"/>
        </svg>
      ),
      gradient: 'blue' as const
    },
    {
      title: '生活服务',
      description: '家政、维修、美容等服务券',
      count: 89,
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
        </svg>
      ),
      gradient: 'green' as const
    },
    {
      title: '健康医疗',
      description: '医院、体检、健身等服务券',
      count: 67,
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M19 8h-2v3h-3v2h3v3h2v-3h3v-2h-3V8zM4 6H2v14c0 1.1.9 2 2 2h14v-2H4V6zm16-4H8c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h12c1.1 0 2-.9 2-2V4c0-1.1-.9-2-2-2zm0 14H8V4h12v12z"/>
        </svg>
      ),
      gradient: 'orange' as const
    },
    {
      title: '教育培训',
      description: '学校、培训机构等教育券',
      count: 234,
      icon: (
        <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
          <path d="M5 13.18v4L12 21l7-3.82v-4L12 17l-7-3.82zM12 3L1 9l11 6 9-4.91V17h2V9L12 3z"/>
        </svg>
      ),
      gradient: 'purple' as const
    },
    { id: 'entertainment', name: '娱乐休闲', count: 1234 },
    { id: 'housing', name: '住房服务', count: 890 },
    { id: 'transport', name: '交通出行', count: 3296 }
  ];

  // 热门商品
  const featuredProducts = [
    {
      name: '海底捞火锅双人套餐',
      description: '价值298元的双人火锅套餐，包含锅底、菜品、饮料，免费享用',
      image: '/api/placeholder/300/200',
      originalPrice: '￥298',
      badge: { text: '热门', color: 'orange' as const },
      stats: { rating: 4.8, reviews: 1234, stock: 500 }
    },
    {
      name: '全面体检套餐',
      description: '三甲医院专业体检，包含血检、心电图、B超等项目，关爱健康',
      image: '/api/placeholder/300/200',
      originalPrice: '￥680',
      badge: { text: '推荐', color: 'green' as const },
      stats: { rating: 4.9, reviews: 856, stock: 200 }
    },
    {
      name: '英语口语培训课',
      description: '专业外教一对一口语培训，提升英语交流能力，免费学习',
      image: '/api/placeholder/300/200',
      originalPrice: '￥450',
      badge: { text: '限时', color: 'blue' as const },
      stats: { rating: 4.7, reviews: 678, stock: 150 }
    },
    {
      name: '家政保洁服务',
      description: '专业保洁人员上门服务，深度清洁家居环境，品质生活',
      image: '/api/placeholder/300/200',
      originalPrice: '￥180',
      badge: { text: '新品', color: 'purple' as const },
      stats: { rating: 4.6, reviews: 432, stock: 300 }
    },
    {
      name: '五星级酒店住宿',
      description: '豪华酒店标准间一晚，含早餐和WiFi，舒适体验',
      image: '/api/placeholder/300/200',
      originalPrice: '￥598',
      badge: { text: '精选', color: 'orange' as const },
      stats: { rating: 4.8, reviews: 934, stock: 80 }
    },
    {
      name: '高端美容护理',
      description: '专业美容师面部护理，使用进口护肤品，焕发美丽',
      image: '/api/placeholder/300/200',
      originalPrice: '￥380',
      badge: { text: '女士', color: 'pink' as const },
      stats: { rating: 4.9, reviews: 567, stock: 120 }
    }
  ];

  return (
    <div className="min-h-screen bg-gray-50">
      {/* 页面头部 */}
      <div className="bg-white border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-8 py-8">
          <Breadcrumb items={breadcrumbItems} className="mb-6" />
          
          <div className="flex flex-col desktop:flex-row desktop:items-center desktop:justify-between">
            <div>
              <h1 className="text-4xl desktop:text-6xl font-bold text-gray-800 mb-4">
                B2C电商平台
              </h1>
              <p className="text-xl text-gray-600">
                免费获取提货券，享受优质商品和服务
              </p>
            </div>
            
            <div className="flex space-x-4 mt-6 desktop:mt-0">
              <Button variant="outline" size="lg">
                商家入驻
              </Button>
              <Button 
                variant="gradient" 
                gradient="blue" 
                size="lg"
                icon={
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 3h2l.4 2M7 13h10l4-8H5.4m0 0L7 13m0 0l-2.5 5L17 18M9 19.5a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zM20 19.5a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0z" />
                  </svg>
                }
              >
                立即购物
              </Button>
            </div>
          </div>
        </div>
      </div>

      {/* 统计数据 */}
      <section className="py-16">
        <div className="max-w-7xl mx-auto px-8">
          <div className="grid grid-cols-2 desktop:grid-cols-4 gap-8">
            {statsData.map((stat, index) => (
              <StatsCard
                key={stat.title}
                title={stat.title}
                value={stat.value}
                icon={stat.icon}
                gradient={stat.gradient}
                className="stats-animation"
                style={{ animationDelay: `${index * 0.2}s` }}
              />
            ))}
          </div>
        </div>
      </section>

      {/* 商品分类 */}
      <section className="py-16">
        <div className="max-w-7xl mx-auto px-8">
          <h2 className="text-3xl font-bold text-gray-800 mb-12 text-center">商品分类</h2>
          
          <div className="grid grid-cols-2 tablet:grid-cols-4 desktop:grid-cols-8 gap-6">
            {categories.map((category) => (
              <Card
                key={category.id}
                variant="elevated"
                padding="lg"
                hover
                className={`text-center cursor-pointer ${
                  selectedCategory === category.id ? 'ring-2 ring-primary-500' : ''
                }`}
                onClick={() => setSelectedCategory(category.id)}
              >
                <h3 className="font-semibold text-gray-800 mb-2">{category.name}</h3>
                <p className="text-sm text-gray-600">{category.count.toLocaleString()}</p>
              </Card>
            ))}
          </div>
        </div>
      </section>

      {/* 热门商品 */}
      <section className="py-16">
        <div className="max-w-7xl mx-auto px-8">
          <div className="text-center mb-12">
            <h2 className="text-3xl font-bold text-gray-800 mb-4">热门商品</h2>
            <p className="text-lg text-gray-600">精选优质商品，免费获取提货券</p>
          </div>
          
          <div className="grid grid-cols-1 tablet:grid-cols-2 desktop:grid-cols-3 gap-8">
            {featuredProducts.map((product) => (
              <ProductCard
                key={product.name}
                name={product.name}
                description={product.description}
                image={product.image}
                originalPrice={product.originalPrice}
                badge={product.badge}
                stats={product.stats}
                onAction={() => {}}
              />
            ))}
          </div>
        </div>
      </section>

      {/* 平台优势 */}
      <section className="py-16">
        <div className="max-w-7xl mx-auto px-8">
          <Card variant="gradient" gradient="purple" padding="xl" className="text-white">
            <div className="text-center mb-12">
              <h2 className="text-3xl font-bold mb-4">平台优势</h2>
              <p className="text-xl opacity-90">共产主义商业模式的独特优势</p>
            </div>
            
            <div className="grid grid-cols-1 desktop:grid-cols-3 gap-12">
              <div className="text-center">
                <div className="w-16 h-16 bg-white/20 rounded-full flex items-center justify-center mx-auto mb-6">
                  <svg className="w-8 h-8" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/>
                  </svg>
                </div>
                <h3 className="text-xl font-bold mb-4">完全免费</h3>
                <p className="opacity-80">所有商品和服务通过提货券免费获取，无需支付任何费用</p>
              </div>
              
              <div className="text-center">
                <div className="w-16 h-16 bg-white/20 rounded-full flex items-center justify-center mx-auto mb-6">
                  <svg className="w-8 h-8" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M16 6l2.29 2.29-4.88 4.88-4-4L2 16.59 3.41 18l6-6 4 4 6.3-6.29L22 12V6z"/>
                  </svg>
                </div>
                <h3 className="text-xl font-bold mb-4">品质保证</h3>
                <p className="opacity-80">严格筛选优质商家，确保商品和服务的高品质标准</p>
              </div>
              
              <div className="text-center">
                <div className="w-16 h-16 bg-white/20 rounded-full flex items-center justify-center mx-auto mb-6">
                  <svg className="w-8 h-8" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M17 21v-2a4 4 0 00-4-4H5a4 4 0 00-4 4v2m16-13a4 4 0 11-8 0 4 4 0 018 0zm-8 0a4 4 0 11-8 0 4 4 0 018 0z"/>
                  </svg>
                </div>
                <h3 className="text-xl font-bold mb-4">共享经济</h3>
                <p className="opacity-80">通过共享模式实现资源优化配置，促进社会共同发展</p>
              </div>
            </div>
          </Card>
        </div>
      </section>
    </div>
  );
};

export default B2CMarketplacePage; 