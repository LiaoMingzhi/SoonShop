import React, { useState, useEffect } from 'react';

const ConsumptionPage: React.FC = () => {
  const [consumptionAmount, setConsumptionAmount] = useState(100);
  const [qualityScore, setQualityScore] = useState(85);
  const [serviceScore, setServiceScore] = useState(92);
  const [welfareScore, setWelfareScore] = useState(88);
  const [finalMultiplier, setFinalMultiplier] = useState(20);

  // 计算倍增器
  useEffect(() => {
    const qualityBonus = Math.floor(qualityScore / 20);
    const serviceBonus = Math.floor(serviceScore / 20);
    const welfareBonus = Math.floor(welfareScore / 20);
    const total = 10 + qualityBonus + serviceBonus + welfareBonus;
    setFinalMultiplier(total);
  }, [qualityScore, serviceScore, welfareScore]);

  const consumptionHistory = [
    {
      id: 1,
      date: '2024-01-15',
      merchant: '有机农场',
      amount: 156.80,
      multiplier: 18,
      reward: 2822.40,
      category: '食品',
      rating: 4.9
    },
    {
      id: 2,
      date: '2024-01-14',
      merchant: '绿色餐厅',
      amount: 89.50,
      multiplier: 22,
      reward: 1969.00,
      category: '餐饮',
      rating: 4.8
    },
    {
      id: 3,
      date: '2024-01-13',
      merchant: '环保科技',
      amount: 299.99,
      multiplier: 15,
      reward: 4499.85,
      category: '数码',
      rating: 4.6
    },
    {
      id: 4,
      date: '2024-01-12',
      merchant: '公平贸易',
      amount: 78.20,
      multiplier: 25,
      reward: 1955.00,
      category: '服装',
      rating: 5.0
    }
  ];

  const leaderboard = [
    { rank: 1, name: '张三', avatar: '👨‍💼', totalReward: 125680, multiplier: 28 },
    { rank: 2, name: '李四', avatar: '👩‍💼', totalReward: 98540, multiplier: 25 },
    { rank: 3, name: '王五', avatar: '👨‍🎓', totalReward: 87320, multiplier: 22 },
    { rank: 4, name: '赵六', avatar: '👩‍🎓', totalReward: 76450, multiplier: 20 },
    { rank: 5, name: '你', avatar: '👤', totalReward: 65890, multiplier: 18 }
  ];

  const renderStars = (rating: number) => {
    return [...Array(5)].map((_, i) => (
      <i key={i} className={`fas fa-star ${i < Math.floor(rating) ? 'text-yellow-400' : 'text-gray-300'}`}></i>
    ));
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
                <i className="fas fa-chart-line text-2xl text-green-600"></i>
                <span className="ml-2 text-xl font-bold text-gray-800">消费与倍增</span>
              </button>
            </div>
            
            <div className="flex items-center space-x-4">
              <div className="flex items-center space-x-2 bg-green-100 rounded-full px-4 py-2">
                <i className="fas fa-coins text-green-600"></i>
                <span className="text-green-600 text-sm font-medium">当前倍增: {finalMultiplier}x</span>
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
        {/* 智能倍增系统概览 */}
        <div className="text-center mb-8">
          <h2 className="text-3xl font-bold text-gray-800 mb-4">智能倍增系统</h2>
          <p className="text-gray-600">基于共产主义经济原理，消费推动生产，实现经济良性循环</p>
        </div>
        
        {/* 统计概览 */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          <div className="stats-card rounded-2xl p-6 text-white">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-white/80 text-sm">本月消费</p>
                <p className="text-2xl font-bold">¥3,456</p>
              </div>
              <i className="fas fa-shopping-cart text-3xl text-white/60"></i>
            </div>
          </div>
          
          <div className="stats-card-green rounded-2xl p-6 text-white">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-white/80 text-sm">获得奖励</p>
                <p className="text-2xl font-bold">¥45,328</p>
              </div>
              <i className="fas fa-gift text-3xl text-white/60"></i>
            </div>
          </div>
          
          <div className="stats-card-orange rounded-2xl p-6 text-white">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-white/80 text-sm">平均倍增</p>
                <p className="text-2xl font-bold">13.1x</p>
              </div>
              <i className="fas fa-chart-line text-3xl text-white/60"></i>
            </div>
          </div>
          
          <div className="stats-card-purple rounded-2xl p-6 text-white">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-white/80 text-sm">评价得分</p>
                <p className="text-2xl font-bold">4.8/5</p>
              </div>
              <i className="fas fa-award text-3xl text-white/60"></i>
            </div>
          </div>
        </div>

        {/* 倍增计算器和奖励预览 */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-8">
          {/* 倍增计算器 */}
          <div className="bg-white rounded-2xl shadow-sm p-6">
            <h3 className="text-xl font-bold text-gray-800 mb-6 flex items-center">
              <i className="fas fa-calculator text-blue-500 mr-3"></i>
              倍增计算器
            </h3>
            
            <div className="space-y-6">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-3">消费金额 (¥)</label>
                <input
                  type="number"
                  value={consumptionAmount}
                  onChange={(e) => setConsumptionAmount(Number(e.target.value))}
                  className="w-full px-4 py-3 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-3">
                  产品质量评分 ({qualityScore}分)
                </label>
                <input
                  type="range"
                  value={qualityScore}
                  onChange={(e) => setQualityScore(Number(e.target.value))}
                  min="0"
                  max="100"
                  className="w-full"
                />
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-3">
                  服务质量评分 ({serviceScore}分)
                </label>
                <input
                  type="range"
                  value={serviceScore}
                  onChange={(e) => setServiceScore(Number(e.target.value))}
                  min="0"
                  max="100"
                  className="w-full"
                />
              </div>
              
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-3">
                  员工福利评分 ({welfareScore}分)
                </label>
                <input
                  type="range"
                  value={welfareScore}
                  onChange={(e) => setWelfareScore(Number(e.target.value))}
                  min="0"
                  max="100"
                  className="w-full"
                />
              </div>
            </div>
          </div>

          {/* 奖励预览 */}
          <div className="bg-white rounded-2xl shadow-sm p-6">
            <h3 className="text-xl font-bold text-gray-800 mb-6 flex items-center">
              <i className="fas fa-gift text-green-500 mr-3"></i>
              奖励预览
            </h3>
            
            <div className="space-y-4">
              <div className="bg-gray-50 rounded-lg p-4">
                <div className="flex justify-between items-center mb-2">
                  <span className="text-gray-600">消费金额</span>
                  <span className="font-bold text-gray-800">¥{consumptionAmount}</span>
                </div>
                <div className="flex justify-between items-center mb-2">
                  <span className="text-gray-600">倍增系数</span>
                  <span className="font-bold text-blue-600">{finalMultiplier}x</span>
                </div>
                <div className="border-t pt-2 mt-2">
                  <div className="flex justify-between items-center">
                    <span className="text-lg font-bold text-gray-800">预计奖励</span>
                    <span className="text-2xl font-bold text-green-600">
                      ¥{(consumptionAmount * finalMultiplier).toLocaleString()}
                    </span>
                  </div>
                </div>
              </div>
              
              <div className="text-sm text-gray-600 bg-blue-50 rounded-lg p-4">
                <h4 className="font-semibold mb-2">奖励分发机制：</h4>
                <ul className="space-y-1">
                  <li>• 50% - 生产者激励</li>
                  <li>• 30% - 平台发展基金</li>
                  <li>• 20% - 社会公益事业</li>
                </ul>
              </div>
            </div>
          </div>
        </div>

        {/* 消费历史 */}
        <div className="bg-white rounded-2xl shadow-sm p-6 mb-8">
          <h3 className="text-xl font-bold text-gray-800 mb-6 flex items-center">
            <i className="fas fa-history text-purple-500 mr-3"></i>
            消费历史
          </h3>
          
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {consumptionHistory.map((record) => (
              <div key={record.id} className="border border-gray-200 rounded-lg p-4 card-hover">
                <div className="flex justify-between items-start mb-3">
                  <div>
                    <h4 className="font-bold text-gray-800">{record.merchant}</h4>
                    <p className="text-sm text-gray-500">{record.date}</p>
                  </div>
                  <span className="text-xs bg-gray-100 text-gray-600 px-2 py-1 rounded">
                    {record.category}
                  </span>
                </div>
                
                <div className="space-y-2 mb-3">
                  <div className="flex justify-between">
                    <span className="text-sm text-gray-600">消费金额</span>
                    <span className="font-medium">¥{record.amount}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-sm text-gray-600">倍增系数</span>
                    <span className="font-medium text-blue-600">{record.multiplier}x</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-sm text-gray-600">获得奖励</span>
                    <span className="font-bold text-green-600">¥{record.reward}</span>
                  </div>
                </div>
                
                <div className="flex items-center justify-between">
                  <div className="flex items-center">
                    {renderStars(record.rating)}
                    <span className="text-sm text-gray-600 ml-1">{record.rating}</span>
                  </div>
                  <button className="text-sm text-blue-600 hover:text-blue-700">
                    查看详情
                  </button>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* 工作原理说明 */}
        <div className="bg-white rounded-2xl shadow-sm p-6">
          <h3 className="text-xl font-bold text-gray-800 mb-6 flex items-center">
            <i className="fas fa-info-circle text-blue-500 mr-3"></i>
            工作原理
          </h3>
          
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="text-center">
              <div className="w-16 h-16 bg-blue-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <i className="fas fa-shopping-cart text-2xl text-blue-600"></i>
              </div>
              <h4 className="font-bold text-gray-800 mb-2">消费驱动</h4>
              <p className="text-sm text-gray-600">
                消费者购买商品或服务，推动生产和就业
              </p>
            </div>
            
            <div className="text-center">
              <div className="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <i className="fas fa-chart-line text-2xl text-green-600"></i>
              </div>
              <h4 className="font-bold text-gray-800 mb-2">倍增奖励</h4>
              <p className="text-sm text-gray-600">
                根据评价得分计算倍增系数，奖励优质生产者
              </p>
            </div>
            
            <div className="text-center">
              <div className="w-16 h-16 bg-purple-100 rounded-full flex items-center justify-center mx-auto mb-4">
                <i className="fas fa-recycle text-2xl text-purple-600"></i>
              </div>
              <h4 className="font-bold text-gray-800 mb-2">良性循环</h4>
              <p className="text-sm text-gray-600">
                奖励再投入生产，形成可持续发展循环
              </p>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
};

export default ConsumptionPage; 