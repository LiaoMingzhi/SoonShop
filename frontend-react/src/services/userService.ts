import { ApiService } from './api';
import { User, BaseResponse, PaginationResponse } from '../types';

export interface LoginRequest {
  username: string;
  password: string;
}

export interface RegisterRequest {
  username: string;
  email: string;
  password: string;
  walletAddress?: string;
}

export interface AuthResponse {
  token: string;
  refreshToken: string;
  expiresIn: number;
  user: User;
}

export interface UpdateUserRequest {
  username?: string;
  email?: string;
  avatar?: string;
  profile?: {
    firstName?: string;
    lastName?: string;
    bio?: string;
    location?: string;
    website?: string;
  };
}

export interface ChangePasswordRequest {
  currentPassword: string;
  newPassword: string;
}

class UserService extends ApiService {
  private readonly baseUrl = '/api/users';
  private readonly authUrl = '/auth';
  
  // 用户认证
  async login(data: LoginRequest): Promise<AuthResponse> {
    const response = await this.post<BaseResponse<AuthResponse>>(`${this.authUrl}/login`, data);
    
    if (response.success && response.data) {
      // 保存认证信息到本地存储
      localStorage.setItem('access_token', response.data.token);
      localStorage.setItem('refresh_token', response.data.refreshToken);
      localStorage.setItem('user_info', JSON.stringify(response.data.user));
      
      return response.data;
    }
    
    throw new Error(response.error || '登录失败');
  }
  
  async register(data: RegisterRequest): Promise<AuthResponse> {
    const response = await this.post<BaseResponse<AuthResponse>>(`${this.authUrl}/register`, data);
    
    if (response.success && response.data) {
      // 保存认证信息到本地存储
      localStorage.setItem('access_token', response.data.token);
      localStorage.setItem('refresh_token', response.data.refreshToken);
      localStorage.setItem('user_info', JSON.stringify(response.data.user));
      
      return response.data;
    }
    
    throw new Error(response.error || '注册失败');
  }
  
  async refreshToken(): Promise<AuthResponse> {
    const refreshToken = localStorage.getItem('refresh_token');
    if (!refreshToken) {
      throw new Error('刷新令牌不存在');
    }
    
    const response = await this.post<BaseResponse<AuthResponse>>(`${this.authUrl}/refresh`, {
      refreshToken,
    });
    
    if (response.success && response.data) {
      // 更新认证信息
      localStorage.setItem('access_token', response.data.token);
      localStorage.setItem('refresh_token', response.data.refreshToken);
      localStorage.setItem('user_info', JSON.stringify(response.data.user));
      
      return response.data;
    }
    
    throw new Error(response.error || '刷新令牌失败');
  }
  
  async logout(): Promise<void> {
    try {
      await this.post(`${this.authUrl}/logout`);
    } finally {
      // 清除本地存储
      localStorage.removeItem('access_token');
      localStorage.removeItem('refresh_token');
      localStorage.removeItem('user_info');
    }
  }
  
  // 用户信息管理
  async getCurrentUser(): Promise<User> {
    const response = await this.get<BaseResponse<User>>(`${this.baseUrl}/me`);
    
    if (response.success && response.data) {
      // 更新本地存储的用户信息
      localStorage.setItem('user_info', JSON.stringify(response.data));
      return response.data;
    }
    
    throw new Error(response.error || '获取用户信息失败');
  }
  
  async updateCurrentUser(data: UpdateUserRequest): Promise<User> {
    const response = await this.put<BaseResponse<User>>(`${this.baseUrl}/me`, data);
    
    if (response.success && response.data) {
      // 更新本地存储的用户信息
      localStorage.setItem('user_info', JSON.stringify(response.data));
      return response.data;
    }
    
    throw new Error(response.error || '更新用户信息失败');
  }
  
  async changePassword(data: ChangePasswordRequest): Promise<void> {
    const response = await this.post<BaseResponse<void>>(`${this.baseUrl}/me/change-password`, data);
    
    if (!response.success) {
      throw new Error(response.error || '修改密码失败');
    }
  }
  
  async uploadAvatar(file: File): Promise<string> {
    const formData = new FormData();
    formData.append('avatar', file);
    
    const response = await this.post<BaseResponse<{ avatarUrl: string }>>(
      `${this.baseUrl}/me/avatar`,
      formData,
      {
        headers: {
          'Content-Type': 'multipart/form-data',
        },
      }
    );
    
    if (response.success && response.data) {
      return response.data.avatarUrl;
    }
    
    throw new Error(response.error || '上传头像失败');
  }
  
  // 用户管理（管理员功能）
  async getUsers(params?: {
    page?: number;
    limit?: number;
    search?: string;
    role?: string;
    isActive?: boolean;
  }): Promise<PaginationResponse<User>> {
    const response = await this.get<BaseResponse<PaginationResponse<User>>>(this.baseUrl, {
      params,
    });
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '获取用户列表失败');
  }
  
  async getUserById(id: string): Promise<User> {
    const response = await this.get<BaseResponse<User>>(`${this.baseUrl}/${id}`);
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '获取用户信息失败');
  }
  
  async updateUser(id: string, data: UpdateUserRequest): Promise<User> {
    const response = await this.put<BaseResponse<User>>(`${this.baseUrl}/${id}`, data);
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '更新用户信息失败');
  }
  
  async deleteUser(id: string): Promise<void> {
    const response = await this.delete<BaseResponse<void>>(`${this.baseUrl}/${id}`);
    
    if (!response.success) {
      throw new Error(response.error || '删除用户失败');
    }
  }
  
  async activateUser(id: string): Promise<User> {
    const response = await this.post<BaseResponse<User>>(`${this.baseUrl}/${id}/activate`);
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '激活用户失败');
  }
  
  async deactivateUser(id: string): Promise<User> {
    const response = await this.post<BaseResponse<User>>(`${this.baseUrl}/${id}/deactivate`);
    
    if (response.success && response.data) {
      return response.data;
    }
    
    throw new Error(response.error || '停用用户失败');
  }
  
  // 钱包相关
  async connectWallet(walletAddress: string, signature: string): Promise<User> {
    const response = await this.post<BaseResponse<User>>(`${this.baseUrl}/me/connect-wallet`, {
      walletAddress,
      signature,
    });
    
    if (response.success && response.data) {
      // 更新本地存储的用户信息
      localStorage.setItem('user_info', JSON.stringify(response.data));
      return response.data;
    }
    
    throw new Error(response.error || '连接钱包失败');
  }
  
  async disconnectWallet(): Promise<User> {
    const response = await this.post<BaseResponse<User>>(`${this.baseUrl}/me/disconnect-wallet`);
    
    if (response.success && response.data) {
      // 更新本地存储的用户信息
      localStorage.setItem('user_info', JSON.stringify(response.data));
      return response.data;
    }
    
    throw new Error(response.error || '断开钱包连接失败');
  }
}

// 导出单例实例
export const userService = new UserService(); 