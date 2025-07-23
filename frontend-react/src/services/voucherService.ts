import { ApiService } from './api';
import { Voucher, ApiResponse, PaginatedResponse, VoucherCategory, BaseResponse, PaginationResponse } from '../types';

export interface CreateVoucherRequest {
  name: string;
  symbol: string;
  description: string;
  category: VoucherCategory;
  totalSupply: number;
  expiresAt?: Date;
  metadata: {
    imageUrl?: string;
    externalUrl?: string;
    attributes?: Array<{
      trait_type: string;
      value: string | number;
      display_type?: string;
    }>;
    productInfo?: {
      brand?: string;
      model?: string;
      specifications?: Record<string, any>;
      warranty?: string;
      origin?: string;
    };
  };
}

export interface UpdateVoucherRequest {
  name?: string;
  description?: string;
  isActive?: boolean;
  expiresAt?: Date;
  metadata?: CreateVoucherRequest['metadata'];
}

export interface ClaimVoucherRequest {
  quantity: number;
  recipientAddress?: string;
}

export interface ConsumeVoucherRequest {
  voucherId: string;
  quantity: number;
  consumptionProof?: string;
  metadata?: Record<string, any>;
}

export interface VoucherListParams {
  page?: number;
  limit?: number;
  category?: VoucherCategory;
  isActive?: boolean;
  search?: string;
  sortBy?: 'created_at' | 'name' | 'total_supply' | 'claimed_supply';
  sortOrder?: 'asc' | 'desc';
}

export interface UserVoucherBalance {
  voucherId: string;
  voucher: Voucher;
  balance: number;
  claimedAt: Date;
  lastUsedAt?: Date;
}

class VoucherService extends ApiService {
  private readonly baseUrl = '/api/vouchers';
  
  // 提货券管理
  async createVoucher(data: CreateVoucherRequest): Promise<Voucher> {
    const response = await this.post<BaseResponse<Voucher>>(this.baseUrl, data);
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '创建提货券失败');
  }
  
  async getVouchers(params?: VoucherListParams): Promise<PaginationResponse<Voucher>> {
    const response = await this.get<BaseResponse<PaginationResponse<Voucher>>>(this.baseUrl, {
      params,
    });
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '获取提货券列表失败');
  }
  
  async getVoucherById(id: string): Promise<Voucher> {
    const response = await this.get<BaseResponse<Voucher>>(`${this.baseUrl}/${id}`);
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '获取提货券信息失败');
  }
  
  async updateVoucher(id: string, data: UpdateVoucherRequest): Promise<Voucher> {
    const response = await this.put<BaseResponse<Voucher>>(`${this.baseUrl}/${id}`, data);
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '更新提货券失败');
  }
  
  async deleteVoucher(id: string): Promise<void> {
    const response = await this.delete<BaseResponse<void>>(`${this.baseUrl}/${id}`);
    
    if (!response.success) {
      throw new Error(response.error || '删除提货券失败');
    }
  }
  
  // 提货券领取
  async claimVoucher(id: string, data: ClaimVoucherRequest): Promise<{
    transactionHash: string;
    voucherBalance: UserVoucherBalance;
  }> {
    const response = await this.post<BaseResponse<{
      transactionHash: string;
      voucherBalance: UserVoucherBalance;
    }>>(`${this.baseUrl}/${id}/claim`, data);
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '领取提货券失败');
  }
  
  async batchClaimVouchers(claims: Array<{
    voucherId: string;
    quantity: number;
  }>): Promise<{
    transactionHash: string;
    claimedVouchers: UserVoucherBalance[];
  }> {
    const response = await this.post<BaseResponse<{
      transactionHash: string;
      claimedVouchers: UserVoucherBalance[];
    }>>(`${this.baseUrl}/batch-claim`, { claims });
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '批量领取提货券失败');
  }
  
  // 提货券消费
  async consumeVoucher(data: ConsumeVoucherRequest): Promise<{
    transactionHash: string;
    remainingBalance: number;
    rewardInfo?: {
      baseReward: number;
      multiplierFactor: number;
      totalReward: number;
    };
  }> {
    const response = await this.post<BaseResponse<{
      transactionHash: string;
      remainingBalance: number;
      rewardInfo?: {
        baseReward: number;
        multiplierFactor: number;
        totalReward: number;
      };
    }>>(`${this.baseUrl}/${data.voucherId}/consume`, data);
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '消费提货券失败');
  }
  
  // 用户提货券余额
  async getUserVoucherBalances(params?: {
    page?: number;
    limit?: number;
    category?: VoucherCategory;
    hasBalance?: boolean;
  }): Promise<PaginationResponse<UserVoucherBalance>> {
    const response = await this.get<BaseResponse<PaginationResponse<UserVoucherBalance>>>(
      `${this.baseUrl}/my-balances`,
      { params }
    );
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '获取提货券余额失败');
  }
  
  async getUserVoucherBalance(id: string): Promise<UserVoucherBalance | null> {
    const response = await this.get<BaseResponse<UserVoucherBalance | null>>(
      `${this.baseUrl}/${id}/my-balance`
    );
    
    if (response.success) {
      return response.data || null;
    }
    
    throw new Error(response.error || '获取提货券余额失败');
  }
  
  // 提货券历史记录
  async getVoucherTransactions(id: string, params?: {
    page?: number;
    limit?: number;
    type?: 'claim' | 'consume' | 'transfer';
  }): Promise<PaginationResponse<{
    id: string;
    type: 'claim' | 'consume' | 'transfer';
    quantity: number;
    transactionHash: string;
    fromAddress?: string;
    toAddress?: string;
    metadata?: Record<string, any>;
    createdAt: Date;
  }>> {
    const response = await this.get<BaseResponse<PaginationResponse<any>>>(
      `${this.baseUrl}/${id}/transactions`,
      { params }
    );
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '获取提货券交易记录失败');
  }
  
  async getUserVoucherTransactions(params?: {
    page?: number;
    limit?: number;
    voucherId?: string;
    type?: 'claim' | 'consume' | 'transfer';
  }): Promise<PaginationResponse<any>> {
    const response = await this.get<BaseResponse<PaginationResponse<any>>>(
      `${this.baseUrl}/my-transactions`,
      { params }
    );
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '获取用户提货券交易记录失败');
  }
  
  // 提货券统计
  async getVoucherStats(id: string): Promise<{
    totalSupply: number;
    claimedSupply: number;
    consumedSupply: number;
    remainingSupply: number;
    uniqueHolders: number;
    totalTransactions: number;
    averageClaimAmount: number;
    averageConsumeAmount: number;
  }> {
    const response = await this.get<BaseResponse<any>>(`${this.baseUrl}/${id}/stats`);
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '获取提货券统计信息失败');
  }
  
  async getOverallStats(): Promise<{
    totalVouchers: number;
    activeVouchers: number;
    totalSupply: number;
    totalClaimed: number;
    totalConsumed: number;
    totalUsers: number;
    topCategories: Array<{
      category: VoucherCategory;
      count: number;
      totalSupply: number;
    }>;
  }> {
    const response = await this.get<BaseResponse<any>>(`${this.baseUrl}/stats`);
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '获取整体统计信息失败');
  }
  
  // 搜索和筛选
  async searchVouchers(query: string, filters?: {
    category?: VoucherCategory;
    isActive?: boolean;
    hasBalance?: boolean;
  }): Promise<Voucher[]> {
    const response = await this.get<BaseResponse<Voucher[]>>(`${this.baseUrl}/search`, {
      params: { q: query, ...filters },
    });
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '搜索提货券失败');
  }
  
  async getVouchersByCategory(category: VoucherCategory, params?: {
    page?: number;
    limit?: number;
    isActive?: boolean;
  }): Promise<PaginationResponse<Voucher>> {
    const response = await this.get<BaseResponse<PaginationResponse<Voucher>>>(
      `${this.baseUrl}/category/${category}`,
      { params }
    );
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '获取分类提货券失败');
  }
}

// 导出单例实例
export const voucherService = new VoucherService(); 