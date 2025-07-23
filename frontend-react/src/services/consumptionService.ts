import apiClient, { ApiService } from './api';
import type { Consumption, ApiResponse, PaginatedResponse, SearchFilters } from '../types';

export interface ConsumptionFilters extends SearchFilters {
  page?: number;
  limit?: number;
  status?: 'pending' | 'confirmed' | 'rated' | 'disputed';
  dateRange?: {
    start: string;
    end: string;
  };
  multiplierRange?: {
    min: number;
    max: number;
  };
  producerId?: string;
}

export interface ConsumptionCreateData {
  voucherId: string;
  quantity: number;
  consumptionMethod: 'pickup' | 'delivery' | 'digital';
  location?: {
    latitude: number;
    longitude: number;
    address: string;
  };
  proof?: {
    photos: string[];
    description: string;
  };
}

export interface RatingData {
  overallScore: number; // 1-5分
  detailedScores: {
    quality: number;
    service: number;
    experience: number;
    value: number;
  };
  feedback: string;
  photos?: string[];
}

export interface MultiplierCalculation {
  baseMultiplier: number;
  qualityScore: number;
  serviceScore: number;
  finalMultiplier: number;
  totalReward: number;
  breakdown: {
    producerReward: number;
    workerReward: number;
    distributionPercentage: number;
  };
}

class ConsumptionService extends ApiService {
  /**
   * 获取消费记录列表
   */
  async getConsumptions(filters?: ConsumptionFilters): Promise<ApiResponse<PaginatedResponse<Consumption>>> {
    const params = new URLSearchParams();
    
    if (filters) {
      if (filters.page) params.append('page', filters.page.toString());
      if (filters.limit) params.append('limit', filters.limit.toString());
      if (filters.status) params.append('status', filters.status);
      if (filters.dateRange) {
        params.append('startDate', filters.dateRange.start);
        params.append('endDate', filters.dateRange.end);
      }
      if (filters.multiplierRange) {
        params.append('minMultiplier', filters.multiplierRange.min.toString());
        params.append('maxMultiplier', filters.multiplierRange.max.toString());
      }
      if (filters.producerId) params.append('producerId', filters.producerId);
      if (filters.sort) {
        params.append('sortField', filters.sort.field);
        params.append('sortOrder', filters.sort.order);
      }
    }

    return await this.get(`/consumptions?${params.toString()}`);
  }

  /**
   * 获取消费记录详情
   */
  async getConsumptionById(id: string): Promise<ApiResponse<Consumption>> {
    return await this.get(`/consumptions/${id}`);
  }

  /**
   * 创建消费记录
   */
  async createConsumption(data: ConsumptionCreateData): Promise<ApiResponse<Consumption>> {
    return await this.post('/consumptions', data);
  }

  /**
   * 确认消费
   */
  async confirmConsumption(id: string): Promise<ApiResponse<Consumption>> {
    return await this.put(`/consumptions/${id}/confirm`);
  }

  /**
   * 提交评价
   */
  async submitRating(id: string, rating: RatingData): Promise<ApiResponse<Consumption>> {
    return await this.put(`/consumptions/${id}/rate`, rating);
  }

  /**
   * 计算倍增奖励
   */
  async calculateMultiplier(consumptionId: string): Promise<ApiResponse<MultiplierCalculation>> {
    return await this.get(`/consumptions/${consumptionId}/multiplier`);
  }

  /**
   * 获取用户消费统计
   */
  async getUserConsumptionStats(userId?: string): Promise<ApiResponse<{
    totalConsumptions: number;
    totalRewards: number;
    averageMultiplier: number;
    monthlyStats: Array<{
      month: string;
      consumptions: number;
      rewards: number;
    }>;
  }>> {
    const url = userId ? `/consumptions/stats/${userId}` : '/consumptions/stats';
    return await this.get(url);
  }

  /**
   * 获取消费趋势数据
   */
  async getConsumptionTrends(period: 'week' | 'month' | 'year'): Promise<ApiResponse<Array<{
    period: string;
    totalConsumptions: number;
    totalValue: number;
    averageMultiplier: number;
  }>>> {
    return await this.get(`/consumptions/trends?period=${period}`);
  }

  /**
   * 搜索消费记录
   */
  async searchConsumptions(query: string, filters?: ConsumptionFilters): Promise<ApiResponse<PaginatedResponse<Consumption>>> {
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

    return await this.get(`/consumptions/search?${params.toString()}`);
  }

  /**
   * 获取消费证明
   */
  async getConsumptionProof(id: string): Promise<ApiResponse<{
    photos: string[];
    description: string;
    timestamp: string;
    location?: {
      latitude: number;
      longitude: number;
      address: string;
    };
  }>> {
    return await this.get(`/consumptions/${id}/proof`);
  }

  /**
   * 上传消费证明
   */
  async uploadConsumptionProof(id: string, photos: File[], description: string): Promise<ApiResponse<void>> {
    const formData = new FormData();
    photos.forEach((photo, index) => {
      formData.append(`photos[${index}]`, photo);
    });
    formData.append('description', description);

    return await this.post(`/consumptions/${id}/proof`, formData, {
      headers: {
        'Content-Type': 'multipart/form-data',
      },
    });
  }

  /**
   * 发起消费争议
   */
  async createDispute(id: string, reason: string, evidence?: string[]): Promise<ApiResponse<void>> {
    return await this.post(`/consumptions/${id}/dispute`, {
      reason,
      evidence,
    });
  }

  /**
   * 获取奖励分配详情
   */
  async getRewardDistribution(id: string): Promise<ApiResponse<{
    totalReward: number;
    producerReward: number;
    workerReward: number;
    distributionPercentage: number;
    multiplierBreakdown: {
      baseMultiplier: number;
      qualityBonus: number;
      serviceBonus: number;
      finalMultiplier: number;
    };
  }>> {
    return await this.get(`/consumptions/${id}/rewards`);
  }
}

export const consumptionService = new ConsumptionService();
export default consumptionService; 