import apiClient, { ApiService } from './api';
import type { ApiResponse, PaginatedResponse, SearchFilters } from '../types';

export interface Restaurant {
  id: string;
  name: string;
  description: string;
  cuisine: string[];
  address: {
    full: string;
    city: string;
    district: string;
    latitude: number;
    longitude: number;
  };
  contact: {
    phone: string;
    email?: string;
  };
  businessHours: {
    open: string;
    close: string;
    days: string[];
  };
  rating: {
    average: number;
    count: number;
  };
  priceRange: 'budget' | 'moderate' | 'expensive' | 'luxury';
  features: string[];
  images: string[];
  status: 'open' | 'closed' | 'busy';
  capacity: {
    total: number;
    available: number;
  };
}

export interface MenuItem {
  id: string;
  restaurantId: string;
  name: string;
  description: string;
  price: number;
  category: string;
  images: string[];
  ingredients: string[];
  allergens: string[];
  nutritionInfo?: {
    calories: number;
    protein: number;
    carbs: number;
    fat: number;
  };
  availability: boolean;
  preparationTime: number; // 分钟
  spicyLevel?: 1 | 2 | 3 | 4 | 5;
  tags: string[];
}

export interface Reservation {
  id: string;
  restaurantId: string;
  customerId: string;
  partySize: number;
  reservationDate: string;
  reservationTime: string;
  specialRequests?: string;
  status: 'pending' | 'confirmed' | 'seated' | 'completed' | 'cancelled';
  tableNumber?: string;
  createdAt: string;
  contact: {
    name: string;
    phone: string;
  };
}

export interface Order {
  id: string;
  restaurantId: string;
  customerId: string;
  reservationId?: string;
  items: Array<{
    menuItemId: string;
    quantity: number;
    price: number;
    specialInstructions?: string;
    modifications?: string[];
  }>;
  total: {
    subtotal: number;
    tax: number;
    serviceCharge: number;
    tip: number;
    final: number;
  };
  status: 'pending' | 'confirmed' | 'preparing' | 'ready' | 'served' | 'completed' | 'cancelled';
  orderType: 'dine-in' | 'takeout' | 'delivery';
  estimatedTime: number; // 分钟
  createdAt: string;
  completedAt?: string;
}

export interface RestaurantFilters {
  page?: number;
  limit?: number;
  cuisine?: string[];
  priceRange?: ('budget' | 'moderate' | 'expensive' | 'luxury')[];
  rating?: number;
  features?: string[];
  location?: {
    latitude: number;
    longitude: number;
    radius: number; // km
  };
  availability?: boolean;
  sort?: {
    field: string;
    order: 'asc' | 'desc';
  };
}

export interface ReservationRequest {
  restaurantId: string;
  partySize: number;
  reservationDate: string;
  reservationTime: string;
  specialRequests?: string;
  contact: {
    name: string;
    phone: string;
  };
}

export interface OrderRequest {
  restaurantId: string;
  reservationId?: string;
  items: Array<{
    menuItemId: string;
    quantity: number;
    specialInstructions?: string;
    modifications?: string[];
  }>;
  orderType: 'dine-in' | 'takeout' | 'delivery';
  paymentMethod: string;
}

class RestaurantService extends ApiService {
  /**
   * 获取餐厅列表
   */
  async getRestaurants(filters?: RestaurantFilters): Promise<ApiResponse<PaginatedResponse<Restaurant>>> {
    const params = new URLSearchParams();
    
    if (filters) {
      if (filters.page) params.append('page', filters.page.toString());
      if (filters.limit) params.append('limit', filters.limit.toString());
      if (filters.cuisine) params.append('cuisine', filters.cuisine.join(','));
      if (filters.priceRange) params.append('priceRange', filters.priceRange.join(','));
      if (filters.rating) params.append('minRating', filters.rating.toString());
      if (filters.features) params.append('features', filters.features.join(','));
      if (filters.location) {
        params.append('lat', filters.location.latitude.toString());
        params.append('lng', filters.location.longitude.toString());
        params.append('radius', filters.location.radius.toString());
      }
      if (filters.availability !== undefined) params.append('availability', filters.availability.toString());
      if (filters.sort) {
        params.append('sortField', filters.sort.field);
        params.append('sortOrder', filters.sort.order);
      }
    }

    return await this.get(`/restaurants?${params.toString()}`);
  }

  /**
   * 获取餐厅详情
   */
  async getRestaurantById(id: string): Promise<ApiResponse<Restaurant>> {
    return await this.get(`/restaurants/${id}`);
  }

  /**
   * 搜索餐厅
   */
  async searchRestaurants(query: string, filters?: RestaurantFilters): Promise<ApiResponse<PaginatedResponse<Restaurant>>> {
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

    return await this.get(`/restaurants/search?${params.toString()}`);
  }

  /**
   * 获取菜单
   */
  async getMenu(restaurantId: string, category?: string): Promise<ApiResponse<MenuItem[]>> {
    const params = new URLSearchParams();
    if (category) params.append('category', category);
    
    return await this.get(`/restaurants/${restaurantId}/menu?${params.toString()}`);
  }

  /**
   * 获取菜品详情
   */
  async getMenuItem(restaurantId: string, itemId: string): Promise<ApiResponse<MenuItem>> {
    return await this.get(`/restaurants/${restaurantId}/menu/${itemId}`);
  }

  /**
   * 检查餐厅可用性
   */
  async checkAvailability(restaurantId: string, date: string, time: string, partySize: number): Promise<ApiResponse<{
    available: boolean;
    suggestedTimes?: string[];
    waitTime?: number;
  }>> {
    return await this.get(`/restaurants/${restaurantId}/availability?date=${date}&time=${time}&partySize=${partySize}`);
  }

  /**
   * 创建预订
   */
  async createReservation(data: ReservationRequest): Promise<ApiResponse<Reservation>> {
    return await this.post('/restaurants/reservations', data);
  }

  /**
   * 获取预订详情
   */
  async getReservation(id: string): Promise<ApiResponse<Reservation>> {
    return await this.get(`/restaurants/reservations/${id}`);
  }

  /**
   * 取消预订
   */
  async cancelReservation(id: string, reason?: string): Promise<ApiResponse<void>> {
    return await this.put(`/restaurants/reservations/${id}/cancel`, { reason });
  }

  /**
   * 修改预订
   */
  async updateReservation(id: string, data: Partial<ReservationRequest>): Promise<ApiResponse<Reservation>> {
    return await this.put(`/restaurants/reservations/${id}`, data);
  }

  /**
   * 获取用户预订历史
   */
  async getUserReservations(status?: string): Promise<ApiResponse<PaginatedResponse<Reservation>>> {
    const params = new URLSearchParams();
    if (status) params.append('status', status);
    
    return await this.get(`/restaurants/reservations/user?${params.toString()}`);
  }

  /**
   * 创建订单
   */
  async createOrder(data: OrderRequest): Promise<ApiResponse<Order>> {
    return await this.post('/restaurants/orders', data);
  }

  /**
   * 获取订单详情
   */
  async getOrder(id: string): Promise<ApiResponse<Order>> {
    return await this.get(`/restaurants/orders/${id}`);
  }

  /**
   * 更新订单状态
   */
  async updateOrderStatus(id: string, status: Order['status']): Promise<ApiResponse<Order>> {
    return await this.put(`/restaurants/orders/${id}/status`, { status });
  }

  /**
   * 取消订单
   */
  async cancelOrder(id: string, reason?: string): Promise<ApiResponse<void>> {
    return await this.put(`/restaurants/orders/${id}/cancel`, { reason });
  }

  /**
   * 获取用户订单历史
   */
  async getUserOrders(status?: string): Promise<ApiResponse<PaginatedResponse<Order>>> {
    const params = new URLSearchParams();
    if (status) params.append('status', status);
    
    return await this.get(`/restaurants/orders/user?${params.toString()}`);
  }

  /**
   * 计算订单总价
   */
  async calculateOrderTotal(restaurantId: string, items: OrderRequest['items']): Promise<ApiResponse<{
    subtotal: number;
    tax: number;
    serviceCharge: number;
    estimatedTotal: number;
  }>> {
    return await this.post(`/restaurants/${restaurantId}/calculate-total`, { items });
  }

  /**
   * 提交餐厅评价
   */
  async rateRestaurant(restaurantId: string, rating: {
    overallRating: number;
    foodRating: number;
    serviceRating: number;
    atmosphereRating: number;
    valueRating: number;
    comment?: string;
    photos?: string[];
  }): Promise<ApiResponse<void>> {
    return await this.post(`/restaurants/${restaurantId}/reviews`, rating);
  }

  /**
   * 获取餐厅评价
   */
  async getRestaurantReviews(restaurantId: string, page?: number, limit?: number): Promise<ApiResponse<PaginatedResponse<{
    id: string;
    userId: string;
    userName: string;
    rating: number;
    comment: string;
    photos: string[];
    createdAt: string;
  }>>> {
    const params = new URLSearchParams();
    if (page) params.append('page', page.toString());
    if (limit) params.append('limit', limit.toString());
    
    return await this.get(`/restaurants/${restaurantId}/reviews?${params.toString()}`);
  }

  /**
   * 获取热门餐厅
   */
  async getPopularRestaurants(limit?: number): Promise<ApiResponse<Restaurant[]>> {
    const params = new URLSearchParams();
    if (limit) params.append('limit', limit.toString());
    
    return await this.get(`/restaurants/popular?${params.toString()}`);
  }

  /**
   * 获取推荐餐厅
   */
  async getRecommendedRestaurants(based_on?: 'location' | 'preferences' | 'history'): Promise<ApiResponse<Restaurant[]>> {
    const params = new URLSearchParams();
    if (based_on) params.append('based_on', based_on);
    
    return await this.get(`/restaurants/recommended?${params.toString()}`);
  }

  /**
   * 获取餐厅统计数据
   */
  async getRestaurantStats(restaurantId: string): Promise<ApiResponse<{
    totalReservations: number;
    totalOrders: number;
    averageRating: number;
    revenue: number;
    popularDishes: Array<{
      itemId: string;
      itemName: string;
      orderCount: number;
    }>;
    busyHours: Array<{
      hour: number;
      reservationCount: number;
    }>;
  }>> {
    return await this.get(`/restaurants/${restaurantId}/stats`);
  }
}

export const restaurantService = new RestaurantService();
export default restaurantService; 