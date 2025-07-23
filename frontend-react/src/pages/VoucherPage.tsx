import React, { useState } from 'react';

const VoucherPage: React.FC = () => {
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [categoryFilter, setCategoryFilter] = useState('all');

  const vouchers = [
    {
      id: 1,
      name: '有机苹果提货券',
      description: '新鲜有机苹果，产自山东烟台，无农药残留',
      image: 'https://images.unsplash.com/photo-1567306301408-9b74779a11af?w=400&h=300&fit=crop',
      price: 9.99,
      remaining: 856,
      total: 1000,
      claimed: 144,
      daysLeft: 7,
      status: 'active',
      category: 'food',
      rating: 4.8,
      reviews: 324
    },
    {
      id: 2,
      name: '精品大米提货券',
      description: '东北优质大米，粒粒饱满，营养丰富',
      image: 'https://images.unsplash.com/photo-1586511925558-a4c6376fe65f?w=400&h=300&fit=crop',
      price: 25.80,
      remaining: 234,
      total: 500,
      claimed: 266,
      daysLeft: 12,
      status: 'active',
      category: 'food',
      rating: 4.9,
      reviews: 456
    },
    {
      id: 3,
      name: '智能手机提货券',
      description: '最新款智能手机，高性能芯片，拍照清晰',
      image: 'https://images.unsplash.com/photo-1511707171634-5f897ff02aa9?w=400&h=300&fit=crop',
      price: 2999.00,
      remaining: 45,
      total: 100,
      claimed: 55,
      daysLeft: 3,
      status: 'active',
      category: 'digital',
      rating: 4.7,
      reviews: 128
    },
    {
      id: 4,
      name: '纯棉T恤提货券',
      description: '100%纯棉材质，舒适透气，多色可选',
      image: 'https://images.unsplash.com/photo-1521572163474-6864f9cf17ab?w=400&h=300&fit=crop',
      price: 89.90,
      remaining: 0,
      total: 200,
      claimed: 200,
      daysLeft: 0,
      status: 'expired',
      category: 'clothes',
      rating: 4.6,
      reviews: 234
    },
    {
      id: 5,
      name: '高档护肤品提货券',
      description: '知名品牌护肤品，天然成分，深层滋养',
      image: 'https://images.unsplash.com/photo-1596462502278-27bfdc403348?w=400&h=300&fit=crop',
      price: 299.00,
      remaining: 89,
      total: 150,
      claimed: 61,
      daysLeft: 15,
      status: 'active',
      category: 'cosmetics',
      rating: 4.9,
      reviews: 89
    },
    {
      id: 6,
      name: '家居清洁套装提货券',
      description: '全套家居清洁用品，环保配方，安全有效',
      image: 'https://images.unsplash.com/photo-1583947581924-860bda6a26ab?w=400&h=300&fit=crop',
      price: 156.50,
      remaining: 127,
      total: 300,
      claimed: 173,
      daysLeft: 8,
      status: 'active',
      category: 'daily',
      rating: 4.5,
      reviews: 167
    }
  ];

  const filteredVouchers = vouchers.filter(voucher => {
    const matchesSearch = voucher.name.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesStatus = statusFilter === 'all' || voucher.status === statusFilter;
    const matchesCategory = categoryFilter === 'all' || voucher.category === categoryFilter;
    return matchesSearch && matchesStatus && matchesCategory;
  });

  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'active':
        return <span className="bg-green-500 text-white px-3 py-1 rounded-full text-sm font-medium">活跃</span>;
      case 'paused':
        return <span className="bg-yellow-500 text-white px-3 py-1 rounded-full text-sm font-medium">暂停</span>;
      case 'expired':
        return <span className="bg-red-500 text-white px-3 py-1 rounded-full text-sm font-medium">已过期</span>;
      default:
        return <span className="bg-gray-500 text-white px-3 py-1 rounded-full text-sm font-medium">未知</span>;
    }
  };

  const getCategoryIcon = (category: string) => {
    switch (category) {
      case 'food':
        return 'fas fa-utensils';
      case 'digital':
        return 'fas fa-mobile-alt';
      case 'clothes':
        return 'fas fa-tshirt';
      case 'cosmetics':
        return 'fas fa-spa';
      case 'daily':
        return 'fas fa-home';
      default:
        return 'fas fa-box';
    }
  };

  return (
    <div className="bg-gray-50 min-h-screen">
      {/* 导航栏 */}
      <nav className="bg-white shadow-lg sticky top-0 z-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between items-center h-16">
            <div className="flex items-center">
              <button onClick={() => window.history.back()} className="flex items-center">
                <i className="fas fa-arrow-left text-gray-500 mr-3"></i>
                <i className="fas fa-ticket-alt text-2xl text-blue-600"></i>
                <span className="ml-2 text-xl font-bold text-gray-800">提货券管理</span>
              </button>
            </div>
            
            <div className="flex items-center space-x-4">
              <button className="btn-primary text-white px-4 py-2 rounded-lg font-medium">
                <i className="fas fa-plus mr-2"></i>
                创建提货券
              </button>
              <div className="w-8 h-8 bg-gray-200 rounded-full flex items-center justify-center">
                <i className="fas fa-user text-gray-600"></i>
              </div>
            </div>
          </div>
        </div>
      </nav>

      {/* 主要内容 */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* 统计概览 */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          <div className="stats-card rounded-2xl p-6 text-white">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-white/80 text-sm">已发布提货券</p>
                <p className="text-2xl font-bold">156</p>
              </div>
              <i className="fas fa-ticket-alt text-3xl text-white/60"></i>
            </div>
          </div>
          
          <div className="stats-card-green rounded-2xl p-6 text-white">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-white/80 text-sm">活跃提货券</p>
                <p className="text-2xl font-bold">89</p>
              </div>
              <i className="fas fa-check-circle text-3xl text-white/60"></i>
            </div>
          </div>
          
          <div className="stats-card-orange rounded-2xl p-6 text-white">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-white/80 text-sm">今日领取</p>
                <p className="text-2xl font-bold">1,234</p>
              </div>
              <i className="fas fa-download text-3xl text-white/60"></i>
            </div>
          </div>
          
          <div className="stats-card-purple rounded-2xl p-6 text-white">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-white/80 text-sm">总价值</p>
                <p className="text-2xl font-bold">¥45,678</p>
              </div>
              <i className="fas fa-coins text-3xl text-white/60"></i>
            </div>
          </div>
        </div>

        {/* 筛选和搜索 */}
        <div className="bg-white rounded-2xl shadow-sm p-6 mb-8">
          <div className="flex flex-col md:flex-row gap-4">
            <div className="flex-1">
              <div className="relative">
                <i className="fas fa-search absolute left-3 top-3 text-gray-400"></i>
                <input 
                  type="text" 
                  placeholder="搜索提货券..." 
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="w-full pl-10 pr-4 py-2 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>
            </div>
            <div className="flex gap-2">
              <select 
                value={statusFilter}
                onChange={(e) => setStatusFilter(e.target.value)}
                className="px-4 py-2 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="all">全部状态</option>
                <option value="active">活跃</option>
                <option value="paused">已暂停</option>
                <option value="expired">已过期</option>
              </select>
              <select 
                value={categoryFilter}
                onChange={(e) => setCategoryFilter(e.target.value)}
                className="px-4 py-2 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              >
                <option value="all">全部类别</option>
                <option value="food">食品</option>
                <option value="daily">日用品</option>
                <option value="clothes">服装</option>
                <option value="digital">数码</option>
                <option value="cosmetics">美妆</option>
              </select>
            </div>
          </div>
        </div>

        {/* 提货券网格 */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredVouchers.map((voucher) => (
            <div key={voucher.id} className="bg-white rounded-2xl shadow-sm overflow-hidden card-hover">
              <div className="h-48 relative">
                <img 
                  src={voucher.image} 
                  alt={voucher.name} 
                  className="w-full h-full object-cover"
                />
                <div className="absolute top-4 right-4">
                  {getStatusBadge(voucher.status)}
                </div>
              </div>
              <div className="p-6">
                <h3 className="text-lg font-bold text-gray-800 mb-2">{voucher.name}</h3>
                <p className="text-gray-600 text-sm mb-4">{voucher.description}</p>
                
                <div className="flex items-center justify-between mb-4">
                  <div className="text-sm text-gray-500">
                    <i className="fas fa-money-bill-wave mr-1"></i>
                    单价：¥{voucher.price}
                  </div>
                  <div className="text-sm text-gray-500">
                    <i className="fas fa-box mr-1"></i>
                    剩余：{voucher.remaining}/{voucher.total}
                  </div>
                </div>
                
                <div className="flex items-center justify-between mb-4">
                  <div className="text-sm text-gray-500">
                    <i className="fas fa-clock mr-1"></i>
                    {voucher.daysLeft > 0 ? `${voucher.daysLeft}天后过期` : '已过期'}
                  </div>
                  <div className="text-sm text-gray-500">
                    <i className="fas fa-download mr-1"></i>
                    已领取：{voucher.claimed}
                  </div>
                </div>
                
                <div className="flex gap-2">
                  <button className="flex-1 bg-blue-500 text-white py-2 px-4 rounded-lg hover:bg-blue-600 transition-colors">
                    <i className="fas fa-edit mr-2"></i>
                    编辑
                  </button>
                  <button className="px-4 py-2 border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors">
                    <i className="fas fa-chart-bar text-gray-600"></i>
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>
      </main>
    </div>
  );
};

export default VoucherPage; 