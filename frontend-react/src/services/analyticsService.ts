import apiClient, { ApiService } from './api';
import type { ApiResponse, ChartData } from '../types';

export interface DashboardMetrics {
  overview: {
    totalUsers: number;
    totalVouchers: number;
    totalConsumptions: number;
    totalRevenue: number;
    growthRates: {
      users: number;
      vouchers: number;
      consumptions: number;
      revenue: number;
    };
  };
  userMetrics: {
    activeUsers: number;
    newUsers: number;
    userRetention: number;
    averageSessionTime: number;
  };
  voucherMetrics: {
    totalIssued: number;
    totalClaimed: number;
    totalConsumed: number;
    utilizationRate: number;
  };
  economicMetrics: {
    averageMultiplier: number;
    totalRewardsDistributed: number;
    communismIndex: number; // 共产主义指数
    wealthDistributionIndex: number;
  };
}

export interface TrendData {
  period: string;
  value: number;
  change?: number;
  changePercent?: number;
}

export interface AnalyticsFilters {
  dateRange: {
    start: string;
    end: string;
  };
  granularity: 'hour' | 'day' | 'week' | 'month' | 'year';
  categories?: string[];
  regions?: string[];
  userTypes?: string[];
}

export interface ReportConfig {
  type: 'user' | 'voucher' | 'consumption' | 'economic' | 'enterprise' | 'comprehensive';
  format: 'pdf' | 'excel' | 'csv';
  filters: AnalyticsFilters;
  includeCharts: boolean;
  includeRawData: boolean;
}

export interface PredictionData {
  metric: string;
  currentValue: number;
  predictions: Array<{
    period: string;
    predictedValue: number;
    confidence: number;
    factors: string[];
  }>;
  recommendations: string[];
}

class AnalyticsService extends ApiService {
  /**
   * 获取仪表板数据
   */
  async getDashboardMetrics(period: '24h' | '7d' | '30d' | '90d' | '1y'): Promise<ApiResponse<DashboardMetrics>> {
    return await this.get(`/analytics/dashboard?period=${period}`);
  }

  /**
   * 获取用户趋势数据
   */
  async getUserTrends(filters: AnalyticsFilters): Promise<ApiResponse<{
    registration: TrendData[];
    activity: TrendData[];
    retention: TrendData[];
    demographics: {
      ageGroups: Array<{ range: string; count: number; percentage: number }>;
      locations: Array<{ region: string; count: number; percentage: number }>;
      userTypes: Array<{ type: string; count: number; percentage: number }>;
    };
  }>> {
    return await this.post('/analytics/users/trends', filters);
  }

  /**
   * 获取提货券分析数据
   */
  async getVoucherAnalytics(filters: AnalyticsFilters): Promise<ApiResponse<{
    issuance: TrendData[];
    claims: TrendData[];
    consumption: TrendData[];
    categories: Array<{
      category: string;
      issued: number;
      claimed: number;
      consumed: number;
      utilizationRate: number;
    }>;
    topPerformers: Array<{
      voucherId: string;
      voucherName: string;
      claims: number;
      consumptions: number;
      rating: number;
    }>;
  }>> {
    return await this.post('/analytics/vouchers', filters);
  }

  /**
   * 获取消费行为分析
   */
  async getConsumptionAnalytics(filters: AnalyticsFilters): Promise<ApiResponse<{
    volume: TrendData[];
    value: TrendData[];
    multipliers: TrendData[];
    patterns: {
      timeOfDay: Array<{ hour: number; count: number }>;
      dayOfWeek: Array<{ day: string; count: number }>;
      seasonality: Array<{ month: string; count: number }>;
    };
    rewardDistribution: {
      total: number;
      toProducers: number;
      toWorkers: number;
      averageMultiplier: number;
    };
  }>> {
    return await this.post('/analytics/consumption', filters);
  }

  /**
   * 获取经济指标分析
   */
  async getEconomicAnalytics(filters: AnalyticsFilters): Promise<ApiResponse<{
    communismIndex: TrendData[];
    wealthDistribution: {
      giniCoefficient: number;
      quintileShares: number[];
      trend: TrendData[];
    };
    multiplierEffectiveness: {
      averageMultiplier: TrendData[];
      distributionFairness: TrendData[];
      economicImpact: TrendData[];
    };
    socialBenefits: {
      accessibilityIndex: number;
      satisfactionIndex: number;
      communityEngagement: number;
    };
  }>> {
    return await this.post('/analytics/economic', filters);
  }

  /**
   * 获取企业表现分析
   */
  async getEnterpriseAnalytics(filters: AnalyticsFilters): Promise<ApiResponse<{
    performance: Array<{
      enterpriseId: string;
      enterpriseName: string;
      currentMultiplier: number;
      averageRating: number;
      totalVouchers: number;
      totalConsumptions: number;
      revenueGenerated: number;
    }>;
    industryComparison: Array<{
      industry: string;
      averageMultiplier: number;
      averageRating: number;
      marketShare: number;
    }>;
    evaluationTrends: TrendData[];
    improvementAreas: Array<{
      area: string;
      averageScore: number;
      trend: 'improving' | 'declining' | 'stable';
    }>;
  }>> {
    return await this.post('/analytics/enterprises', filters);
  }

  /**
   * 获取地理分析数据
   */
  async getGeographicAnalytics(filters: AnalyticsFilters): Promise<ApiResponse<{
    distribution: Array<{
      region: string;
      users: number;
      vouchers: number;
      consumptions: number;
      revenue: number;
      coordinates: { lat: number; lng: number };
    }>;
    heatmaps: {
      activity: Array<{ lat: number; lng: number; intensity: number }>;
      consumption: Array<{ lat: number; lng: number; intensity: number }>;
    };
    regionalTrends: Array<{
      region: string;
      trend: TrendData[];
    }>;
  }>> {
    return await this.post('/analytics/geographic', filters);
  }

  /**
   * 获取实时数据
   */
  async getRealTimeData(): Promise<ApiResponse<{
    activeUsers: number;
    ongoingTransactions: number;
    currentVoucherClaims: number;
    systemLoad: number;
    recentActivities: Array<{
      type: 'voucher_claim' | 'consumption' | 'user_registration' | 'evaluation';
      timestamp: string;
      description: string;
    }>;
  }>> {
    return await this.get('/analytics/real-time');
  }

  /**
   * 生成图表数据
   */
  async generateChartData(type: 'line' | 'bar' | 'pie' | 'area' | 'scatter', config: {
    metric: string;
    filters: AnalyticsFilters;
    groupBy?: string;
  }): Promise<ApiResponse<ChartData>> {
    return await this.post(`/analytics/charts/${type}`, config);
  }

  /**
   * 获取比较分析
   */
  async getComparativeAnalysis(comparison: {
    type: 'period' | 'segment' | 'cohort';
    baselineFilters: AnalyticsFilters;
    comparisonFilters: AnalyticsFilters;
    metrics: string[];
  }): Promise<ApiResponse<{
    baseline: Record<string, number>;
    comparison: Record<string, number>;
    differences: Record<string, {
      absolute: number;
      percentage: number;
      significance: 'high' | 'medium' | 'low';
    }>;
    insights: string[];
  }>> {
    return await this.post('/analytics/compare', comparison);
  }

  /**
   * 获取预测分析
   */
  async getPredictions(config: {
    metrics: string[];
    period: number; // 预测未来多少天
    confidence: number; // 置信度 0-1
  }): Promise<ApiResponse<PredictionData[]>> {
    return await this.post('/analytics/predictions', config);
  }

  /**
   * 生成报告
   */
  async generateReport(config: ReportConfig): Promise<ApiResponse<{
    reportId: string;
    downloadUrl: string;
    estimatedTime: number;
  }>> {
    return await this.post('/analytics/reports', config);
  }

  /**
   * 获取报告状态
   */
  async getReportStatus(reportId: string): Promise<ApiResponse<{
    status: 'pending' | 'processing' | 'completed' | 'failed';
    progress: number;
    downloadUrl?: string;
    error?: string;
  }>> {
    return await this.get(`/analytics/reports/${reportId}/status`);
  }

  /**
   * 获取异常检测
   */
  async getAnomalyDetection(metric: string, filters: AnalyticsFilters): Promise<ApiResponse<{
    anomalies: Array<{
      timestamp: string;
      value: number;
      expectedValue: number;
      severity: 'low' | 'medium' | 'high';
      description: string;
    }>;
    patterns: string[];
    recommendations: string[];
  }>> {
    return await this.post('/analytics/anomalies', { metric, filters });
  }

  /**
   * 获取用户行为分析
   */
  async getUserBehaviorAnalysis(filters: AnalyticsFilters): Promise<ApiResponse<{
    userJourney: Array<{
      step: string;
      users: number;
      conversionRate: number;
      avgTimeSpent: number;
    }>;
    engagementMetrics: {
      dailyActiveUsers: TrendData[];
      sessionDuration: TrendData[];
      bounceRate: TrendData[];
      retentionCohorts: Array<{
        cohort: string;
        week0: number;
        week1: number;
        week4: number;
        week12: number;
      }>;
    };
    behaviorSegments: Array<{
      segment: string;
      size: number;
      characteristics: string[];
      averageValue: number;
    }>;
  }>> {
    return await this.post('/analytics/user-behavior', filters);
  }

  /**
   * 导出数据
   */
  async exportData(config: {
    type: 'raw' | 'aggregated';
    format: 'csv' | 'json' | 'excel';
    filters: AnalyticsFilters;
    fields: string[];
  }): Promise<ApiResponse<{
    downloadUrl: string;
    fileName: string;
    fileSize: number;
  }>> {
    return await this.post('/analytics/export', config);
  }
}

export const analyticsService = new AnalyticsService();
export default analyticsService; 