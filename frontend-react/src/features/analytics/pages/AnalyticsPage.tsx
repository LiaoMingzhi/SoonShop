import React, { useState, useEffect } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { 
  AnalyticsIcon, 
  PeopleIcon, 
  UnityIcon, 
  ProsperityIcon,
  CommunistIcon,
  StarIcon,
  CheckIcon,
  ChevronRightIcon,
  TrendingUpIcon,
  TrendingDownIcon
} from '@/components/ui/Icon';
import { cn } from '@/utils/classnames';

// 数据接口
interface PlatformMetrics {
  totalUsers: number;
  activeUsers: number;
  totalTransactions: number;
  totalValue: number;
  voucherUtilization: number;
  productionEfficiency: number;
  peopleRating: number;
  socialValue: number;
}

interface TrendData {
  period: string;
  value: number;
  growth: number;
}

interface CategoryData {
  category: string;
  displayName: string;
  value: number;
  percentage: number;
  growth: number;
}

interface RegionData {
  region: string;
  users: number;
  transactions: number;
  satisfaction: number;
}

export const AnalyticsPage: React.FC = () => {
  const [metrics, setMetrics] = useState<PlatformMetrics>({
    totalUsers: 0,
    activeUsers: 0,
    totalTransactions: 0,
    totalValue: 0,
    voucherUtilization: 0,
    productionEfficiency: 0,
    peopleRating: 0,
    socialValue: 0
  });

  const [timeRange, setTimeRange] = useState<'week' | 'month' | 'quarter' | 'year'>('month');
  const [selectedMetric, setSelectedMetric] = useState<'users' | 'transactions' | 'value' | 'satisfaction'>('users');

  // 模拟数据
  useEffect(() => {
    const mockMetrics: PlatformMetrics = {
      totalUsers: 89240,
      activeUsers: 67180,
      totalTransactions: 234567,
      totalValue: 12456789,
      voucherUtilization: 87.5,
      productionEfficiency: 92.3,
      peopleRating: 4.8,
      socialValue: 94.2
    };
    setMetrics(mockMetrics);
  }, []);

  // 趋势数据
  const trendData: TrendData[] = [
    { period: '1月', value: 45230, growth: 12.5 },
    { period: '2月', value: 52340, growth: 15.7 },
    { period: '3月', value: 48560, growth: -7.2 },
    { period: '4月', value: 61230, growth: 26.1 },
    { period: '5月', value: 67180, growth: 9.7 },
    { period: '6月', value: 73450, growth: 9.3 }
  ];

  // 分类数据
  const categoryData: CategoryData[] = [
    { category: 'daily', displayName: '日用品', value: 3456789, percentage: 35.2, growth: 8.5 },
    { category: 'food', displayName: '食品', value: 2345678, percentage: 23.8, growth: 12.3 },
    { category: 'education', displayName: '教育', value: 1789012, percentage: 18.1, growth: 15.7 },
    { category: 'healthcare', displayName: '医疗', value: 1234567, percentage: 12.5, growth: 6.2 },
    { category: 'housing', displayName: '住房', value: 890123, percentage: 9.0, growth: 4.8 },
    { category: 'others', displayName: '其他', value: 134567, percentage: 1.4, growth: -2.1 }
  ];

  // 地区数据
  const regionData: RegionData[] = [
    { region: '华北地区', users: 23456, transactions: 45678, satisfaction: 4.9 },
    { region: '华东地区', users: 34567, transactions: 67890, satisfaction: 4.8 },
    { region: '华南地区', users: 19876, transactions: 38765, satisfaction: 4.7 },
    { region: '华中地区', users: 15432, transactions: 29876, satisfaction: 4.8 },
    { region: '西南地区', users: 12345, transactions: 23456, satisfaction: 4.6 },
    { region: '西北地区', users: 8765, transactions: 16543, satisfaction: 4.5 }
  ];

  const getGrowthColor = (growth: number) => {
    if (growth > 0) return 'text-green-600';
    if (growth < 0) return 'text-red-600';
    return 'text-gray-600';
  };

  const getGrowthIcon = (growth: number) => {
    if (growth > 0) return <TrendingUpIcon size="xs" className="text-green-600" />;
    if (growth < 0) return <TrendingDownIcon size="xs" className="text-red-600" />;
    return null;
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-indigo-50 via-white to-purple-50">
      {/* 头部区域 - 桌面优化 */}
      <div className="bg-analytics-gradient text-white">
        <div className="container mx-auto px-8 py-16 desktop:py-20 large:py-24">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-6 desktop:space-x-8">
              <div className="p-4 desktop:p-6 bg-white/20 rounded-2xl backdrop-blur-sm">
                <AnalyticsIcon size="xl" className="text-white desktop:w-16 desktop:h-16" />
              </div>
              <div>
                <h1 className="text-5xl desktop:text-6xl large:text-7xl font-bold">数据分析中心</h1>
                <p className="text-2xl desktop:text-3xl opacity-90 mt-3 desktop:mt-4">透明管理 · 科学决策 · 数据驱动</p>
              </div>
            </div>
            <div className="flex gap-4 desktop:gap-6">
              <Button 
                variant={timeRange === 'week' ? 'primary' : 'outline'}
                size="lg"
                onClick={() => setTimeRange('week')}
                className="text-lg desktop:text-xl"
              >
                本周
              </Button>
              <Button 
                variant={timeRange === 'month' ? 'primary' : 'outline'}
                size="lg"
                onClick={() => setTimeRange('month')}
                className="text-lg desktop:text-xl"
              >
                本月
              </Button>
              <Button 
                variant={timeRange === 'quarter' ? 'primary' : 'outline'}
                size="lg"
                onClick={() => setTimeRange('quarter')}
                className="text-lg desktop:text-xl"
              >
                本季度
              </Button>
              <Button 
                variant={timeRange === 'year' ? 'primary' : 'outline'}
                size="lg"
                onClick={() => setTimeRange('year')}
                className="text-lg desktop:text-xl"
              >
                本年
              </Button>
            </div>
          </div>
        </div>
      </div>

      <div className="container mx-auto px-8 py-12 desktop:py-16 large:py-20 space-y-12 desktop:space-y-16">
        {/* 核心指标 - 桌面优化 */}
        <section className="grid grid-cols-8 gap-6 desktop:gap-8 large:gap-10">
          <Card className="text-center p-6 desktop:p-8 large:p-10 hover-lift">
            <CardContent className="space-y-4 desktop:space-y-6">
              <div className="text-3xl desktop:text-4xl large:text-5xl font-bold text-communist-gradient">
                {metrics.totalUsers.toLocaleString()}
              </div>
              <div className="text-lg desktop:text-xl text-gray-600">总用户数</div>
              <div className="flex items-center justify-center space-x-2 desktop:space-x-3">
                {getGrowthIcon(12.5)}
                <span className={cn("text-base desktop:text-lg", getGrowthColor(12.5))}>
                  +12.5%
                </span>
              </div>
            </CardContent>
          </Card>

          <Card className="text-center p-6 desktop:p-8 large:p-10 hover-lift">
            <CardContent className="space-y-4 desktop:space-y-6">
              <div className="text-3xl desktop:text-4xl large:text-5xl font-bold text-people-gradient">
                {metrics.activeUsers.toLocaleString()}
              </div>
              <div className="text-lg desktop:text-xl text-gray-600">活跃用户</div>
              <div className="flex items-center justify-center space-x-2 desktop:space-x-3">
                {getGrowthIcon(8.3)}
                <span className={cn("text-base desktop:text-lg", getGrowthColor(8.3))}>
                  +8.3%
                </span>
              </div>
            </CardContent>
          </Card>

          <Card className="text-center p-6 desktop:p-8 large:p-10 hover-lift">
            <CardContent className="space-y-4 desktop:space-y-6">
              <div className="text-3xl desktop:text-4xl large:text-5xl font-bold text-prosperity-gradient">
                {metrics.totalTransactions.toLocaleString()}
              </div>
              <div className="text-lg desktop:text-xl text-gray-600">总交易数</div>
              <div className="flex items-center justify-center space-x-2 desktop:space-x-3">
                {getGrowthIcon(15.7)}
                <span className={cn("text-base desktop:text-lg", getGrowthColor(15.7))}>
                  +15.7%
                </span>
              </div>
            </CardContent>
          </Card>

          <Card className="text-center p-6 desktop:p-8 large:p-10 hover-lift">
            <CardContent className="space-y-4 desktop:space-y-6">
              <div className="text-3xl desktop:text-4xl large:text-5xl font-bold text-orange-500">
                ¥{(metrics.totalValue / 10000).toFixed(1)}万
              </div>
              <div className="text-lg desktop:text-xl text-gray-600">总价值</div>
              <div className="flex items-center justify-center space-x-2 desktop:space-x-3">
                {getGrowthIcon(23.4)}
                <span className={cn("text-base desktop:text-lg", getGrowthColor(23.4))}>
                  +23.4%
                </span>
              </div>
            </CardContent>
          </Card>

          <Card className="text-center p-6 desktop:p-8 large:p-10 hover-lift">
            <CardContent className="space-y-4 desktop:space-y-6">
              <div className="text-3xl desktop:text-4xl large:text-5xl font-bold text-green-500">
                {metrics.voucherUtilization}%
              </div>
              <div className="text-lg desktop:text-xl text-gray-600">券使用率</div>
              <div className="flex items-center justify-center space-x-2 desktop:space-x-3">
                {getGrowthIcon(5.2)}
                <span className={cn("text-base desktop:text-lg", getGrowthColor(5.2))}>
                  +5.2%
                </span>
              </div>
            </CardContent>
          </Card>

          <Card className="text-center p-6 desktop:p-8 large:p-10 hover-lift">
            <CardContent className="space-y-4 desktop:space-y-6">
              <div className="text-3xl desktop:text-4xl large:text-5xl font-bold text-blue-500">
                {metrics.productionEfficiency}%
              </div>
              <div className="text-lg desktop:text-xl text-gray-600">生产效率</div>
              <div className="flex items-center justify-center space-x-2 desktop:space-x-3">
                {getGrowthIcon(3.8)}
                <span className={cn("text-base desktop:text-lg", getGrowthColor(3.8))}>
                  +3.8%
                </span>
              </div>
            </CardContent>
          </Card>

          <Card className="text-center p-6 desktop:p-8 large:p-10 hover-lift">
            <CardContent className="space-y-4 desktop:space-y-6">
              <div className="text-3xl desktop:text-4xl large:text-5xl font-bold text-yellow-500 flex items-center justify-center">
                {metrics.peopleRating}
                <StarIcon size="lg" className="ml-2 desktop:ml-3" />
              </div>
              <div className="text-lg desktop:text-xl text-gray-600">人民评分</div>
              <div className="flex items-center justify-center space-x-2 desktop:space-x-3">
                {getGrowthIcon(2.1)}
                <span className={cn("text-base desktop:text-lg", getGrowthColor(2.1))}>
                  +2.1%
                </span>
              </div>
            </CardContent>
          </Card>

          <Card className="text-center p-6 desktop:p-8 large:p-10 hover-lift">
            <CardContent className="space-y-4 desktop:space-y-6">
              <div className="text-3xl desktop:text-4xl large:text-5xl font-bold text-purple-500">
                {metrics.socialValue}
              </div>
              <div className="text-lg desktop:text-xl text-gray-600">社会价值</div>
              <div className="flex items-center justify-center space-x-2 desktop:space-x-3">
                {getGrowthIcon(7.6)}
                <span className={cn("text-base desktop:text-lg", getGrowthColor(7.6))}>
                  +7.6%
                </span>
              </div>
            </CardContent>
          </Card>
        </section>

        {/* 趋势图表 */}
        <section className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <TrendingUpIcon size="md" className="text-blue-500" />
                <span>用户增长趋势</span>
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                {trendData.map((item, index) => (
                  <div key={index} className="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
                    <div className="flex items-center space-x-3">
                      <div className="w-2 h-2 bg-blue-500 rounded-full"></div>
                      <span className="font-medium">{item.period}</span>
                    </div>
                    <div className="flex items-center space-x-3">
                      <span className="text-lg font-semibold">{item.value.toLocaleString()}</span>
                      <div className="flex items-center space-x-1">
                        {getGrowthIcon(item.growth)}
                        <span className={cn("text-sm", getGrowthColor(item.growth))}>
                          {item.growth > 0 ? '+' : ''}{item.growth}%
                        </span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <ProsperityIcon size="md" className="text-green-500" />
                <span>分类分析</span>
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                {categoryData.map((item, index) => (
                  <div key={index} className="space-y-2">
                    <div className="flex justify-between items-center">
                      <span className="font-medium">{item.displayName}</span>
                      <div className="flex items-center space-x-2">
                        <span className="text-sm text-gray-600">{item.percentage}%</span>
                        <div className="flex items-center space-x-1">
                          {getGrowthIcon(item.growth)}
                          <span className={cn("text-xs", getGrowthColor(item.growth))}>
                            {item.growth > 0 ? '+' : ''}{item.growth}%
                          </span>
                        </div>
                      </div>
                    </div>
                    <div className="w-full bg-gray-200 rounded-full h-2">
                      <div 
                        className="bg-gradient-to-r from-green-400 to-green-600 h-2 rounded-full transition-all duration-500"
                        style={{ width: `${item.percentage}%` }}
                      ></div>
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </section>

        {/* 地区分析 */}
        <section>
          <Card>
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <UnityIcon size="md" className="text-purple-500" />
                <span>地区分析</span>
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                {regionData.map((region, index) => (
                  <div key={index} className="p-4 bg-gradient-to-br from-purple-50 to-pink-50 rounded-xl">
                    <div className="space-y-3">
                      <div className="flex justify-between items-center">
                        <h4 className="font-semibold text-gray-900">{region.region}</h4>
                        <div className="flex items-center space-x-1">
                          <StarIcon size="xs" className="text-yellow-500" />
                          <span className="text-sm font-medium">{region.satisfaction}</span>
                        </div>
                      </div>
                      
                      <div className="grid grid-cols-2 gap-3 text-sm">
                        <div className="text-center">
                          <div className="text-lg font-bold text-people-gradient">
                            {region.users.toLocaleString()}
                          </div>
                          <div className="text-gray-600">用户数</div>
                        </div>
                        <div className="text-center">
                          <div className="text-lg font-bold text-prosperity-gradient">
                            {region.transactions.toLocaleString()}
                          </div>
                          <div className="text-gray-600">交易数</div>
                        </div>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </section>

        {/* 共产主义理念实践指标 */}
        <section>
          <Card variant="gradient" gradient="blue">
            <CardHeader>
              <CardTitle className="flex items-center space-x-2">
                <CommunistIcon size="md" className="text-primary-600" />
                <span>共产主义理念实践指标</span>
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <div className="text-center space-y-2">
                  <div className="p-4 bg-primary-100 rounded-full w-16 h-16 mx-auto flex items-center justify-center">
                    <PeopleIcon size="lg" className="text-primary-600" />
                  </div>
                  <div className="text-2xl font-bold text-primary-600">98.5%</div>
                  <div className="text-sm text-gray-600">人民当家作主参与率</div>
                  <div className="text-xs text-gray-500">集体决策参与度</div>
                </div>

                <div className="text-center space-y-2">
                  <div className="p-4 bg-secondary-100 rounded-full w-16 h-16 mx-auto flex items-center justify-center">
                    <UnityIcon size="lg" className="text-secondary-600" />
                  </div>
                  <div className="text-2xl font-bold text-secondary-600">92.3%</div>
                  <div className="text-sm text-gray-600">按需生产达成率</div>
                  <div className="text-xs text-gray-500">生产计划完成度</div>
                </div>

                <div className="text-center space-y-2">
                  <div className="p-4 bg-success-100 rounded-full w-16 h-16 mx-auto flex items-center justify-center">
                    <ProsperityIcon size="lg" className="text-success-600" />
                  </div>
                  <div className="text-2xl font-bold text-success-600">89.7%</div>
                  <div className="text-sm text-gray-600">共同富裕实现度</div>
                  <div className="text-xs text-gray-500">财富分配公平性</div>
                </div>

                <div className="text-center space-y-2">
                  <div className="p-4 bg-orange-100 rounded-full w-16 h-16 mx-auto flex items-center justify-center">
                    <StarIcon size="lg" className="text-orange-600" />
                  </div>
                  <div className="text-2xl font-bold text-orange-600">4.9</div>
                  <div className="text-sm text-gray-600">人民满意度</div>
                  <div className="text-xs text-gray-500">综合评价分数</div>
                </div>
              </div>
            </CardContent>
          </Card>
        </section>
      </div>
    </div>
  );
};

export default AnalyticsPage; 