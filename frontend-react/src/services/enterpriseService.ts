import apiClient, { ApiService } from './api';
import type { EnterpriseEvaluation, ApiResponse, PaginatedResponse, SearchFilters } from '../types';

export interface Enterprise {
  id: string;
  name: string;
  description: string;
  industry: string;
  location: {
    country: string;
    province: string;
    city: string;
    address: string;
  };
  contactInfo: {
    email: string;
    phone: string;
    website?: string;
  };
  currentMultiplier: number;
  lastEvaluationDate?: string;
  status: 'active' | 'suspended' | 'pending';
  registrationDate: string;
  metrics: {
    totalVouchers: number;
    totalConsumptions: number;
    averageRating: number;
    employeeCount: number;
  };
}

export interface EnterpriseFilters extends SearchFilters {
  page?: number;
  limit?: number;
  industry?: string;
  status?: 'active' | 'suspended' | 'pending';
  multiplierRange?: {
    min: number;
    max: number;
  };
  location?: string;
  lastEvaluated?: 'week' | 'month' | 'quarter' | 'year';
}

export interface EvaluationSubmission {
  enterpriseId: string;
  evaluationPeriod: {
    startDate: string;
    endDate: string;
  };
  scores: {
    productQualityScore: number; // 1-100
    serviceQualityScore: number; // 1-100
    workerWelfareScore: number; // 1-100
    environmentalScore: number; // 1-100
    safetyScore: number; // 1-100
    ideologyScore: number; // 1-100
  };
  evaluationDetails: {
    productQuality: {
      evidence: string[];
      improvementAreas: string[];
    };
    workerWelfare: {
      evidence: string[];
      workerBenefits: {
        averageSalary: number;
        insuranceCoverage: number;
        paidLeaveDays: number;
        trainingHours: number;
      };
    };
    environmentalProtection: {
      carbonFootprint: number;
      wasteRecyclingRate: number;
      renewableEnergyUsage: number;
    };
  };
  evaluatorNotes: string;
  recommendedMultiplier: number;
}

class EnterpriseService extends ApiService {
  /**
   * 获取企业列表
   */
  async getEnterprises(filters?: EnterpriseFilters): Promise<ApiResponse<PaginatedResponse<Enterprise>>> {
    const params = new URLSearchParams();
    
    if (filters) {
      if (filters.page) params.append('page', filters.page.toString());
      if (filters.limit) params.append('limit', filters.limit.toString());
      if (filters.industry) params.append('industry', filters.industry);
      if (filters.status) params.append('status', filters.status);
      if (filters.location) params.append('location', filters.location);
      if (filters.lastEvaluated) params.append('lastEvaluated', filters.lastEvaluated);
      if (filters.multiplierRange) {
        params.append('minMultiplier', filters.multiplierRange.min.toString());
        params.append('maxMultiplier', filters.multiplierRange.max.toString());
      }
      if (filters.sort) {
        params.append('sortField', filters.sort.field);
        params.append('sortOrder', filters.sort.order);
      }
    }

    return await this.get(`/enterprises?${params.toString()}`);
  }

  /**
   * 获取企业详情
   */
  async getEnterpriseById(id: string): Promise<ApiResponse<Enterprise>> {
    return await this.get(`/enterprises/${id}`);
  }

  /**
   * 创建企业
   */
  async createEnterprise(data: Omit<Enterprise, 'id' | 'registrationDate' | 'metrics'>): Promise<ApiResponse<Enterprise>> {
    return await this.post('/enterprises', data);
  }

  /**
   * 更新企业信息
   */
  async updateEnterprise(id: string, data: Partial<Enterprise>): Promise<ApiResponse<Enterprise>> {
    return await this.put(`/enterprises/${id}`, data);
  }

  /**
   * 获取企业评估历史
   */
  async getEnterpriseEvaluations(enterpriseId: string): Promise<ApiResponse<PaginatedResponse<EnterpriseEvaluation>>> {
    return await this.get(`/enterprises/${enterpriseId}/evaluations`);
  }

  /**
   * 提交企业评估
   */
  async submitEvaluation(data: EvaluationSubmission): Promise<ApiResponse<EnterpriseEvaluation>> {
    return await this.post('/enterprises/evaluations', data);
  }

  /**
   * 获取评估详情
   */
  async getEvaluationById(id: string): Promise<ApiResponse<EnterpriseEvaluation>> {
    return await this.get(`/enterprises/evaluations/${id}`);
  }

  /**
   * 更新评估状态
   */
  async updateEvaluationStatus(id: string, status: 'draft' | 'pending' | 'completed' | 'disputed'): Promise<ApiResponse<EnterpriseEvaluation>> {
    return await this.put(`/enterprises/evaluations/${id}/status`, { status });
  }

  /**
   * 获取企业倍增系数历史
   */
  async getMultiplierHistory(enterpriseId: string): Promise<ApiResponse<Array<{
    date: string;
    multiplier: number;
    evaluationId: string;
    reason: string;
  }>>> {
    return await this.get(`/enterprises/${enterpriseId}/multiplier-history`);
  }

  /**
   * 更新企业倍增系数
   */
  async updateMultiplier(enterpriseId: string, multiplier: number, reason: string): Promise<ApiResponse<void>> {
    return await this.put(`/enterprises/${enterpriseId}/multiplier`, {
      multiplier,
      reason,
    });
  }

  /**
   * 搜索企业
   */
  async searchEnterprises(query: string, filters?: EnterpriseFilters): Promise<ApiResponse<PaginatedResponse<Enterprise>>> {
    const params = new URLSearchParams();
    params.append('q', query);
    
    if (filters) {
      Object.entries(filters).forEach(([key, value]) => {
        if (value !== undefined && value !== null) {
          if (typeof value === 'object') {
            params.append(key, JSON.stringify(value));
          } else {
            params.append(key, value.toString());
          }
        }
      });
    }

    return await this.get(`/enterprises/search?${params.toString()}`);
  }

  /**
   * 获取企业统计数据
   */
  async getEnterpriseStats(enterpriseId: string): Promise<ApiResponse<{
    totalVouchers: number;
    totalConsumptions: number;
    totalRevenueGenerated: number;
    averageRating: number;
    currentMultiplier: number;
    monthlyMetrics: Array<{
      month: string;
      vouchers: number;
      consumptions: number;
      revenue: number;
      rating: number;
    }>;
  }>> {
    return await this.get(`/enterprises/${enterpriseId}/stats`);
  }

  /**
   * 获取行业平均数据
   */
  async getIndustryAverages(industry: string): Promise<ApiResponse<{
    averageMultiplier: number;
    averageRating: number;
    averageEmployeeCount: number;
    averageScores: {
      productQuality: number;
      serviceQuality: number;
      workerWelfare: number;
      environmental: number;
      safety: number;
      ideology: number;
    };
  }>> {
    return await this.get(`/enterprises/industry/${industry}/averages`);
  }

  /**
   * 获取评估员的评估历史
   */
  async getEvaluatorHistory(evaluatorId: string): Promise<ApiResponse<PaginatedResponse<EnterpriseEvaluation>>> {
    return await this.get(`/enterprises/evaluations/evaluator/${evaluatorId}`);
  }

  /**
   * 获取待评估企业列表
   */
  async getPendingEvaluations(): Promise<ApiResponse<PaginatedResponse<Enterprise>>> {
    return await this.get('/enterprises/pending-evaluation');
  }

  /**
   * 导出企业评估报告
   */
  async exportEvaluationReport(evaluationId: string, format: 'pdf' | 'excel'): Promise<ApiResponse<{ downloadUrl: string }>> {
    return await this.get(`/enterprises/evaluations/${evaluationId}/export?format=${format}`);
  }

  /**
   * 获取企业对比分析
   */
  async compareEnterprises(enterpriseIds: string[]): Promise<ApiResponse<{
    enterprises: Array<{
      id: string;
      name: string;
      currentMultiplier: number;
      scores: {
        productQuality: number;
        serviceQuality: number;
        workerWelfare: number;
        environmental: number;
        safety: number;
        ideology: number;
      };
      totalScore: number;
    }>;
    analysis: {
      strongest: string;
      weakest: string;
      recommendations: string[];
    };
  }>> {
    return await this.post('/enterprises/compare', { enterpriseIds });
  }
}

export const enterpriseService = new EnterpriseService();
export default enterpriseService; 