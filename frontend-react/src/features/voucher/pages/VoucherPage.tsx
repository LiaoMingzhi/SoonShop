import React, { useState } from 'react'

export const VoucherPage: React.FC = () => {
  const [activeTab, setActiveTab] = useState<'available' | 'used' | 'expired'>('available')
  const [searchTerm, setSearchTerm] = useState('')
  const [selectedCategory, setSelectedCategory] = useState('all')

  const categories = [
    { id: 'all', name: '全部' },
    { id: 'food', name: '餐饮' },
    { id: 'shopping', name: '购物' },
    { id: 'health', name: '健康' },
    { id: 'education', name: '教育' },
    { id: 'entertainment', name: '娱乐' },
  ]

  const vouchers = [
    {
      id: 1,
      title: '星巴克咖啡提货券',
      description: '可在任意星巴克门店使用',
      originalPrice: 38,
      category: 'food',
      expiryDate: '2024-12-31',
      image: 'https://images.unsplash.com/photo-1509042239860-f550ce710b93?w=300&h=200&fit=crop',
      isUsed: false,
      isExpired: false,
      rating: 4.8,
      reviews: 256,
      remaining: 89,
      claimed: 211,
    },
    {
      id: 2,
      title: '海底捞火锅提货券',
      description: '双人套餐，含锅底和小料',
      originalPrice: 198,
      category: 'food',
      expiryDate: '2024-11-30',
      image: 'https://images.unsplash.com/photo-1565299507177-b0ac66763376?w=300&h=200&fit=crop',
      isUsed: false,
      isExpired: false,
      rating: 4.9,
      reviews: 189,
      remaining: 45,
      claimed: 155,
    },
    {
      id: 3,
      title: '健身房月卡提货券',
      description: '包含私教课程和器械使用',
      originalPrice: 299,
      category: 'health',
      expiryDate: '2024-12-15',
      image: 'https://images.unsplash.com/photo-1534438327276-14e5300c3a48?w=300&h=200&fit=crop',
      isUsed: false,
      isExpired: false,
      rating: 4.7,
      reviews: 134,
      remaining: 78,
      claimed: 122,
    },
    {
      id: 4,
      title: '电影院观影券',
      description: 'IMAX厅观影，含爆米花饮料',
      originalPrice: 68,
      category: 'entertainment',
      expiryDate: '2024-11-20',
      image: 'https://images.unsplash.com/photo-1489185078373-5ac9b8c5e7b2?w=300&h=200&fit=crop',
      isUsed: false,
      isExpired: false,
      rating: 4.6,
      reviews: 298,
      remaining: 156,
      claimed: 144,
    },
    {
      id: 5,
      title: '知名品牌护肤套装',
      description: '包含洁面、水乳、精华',
      originalPrice: 158,
      category: 'shopping',
      expiryDate: '2024-12-25',
      image: 'https://images.unsplash.com/photo-1620916566398-39f1143ab7be?w=300&h=200&fit=crop',
      isUsed: false,
      isExpired: false,
      rating: 4.8,
      reviews: 167,
      remaining: 67,
      claimed: 233,
    },
    {
      id: 6,
      title: '在线课程学习券',
      description: '编程、设计、语言等课程',
      originalPrice: 99,
      category: 'education',
      expiryDate: '2024-12-31',
      image: 'https://images.unsplash.com/photo-1522202176988-66273c2fd55f?w=300&h=200&fit=crop',
      isUsed: false,
      isExpired: false,
      rating: 4.5,
      reviews: 89,
      remaining: 234,
      claimed: 66,
    },
  ]

  const filteredVouchers = vouchers.filter(voucher => {
    const matchesSearch = voucher.title.toLowerCase().includes(searchTerm.toLowerCase())
    const matchesCategory = selectedCategory === 'all' || voucher.category === selectedCategory
    
    switch (activeTab) {
      case 'available':
        return !voucher.isUsed && !voucher.isExpired && matchesSearch && matchesCategory
      case 'used':
        return voucher.isUsed && matchesSearch && matchesCategory
      case 'expired':
        return voucher.isExpired && matchesSearch && matchesCategory
      default:
        return matchesSearch && matchesCategory
    }
  })

  const handleUseVoucher = (voucherId: number) => {
    console.log('使用提货券:', voucherId)
  }

  return (
    <div className="min-h-screen bg-gray-50">
      {/* Header */}
      <div className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 py-6">
          <div className="flex items-center justify-between mb-6">
            <div>
              <h1 className="text-2xl font-bold text-gray-800">提货券管理</h1>
              <p className="text-gray-600 mt-1">管理您的提货券，随时查看使用状态</p>
            </div>
            <div className="flex items-center space-x-4">
              <div className="text-center">
                <div className="text-2xl font-bold text-blue-600">
                  {filteredVouchers.filter(v => !v.isUsed && !v.isExpired).length}
                </div>
                <div className="text-sm text-gray-600">可用</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-green-600">
                  {filteredVouchers.filter(v => v.isUsed).length}
                </div>
                <div className="text-sm text-gray-600">已用</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-red-600">
                  {filteredVouchers.filter(v => v.isExpired).length}
                </div>
                <div className="text-sm text-gray-600">过期</div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 py-6">
        {/* 统计卡片 */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          <div className="bg-white rounded-2xl p-6 shadow-sm">
            <div className="flex items-center">
              <div className="w-12 h-12 bg-gradient-to-br from-blue-500 to-cyan-500 rounded-xl flex items-center justify-center mr-4">
                <i className="fas fa-ticket-alt text-white"></i>
              </div>
              <div>
                <div className="text-2xl font-bold text-gray-800">24</div>
                <div className="text-sm text-gray-600">总提货券</div>
              </div>
            </div>
          </div>
          
          <div className="bg-white rounded-2xl p-6 shadow-sm">
            <div className="flex items-center">
              <div className="w-12 h-12 bg-gradient-to-br from-green-500 to-emerald-500 rounded-xl flex items-center justify-center mr-4">
                <i className="fas fa-check-circle text-white"></i>
              </div>
              <div>
                <div className="text-2xl font-bold text-gray-800">16</div>
                <div className="text-sm text-gray-600">可使用</div>
              </div>
            </div>
          </div>
          
          <div className="bg-white rounded-2xl p-6 shadow-sm">
            <div className="flex items-center">
              <div className="w-12 h-12 bg-gradient-to-br from-orange-500 to-red-500 rounded-xl flex items-center justify-center mr-4">
                <i className="fas fa-clock text-white"></i>
              </div>
              <div>
                <div className="text-2xl font-bold text-gray-800">5</div>
                <div className="text-sm text-gray-600">即将过期</div>
              </div>
            </div>
          </div>
          
          <div className="bg-white rounded-2xl p-6 shadow-sm">
            <div className="flex items-center">
              <div className="w-12 h-12 bg-gradient-to-br from-purple-500 to-indigo-500 rounded-xl flex items-center justify-center mr-4">
                <i className="fas fa-chart-line text-white"></i>
              </div>
              <div>
                <div className="text-2xl font-bold text-gray-800">￥2,856</div>
                <div className="text-sm text-gray-600">总价值</div>
              </div>
            </div>
          </div>
        </div>

        {/* 搜索和筛选 */}
        <div className="bg-white rounded-2xl shadow-sm p-6 mb-8">
          {/* 标签页 */}
          <div className="flex space-x-1 mb-6 bg-gray-100 rounded-xl p-1">
            <button
              onClick={() => setActiveTab('available')}
              className={`px-4 py-2 rounded-lg font-medium transition-colors flex-1 ${
                activeTab === 'available'
                  ? 'bg-white text-blue-600 shadow-sm'
                  : 'text-gray-600 hover:text-gray-800'
              }`}
            >
              <i className="fas fa-ticket-alt mr-2"></i>
              可用券
            </button>
            <button
              onClick={() => setActiveTab('used')}
              className={`px-4 py-2 rounded-lg font-medium transition-colors flex-1 ${
                activeTab === 'used'
                  ? 'bg-white text-green-600 shadow-sm'
                  : 'text-gray-600 hover:text-gray-800'
              }`}
            >
              <i className="fas fa-check-circle mr-2"></i>
              已使用
            </button>
            <button
              onClick={() => setActiveTab('expired')}
              className={`px-4 py-2 rounded-lg font-medium transition-colors flex-1 ${
                activeTab === 'expired'
                  ? 'bg-white text-red-600 shadow-sm'
                  : 'text-gray-600 hover:text-gray-800'
              }`}
            >
              <i className="fas fa-times-circle mr-2"></i>
              已过期
            </button>
          </div>

          {/* 搜索框 */}
          <div className="relative mb-6">
            <div className="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
              <i className="fas fa-search text-gray-400"></i>
            </div>
            <input
              type="text"
              placeholder="搜索提货券..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              className="w-full pl-12 pr-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-blue-500 focus:border-transparent"
            />
          </div>

          {/* 分类筛选 */}
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
                {category.name}
              </button>
            ))}
          </div>
        </div>

        {/* 提货券列表 */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredVouchers.map((voucher) => (
            <div key={voucher.id} className="bg-white rounded-2xl shadow-sm overflow-hidden card-hover">
              <div className="relative">
                <img
                  src={voucher.image}
                  alt={voucher.title}
                  className="w-full h-48 object-cover"
                />
                <div className="absolute top-3 left-3">
                  <span className="voucher-badge text-white px-3 py-1 rounded-full text-xs font-medium">
                    免费提货券
                  </span>
                </div>
                <div className="absolute top-3 right-3">
                  <div className="bg-white/90 backdrop-blur-sm rounded-full px-3 py-1">
                    <span className="text-xs font-medium text-gray-800">
                      {voucher.isUsed ? '已使用' : voucher.isExpired ? '已过期' : '可使用'}
                    </span>
                  </div>
                </div>
              </div>
              
              <div className="p-6">
                <h3 className="font-bold text-gray-800 mb-2">{voucher.title}</h3>
                <p className="text-sm text-gray-600 mb-4">{voucher.description}</p>
                
                <div className="flex items-center justify-between mb-4">
                  <div className="text-lg font-bold text-green-600">免费</div>
                  <div className="text-sm text-gray-500">原价: ¥{voucher.originalPrice}</div>
                </div>
                
                <div className="flex items-center justify-between mb-4">
                  <div className="text-sm text-gray-500">
                    <i className="fas fa-box mr-1"></i>
                    剩余: {voucher.remaining}
                  </div>
                  <div className="text-sm text-gray-500">
                    <i className="fas fa-users mr-1"></i>
                    已获取: {voucher.claimed}
                  </div>
                </div>
                
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center">
                    <div className="flex text-yellow-400">
                      {[...Array(5)].map((_, i) => (
                        <i key={i} className={`fas fa-star ${i < Math.floor(voucher.rating) ? '' : 'opacity-30'}`}></i>
                      ))}
                    </div>
                    <span className="text-sm text-gray-600 ml-2">{voucher.rating} ({voucher.reviews})</span>
                  </div>
                  <div className="text-sm text-gray-500">
                    <i className="fas fa-calendar mr-1"></i>
                    {voucher.expiryDate}
                  </div>
                </div>
                
                <button
                  onClick={() => handleUseVoucher(voucher.id)}
                  disabled={voucher.isUsed || voucher.isExpired}
                  className={`w-full py-3 px-4 rounded-xl font-medium transition-all duration-300 ${
                    voucher.isUsed || voucher.isExpired
                      ? 'bg-gray-300 text-gray-500 cursor-not-allowed'
                      : 'btn-primary text-white'
                  }`}
                >
                  {voucher.isUsed ? (
                    <>
                      <i className="fas fa-check mr-2"></i>
                      已使用
                    </>
                  ) : voucher.isExpired ? (
                    <>
                      <i className="fas fa-times mr-2"></i>
                      已过期
                    </>
                  ) : (
                    <>
                      <i className="fas fa-download mr-2"></i>
                      使用提货券
                    </>
                  )}
                </button>
              </div>
            </div>
          ))}
        </div>

        {/* 空状态 */}
        {filteredVouchers.length === 0 && (
          <div className="text-center py-12">
            <div className="w-24 h-24 bg-gray-100 rounded-full flex items-center justify-center mx-auto mb-4">
              <i className="fas fa-ticket-alt text-4xl text-gray-400"></i>
            </div>
            <h3 className="text-lg font-medium text-gray-900 mb-2">暂无提货券</h3>
            <p className="text-gray-600">
              {activeTab === 'available' && '您还没有可用的提货券'}
              {activeTab === 'used' && '您还没有使用过的提货券'}
              {activeTab === 'expired' && '您没有过期的提货券'}
            </p>
          </div>
        )}

        {/* 分页 */}
        {filteredVouchers.length > 0 && (
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
        )}
      </div>
    </div>
  )
} 