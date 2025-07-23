import React, { useState } from 'react';

const B2CPage: React.FC = () => {
  const [selectedCategory, setSelectedCategory] = useState('all');
  const [searchTerm, setSearchTerm] = useState('');
  const [cart, setCart] = useState<{ [key: number]: number }>({});
  const [likedProducts, setLikedProducts] = useState<Set<number>>(new Set());

  const products = [
    {
      id: 1,
      name: '有机苹果',
      description: '新鲜有机苹果，产自山东烟台，无农药残留',
      image: 'https://images.unsplash.com/photo-1567306301408-9b74779a11af?w=300&h=200&fit=crop',
      price: 9.99,
      originalPrice: 12.99,
      category: 'food',
      rating: 4.9,
      reviews: 234,
      stock: 856,
      claimed: 144,
      daysLeft: 7,
      hasVoucher: true
    },
    {
      id: 2,
      name: '精品大米',
      description: '东北优质大米，粒粒饱满，营养丰富',
      image: 'https://images.unsplash.com/photo-1586511925558-a4c6376fe65f?w=300&h=200&fit=crop',
      price: 25.80,
      originalPrice: 35.80,
      category: 'food',
      rating: 4.8,
      reviews: 567,
      stock: 234,
      claimed: 266,
      daysLeft: 12,
      hasVoucher: true
    },
    {
      id: 3,
      name: '智能手机',
      description: '最新款智能手机，高性能芯片，拍照清晰',
      image: 'https://images.unsplash.com/photo-1511707171634-5f897ff02aa9?w=300&h=200&fit=crop',
      price: 2999.00,
      originalPrice: 3999.00,
      category: 'digital',
      rating: 4.7,
      reviews: 128,
      stock: 45,
      claimed: 55,
      daysLeft: 3,
      hasVoucher: true
    },
    {
      id: 4,
      name: '纯棉T恤',
      description: '100%纯棉材质，舒适透气，多色可选',
      image: 'https://images.unsplash.com/photo-1521572163474-6864f9cf17ab?w=300&h=200&fit=crop',
      price: 89.90,
      originalPrice: 129.90,
      category: 'clothes',
      rating: 4.6,
      reviews: 234,
      stock: 156,
      claimed: 89,
      daysLeft: 15,
      hasVoucher: true
    },
    {
      id: 5,
      name: '护肤精华',
      description: '高端护肤精华，深层滋养，改善肌肤',
      image: 'https://images.unsplash.com/photo-1596462502278-27bfdc403348?w=300&h=200&fit=crop',
      price: 299.00,
      originalPrice: 399.00,
      category: 'beauty',
      rating: 4.9,
      reviews: 89,
      stock: 78,
      claimed: 45,
      daysLeft: 8,
      hasVoucher: true
    },
    {
      id: 6,
      name: '智能家电',
      description: '智能扫地机器人，自动清洁，智能导航',
      image: 'https://images.unsplash.com/photo-1558618047-3c8c76ca7d13?w=300&h=200&fit=crop',
      price: 1999.00,
      originalPrice: 2999.00,
      category: 'home',
      rating: 4.8,
      reviews: 178,
      stock: 34,
      claimed: 67,
      daysLeft: 5,
      hasVoucher: true
    }
  ];

  const filteredProducts = products.filter(product => {
    const matchesCategory = selectedCategory === 'all' || product.category === selectedCategory;
    const matchesSearch = product.name.toLowerCase().includes(searchTerm.toLowerCase());
    return matchesCategory && matchesSearch;
  });

  const toggleLike = (productId: number) => {
    setLikedProducts(prev => {
      const newSet = new Set(prev);
      if (newSet.has(productId)) {
        newSet.delete(productId);
      } else {
        newSet.add(productId);
      }
      return newSet;
    });
  };

  const claimVoucher = (productId: number) => {
    // 实现获取提货券逻辑
    console.log('获取提货券:', productId);
  };

  const getCartItemCount = () => {
    return Object.values(cart).reduce((total, quantity) => total + quantity, 0);
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
                <i className="fas fa-shopping-cart text-2xl text-pink-600"></i>
                <span className="ml-2 text-xl font-bold text-gray-800">B2C电商平台</span>
              </button>
            </div>
            
            <div className="flex items-center space-x-4">
              <div className="relative">
                <input 
                  type="text" 
                  placeholder="搜索商品..." 
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="w-64 pl-10 pr-4 py-2 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
                <i className="fas fa-search absolute left-3 top-3 text-gray-400"></i>
              </div>
              <div className="relative">
                <i className="fas fa-shopping-bag text-gray-600 text-xl cursor-pointer"></i>
                {getCartItemCount() > 0 && (
                  <span className="absolute -top-2 -right-2 bg-red-500 text-white text-xs rounded-full w-5 h-5 flex items-center justify-center">
                    {getCartItemCount()}
                  </span>
                )}
              </div>
              <div className="w-8 h-8 bg-gray-200 rounded-full flex items-center justify-center">
                <i className="fas fa-user text-gray-600"></i>
              </div>
            </div>
          </div>
        </div>
      </nav>

      {/* 主要内容 */}
      <main className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* 轮播图 */}
        <div className="bg-gradient-to-r from-blue-500 to-purple-600 rounded-2xl mb-8 overflow-hidden">
          <div className="p-8 text-center text-white">
            <h2 className="text-3xl font-bold mb-4">免费获取提货券，享受优质商品</h2>
            <p className="text-lg mb-6">基于共产主义经济原理，让每个人都能享受到优质的商品和服务</p>
            <button className="bg-white text-blue-600 px-8 py-3 rounded-lg font-medium hover:bg-gray-100 transition-colors">
              立即体验
            </button>
          </div>
        </div>

        {/* 分类导航 */}
        <div className="flex flex-wrap gap-4 mb-8">
          <button 
            onClick={() => setSelectedCategory('all')}
            className={`px-6 py-3 rounded-lg font-medium transition-colors ${
              selectedCategory === 'all' 
                ? 'category-tab active text-white' 
                : 'category-tab bg-white hover:bg-gray-50 text-gray-700'
            }`}
          >
            全部商品
          </button>
          <button 
            onClick={() => setSelectedCategory('food')}
            className={`px-6 py-3 rounded-lg font-medium transition-colors ${
              selectedCategory === 'food' 
                ? 'category-tab active text-white' 
                : 'category-tab bg-white hover:bg-gray-50 text-gray-700'
            }`}
          >
            食品
          </button>
          <button 
            onClick={() => setSelectedCategory('daily')}
            className={`px-6 py-3 rounded-lg font-medium transition-colors ${
              selectedCategory === 'daily' 
                ? 'category-tab active text-white' 
                : 'category-tab bg-white hover:bg-gray-50 text-gray-700'
            }`}
          >
            日用品
          </button>
          <button 
            onClick={() => setSelectedCategory('clothes')}
            className={`px-6 py-3 rounded-lg font-medium transition-colors ${
              selectedCategory === 'clothes' 
                ? 'category-tab active text-white' 
                : 'category-tab bg-white hover:bg-gray-50 text-gray-700'
            }`}
          >
            服装
          </button>
          <button 
            onClick={() => setSelectedCategory('digital')}
            className={`px-6 py-3 rounded-lg font-medium transition-colors ${
              selectedCategory === 'digital' 
                ? 'category-tab active text-white' 
                : 'category-tab bg-white hover:bg-gray-50 text-gray-700'
            }`}
          >
            数码
          </button>
          <button 
            onClick={() => setSelectedCategory('home')}
            className={`px-6 py-3 rounded-lg font-medium transition-colors ${
              selectedCategory === 'home' 
                ? 'category-tab active text-white' 
                : 'category-tab bg-white hover:bg-gray-50 text-gray-700'
            }`}
          >
            家居
          </button>
        </div>

        {/* 商品网格 */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
          {filteredProducts.map((product) => (
            <div key={product.id} className="bg-white rounded-2xl shadow-sm overflow-hidden card-hover">
              <div className="relative">
                <img 
                  src={product.image} 
                  alt={product.name} 
                  className="w-full h-48 object-cover"
                />
                {product.hasVoucher && (
                  <div className="absolute top-3 left-3">
                    <span className="voucher-badge text-white px-3 py-1 rounded-full text-xs font-medium">
                      免费提货券
                    </span>
                  </div>
                )}
                <div className="absolute top-3 right-3">
                  <button 
                    onClick={() => toggleLike(product.id)}
                    className="w-8 h-8 bg-white/80 rounded-full flex items-center justify-center hover:bg-white transition-colors"
                  >
                    <i className={`fas fa-heart ${
                      likedProducts.has(product.id) ? 'text-red-500' : 'text-gray-600'
                    }`}></i>
                  </button>
                </div>
              </div>
              <div className="p-4">
                <h3 className="font-bold text-gray-800 mb-2">{product.name}</h3>
                <p className="text-sm text-gray-600 mb-3">{product.description}</p>
                
                <div className="flex items-center justify-between mb-3">
                  <div className="text-lg font-bold text-green-600">免费</div>
                  <div className="text-sm text-gray-500">原价: ¥{product.price}</div>
                </div>
                
                <div className="flex items-center justify-between mb-3">
                  <div className="text-sm text-gray-500">
                    <i className="fas fa-box mr-1"></i>
                    剩余: {product.stock}
                  </div>
                  <div className="text-sm text-gray-500">
                    <i className="fas fa-users mr-1"></i>
                    已获取: {product.claimed}
                  </div>
                </div>
                
                <div className="flex items-center justify-between mb-3">
                  <div className="flex items-center">
                    <div className="flex text-yellow-400">
                      {[1, 2, 3, 4, 5].map((star) => (
                        <i key={star} className="fas fa-star"></i>
                      ))}
                    </div>
                    <span className="text-sm text-gray-600 ml-2">{product.rating} ({product.reviews})</span>
                  </div>
                  <div className="text-sm text-gray-500">
                    <i className="fas fa-clock mr-1"></i>
                    {product.daysLeft}天后过期
                  </div>
                </div>
                
                <button 
                  onClick={() => claimVoucher(product.id)}
                  className="w-full btn-primary text-white py-2 px-4 rounded-lg font-medium"
                >
                  <i className="fas fa-download mr-2"></i>
                  获取提货券
                </button>
              </div>
            </div>
          ))}
        </div>
      </main>
    </div>
  );
};

export default B2CPage; 