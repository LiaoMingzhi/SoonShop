import apiClient, { ApiService } from './api';
import type { ApiResponse } from '../types';

export interface CartItem {
  id: string;
  voucherId: string;
  voucherName: string;
  voucherImage: string;
  unitPrice: number;
  quantity: number;
  maxQuantity: number;
  category: string;
  producerId: string;
  producerName: string;
  expiresAt: string;
  addedAt: string;
}

export interface Cart {
  id: string;
  userId: string;
  items: CartItem[];
  summary: {
    totalItems: number;
    totalQuantity: number;
    subtotal: number;
    estimatedRewards: number;
    totalSavings: number;
  };
  lastUpdated: string;
}

export interface CartUpdateRequest {
  voucherId: string;
  quantity: number;
  action: 'add' | 'update' | 'remove';
}

export interface CheckoutRequest {
  items: Array<{
    voucherId: string;
    quantity: number;
  }>;
  deliveryMethod: 'pickup' | 'delivery';
  deliveryAddress?: {
    full: string;
    city: string;
    district: string;
    coordinates: {
      latitude: number;
      longitude: number;
    };
  };
  contactInfo: {
    name: string;
    phone: string;
    email?: string;
  };
  specialInstructions?: string;
  paymentMethod: 'voucher' | 'points' | 'hybrid';
}

export interface CheckoutResult {
  orderId: string;
  orderNumber: string;
  total: {
    subtotal: number;
    deliveryFee: number;
    serviceFee: number;
    finalTotal: number;
  };
  estimatedDelivery: string;
  paymentStatus: 'pending' | 'completed' | 'failed';
  items: Array<{
    voucherId: string;
    quantity: number;
    status: 'confirmed' | 'pending' | 'out_of_stock';
  }>;
}

class CartService extends ApiService {
  /**
   * 获取购物车
   */
  async getCart(): Promise<ApiResponse<Cart>> {
    return await this.get('/cart');
  }

  /**
   * 添加商品到购物车
   */
  async addToCart(voucherId: string, quantity: number = 1): Promise<ApiResponse<Cart>> {
    return await this.post('/cart/items', {
      voucherId,
      quantity,
      action: 'add'
    });
  }

  /**
   * 更新购物车商品数量
   */
  async updateCartItem(voucherId: string, quantity: number): Promise<ApiResponse<Cart>> {
    return await this.put('/cart/items', {
      voucherId,
      quantity,
      action: 'update'
    });
  }

  /**
   * 从购物车移除商品
   */
  async removeFromCart(voucherId: string): Promise<ApiResponse<Cart>> {
    return await this.delete(`/cart/items/${voucherId}`);
  }

  /**
   * 批量更新购物车
   */
  async batchUpdateCart(updates: CartUpdateRequest[]): Promise<ApiResponse<Cart>> {
    return await this.put('/cart/batch', { updates });
  }

  /**
   * 清空购物车
   */
  async clearCart(): Promise<ApiResponse<void>> {
    return await this.delete('/cart');
  }

  /**
   * 计算购物车总价
   */
  async calculateCart(): Promise<ApiResponse<{
    subtotal: number;
    totalQuantity: number;
    estimatedRewards: number;
    totalSavings: number;
    breakdown: Array<{
      voucherId: string;
      unitPrice: number;
      quantity: number;
      lineTotal: number;
      estimatedReward: number;
    }>;
  }>> {
    return await this.get('/cart/calculate');
  }

  /**
   * 检查库存可用性
   */
  async checkAvailability(): Promise<ApiResponse<{
    available: boolean;
    unavailableItems: Array<{
      voucherId: string;
      reason: 'out_of_stock' | 'expired' | 'limit_exceeded';
      availableQuantity: number;
    }>;
  }>> {
    return await this.get('/cart/availability');
  }

  /**
   * 获取推荐商品
   */
  async getRecommendations(): Promise<ApiResponse<Array<{
    voucherId: string;
    voucherName: string;
    voucherImage: string;
    unitPrice: number;
    category: string;
    reason: 'frequently_bought_together' | 'similar_category' | 'trending' | 'personalized';
    compatibilityScore: number;
  }>>> {
    return await this.get('/cart/recommendations');
  }

  /**
   * 保存购物车（用于未登录用户）
   */
  async saveCart(cartData: {
    items: Array<{
      voucherId: string;
      quantity: number;
    }>;
  }): Promise<ApiResponse<{ cartId: string }>> {
    return await this.post('/cart/save', cartData);
  }

  /**
   * 恢复购物车（用于登录后）
   */
  async restoreCart(cartId: string): Promise<ApiResponse<Cart>> {
    return await this.post('/cart/restore', { cartId });
  }

  /**
   * 合并购物车（本地和远程）
   */
  async mergeCart(localItems: Array<{
    voucherId: string;
    quantity: number;
  }>): Promise<ApiResponse<Cart>> {
    return await this.post('/cart/merge', { items: localItems });
  }

  /**
   * 获取购物车历史
   */
  async getCartHistory(limit?: number): Promise<ApiResponse<Array<{
    date: string;
    items: Array<{
      voucherId: string;
      voucherName: string;
      quantity: number;
      action: 'added' | 'removed' | 'updated';
    }>;
  }>>> {
    const params = new URLSearchParams();
    if (limit) params.append('limit', limit.toString());
    
    return await this.get(`/cart/history?${params.toString()}`);
  }

  /**
   * 预订购物车商品
   */
  async reserveItems(duration: number = 15): Promise<ApiResponse<{
    reservationId: string;
    expiresAt: string;
    items: Array<{
      voucherId: string;
      reservedQuantity: number;
      status: 'reserved' | 'failed';
    }>;
  }>> {
    return await this.post('/cart/reserve', { duration });
  }

  /**
   * 释放预订
   */
  async releaseReservation(reservationId: string): Promise<ApiResponse<void>> {
    return await this.delete(`/cart/reserve/${reservationId}`);
  }

  /**
   * 结算购物车
   */
  async checkout(request: CheckoutRequest): Promise<ApiResponse<CheckoutResult>> {
    return await this.post('/cart/checkout', request);
  }

  /**
   * 获取配送选项
   */
  async getDeliveryOptions(address?: {
    latitude: number;
    longitude: number;
  }): Promise<ApiResponse<Array<{
    method: 'pickup' | 'standard_delivery' | 'express_delivery';
    name: string;
    description: string;
    fee: number;
    estimatedTime: string;
    available: boolean;
  }>>> {
    const params = new URLSearchParams();
    if (address) {
      params.append('lat', address.latitude.toString());
      params.append('lng', address.longitude.toString());
    }
    
    return await this.get(`/cart/delivery-options?${params.toString()}`);
  }

  /**
   * 验证优惠券
   */
  async validateCoupon(couponCode: string): Promise<ApiResponse<{
    valid: boolean;
    coupon?: {
      code: string;
      name: string;
      type: 'percentage' | 'fixed' | 'free_shipping';
      value: number;
      minimumAmount: number;
      applicableItems: string[];
    };
    discount: number;
    error?: string;
  }>> {
    return await this.post('/cart/validate-coupon', { couponCode });
  }

  /**
   * 应用优惠券
   */
  async applyCoupon(couponCode: string): Promise<ApiResponse<Cart>> {
    return await this.post('/cart/apply-coupon', { couponCode });
  }

  /**
   * 移除优惠券
   */
  async removeCoupon(): Promise<ApiResponse<Cart>> {
    return await this.delete('/cart/coupon');
  }

  /**
   * 获取购物车分享链接
   */
  async shareCart(): Promise<ApiResponse<{
    shareUrl: string;
    shareCode: string;
    expiresAt: string;
  }>> {
    return await this.post('/cart/share');
  }

  /**
   * 从分享链接加载购物车
   */
  async loadSharedCart(shareCode: string): Promise<ApiResponse<{
    items: CartItem[];
    sharedBy: {
      name: string;
      avatar?: string;
    };
    sharedAt: string;
  }>> {
    return await this.get(`/cart/shared/${shareCode}`);
  }

  /**
   * 导入分享的购物车
   */
  async importSharedCart(shareCode: string, selectedItems?: string[]): Promise<ApiResponse<Cart>> {
    return await this.post('/cart/import-shared', {
      shareCode,
      selectedItems
    });
  }
}

export const cartService = new CartService();
export default cartService; 