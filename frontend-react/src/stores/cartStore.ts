import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { CartItem, Cart, CheckoutRequest, CheckoutResult } from '../services/cartService';
import { cartService } from '../services/cartService';

export interface CartState {
  // 购物车数据
  cart: Cart | null;
  items: CartItem[];
  summary: {
    totalItems: number;
    totalQuantity: number;
    subtotal: number;
    estimatedRewards: number;
    totalSavings: number;
  };
  
  // UI状态
  loading: boolean;
  error: string | null;
  isOpen: boolean; // 购物车侧边栏是否打开
  
  // 结算状态
  checkoutLoading: boolean;
  checkoutResult: CheckoutResult | null;
  
  // 本地状态（未登录用户）
  localItems: Array<{
    voucherId: string;
    quantity: number;
    addedAt: string;
  }>;
  
  // 同步状态
  lastSync: number | null;
  needsSync: boolean;
}

export interface CartActions {
  // 基础操作
  fetchCart: () => Promise<void>;
  addToCart: (voucherId: string, quantity?: number) => Promise<void>;
  updateQuantity: (voucherId: string, quantity: number) => Promise<void>;
  removeFromCart: (voucherId: string) => Promise<void>;
  clearCart: () => Promise<void>;
  
  // 批量操作
  batchUpdate: (updates: Array<{ voucherId: string; quantity: number; action: 'add' | 'update' | 'remove' }>) => Promise<void>;
  
  // UI控制
  toggleCart: () => void;
  openCart: () => void;
  closeCart: () => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  
  // 本地购物车（未登录用户）
  addToLocalCart: (voucherId: string, quantity?: number) => void;
  updateLocalQuantity: (voucherId: string, quantity: number) => void;
  removeFromLocalCart: (voucherId: string) => void;
  clearLocalCart: () => void;
  
  // 同步操作
  syncCart: () => Promise<void>;
  mergeLocalCart: () => Promise<void>;
  
  // 计算和验证
  calculateCart: () => Promise<void>;
  checkAvailability: () => Promise<boolean>;
  
  // 结算相关
  checkout: (request: CheckoutRequest) => Promise<CheckoutResult | null>;
  clearCheckoutResult: () => void;
  
  // 推荐和相关
  getRecommendations: () => Promise<void>;
  
  // 优惠券
  applyCoupon: (couponCode: string) => Promise<void>;
  removeCoupon: () => Promise<void>;
  
  // 分享功能
  shareCart: () => Promise<string | null>;
  importSharedCart: (shareCode: string, selectedItems?: string[]) => Promise<void>;
  
  // 重置
  reset: () => void;
}

type CartStore = CartState & CartActions;

const initialState: CartState = {
  cart: null,
  items: [],
  summary: {
    totalItems: 0,
    totalQuantity: 0,
    subtotal: 0,
    estimatedRewards: 0,
    totalSavings: 0,
  },
  loading: false,
  error: null,
  isOpen: false,
  checkoutLoading: false,
  checkoutResult: null,
  localItems: [],
  lastSync: null,
  needsSync: false,
};

export const useCartStore = create<CartStore>()(
  persist(
    (set, get) => ({
      ...initialState,

      // 获取购物车
      fetchCart: async () => {
        set({ loading: true, error: null });

        try {
          const response = await cartService.getCart();
          
          if (response.code === 200 && response.data) {
            const cart = response.data;
            set({
              cart,
              items: cart.items,
              summary: cart.summary,
              loading: false,
              lastSync: Date.now(),
              needsSync: false,
            });
          } else {
            throw new Error(response.message || '获取购物车失败');
          }
        } catch (error) {
          console.error('Failed to fetch cart:', error);
          set({
            error: error instanceof Error ? error.message : '获取购物车失败',
            loading: false,
          });
        }
      },

      // 添加到购物车
      addToCart: async (voucherId: string, quantity = 1) => {
        const isLoggedIn = true; // 这里应该从auth store获取登录状态
        
        if (!isLoggedIn) {
          get().addToLocalCart(voucherId, quantity);
          return;
        }

        set({ loading: true, error: null });

        try {
          const response = await cartService.addToCart(voucherId, quantity);
          
          if (response.code === 200 && response.data) {
            const cart = response.data;
            set({
              cart,
              items: cart.items,
              summary: cart.summary,
              loading: false,
              lastSync: Date.now(),
            });
            
            // 自动打开购物车
            set({ isOpen: true });
          } else {
            throw new Error(response.message || '添加到购物车失败');
          }
        } catch (error) {
          console.error('Failed to add to cart:', error);
          set({
            error: error instanceof Error ? error.message : '添加到购物车失败',
            loading: false,
          });
        }
      },

      // 更新商品数量
      updateQuantity: async (voucherId: string, quantity: number) => {
        const isLoggedIn = true; // 这里应该从auth store获取登录状态
        
        if (!isLoggedIn) {
          get().updateLocalQuantity(voucherId, quantity);
          return;
        }

        set({ loading: true, error: null });

        try {
          const response = await cartService.updateCartItem(voucherId, quantity);
          
          if (response.code === 200 && response.data) {
            const cart = response.data;
            set({
              cart,
              items: cart.items,
              summary: cart.summary,
              loading: false,
              lastSync: Date.now(),
            });
          } else {
            throw new Error(response.message || '更新购物车失败');
          }
        } catch (error) {
          console.error('Failed to update cart:', error);
          set({
            error: error instanceof Error ? error.message : '更新购物车失败',
            loading: false,
          });
        }
      },

      // 从购物车移除
      removeFromCart: async (voucherId: string) => {
        const isLoggedIn = true; // 这里应该从auth store获取登录状态
        
        if (!isLoggedIn) {
          get().removeFromLocalCart(voucherId);
          return;
        }

        set({ loading: true, error: null });

        try {
          const response = await cartService.removeFromCart(voucherId);
          
          if (response.code === 200) {
            // 重新获取购物车
            await get().fetchCart();
          } else {
            throw new Error(response.message || '移除商品失败');
          }
        } catch (error) {
          console.error('Failed to remove from cart:', error);
          set({
            error: error instanceof Error ? error.message : '移除商品失败',
            loading: false,
          });
        }
      },

      // 清空购物车
      clearCart: async () => {
        const isLoggedIn = true; // 这里应该从auth store获取登录状态
        
        if (!isLoggedIn) {
          get().clearLocalCart();
          return;
        }

        set({ loading: true, error: null });

        try {
          const response = await cartService.clearCart();
          
          if (response.code === 200) {
            set({
              cart: null,
              items: [],
              summary: {
                totalItems: 0,
                totalQuantity: 0,
                subtotal: 0,
                estimatedRewards: 0,
                totalSavings: 0,
              },
              loading: false,
              lastSync: Date.now(),
            });
          } else {
            throw new Error(response.message || '清空购物车失败');
          }
        } catch (error) {
          console.error('Failed to clear cart:', error);
          set({
            error: error instanceof Error ? error.message : '清空购物车失败',
            loading: false,
          });
        }
      },

      // 批量更新
      batchUpdate: async (updates) => {
        set({ loading: true, error: null });

        try {
          const response = await cartService.batchUpdateCart(updates);
          
          if (response.code === 200 && response.data) {
            const cart = response.data;
            set({
              cart,
              items: cart.items,
              summary: cart.summary,
              loading: false,
              lastSync: Date.now(),
            });
          } else {
            throw new Error(response.message || '批量更新失败');
          }
        } catch (error) {
          console.error('Failed to batch update cart:', error);
          set({
            error: error instanceof Error ? error.message : '批量更新失败',
            loading: false,
          });
        }
      },

      // UI控制
      toggleCart: () => {
        set({ isOpen: !get().isOpen });
      },

      openCart: () => {
        set({ isOpen: true });
      },

      closeCart: () => {
        set({ isOpen: false });
      },

      setLoading: (loading: boolean) => {
        set({ loading });
      },

      setError: (error: string | null) => {
        set({ error });
      },

      // 本地购物车操作（未登录用户）
      addToLocalCart: (voucherId: string, quantity = 1) => {
        const localItems = get().localItems;
        const existingItem = localItems.find(item => item.voucherId === voucherId);
        
        if (existingItem) {
          // 更新数量
          const updatedItems = localItems.map(item =>
            item.voucherId === voucherId
              ? { ...item, quantity: item.quantity + quantity }
              : item
          );
          set({ localItems: updatedItems, needsSync: true });
        } else {
          // 添加新商品
          const newItem = {
            voucherId,
            quantity,
            addedAt: new Date().toISOString(),
          };
          set({ 
            localItems: [...localItems, newItem],
            needsSync: true
          });
        }
      },

      updateLocalQuantity: (voucherId: string, quantity: number) => {
        const localItems = get().localItems;
        
        if (quantity <= 0) {
          get().removeFromLocalCart(voucherId);
          return;
        }
        
        const updatedItems = localItems.map(item =>
          item.voucherId === voucherId
            ? { ...item, quantity }
            : item
        );
        
        set({ localItems: updatedItems, needsSync: true });
      },

      removeFromLocalCart: (voucherId: string) => {
        const localItems = get().localItems;
        const updatedItems = localItems.filter(item => item.voucherId !== voucherId);
        set({ localItems: updatedItems, needsSync: true });
      },

      clearLocalCart: () => {
        set({ localItems: [], needsSync: false });
      },

      // 同步操作
      syncCart: async () => {
        const { localItems, needsSync } = get();
        
        if (!needsSync || localItems.length === 0) {
          return;
        }

        try {
          const response = await cartService.mergeCart(localItems);
          
          if (response.code === 200 && response.data) {
            const cart = response.data;
            set({
              cart,
              items: cart.items,
              summary: cart.summary,
              localItems: [],
              needsSync: false,
              lastSync: Date.now(),
            });
          }
        } catch (error) {
          console.error('Failed to sync cart:', error);
        }
      },

      mergeLocalCart: async () => {
        await get().syncCart();
      },

      // 计算购物车
      calculateCart: async () => {
        try {
          const response = await cartService.calculateCart();
          
          if (response.code === 200 && response.data) {
            const calculation = response.data;
            set({
              summary: {
                totalItems: get().items.length,
                totalQuantity: calculation.totalQuantity,
                subtotal: calculation.subtotal,
                estimatedRewards: calculation.estimatedRewards,
                totalSavings: calculation.totalSavings,
              },
            });
          }
        } catch (error) {
          console.error('Failed to calculate cart:', error);
        }
      },

      // 检查库存
      checkAvailability: async (): Promise<boolean> => {
        try {
          const response = await cartService.checkAvailability();
          
          if (response.code === 200 && response.data) {
            return response.data.available;
          }
          
          return false;
        } catch (error) {
          console.error('Failed to check availability:', error);
          return false;
        }
      },

      // 结算
      checkout: async (request: CheckoutRequest): Promise<CheckoutResult | null> => {
        set({ checkoutLoading: true, error: null });

        try {
          const response = await cartService.checkout(request);
          
          if (response.code === 200 && response.data) {
            const result = response.data;
            set({
              checkoutResult: result,
              checkoutLoading: false,
            });
            
            // 结算成功后清空购物车
            if (result.paymentStatus === 'completed') {
              await get().clearCart();
            }
            
            return result;
          } else {
            throw new Error(response.message || '结算失败');
          }
        } catch (error) {
          console.error('Failed to checkout:', error);
          set({
            error: error instanceof Error ? error.message : '结算失败',
            checkoutLoading: false,
          });
          return null;
        }
      },

      clearCheckoutResult: () => {
        set({ checkoutResult: null });
      },

      // 获取推荐
      getRecommendations: async () => {
        try {
          const response = await cartService.getRecommendations();
          // 推荐数据可以存储在单独的状态中
          // 这里暂时不处理
        } catch (error) {
          console.error('Failed to get recommendations:', error);
        }
      },

      // 应用优惠券
      applyCoupon: async (couponCode: string) => {
        set({ loading: true, error: null });

        try {
          const response = await cartService.applyCoupon(couponCode);
          
          if (response.code === 200 && response.data) {
            const cart = response.data;
            set({
              cart,
              items: cart.items,
              summary: cart.summary,
              loading: false,
            });
          } else {
            throw new Error(response.message || '应用优惠券失败');
          }
        } catch (error) {
          console.error('Failed to apply coupon:', error);
          set({
            error: error instanceof Error ? error.message : '应用优惠券失败',
            loading: false,
          });
        }
      },

      // 移除优惠券
      removeCoupon: async () => {
        set({ loading: true, error: null });

        try {
          const response = await cartService.removeCoupon();
          
          if (response.code === 200 && response.data) {
            const cart = response.data;
            set({
              cart,
              items: cart.items,
              summary: cart.summary,
              loading: false,
            });
          } else {
            throw new Error(response.message || '移除优惠券失败');
          }
        } catch (error) {
          console.error('Failed to remove coupon:', error);
          set({
            error: error instanceof Error ? error.message : '移除优惠券失败',
            loading: false,
          });
        }
      },

      // 分享购物车
      shareCart: async (): Promise<string | null> => {
        try {
          const response = await cartService.shareCart();
          
          if (response.code === 200 && response.data) {
            return response.data.shareUrl;
          }
          
          return null;
        } catch (error) {
          console.error('Failed to share cart:', error);
          return null;
        }
      },

      // 导入分享的购物车
      importSharedCart: async (shareCode: string, selectedItems?: string[]) => {
        set({ loading: true, error: null });

        try {
          const response = await cartService.importSharedCart(shareCode, selectedItems);
          
          if (response.code === 200 && response.data) {
            const cart = response.data;
            set({
              cart,
              items: cart.items,
              summary: cart.summary,
              loading: false,
            });
          } else {
            throw new Error(response.message || '导入购物车失败');
          }
        } catch (error) {
          console.error('Failed to import shared cart:', error);
          set({
            error: error instanceof Error ? error.message : '导入购物车失败',
            loading: false,
          });
        }
      },

      // 重置状态
      reset: () => {
        set({
          ...initialState,
          localItems: get().localItems, // 保留本地购物车
        });
      },
    }),
    {
      name: 'cart-store',
      partialize: (state) => ({
        // 只持久化本地购物车和UI状态
        localItems: state.localItems,
        isOpen: state.isOpen,
        needsSync: state.needsSync,
      }),
    }
  )
); 