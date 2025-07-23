import React, { useState, useEffect } from 'react'

export const B2CPage: React.FC = () => {
  const [searchTerm, setSearchTerm] = useState('')
  const [selectedCategory, setSelectedCategory] = useState('all')
  const [currentSlide, setCurrentSlide] = useState(0)
  const [likedProducts, setLikedProducts] = useState<Set<number>>(new Set())
  const [cartItems, setCartItems] = useState<Set<number>>(new Set())

  const categories = [
    { id: 'all', name: '全部', icon: 'fas fa-th' },
    { id: 'food', name: '餐饮美食', icon: 'fas fa-utensils' },
    { id: 'fashion', name: '时尚服饰', icon: 'fas fa-tshirt' },
    { id: 'electronics', name: '数码电器', icon: 'fas fa-mobile-alt' },
    { id: 'home', name: '家居用品', icon: 'fas fa-home' },
    { id: 'beauty', name: '美妆护肤', icon: 'fas fa-spa' },
    { id: 'sports', name: '运动健身', icon: 'fas fa-dumbbell' },
    { id: 'books', name: '图书文创', icon: 'fas fa-book' },
  ]

  const bannerImages = [
    'https://images.unsplash.com/photo-1556742049-0cfed4f6a45d?w=800&h=400&fit=crop',
    'https://images.unsplash.com/photo-1556909114-f6e7ad7d3136?w=800&h=400&fit=crop',
    'https://images.unsplash.com/photo-1556909043-f132a02f5e95?w=800&h=400&fit=crop',
  ]

  const toggleLike = (productId: number) => {
    setLikedProducts(prev => {
      const newSet = new Set(prev)
      if (newSet.has(productId)) {
        newSet.delete(productId)
      } else {
        newSet.add(productId)
      }
      return newSet
    })
  }

  const toggleCart = (productId: number) => {
    setCartItems(prev => {
      const newSet = new Set(prev)
      if (newSet.has(productId)) {
        newSet.delete(productId)
      } else {
        newSet.add(productId)
      }
      return newSet
    })
  }

  useEffect(() => {
    const timer = setInterval(() => {
      setCurrentSlide((prev) => (prev + 1) % bannerImages.length)
    }, 5000)
    return () => clearInterval(timer)
  }, [])

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 py-6">
          <div className="flex items-center justify-between mb-6">
            <div>
              <h1 className="text-2xl font-bold text-gray-800">B2C电商平台</h1>
              <p className="text-gray-600 mt-1">免费获取提货券，享受优质商品服务</p>
            </div>
            <div className="flex items-center space-x-3">
              <div className="relative">
                <i className="fas fa-shopping-cart text-gray-600 text-xl"></i>
                {cartItems.size > 0 && (
                  <span className="absolute -top-2 -right-2 bg-red-500 text-white rounded-full w-5 h-5 flex items-center justify-center text-xs">
                    {cartItems.size}
                  </span>
                )}
              </div>
              <div className="relative">
                <i className="fas fa-heart text-gray-600 text-xl"></i>
                {likedProducts.size > 0 && (
                  <span className="absolute -top-2 -right-2 bg-red-500 text-white rounded-full w-5 h-5 flex items-center justify-center text-xs">
                    {likedProducts.size}
                  </span>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 py-6">
        {/* 轮播图 */}
        <div className="relative mb-8 rounded-2xl overflow-hidden">
          <div className="relative h-64 md:h-80">
            {bannerImages.map((image, index) => (
              <div
                key={index}
                className={`absolute inset-0 transition-opacity duration-500 ${
                  index === currentSlide ? 'opacity-100' : 'opacity-0'
                }`}
              >
                <img
                  src={image}
                  alt={`Banner ${index + 1}`}
                  className="w-full h-full object-cover"
                />
                <div className="absolute inset-0 bg-black bg-opacity-30 flex items-center justify-center">
                  <div className="text-center text-white">
                    <h2 className="text-3xl font-bold mb-4">精选商品特惠</h2>
                    <p className="text-xl">免费提货券，无门槛享受</p>
                  </div>
                </div>
              </div>
            ))}
          </div>
          
          {/* 轮播指示器 */}
          <div className="absolute bottom-4 left-1/2 transform -translate-x-1/2 flex space-x-2">
            {bannerImages.map((_, index) => (
              <button
                key={index}
                onClick={() => setCurrentSlide(index)}
                className={`w-3 h-3 rounded-full transition-colors ${
                  index === currentSlide ? 'bg-white' : 'bg-white/50'
                }`}
              />
            ))}
          </div>
        </div>

        {/* 搜索和分类 */}
        <div className="bg-white rounded-2xl shadow-sm p-6 mb-8">
          {/* 搜索框 */}
          <div className="relative mb-6">
            <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
              <i className="fas fa-search text-gray-400"></i>
            </div>
            <input
              type="text"
              placeholder="搜索商品..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-full pl-12 pr-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>
          
          {/* 分类标签 */}
          <div className="flex flex-wrap gap-3">
            {categories.map((category) => (
              <button
                key={category.id}
                onClick={() => setSelectedCategory(category.id)}
                className={`px-4 py-2 rounded-full text-sm font-medium transition-colors ${
                  selectedCategory === category.id
                    ? 'bg-blue-500 text-white'
                    : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                }`}
              >
                <i className={`${category.icon} mr-2`}></i>
                {category.name}
              </button>
            ))}
          </div>
        </div>

        {/* 商品网格 */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
          {/* 商品卡片 1 */}
          <div className="bg-white rounded-2xl shadow-sm overflow-hidden card-hover">
            <div className="relative">
              <img 
                src="https://images.unsplash.com/photo-1567306301408-9b74779a11af?w=300&h=200&fit=crop" 
                alt="有机苹果" 
                className="w-full h-48 object-cover"
              />
              <div className="absolute top-3 left-3">
                <span className="voucher-badge text-white px-3 py-1 rounded-full text-xs font-medium">免费提货券</span>
              </div>
              <div className="absolute top-3 right-3">
                <button className="w-8 h-8 bg-white/80 rounded-full flex items-center justify-center hover:bg-white transition-colors">
                  <i className="fas fa-heart text-gray-600 hover:text-red-500"></i>
                </button>
              </div>
            </div>
            <div className="p-4">
              <h3 className="font-bold text-gray-800 mb-2">有机苹果</h3>
              <p className="text-sm text-gray-600 mb-3">新鲜有机苹果，产自山东烟台，无农药残留</p>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-lg font-bold text-green-600">免费</div>
                <div className="text-sm text-gray-500">原价: ¥9.99</div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-sm text-gray-500">
                  <i className="fas fa-box mr-1"></i>
                  剩余: 856
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-users mr-1"></i>
                  已获取: 144
                </div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center">
                  <div className="flex text-yellow-400">
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                  </div>
                  <span className="text-sm text-gray-600 ml-2">4.9 (234)</span>
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-clock mr-1"></i>
                  7天后过期
                </div>
              </div>
              
              <button className="w-full btn-primary text-white py-2 px-4 rounded-lg font-medium">
                <i className="fas fa-download mr-2"></i>
                获取提货券
              </button>
            </div>
          </div>

          {/* 商品卡片 2 */}
          <div className="bg-white rounded-2xl shadow-sm overflow-hidden card-hover">
            <div className="relative">
              <img 
                src="https://images.unsplash.com/photo-1586511925558-a4c6376fe65f?w=300&h=200&fit=crop" 
                alt="精品大米" 
                className="w-full h-48 object-cover"
              />
              <div className="absolute top-3 left-3">
                <span className="voucher-badge text-white px-3 py-1 rounded-full text-xs font-medium">免费提货券</span>
              </div>
              <div className="absolute top-3 right-3">
                <button className="w-8 h-8 bg-white/80 rounded-full flex items-center justify-center hover:bg-white transition-colors">
                  <i className="fas fa-heart text-gray-600 hover:text-red-500"></i>
                </button>
              </div>
            </div>
            <div className="p-4">
              <h3 className="font-bold text-gray-800 mb-2">精品大米</h3>
              <p className="text-sm text-gray-600 mb-3">东北优质大米，粒粒饱满，口感香甜</p>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-lg font-bold text-green-600">免费</div>
                <div className="text-sm text-gray-500">原价: ¥25.00</div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-sm text-gray-500">
                  <i className="fas fa-box mr-1"></i>
                  剩余: 234
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-users mr-1"></i>
                  已获取: 266
                </div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center">
                  <div className="flex text-yellow-400">
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="far fa-star"></i>
                  </div>
                  <span className="text-sm text-gray-600 ml-2">4.7 (189)</span>
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-clock mr-1"></i>
                  15天后过期
                </div>
              </div>
              
              <button className="w-full btn-primary text-white py-2 px-4 rounded-lg font-medium">
                <i className="fas fa-download mr-2"></i>
                获取提货券
              </button>
            </div>
          </div>

          {/* 商品卡片 3 */}
          <div className="bg-white rounded-2xl shadow-sm overflow-hidden card-hover">
            <div className="relative">
              <img 
                src="https://images.unsplash.com/photo-1542838132-92c53300491e?w=300&h=200&fit=crop" 
                alt="运动鞋" 
                className="w-full h-48 object-cover"
              />
              <div className="absolute top-3 left-3">
                <span className="voucher-badge text-white px-3 py-1 rounded-full text-xs font-medium">免费提货券</span>
              </div>
              <div className="absolute top-3 right-3">
                <button className="w-8 h-8 bg-white/80 rounded-full flex items-center justify-center hover:bg-white transition-colors">
                  <i className="fas fa-heart text-gray-600 hover:text-red-500"></i>
                </button>
              </div>
            </div>
            <div className="p-4">
              <h3 className="font-bold text-gray-800 mb-2">舒适运动鞋</h3>
              <p className="text-sm text-gray-600 mb-3">舒适透气运动鞋，多种颜色可选</p>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-lg font-bold text-green-600">免费</div>
                <div className="text-sm text-gray-500">原价: ¥299.00</div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-sm text-gray-500">
                  <i className="fas fa-box mr-1"></i>
                  剩余: 45
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-users mr-1"></i>
                  已获取: 55
                </div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center">
                  <div className="flex text-yellow-400">
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                  </div>
                  <span className="text-sm text-gray-600 ml-2">4.8 (156)</span>
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-clock mr-1"></i>
                  25天后过期
                </div>
              </div>
              
              <button className="w-full bg-gray-400 text-white py-2 px-4 rounded-lg font-medium cursor-not-allowed">
                <i className="fas fa-pause mr-2"></i>
                暂时缺货
              </button>
            </div>
          </div>

          {/* 商品卡片 4 */}
          <div className="bg-white rounded-2xl shadow-sm overflow-hidden card-hover">
            <div className="relative">
              <img 
                src="https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=300&h=200&fit=crop" 
                alt="蓝牙耳机" 
                className="w-full h-48 object-cover"
              />
              <div className="absolute top-3 left-3">
                <span className="voucher-badge text-white px-3 py-1 rounded-full text-xs font-medium">免费提货券</span>
              </div>
              <div className="absolute top-3 right-3">
                <button className="w-8 h-8 bg-white/80 rounded-full flex items-center justify-center hover:bg-white transition-colors">
                  <i className="fas fa-heart text-gray-600 hover:text-red-500"></i>
                </button>
              </div>
            </div>
            <div className="p-4">
              <h3 className="font-bold text-gray-800 mb-2">无线蓝牙耳机</h3>
              <p className="text-sm text-gray-600 mb-3">高保真音质，降噪设计，长续航</p>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-lg font-bold text-green-600">免费</div>
                <div className="text-sm text-gray-500">原价: ¥199.00</div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-sm text-gray-500">
                  <i className="fas fa-box mr-1"></i>
                  剩余: 89
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-users mr-1"></i>
                  已获取: 211
                </div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center">
                  <div className="flex text-yellow-400">
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="far fa-star"></i>
                  </div>
                  <span className="text-sm text-gray-600 ml-2">4.6 (324)</span>
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-clock mr-1"></i>
                  12天后过期
                </div>
              </div>
              
              <button className="w-full btn-primary text-white py-2 px-4 rounded-lg font-medium">
                <i className="fas fa-download mr-2"></i>
                获取提货券
              </button>
            </div>
          </div>

          {/* 商品卡片 5 */}
          <div className="bg-white rounded-2xl shadow-sm overflow-hidden hover:shadow-lg transition-all duration-300 hover:-translate-y-2">
            <div className="relative">
              <img
                src="https://images.unsplash.com/photo-1505740420928-5e560c06d30e?w=300&h=200&fit=crop"
                alt="美食套餐"
                className="w-full h-48 object-cover"
              />
              <div className="absolute top-3 left-3">
                <span className="bg-green-500 text-white px-3 py-1 rounded-full text-xs font-medium">免费提货券</span>
              </div>
              <div className="absolute top-3 right-3">
                <button
                  onClick={() => toggleLike(5)}
                  className={`w-8 h-8 rounded-full flex items-center justify-center transition-colors ${
                    likedProducts.has(5) ? 'bg-red-500 text-white' : 'bg-white/80 text-gray-600 hover:bg-white'
                  }`}
                >
                  <i className="fas fa-heart text-sm"></i>
                </button>
              </div>
            </div>
            <div className="p-4">
              <h3 className="font-bold text-gray-800 mb-2">美食套餐</h3>
              <p className="text-sm text-gray-600 mb-3">健康营养搭配，精选食材，口感丰富</p>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-lg font-bold text-green-600">免费</div>
                <div className="text-sm text-gray-500">原价: ¥45.00</div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-sm text-gray-500">
                  <i className="fas fa-box mr-1"></i>
                  剩余: 567
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-users mr-1"></i>
                  已获取: 133
                </div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center">
                  <div className="flex text-yellow-400">
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="far fa-star"></i>
                  </div>
                  <span className="text-sm text-gray-600 ml-2">4.5 (167)</span>
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-clock mr-1"></i>
                  18天后过期
                </div>
              </div>
              
              <button 
                onClick={() => toggleCart(5)}
                className="w-full bg-gradient-to-r from-blue-500 to-purple-500 text-white py-2 px-4 rounded-lg font-medium hover:shadow-lg transition-all duration-300"
              >
                <i className="fas fa-download mr-2"></i>
                获取提货券
              </button>
            </div>
          </div>

          {/* 商品卡片 6 */}
          <div className="bg-white rounded-2xl shadow-sm overflow-hidden hover:shadow-lg transition-all duration-300 hover:-translate-y-2">
            <div className="relative">
              <img
                src="https://images.unsplash.com/photo-1542291026-7eec264c27ff?w=300&h=200&fit=crop"
                alt="运动鞋"
                className="w-full h-48 object-cover"
              />
              <div className="absolute top-3 left-3">
                <span className="bg-green-500 text-white px-3 py-1 rounded-full text-xs font-medium">免费提货券</span>
              </div>
              <div className="absolute top-3 right-3">
                <button
                  onClick={() => toggleLike(6)}
                  className={`w-8 h-8 rounded-full flex items-center justify-center transition-colors ${
                    likedProducts.has(6) ? 'bg-red-500 text-white' : 'bg-white/80 text-gray-600 hover:bg-white'
                  }`}
                >
                  <i className="fas fa-heart text-sm"></i>
                </button>
              </div>
            </div>
            <div className="p-4">
              <h3 className="font-bold text-gray-800 mb-2">运动鞋</h3>
              <p className="text-sm text-gray-600 mb-3">舒适透气设计，运动专用，多种颜色可选</p>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-lg font-bold text-green-600">免费</div>
                <div className="text-sm text-gray-500">原价: ¥299.00</div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-sm text-gray-500">
                  <i className="fas fa-box mr-1"></i>
                  剩余: 123
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-users mr-1"></i>
                  已获取: 177
                </div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center">
                  <div className="flex text-yellow-400">
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                  </div>
                  <span className="text-sm text-gray-600 ml-2">4.8 (245)</span>
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-clock mr-1"></i>
                  12天后过期
                </div>
              </div>
              
              <button 
                onClick={() => toggleCart(6)}
                className="w-full bg-gradient-to-r from-blue-500 to-purple-500 text-white py-2 px-4 rounded-lg font-medium hover:shadow-lg transition-all duration-300"
              >
                <i className="fas fa-download mr-2"></i>
                获取提货券
              </button>
            </div>
          </div>

          {/* 商品卡片 7 */}
          <div className="bg-white rounded-2xl shadow-sm overflow-hidden hover:shadow-lg transition-all duration-300 hover:-translate-y-2">
            <div className="relative">
              <img
                src="https://images.unsplash.com/photo-1526170375885-4d8ecf77b99f?w=300&h=200&fit=crop"
                alt="护肤品"
                className="w-full h-48 object-cover"
              />
              <div className="absolute top-3 left-3">
                <span className="bg-green-500 text-white px-3 py-1 rounded-full text-xs font-medium">免费提货券</span>
              </div>
              <div className="absolute top-3 right-3">
                <button
                  onClick={() => toggleLike(7)}
                  className={`w-8 h-8 rounded-full flex items-center justify-center transition-colors ${
                    likedProducts.has(7) ? 'bg-red-500 text-white' : 'bg-white/80 text-gray-600 hover:bg-white'
                  }`}
                >
                  <i className="fas fa-heart text-sm"></i>
                </button>
              </div>
            </div>
            <div className="p-4">
              <h3 className="font-bold text-gray-800 mb-2">护肤品套装</h3>
              <p className="text-sm text-gray-600 mb-3">天然植物精华，温和护肤，适合所有肌肤</p>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-lg font-bold text-green-600">免费</div>
                <div className="text-sm text-gray-500">原价: ¥158.00</div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-sm text-gray-500">
                  <i className="fas fa-box mr-1"></i>
                  剩余: 89
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-users mr-1"></i>
                  已获取: 211
                </div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center">
                  <div className="flex text-yellow-400">
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                  </div>
                  <span className="text-sm text-gray-600 ml-2">4.9 (178)</span>
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-clock mr-1"></i>
                  25天后过期
                </div>
              </div>
              
              <button 
                onClick={() => toggleCart(7)}
                className="w-full bg-gradient-to-r from-blue-500 to-purple-500 text-white py-2 px-4 rounded-lg font-medium hover:shadow-lg transition-all duration-300"
              >
                <i className="fas fa-download mr-2"></i>
                获取提货券
              </button>
            </div>
          </div>

          {/* 商品卡片 8 */}
          <div className="bg-white rounded-2xl shadow-sm overflow-hidden hover:shadow-lg transition-all duration-300 hover:-translate-y-2">
            <div className="relative">
              <img
                src="https://images.unsplash.com/photo-1515186629930-3c69b2b42829?w=300&h=200&fit=crop"
                alt="书籍"
                className="w-full h-48 object-cover"
              />
              <div className="absolute top-3 left-3">
                <span className="bg-green-500 text-white px-3 py-1 rounded-full text-xs font-medium">免费提货券</span>
              </div>
              <div className="absolute top-3 right-3">
                <button
                  onClick={() => toggleLike(8)}
                  className={`w-8 h-8 rounded-full flex items-center justify-center transition-colors ${
                    likedProducts.has(8) ? 'bg-red-500 text-white' : 'bg-white/80 text-gray-600 hover:bg-white'
                  }`}
                >
                  <i className="fas fa-heart text-sm"></i>
                </button>
              </div>
            </div>
            <div className="p-4">
              <h3 className="font-bold text-gray-800 mb-2">经典书籍</h3>
              <p className="text-sm text-gray-600 mb-3">知识与智慧的结晶，经典文学作品收藏</p>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-lg font-bold text-green-600">免费</div>
                <div className="text-sm text-gray-500">原价: ¥35.00</div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="text-sm text-gray-500">
                  <i className="fas fa-box mr-1"></i>
                  剩余: 345
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-users mr-1"></i>
                  已获取: 155
                </div>
              </div>
              
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center">
                  <div className="flex text-yellow-400">
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="fas fa-star"></i>
                    <i className="far fa-star"></i>
                  </div>
                  <span className="text-sm text-gray-600 ml-2">4.7 (123)</span>
                </div>
                <div className="text-sm text-gray-500">
                  <i className="fas fa-clock mr-1"></i>
                  45天后过期
                </div>
              </div>
              
              <button 
                onClick={() => toggleCart(8)}
                className="w-full bg-gradient-to-r from-blue-500 to-purple-500 text-white py-2 px-4 rounded-lg font-medium hover:shadow-lg transition-all duration-300"
              >
                <i className="fas fa-download mr-2"></i>
                获取提货券
              </button>
            </div>
          </div>
        </div>

        {/* 分页 */}
        <div className="flex justify-center mt-12">
          <nav className="flex space-x-2">
            <button className="px-4 py-2 bg-white border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors">
              <i className="fas fa-chevron-left"></i>
            </button>
            <button className="px-4 py-2 bg-blue-500 text-white rounded-lg">1</button>
            <button className="px-4 py-2 bg-white border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors">2</button>
            <button className="px-4 py-2 bg-white border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors">3</button>
            <button className="px-4 py-2 bg-white border border-gray-200 rounded-lg hover:bg-gray-50 transition-colors">
              <i className="fas fa-chevron-right"></i>
            </button>
          </nav>
        </div>
      </div>
    </div>
  )
} 