import React, { useState } from 'react';

const RestaurantPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState('restaurants');
  const [searchTerm, setSearchTerm] = useState('');
  const [filterCategory, setFilterCategory] = useState('all');

  const restaurants = [
    {
      id: 1,
      name: '绿色有机餐厅',
      category: '健康餐饮',
      image: 'https://images.unsplash.com/photo-1517248135467-4c7edcad34c4?w=400&h=300&fit=crop',
      rating: 4.8,
      reviews: 234,
      address: '朝阳区三里屯',
      phone: '010-12345678',
      openTime: '09:00-22:00',
      deliveryTime: '30-45分钟',
      deliveryFee: 5,
      minOrder: 30,
      features: ['有机食材', '环保包装', '营养标识'],
      specialties: ['有机沙拉', '全麦汉堡', '鲜榨果汁'],
      status: 'open'
    },
    {
      id: 2,
      name: '公平贸易咖啡馆',
      category: '咖啡饮品',
      image: 'https://images.unsplash.com/photo-1501339847302-ac426a4a7cbb?w=400&h=300&fit=crop',
      rating: 4.6,
      reviews: 156,
      address: '海淀区中关村',
      phone: '010-87654321',
      openTime: '07:00-21:00',
      deliveryTime: '20-35分钟',
      deliveryFee: 3,
      minOrder: 20,
      features: ['公平贸易', '现烘咖啡', '手工制作'],
      specialties: ['精品咖啡', '手工甜点', '素食轻食'],
      status: 'open'
    },
    {
      id: 3,
      name: '素食养生馆',
      category: '素食餐厅',
      image: 'https://images.unsplash.com/photo-1565299624946-b28f40a0ca4b?w=400&h=300&fit=crop',
      rating: 4.9,
      reviews: 189,
      address: '西城区金融街',
      phone: '010-11223344',
      openTime: '11:00-20:00',
      deliveryTime: '40-60分钟',
      deliveryFee: 8,
      minOrder: 50,
      features: ['纯素食', '养生配方', '本地食材'],
      specialties: ['养生汤品', '素食火锅', '时令蔬菜'],
      status: 'busy'
    },
    {
      id: 4,
      name: '循环经济食堂',
      category: '快餐简餐',
      image: 'https://images.unsplash.com/photo-1555939594-58d7cb561ad1?w=400&h=300&fit=crop',
      rating: 4.5,
      reviews: 98,
      address: '丰台区科技园',
      phone: '010-99887766',
      openTime: '10:00-21:00',
      deliveryTime: '25-40分钟',
      deliveryFee: 0,
      minOrder: 25,
      features: ['零废弃', '可回收包装', '本地采购'],
      specialties: ['健康套餐', '季节菜品', '营养汤品'],
      status: 'closed'
    }
  ];

  const recentOrders = [
    {
      id: 1,
      restaurant: '绿色有机餐厅',
      customer: '张三',
      items: ['有机沙拉', '全麦汉堡'],
      amount: 68.50,
      time: '12:30',
      status: 'preparing',
      estimatedTime: '15分钟'
    },
    {
      id: 2,
      restaurant: '公平贸易咖啡馆',
      customer: '李四',
      items: ['精品咖啡', '手工甜点'],
      amount: 45.00,
      time: '13:15',
      status: 'ready',
      estimatedTime: '已完成'
    },
    {
      id: 3,
      restaurant: '素食养生馆',
      customer: '王五',
      items: ['养生汤品', '时令蔬菜'],
      amount: 89.00,
      time: '13:45',
      status: 'delivered',
      estimatedTime: '已送达'
    }
  ];

  const serviceStats = [
    {
      title: '今日订单',
      value: '156',
      change: '+12.5%',
      icon: 'fas fa-shopping-bag',
      color: 'stats-card'
    },
    {
      title: '活跃餐厅',
      value: '24',
      change: '+8.3%',
      icon: 'fas fa-store',
      color: 'icon-gradient-green'
    },
    {
      title: '配送中',
      value: '38',
      change: '+15.6%',
      icon: 'fas fa-truck',
      color: 'icon-gradient-orange'
    },
    {
      title: '客户满意度',
      value: '4.7',
      change: '+2.1%',
      icon: 'fas fa-star',
      color: 'icon-gradient-purple'
    }
  ];

  const filteredRestaurants = restaurants.filter(restaurant => {
    const matchesSearch = restaurant.name.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesCategory = filterCategory === 'all' || restaurant.category === filterCategory;
    return matchesSearch && matchesCategory;
  });

  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'open':
        return <span className="bg-green-100 text-green-800 px-3 py-1 rounded-full text-sm font-medium">营业中</span>;
      case 'busy':
        return <span className="bg-yellow-100 text-yellow-800 px-3 py-1 rounded-full text-sm font-medium">繁忙</span>;
      case 'closed':
        return <span className="bg-red-100 text-red-800 px-3 py-1 rounded-full text-sm font-medium">已打烊</span>;
      default:
        return <span className="bg-gray-100 text-gray-800 px-3 py-1 rounded-full text-sm font-medium">未知</span>;
    }
  };

  const getOrderStatusBadge = (status: string) => {
    switch (status) {
      case 'preparing':
        return <span className="bg-orange-100 text-orange-800 px-2 py-1 rounded-full text-xs font-medium">制作中</span>;
      case 'ready':
        return <span className="bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs font-medium">待取餐</span>;
      case 'delivered':
        return <span className="bg-blue-100 text-blue-800 px-2 py-1 rounded-full text-xs font-medium">已送达</span>;
      default:
        return <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs font-medium">未知</span>;
    }
  };

  const renderStars = (rating: number) => {
    return [...Array(5)].map((_, i) => (
      <i key={i} className={`fas fa-star ${i < Math.floor(rating) ? 'text-yellow-400' : 'text-gray-300'}`}></i>
    ));
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* 页面标题 */}
      <div className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center">
              <i className="fas fa-utensils text-3xl text-orange-600 mr-4"></i>
              <div>
                <h1 className="text-3xl font-bold text-gray-800">餐饮场景服务</h1>
                <p className="text-gray-600 mt-1">绿色餐饮，健康生活，支持可持续发展的餐厅</p>
              </div>
            </div>
            <div className="flex space-x-3">
              <button
                onClick={() => setActiveTab('restaurants')}
                className={`px-4 py-2 rounded-lg font-medium transition-colors ${
                  activeTab === 'restaurants'
                    ? 'bg-orange-500 text-white'
                    : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                }`}
              >
                <i className="fas fa-store mr-2"></i>
                餐厅列表
              </button>
              <button
                onClick={() => setActiveTab('orders')}
                className={`px-4 py-2 rounded-lg font-medium transition-colors ${
                  activeTab === 'orders'
                    ? 'bg-orange-500 text-white'
                    : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                }`}
              >
                <i className="fas fa-list mr-2"></i>
                订单管理
              </button>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* 服务统计 */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          {serviceStats.map((stat, index) => (
            <div key={index} className={`${stat.color} rounded-2xl p-6 text-white`}>
              <div className="flex items-center justify-between mb-4">
                <div className="bg-white/20 w-12 h-12 rounded-xl flex items-center justify-center">
                  <i className={`${stat.icon} text-xl`}></i>
                </div>
                <span className="text-green-200 text-sm font-medium">{stat.change}</span>
              </div>
              <h3 className="text-white/80 text-sm mb-2">{stat.title}</h3>
              <p className="text-3xl font-bold">{stat.value}</p>
            </div>
          ))}
        </div>

        {activeTab === 'restaurants' && (
          <>
            {/* 搜索和筛选 */}
            <div className="bg-white rounded-2xl shadow-sm p-6 mb-8">
              <div className="flex flex-col md:flex-row gap-4">
                <div className="flex-1">
                  <div className="relative">
                    <i className="fas fa-search absolute left-3 top-3 text-gray-400"></i>
                    <input
                      type="text"
                      placeholder="搜索餐厅名称..."
                      value={searchTerm}
                      onChange={(e) => setSearchTerm(e.target.value)}
                      className="w-full pl-10 pr-4 py-3 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent transition-all duration-200"
                    />
                  </div>
                </div>
                <div className="flex gap-3">
                  <select
                    value={filterCategory}
                    onChange={(e) => setFilterCategory(e.target.value)}
                    className="px-4 py-3 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-orange-500 focus:border-transparent bg-white"
                  >
                    <option value="all">全部分类</option>
                    <option value="健康餐饮">健康餐饮</option>
                    <option value="咖啡饮品">咖啡饮品</option>
                    <option value="素食餐厅">素食餐厅</option>
                    <option value="快餐简餐">快餐简餐</option>
                  </select>
                  <button className="px-6 py-3 bg-orange-500 text-white rounded-xl hover:bg-orange-600 transition-colors">
                    <i className="fas fa-filter mr-2"></i>
                    筛选
                  </button>
                </div>
              </div>
            </div>

            {/* 餐厅列表 */}
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
              {filteredRestaurants.map((restaurant) => (
                <div key={restaurant.id} className="bg-white rounded-2xl shadow-sm overflow-hidden card-hover">
                  <div className="relative h-48">
                    <img
                      src={restaurant.image}
                      alt={restaurant.name}
                      className="w-full h-full object-cover"
                    />
                    <div className="absolute top-4 right-4">
                      {getStatusBadge(restaurant.status)}
                    </div>
                    <div className="absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/60 to-transparent p-4">
                      <h3 className="text-white text-xl font-bold">{restaurant.name}</h3>
                      <p className="text-white/80 text-sm">{restaurant.category}</p>
                    </div>
                  </div>
                  
                  <div className="p-6">
                    <div className="flex items-center justify-between mb-4">
                      <div className="flex items-center">
                        <div className="flex mr-2">
                          {renderStars(restaurant.rating)}
                        </div>
                        <span className="text-sm text-gray-600">{restaurant.rating} ({restaurant.reviews})</span>
                      </div>
                      <div className="text-sm text-gray-600">
                        <i className="fas fa-clock mr-1"></i>
                        {restaurant.deliveryTime}
                      </div>
                    </div>
                    
                    <div className="grid grid-cols-2 gap-4 mb-4">
                      <div>
                        <p className="text-sm text-gray-600">营业时间</p>
                        <p className="font-medium text-gray-800">{restaurant.openTime}</p>
                      </div>
                      <div>
                        <p className="text-sm text-gray-600">起送金额</p>
                        <p className="font-medium text-gray-800">¥{restaurant.minOrder}</p>
                      </div>
                    </div>
                    
                    <div className="mb-4">
                      <p className="text-sm text-gray-600 mb-2">特色标签</p>
                      <div className="flex flex-wrap gap-2">
                        {restaurant.features.map((feature, index) => (
                          <span key={index} className="bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs font-medium">
                            {feature}
                          </span>
                        ))}
                      </div>
                    </div>
                    
                    <div className="mb-4">
                      <p className="text-sm text-gray-600 mb-2">招牌菜品</p>
                      <p className="text-sm text-gray-800">{restaurant.specialties.join(' • ')}</p>
                    </div>
                    
                    <div className="flex items-center justify-between mb-4">
                      <div className="flex items-center text-sm text-gray-600">
                        <i className="fas fa-map-marker-alt mr-1"></i>
                        {restaurant.address}
                      </div>
                      <div className="flex items-center text-sm text-gray-600">
                        <i className="fas fa-shipping-fast mr-1"></i>
                        配送费 ¥{restaurant.deliveryFee}
                      </div>
                    </div>
                    
                    <div className="flex gap-2">
                      <button className="flex-1 bg-orange-500 text-white py-2 px-4 rounded-lg hover:bg-orange-600 transition-colors">
                        <i className="fas fa-shopping-cart mr-2"></i>
                        立即订餐
                      </button>
                      <button className="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors">
                        <i className="fas fa-phone"></i>
                      </button>
                      <button className="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors">
                        <i className="fas fa-heart"></i>
                      </button>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          </>
        )}

        {activeTab === 'orders' && (
          <div className="bg-white rounded-2xl shadow-sm p-6">
            <h3 className="text-xl font-bold text-gray-800 mb-6">最近订单</h3>
            
            <div className="space-y-4">
              {recentOrders.map((order) => (
                <div key={order.id} className="border border-gray-200 rounded-xl p-6 hover:shadow-md transition-shadow">
                  <div className="flex items-center justify-between mb-4">
                    <div className="flex items-center">
                      <div className="w-12 h-12 bg-orange-100 rounded-full flex items-center justify-center mr-4">
                        <i className="fas fa-utensils text-orange-600"></i>
                      </div>
                      <div>
                        <h4 className="font-bold text-gray-800">订单 #{order.id}</h4>
                        <p className="text-sm text-gray-600">{order.restaurant}</p>
                      </div>
                    </div>
                    {getOrderStatusBadge(order.status)}
                  </div>
                  
                  <div className="grid grid-cols-1 md:grid-cols-4 gap-4 mb-4">
                    <div>
                      <p className="text-sm text-gray-600">客户</p>
                      <p className="font-medium text-gray-800">{order.customer}</p>
                    </div>
                    <div>
                      <p className="text-sm text-gray-600">下单时间</p>
                      <p className="font-medium text-gray-800">{order.time}</p>
                    </div>
                    <div>
                      <p className="text-sm text-gray-600">订单金额</p>
                      <p className="font-medium text-gray-800">¥{order.amount}</p>
                    </div>
                    <div>
                      <p className="text-sm text-gray-600">预计时间</p>
                      <p className="font-medium text-gray-800">{order.estimatedTime}</p>
                    </div>
                  </div>
                  
                  <div className="mb-4">
                    <p className="text-sm text-gray-600 mb-2">订单内容</p>
                    <p className="text-gray-800">{order.items.join(' • ')}</p>
                  </div>
                  
                  <div className="flex gap-2">
                    <button className="bg-blue-500 text-white py-2 px-4 rounded-lg hover:bg-blue-600 transition-colors text-sm">
                      <i className="fas fa-eye mr-2"></i>
                      查看详情
                    </button>
                    {order.status === 'preparing' && (
                      <button className="bg-green-500 text-white py-2 px-4 rounded-lg hover:bg-green-600 transition-colors text-sm">
                        <i className="fas fa-check mr-2"></i>
                        标记完成
                      </button>
                    )}
                    <button className="bg-gray-100 text-gray-700 py-2 px-4 rounded-lg hover:bg-gray-200 transition-colors text-sm">
                      <i className="fas fa-phone mr-2"></i>
                      联系客户
                    </button>
                  </div>
                </div>
              ))}
            </div>
            
            <div className="mt-8 text-center">
              <button className="text-orange-600 hover:text-orange-700 font-medium">
                查看更多订单 →
              </button>
            </div>
          </div>
        )}

        {/* 空状态 */}
        {activeTab === 'restaurants' && filteredRestaurants.length === 0 && (
          <div className="bg-white rounded-2xl shadow-sm p-12 text-center">
            <i className="fas fa-search text-6xl text-gray-300 mb-4"></i>
            <h3 className="text-lg font-medium text-gray-800 mb-2">未找到匹配的餐厅</h3>
            <p className="text-gray-600 mb-6">试试调整搜索关键词或选择其他分类</p>
            <button
              onClick={() => {
                setSearchTerm('');
                setFilterCategory('all');
              }}
              className="btn-primary text-white px-6 py-3 rounded-xl font-medium"
            >
              <i className="fas fa-refresh mr-2"></i>
              重置筛选
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

export default RestaurantPage; 