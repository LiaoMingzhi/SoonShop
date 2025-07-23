import React, { useState } from 'react';

const AnalyticsPage: React.FC = () => {
  const [selectedPeriod, setSelectedPeriod] = useState('week');
  const [selectedMetric, setSelectedMetric] = useState('users');

  const keyMetrics = [
    {
      title: '活跃用户',
      value: '12,567',
      change: '+15.8%',
      icon: 'fas fa-users',
      color: 'stats-card',
      trend: 'up'
    },
    {
      title: '提货券发放',
      value: '8,943',
      change: '+23.5%',
      icon: 'fas fa-ticket-alt',
      color: 'icon-gradient-green',
      trend: 'up'
    },
    {
      title: '交易总额',
      value: '¥1.2M',
      change: '+18.2%',
      icon: 'fas fa-chart-line',
      color: 'icon-gradient-orange',
      trend: 'up'
    },
    {
      title: '企业评估',
      value: '234',
      change: '-2.1%',
      icon: 'fas fa-building',
      color: 'icon-gradient-purple',
      trend: 'down'
    }
  ];

  const chartData = {
    users: {
      title: '用户增长趋势',
      data: [1200, 1350, 1280, 1560, 1690, 1820, 1950]
    },
    vouchers: {
      title: '提货券发放趋势',
      data: [800, 920, 850, 1100, 1230, 1180, 1340]
    },
    transactions: {
      title: '交易额趋势',
      data: [95000, 108000, 102000, 125000, 138000, 142000, 156000]
    },
    evaluation: {
      title: '企业评估趋势',
      data: [25, 32, 28, 35, 42, 38, 45]
    }
  };

  const recentTransactions = [
    {
      id: 1,
      user: '张三',
      item: '有机苹果提货券',
      amount: 9.99,
      type: 'claim',
      time: '2小时前',
      status: 'completed'
    },
    {
      id: 2,
      user: '李四',
      item: '智能手机提货券',
      amount: 2999.00,
      type: 'claim',
      time: '3小时前',
      status: 'processing'
    },
    {
      id: 3,
      user: '王五',
      item: '企业评估服务',
      amount: 0,
      type: 'evaluation',
      time: '5小时前',
      status: 'completed'
    },
    {
      id: 4,
      user: '赵六',
      item: '纯棉T恤提货券',
      amount: 89.90,
      type: 'claim',
      time: '6小时前',
      status: 'completed'
    },
    {
      id: 5,
      user: '钱七',
      item: '消费倍增奖励',
      amount: 456.78,
      type: 'reward',
      time: '8小时前',
      status: 'completed'
    }
  ];

  const topCategories = [
    { name: '食品', value: 35, color: 'bg-green-500' },
    { name: '数码', value: 28, color: 'bg-blue-500' },
    { name: '服装', value: 18, color: 'bg-purple-500' },
    { name: '日用品', value: 12, color: 'bg-orange-500' },
    { name: '其他', value: 7, color: 'bg-gray-500' }
  ];

  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'completed':
        return <span className="bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs font-medium">已完成</span>;
      case 'processing':
        return <span className="bg-yellow-100 text-yellow-800 px-2 py-1 rounded-full text-xs font-medium">处理中</span>;
      default:
        return <span className="bg-gray-100 text-gray-800 px-2 py-1 rounded-full text-xs font-medium">未知</span>;
    }
  };

  const getTypeIcon = (type: string) => {
    switch (type) {
      case 'claim':
        return <i className="fas fa-download text-blue-500"></i>;
      case 'evaluation':
        return <i className="fas fa-search text-purple-500"></i>;
      case 'reward':
        return <i className="fas fa-gift text-green-500"></i>;
      default:
        return <i className="fas fa-circle text-gray-500"></i>;
    }
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* 页面标题 */}
      <div className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center">
              <i className="fas fa-chart-bar text-3xl text-blue-600 mr-4"></i>
              <div>
                <h1 className="text-3xl font-bold text-gray-800">数据分析中心</h1>
                <p className="text-gray-600 mt-1">实时监控系统运行状况，洞察业务发展趋势</p>
              </div>
            </div>
            <div className="flex items-center space-x-3">
              <select
                value={selectedPeriod}
                onChange={(e) => setSelectedPeriod(e.target.value)}
                className="px-4 py-2 border border-gray-200 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 bg-white"
              >
                <option value="week">近7天</option>
                <option value="month">近30天</option>
                <option value="quarter">近90天</option>
                <option value="year">近1年</option>
              </select>
              <button className="btn-primary text-white px-6 py-2 rounded-lg font-medium">
                <i className="fas fa-download mr-2"></i>
                导出报告
              </button>
            </div>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* 关键指标概览 */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          {keyMetrics.map((metric, index) => (
            <div key={index} className={`${metric.color} rounded-2xl p-6 text-white`}>
              <div className="flex items-center justify-between mb-4">
                <div className="bg-white/20 w-12 h-12 rounded-xl flex items-center justify-center">
                  <i className={`${metric.icon} text-xl`}></i>
                </div>
                <span className={`text-sm font-medium ${
                  metric.trend === 'up' ? 'text-green-200' : 'text-red-200'
                }`}>
                  {metric.change}
                </span>
              </div>
              <h3 className="text-white/80 text-sm mb-2">{metric.title}</h3>
              <p className="text-3xl font-bold">{metric.value}</p>
            </div>
          ))}
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* 主要图表区域 */}
          <div className="lg:col-span-2 space-y-6">
            {/* 趋势图表 */}
            <div className="bg-white rounded-2xl shadow-sm p-6">
              <div className="flex items-center justify-between mb-6">
                <h3 className="text-xl font-bold text-gray-800">趋势分析</h3>
                <div className="flex space-x-2">
                  {Object.keys(chartData).map((key) => (
                    <button
                      key={key}
                      onClick={() => setSelectedMetric(key)}
                      className={`px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
                        selectedMetric === key
                          ? 'bg-blue-500 text-white'
                          : 'bg-gray-100 text-gray-600 hover:bg-gray-200'
                      }`}
                    >
                      {chartData[key as keyof typeof chartData].title.replace('趋势', '')}
                    </button>
                  ))}
                </div>
              </div>
              
              {/* 简化的图表显示 */}
              <div className="relative h-64 bg-gradient-to-t from-blue-50 to-transparent rounded-lg">
                <div className="absolute inset-0 flex items-end justify-between px-4 pb-4">
                  {chartData[selectedMetric as keyof typeof chartData].data.map((value, index) => {
                    const maxValue = Math.max(...chartData[selectedMetric as keyof typeof chartData].data);
                    const height = (value / maxValue) * 180;
                    return (
                      <div key={index} className="flex flex-col items-center">
                        <div
                          className="bg-blue-500 w-8 rounded-t transition-all duration-500 hover:bg-blue-600"
                          style={{ height: `${height}px` }}
                        ></div>
                        <span className="text-xs text-gray-600 mt-2">
                          {index === 0 ? '6天前' : 
                           index === 1 ? '5天前' :
                           index === 2 ? '4天前' :
                           index === 3 ? '3天前' :
                           index === 4 ? '2天前' :
                           index === 5 ? '昨天' : '今天'}
                        </span>
                      </div>
                    );
                  })}
                </div>
              </div>
              
              <div className="mt-4 text-center">
                <h4 className="font-medium text-gray-800 mb-2">
                  {chartData[selectedMetric as keyof typeof chartData].title}
                </h4>
                <p className="text-sm text-gray-600">
                  过去7天的数据变化趋势
                </p>
              </div>
            </div>

            {/* 最近交易 */}
            <div className="bg-white rounded-2xl shadow-sm p-6">
              <h3 className="text-xl font-bold text-gray-800 mb-6">最近交易记录</h3>
              
              <div className="space-y-4">
                {recentTransactions.map((transaction) => (
                  <div key={transaction.id} className="flex items-center justify-between p-4 bg-gray-50 rounded-xl hover:bg-gray-100 transition-colors">
                    <div className="flex items-center">
                      <div className="w-10 h-10 bg-white rounded-full flex items-center justify-center mr-4">
                        {getTypeIcon(transaction.type)}
                      </div>
                      <div>
                        <h4 className="font-medium text-gray-800">{transaction.user}</h4>
                        <p className="text-sm text-gray-600">{transaction.item}</p>
                      </div>
                    </div>
                    <div className="text-right">
                      <div className="flex items-center space-x-3">
                        <div>
                          {transaction.amount > 0 && (
                            <p className="font-bold text-gray-800">¥{transaction.amount}</p>
                          )}
                          <p className="text-sm text-gray-600">{transaction.time}</p>
                        </div>
                        {getStatusBadge(transaction.status)}
                      </div>
                    </div>
                  </div>
                ))}
              </div>
              
              <div className="mt-6 text-center">
                <button className="text-blue-600 hover:text-blue-700 font-medium">
                  查看更多交易记录 →
                </button>
              </div>
            </div>
          </div>

          {/* 侧边栏分析 */}
          <div className="space-y-6">
            {/* 分类统计 */}
            <div className="bg-white rounded-2xl shadow-sm p-6">
              <h3 className="text-lg font-bold text-gray-800 mb-6">热门分类</h3>
              
              <div className="space-y-4">
                {topCategories.map((category, index) => (
                  <div key={index}>
                    <div className="flex items-center justify-between mb-2">
                      <span className="text-sm font-medium text-gray-700">{category.name}</span>
                      <span className="text-sm text-gray-600">{category.value}%</span>
                    </div>
                    <div className="w-full bg-gray-200 rounded-full h-2">
                      <div
                        className={`${category.color} h-2 rounded-full transition-all duration-500`}
                        style={{ width: `${category.value}%` }}
                      ></div>
                    </div>
                  </div>
                ))}
              </div>
            </div>

            {/* 实时统计 */}
            <div className="bg-white rounded-2xl shadow-sm p-6">
              <h3 className="text-lg font-bold text-gray-800 mb-6">实时统计</h3>
              
              <div className="space-y-4">
                <div className="flex items-center justify-between p-3 bg-green-50 rounded-lg">
                  <div className="flex items-center">
                    <i className="fas fa-users text-green-600 mr-3"></i>
                    <span className="text-sm text-gray-700">在线用户</span>
                  </div>
                  <span className="font-bold text-green-600">1,234</span>
                </div>
                
                <div className="flex items-center justify-between p-3 bg-blue-50 rounded-lg">
                  <div className="flex items-center">
                    <i className="fas fa-download text-blue-600 mr-3"></i>
                    <span className="text-sm text-gray-700">今日提货券</span>
                  </div>
                  <span className="font-bold text-blue-600">567</span>
                </div>
                
                <div className="flex items-center justify-between p-3 bg-orange-50 rounded-lg">
                  <div className="flex items-center">
                    <i className="fas fa-coins text-orange-600 mr-3"></i>
                    <span className="text-sm text-gray-700">今日交易额</span>
                  </div>
                  <span className="font-bold text-orange-600">¥89.2K</span>
                </div>
                
                <div className="flex items-center justify-between p-3 bg-purple-50 rounded-lg">
                  <div className="flex items-center">
                    <i className="fas fa-building text-purple-600 mr-3"></i>
                    <span className="text-sm text-gray-700">新增企业</span>
                  </div>
                  <span className="font-bold text-purple-600">12</span>
                </div>
              </div>
            </div>

            {/* 快速操作 */}
            <div className="bg-white rounded-2xl shadow-sm p-6">
              <h3 className="text-lg font-bold text-gray-800 mb-6">快速操作</h3>
              
              <div className="space-y-3">
                <button className="w-full bg-blue-500 text-white py-3 px-4 rounded-lg hover:bg-blue-600 transition-colors text-left">
                  <i className="fas fa-chart-line mr-3"></i>
                  生成报告
                </button>
                <button className="w-full bg-green-500 text-white py-3 px-4 rounded-lg hover:bg-green-600 transition-colors text-left">
                  <i className="fas fa-filter mr-3"></i>
                  数据筛选
                </button>
                <button className="w-full bg-orange-500 text-white py-3 px-4 rounded-lg hover:bg-orange-600 transition-colors text-left">
                  <i className="fas fa-bell mr-3"></i>
                  设置提醒
                </button>
                <button className="w-full bg-purple-500 text-white py-3 px-4 rounded-lg hover:bg-purple-600 transition-colors text-left">
                  <i className="fas fa-cog mr-3"></i>
                  配置面板
                </button>
              </div>
            </div>

            {/* 系统状态 */}
            <div className="bg-white rounded-2xl shadow-sm p-6">
              <h3 className="text-lg font-bold text-gray-800 mb-6">系统状态</h3>
              
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <span className="text-sm text-gray-700">API响应时间</span>
                  <div className="flex items-center">
                    <div className="w-2 h-2 bg-green-500 rounded-full mr-2"></div>
                    <span className="text-sm font-medium text-green-600">125ms</span>
                  </div>
                </div>
                
                <div className="flex items-center justify-between">
                  <span className="text-sm text-gray-700">数据库状态</span>
                  <div className="flex items-center">
                    <div className="w-2 h-2 bg-green-500 rounded-full mr-2"></div>
                    <span className="text-sm font-medium text-green-600">正常</span>
                  </div>
                </div>
                
                <div className="flex items-center justify-between">
                  <span className="text-sm text-gray-700">缓存命中率</span>
                  <div className="flex items-center">
                    <div className="w-2 h-2 bg-yellow-500 rounded-full mr-2"></div>
                    <span className="text-sm font-medium text-yellow-600">89.2%</span>
                  </div>
                </div>
                
                <div className="flex items-center justify-between">
                  <span className="text-sm text-gray-700">服务器负载</span>
                  <div className="flex items-center">
                    <div className="w-2 h-2 bg-green-500 rounded-full mr-2"></div>
                    <span className="text-sm font-medium text-green-600">23%</span>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* 详细数据表格 */}
        <div className="bg-white rounded-2xl shadow-sm p-6 mt-8">
          <h3 className="text-xl font-bold text-gray-800 mb-6">详细数据分析</h3>
          
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-gray-200">
                  <th className="text-left py-3 px-4 font-medium text-gray-700">时间</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-700">活跃用户</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-700">提货券发放</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-700">交易额</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-700">企业评估</th>
                  <th className="text-left py-3 px-4 font-medium text-gray-700">增长率</th>
                </tr>
              </thead>
              <tbody>
                {Array.from({ length: 7 }, (_, i) => (
                  <tr key={i} className="border-b border-gray-100 hover:bg-gray-50">
                    <td className="py-3 px-4 text-gray-600">
                      {new Date(Date.now() - i * 24 * 60 * 60 * 1000).toLocaleDateString()}
                    </td>
                    <td className="py-3 px-4 font-medium text-gray-800">{1200 + i * 150}</td>
                    <td className="py-3 px-4 font-medium text-gray-800">{800 + i * 120}</td>
                    <td className="py-3 px-4 font-medium text-gray-800">¥{(95 + i * 8).toFixed(1)}K</td>
                    <td className="py-3 px-4 font-medium text-gray-800">{25 + i * 7}</td>
                    <td className="py-3 px-4">
                      <span className="bg-green-100 text-green-800 px-2 py-1 rounded-full text-xs font-medium">
                        +{(12 + i * 2).toFixed(1)}%
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
};

export default AnalyticsPage; 