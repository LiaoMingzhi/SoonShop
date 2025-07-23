import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios';

// API配置
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8000';
const API_TIMEOUT = 30000;

// 创建axios实例
const apiClient: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  timeout: API_TIMEOUT,
  headers: {
    'Content-Type': 'application/json',
  },
});

// 请求拦截器
apiClient.interceptors.request.use(
  (config) => {
    // 添加认证令牌
    const token = localStorage.getItem('access_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    
    // 添加请求ID用于追踪
    config.headers['X-Request-ID'] = generateRequestId();
    
    // 添加时间戳
    config.headers['X-Timestamp'] = new Date().toISOString();
    
    console.log(`🚀 API Request: ${config.method?.toUpperCase()} ${config.url}`, config.data);
    return config;
  },
  (error) => {
    console.error('❌ Request Error:', error);
    return Promise.reject(error);
  }
);

// 响应拦截器
apiClient.interceptors.response.use(
  (response: AxiosResponse) => {
    console.log(`✅ API Response: ${response.config.method?.toUpperCase()} ${response.config.url}`, response.data);
    return response;
  },
  async (error) => {
    console.error('❌ Response Error:', error);
    
    const { response } = error;
    
    if (response) {
      switch (response.status) {
        case 401:
          // 未授权，清除本地存储并跳转到登录页
          localStorage.removeItem('access_token');
          localStorage.removeItem('refresh_token');
          window.location.href = '/auth/login';
          console.error('登录已过期，请重新登录');
          break;
          
        case 403:
          console.error('权限不足，无法访问此资源');
          break;
          
        case 404:
          console.error('请求的资源不存在');
          break;
          
        case 429:
          console.error('请求过于频繁，请稍后再试');
          break;
          
        case 500:
          console.error('服务器内部错误，请稍后再试');
          break;
          
        default:
          const errorMessage = response.data?.message || response.data?.error || '请求失败';
          console.error(errorMessage);
      }
    } else {
      // 网络错误
      console.error('网络连接失败，请检查网络设置');
    }
    
    return Promise.reject(error);
  }
);

// 工具函数
function generateRequestId(): string {
  return Math.random().toString(36).substr(2, 9);
}

// API基础服务类
export class ApiService {
  protected client: AxiosInstance;
  
  constructor() {
    this.client = apiClient;
  }
  
  // GET请求
  protected async get<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.get<T>(url, config);
    return response.data;
  }
  
  // POST请求
  protected async post<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.post<T>(url, data, config);
    return response.data;
  }
  
  // PUT请求
  protected async put<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.put<T>(url, data, config);
    return response.data;
  }
  
  // DELETE请求
  protected async delete<T>(url: string, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.delete<T>(url, config);
    return response.data;
  }
  
  // PATCH请求
  protected async patch<T>(url: string, data?: any, config?: AxiosRequestConfig): Promise<T> {
    const response = await this.client.patch<T>(url, data, config);
    return response.data;
  }
}

// 导出配置好的axios实例
export default apiClient; 