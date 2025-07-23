import React, { useState } from 'react';

const EvaluationPage: React.FC = () => {
  const [selectedCompany, setSelectedCompany] = useState<number | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [filterIndustry, setFilterIndustry] = useState('all');

  const evaluationDimensions = [
    {
      id: 'social',
      name: '社会责任',
      icon: 'fas fa-hands-helping',
      color: 'text-blue-600',
      bgColor: 'bg-blue-50',
      description: '企业对社会的贡献和责任承担'
    },
    {
      id: 'environment',
      name: '环保指标',
      icon: 'fas fa-leaf',
      color: 'text-green-600',
      bgColor: 'bg-green-50',
      description: '环境保护和可持续发展措施'
    },
    {
      id: 'quality',
      name: '产品质量',
      icon: 'fas fa-award',
      color: 'text-yellow-600',
      bgColor: 'bg-yellow-50',
      description: '产品质量和用户满意度'
    },
    {
      id: 'innovation',
      name: '创新能力',
      icon: 'fas fa-lightbulb',
      color: 'text-purple-600',
      bgColor: 'bg-purple-50',
      description: '技术创新和研发投入'
    },
    {
      id: 'employee',
      name: '员工福利',
      icon: 'fas fa-users',
      color: 'text-orange-600',
      bgColor: 'bg-orange-50',
      description: '员工薪酬福利和工作环境'
    },
    {
      id: 'transparency',
      name: '财务透明',
      icon: 'fas fa-chart-pie',
      color: 'text-indigo-600',
      bgColor: 'bg-indigo-50',
      description: '财务信息透明度和治理结构'
    }
  ];

  const companies = [
    {
      id: 1,
      name: '绿色科技有限公司',
      industry: '科技',
      logo: '🌱',
      overallScore: 92,
      scores: {
        social: 88,
        environment: 95,
        quality: 90,
        innovation: 94,
        employee: 89,
        transparency: 92
      },
      employees: 2500,
      revenue: 12.5,
      founded: 2015,
      location: '深圳',
      description: '专注于环保技术和可再生能源解决方案'
    },
    {
      id: 2,
      name: '有机农业集团',
      industry: '农业',
      logo: '🌾',
      overallScore: 89,
      scores: {
        social: 91,
        environment: 93,
        quality: 87,
        innovation: 82,
        employee: 90,
        transparency: 88
      },
      employees: 1800,
      revenue: 8.3,
      founded: 2012,
      location: '成都',
      description: '有机农产品种植和生产，推广绿色农业'
    },
    {
      id: 3,
      name: '公平贸易服装',
      industry: '制造',
      logo: '👕',
      overallScore: 85,
      scores: {
        social: 92,
        environment: 85,
        quality: 88,
        innovation: 78,
        employee: 95,
        transparency: 87
      },
      employees: 3200,
      revenue: 15.7,
      founded: 2010,
      location: '上海',
      description: '公平贸易服装生产，关注工人权益和环保'
    },
    {
      id: 4,
      name: '清洁能源股份',
      industry: '能源',
      logo: '⚡',
      overallScore: 87,
      scores: {
        social: 84,
        environment: 96,
        quality: 86,
        innovation: 91,
        employee: 83,
        transparency: 90
      },
      employees: 4500,
      revenue: 28.9,
      founded: 2008,
      location: '北京',
      description: '太阳能和风能发电设施建设运营'
    },
    {
      id: 5,
      name: '智慧医疗科技',
      industry: '医疗',
      logo: '🏥',
      overallScore: 91,
      scores: {
        social: 95,
        environment: 82,
        quality: 93,
        innovation: 96,
        employee: 88,
        transparency: 91
      },
      employees: 1950,
      revenue: 18.6,
      founded: 2016,
      location: '杭州',
      description: '人工智能医疗诊断和健康管理系统'
    },
    {
      id: 6,
      name: '循环经济工业',
      industry: '制造',
      logo: '♻️',
      overallScore: 84,
      scores: {
        social: 86,
        environment: 89,
        quality: 82,
        innovation: 85,
        employee: 81,
        transparency: 83
      },
      employees: 2800,
      revenue: 22.1,
      founded: 2013,
      location: '广州',
      description: '废料回收再利用和循环经济产业链'
    }
  ];

  const filteredCompanies = companies.filter(company => {
    const matchesSearch = company.name.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesIndustry = filterIndustry === 'all' || company.industry === filterIndustry;
    return matchesSearch && matchesIndustry;
  });

  const industries = ['all', '科技', '农业', '制造', '能源', '医疗'];

  const getScoreColor = (score: number) => {
    if (score >= 90) return 'text-green-600';
    if (score >= 80) return 'text-yellow-600';
    if (score >= 70) return 'text-orange-600';
    return 'text-red-600';
  };

  const getScoreBgColor = (score: number) => {
    if (score >= 90) return 'bg-green-500';
    if (score >= 80) return 'bg-yellow-500';
    if (score >= 70) return 'bg-orange-500';
    return 'bg-red-500';
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* 页面标题 */}
      <div className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
          <div className="flex items-center justify-between">
            <div className="flex items-center">
              <i className="fas fa-search text-3xl text-purple-600 mr-4"></i>
              <div>
                <h1 className="text-3xl font-bold text-gray-800">企业评估系统</h1>
                <p className="text-gray-600 mt-1">6维度全面评估企业社会责任和可持续发展能力</p>
              </div>
            </div>
            <button className="btn-primary text-white px-6 py-3 rounded-xl font-medium hover:scale-105 transition-all duration-200">
              <i className="fas fa-plus mr-2"></i>
              提交评估
            </button>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        {/* 评估统计概览 */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          <div className="stats-card rounded-2xl p-6 text-white">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-white/80 text-sm">已评估企业</p>
                <p className="text-3xl font-bold mt-2">1,248</p>
              </div>
              <i className="fas fa-building text-4xl text-white/60"></i>
            </div>
          </div>
          
          <div className="icon-gradient-green rounded-2xl p-6 text-white">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-white/80 text-sm">优秀企业</p>
                <p className="text-3xl font-bold mt-2">356</p>
              </div>
              <i className="fas fa-star text-4xl text-white/60"></i>
            </div>
          </div>
          
          <div className="icon-gradient-orange rounded-2xl p-6 text-white">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-white/80 text-sm">本月新增</p>
                <p className="text-3xl font-bold mt-2">48</p>
              </div>
              <i className="fas fa-plus-circle text-4xl text-white/60"></i>
            </div>
          </div>
          
          <div className="icon-gradient-purple rounded-2xl p-6 text-white">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-white/80 text-sm">平均得分</p>
                <p className="text-3xl font-bold mt-2">84.2</p>
              </div>
              <i className="fas fa-chart-line text-4xl text-white/60"></i>
            </div>
          </div>
        </div>

        {/* 评估维度说明 */}
        <div className="bg-white rounded-2xl shadow-sm p-6 mb-8">
          <h3 className="text-xl font-bold text-gray-800 mb-6 flex items-center">
            <i className="fas fa-compass text-blue-500 mr-3"></i>
            六维度评估体系
          </h3>
          
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
            {evaluationDimensions.map((dimension) => (
              <div key={dimension.id} className="text-center">
                <div className={`${dimension.bgColor} w-20 h-20 rounded-full flex items-center justify-center mx-auto mb-4`}>
                  <i className={`${dimension.icon} ${dimension.color} text-2xl`}></i>
                </div>
                <h4 className="font-bold text-gray-800 mb-2">{dimension.name}</h4>
                <p className="text-sm text-gray-600">{dimension.description}</p>
              </div>
            ))}
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
                  placeholder="搜索企业名称..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="w-full pl-10 pr-4 py-3 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200"
                />
              </div>
            </div>
            <div className="flex gap-3">
              <select
                value={filterIndustry}
                onChange={(e) => setFilterIndustry(e.target.value)}
                className="px-4 py-3 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent bg-white"
              >
                <option value="all">全部行业</option>
                {industries.slice(1).map(industry => (
                  <option key={industry} value={industry}>{industry}</option>
                ))}
              </select>
              <button className="px-6 py-3 bg-blue-500 text-white rounded-xl hover:bg-blue-600 transition-colors">
                <i className="fas fa-filter mr-2"></i>
                筛选
              </button>
            </div>
          </div>
        </div>

        {/* 企业列表 */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {filteredCompanies.map((company) => (
            <div key={company.id} className="bg-white rounded-2xl shadow-sm overflow-hidden card-hover">
              <div className="p-6">
                <div className="flex items-center justify-between mb-4">
                  <div className="flex items-center">
                    <div className="text-4xl mr-4">{company.logo}</div>
                    <div>
                      <h3 className="text-lg font-bold text-gray-800">{company.name}</h3>
                      <p className="text-sm text-gray-600">{company.industry} • {company.location}</p>
                    </div>
                  </div>
                  <div className="text-right">
                    <div className={`text-3xl font-bold ${getScoreColor(company.overallScore)}`}>
                      {company.overallScore}
                    </div>
                    <div className="text-sm text-gray-600">综合得分</div>
                  </div>
                </div>
                
                <p className="text-gray-600 text-sm mb-4">{company.description}</p>
                
                <div className="grid grid-cols-3 gap-4 mb-4">
                  <div className="text-center">
                    <p className="text-sm text-gray-600">员工数</p>
                    <p className="font-bold text-gray-800">{company.employees.toLocaleString()}</p>
                  </div>
                  <div className="text-center">
                    <p className="text-sm text-gray-600">年收入</p>
                    <p className="font-bold text-gray-800">{company.revenue}亿</p>
                  </div>
                  <div className="text-center">
                    <p className="text-sm text-gray-600">成立年份</p>
                    <p className="font-bold text-gray-800">{company.founded}</p>
                  </div>
                </div>
                
                {/* 六维度评分 */}
                <div className="mb-4">
                  <h4 className="font-medium text-gray-800 mb-3">六维度评分</h4>
                  <div className="grid grid-cols-3 gap-3">
                    {evaluationDimensions.map((dimension) => (
                      <div key={dimension.id} className="flex items-center">
                        <i className={`${dimension.icon} ${dimension.color} text-sm mr-2`}></i>
                        <span className="text-sm text-gray-600 flex-1">{dimension.name}</span>
                        <span className={`text-sm font-bold ${getScoreColor(company.scores[dimension.id as keyof typeof company.scores])}`}>
                          {company.scores[dimension.id as keyof typeof company.scores]}
                        </span>
                      </div>
                    ))}
                  </div>
                </div>
                
                {/* 评分进度条 */}
                <div className="mb-4">
                  <div className="flex justify-between text-sm text-gray-600 mb-2">
                    <span>综合评分</span>
                    <span>{company.overallScore}/100</span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-3">
                    <div
                      className={`h-3 rounded-full transition-all duration-500 ${getScoreBgColor(company.overallScore)}`}
                      style={{ width: `${company.overallScore}%` }}
                    ></div>
                  </div>
                </div>
                
                <div className="flex gap-2">
                  <button
                    onClick={() => setSelectedCompany(company.id)}
                    className="flex-1 bg-blue-500 text-white py-2 px-4 rounded-lg hover:bg-blue-600 transition-colors"
                  >
                    <i className="fas fa-eye mr-2"></i>
                    查看详情
                  </button>
                  <button className="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors">
                    <i className="fas fa-heart"></i>
                  </button>
                  <button className="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors">
                    <i className="fas fa-share-alt"></i>
                  </button>
                </div>
              </div>
            </div>
          ))}
        </div>

        {/* 空状态 */}
        {filteredCompanies.length === 0 && (
          <div className="bg-white rounded-2xl shadow-sm p-12 text-center">
            <i className="fas fa-search text-6xl text-gray-300 mb-4"></i>
            <h3 className="text-lg font-medium text-gray-800 mb-2">未找到匹配的企业</h3>
            <p className="text-gray-600 mb-6">试试调整搜索关键词或选择其他行业</p>
            <button
              onClick={() => {
                setSearchTerm('');
                setFilterIndustry('all');
              }}
              className="btn-primary text-white px-6 py-3 rounded-xl font-medium"
            >
              <i className="fas fa-refresh mr-2"></i>
              重置筛选
            </button>
          </div>
        )}

        {/* 行业分布图表 */}
        <div className="bg-white rounded-2xl shadow-sm p-6 mt-8">
          <h3 className="text-xl font-bold text-gray-800 mb-6 flex items-center">
            <i className="fas fa-chart-donut text-green-500 mr-3"></i>
            行业分布统计
          </h3>
          
          <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
            {industries.slice(1).map((industry, index) => {
              const count = companies.filter(c => c.industry === industry).length;
              const percentage = Math.round((count / companies.length) * 100);
              const colors = [
                'bg-blue-500',
                'bg-green-500',
                'bg-yellow-500',
                'bg-purple-500',
                'bg-orange-500'
              ];
              
              return (
                <div key={industry} className="text-center">
                  <div className={`${colors[index]} w-16 h-16 rounded-full flex items-center justify-center mx-auto mb-3`}>
                    <span className="text-white font-bold text-lg">{count}</span>
                  </div>
                  <h4 className="font-medium text-gray-800 mb-1">{industry}</h4>
                  <p className="text-sm text-gray-600">{percentage}%</p>
                </div>
              );
            })}
          </div>
        </div>

        {/* 快速评估工具 */}
        <div className="bg-white rounded-2xl shadow-sm p-6 mt-8">
          <h3 className="text-xl font-bold text-gray-800 mb-6 flex items-center">
            <i className="fas fa-bolt text-yellow-500 mr-3"></i>
            快速评估工具
          </h3>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">企业名称</label>
                <input
                  type="text"
                  placeholder="请输入企业名称"
                  className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                />
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">所属行业</label>
                <select className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent bg-white">
                  <option value="">选择行业</option>
                  {industries.slice(1).map(industry => (
                    <option key={industry} value={industry}>{industry}</option>
                  ))}
                </select>
              </div>
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">企业规模</label>
                <select className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent bg-white">
                  <option value="">选择规模</option>
                  <option value="small">小型企业（50人以下）</option>
                  <option value="medium">中型企业（50-500人）</option>
                  <option value="large">大型企业（500人以上）</option>
                </select>
              </div>
            </div>
            
            <div className="space-y-4">
              <div>
                <label className="block text-sm font-medium text-gray-700 mb-2">评估描述</label>
                <textarea
                  placeholder="请描述企业的特点、优势和社会责任表现..."
                  rows={4}
                  className="w-full px-4 py-3 border border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent resize-none"
                ></textarea>
              </div>
              <button className="w-full bg-gradient-to-r from-blue-500 to-purple-600 text-white py-3 px-6 rounded-xl font-medium hover:scale-105 transition-all duration-200">
                <i className="fas fa-magic mr-2"></i>
                开始评估
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default EvaluationPage; 