import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { Voucher, VoucherCategory, PaginationResponse } from '../types';
import { voucherService, VoucherListParams } from '../services/voucherService';

export interface VoucherFilters {
  category?: VoucherCategory;
  status?: 'active' | 'paused' | 'expired' | 'cancelled';
  priceRange?: {
    min: number;
    max: number;
  };
  location?: string;
  producerId?: string;
  expiryDate?: 'week' | 'month' | 'quarter';
  sort?: {
    field: string;
    order: 'asc' | 'desc';
  };
}

export interface VoucherState {
  // 数据状态
  vouchers: Voucher[];
  currentVoucher: Voucher | null;
  pagination: {
    current: number;
    total: number;
    pageSize: number;
    totalPages: number;
  };
  
  // UI状态
  loading: boolean;
  error: string | null;
  filters: VoucherFilters;
  searchQuery: string;
  sortBy: 'newest' | 'oldest' | 'price_low' | 'price_high' | 'popularity' | 'rating';
  viewMode: 'grid' | 'list';
  
  // 收藏和购物车
  favorites: string[];
  recentViews: string[];
  
  // 缓存控制
  lastFetch: number | null;
  cacheTimeout: number; // 毫秒
}

export interface VoucherActions {
  // 数据操作
  fetchVouchers: (page?: number, resetList?: boolean) => Promise<void>;
  fetchVoucherById: (id: string) => Promise<void>;
  searchVouchers: (query: string) => Promise<void>;
  refreshVouchers: () => Promise<void>;
  
  // 筛选和排序
  setFilters: (filters: Partial<VoucherFilters>) => void;
  clearFilters: () => void;
  setSortBy: (sortBy: VoucherState['sortBy']) => void;
  setSearchQuery: (query: string) => void;
  
  // UI控制
  setViewMode: (mode: 'grid' | 'list') => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
  
  // 收藏功能
  toggleFavorite: (voucherId: string) => void;
  getFavorites: () => Promise<Voucher[]>;
  
  // 浏览历史
  addToRecentViews: (voucherId: string) => void;
  clearRecentViews: () => void;
  
  // 缓存管理
  shouldRefresh: () => boolean;
  clearCache: () => void;
  
  // 重置状态
  reset: () => void;
}

type VoucherStore = VoucherState & VoucherActions;

const initialState: VoucherState = {
  vouchers: [],
  currentVoucher: null,
  pagination: {
    current: 1,
    total: 0,
    pageSize: 12,
    totalPages: 0,
  },
  loading: false,
  error: null,
  filters: {},
  searchQuery: '',
  sortBy: 'newest',
  viewMode: 'grid',
  favorites: [],
  recentViews: [],
  lastFetch: null,
  cacheTimeout: 5 * 60 * 1000, // 5分钟
};

export const useVoucherStore = create<VoucherStore>()(
  persist(
    (set, get) => ({
      ...initialState,

      // 获取提货券列表
      fetchVouchers: async (page = 1, resetList = false) => {
        const state = get();
        
        // 如果是刷新操作或页面变化，设置loading
        if (resetList || page !== state.pagination.current) {
          set({ loading: true, error: null });
        }

        try {
          const params: VoucherListParams = {
            page,
            limit: state.pagination.pageSize,
            category: state.filters.category,
            isActive: state.filters.status === 'active',
            sortBy: state.sortBy.includes('price') ? 'total_supply' : 
                   state.sortBy === 'popularity' ? 'claimed_supply' :
                   state.sortBy === 'oldest' ? 'created_at' : 'created_at',
            sortOrder: state.sortBy.includes('low') || state.sortBy === 'oldest' ? 'asc' : 'desc'
          };

          const response = await voucherService.getVouchers(params);
          
          const { items, total, page: currentPage, totalPages } = response;
          
          set({
            vouchers: resetList ? items : [...state.vouchers, ...items],
            pagination: {
              current: currentPage,
              total,
              pageSize: state.pagination.pageSize,
              totalPages,
            },
            loading: false,
            lastFetch: Date.now(),
          });
        } catch (error) {
          console.error('Failed to fetch vouchers:', error);
          set({
            error: error instanceof Error ? error.message : '获取提货券失败',
            loading: false,
          });
        }
      },

      // 获取单个提货券详情
      fetchVoucherById: async (id: string) => {
        set({ loading: true, error: null });

        try {
          const voucher = await voucherService.getVoucherById(id);
          
          set({
            currentVoucher: voucher,
            loading: false,
          });
          
          // 添加到浏览历史
          get().addToRecentViews(id);
        } catch (error) {
          console.error('Failed to fetch voucher:', error);
          set({
            error: error instanceof Error ? error.message : '获取提货券详情失败',
            loading: false,
          });
        }
      },

      // 搜索提货券
      searchVouchers: async (query: string) => {
        set({ loading: true, error: null, searchQuery: query });

        try {
          const filters = {
            category: get().filters.category,
            isActive: get().filters.status === 'active',
          };

          const vouchers = await voucherService.searchVouchers(query, filters);
          
          set({
            vouchers,
            pagination: {
              current: 1,
              total: vouchers.length,
              pageSize: get().pagination.pageSize,
              totalPages: Math.ceil(vouchers.length / get().pagination.pageSize),
            },
            loading: false,
            lastFetch: Date.now(),
          });
        } catch (error) {
          console.error('Failed to search vouchers:', error);
          set({
            error: error instanceof Error ? error.message : '搜索提货券失败',
            loading: false,
          });
        }
      },

      // 刷新提货券列表
      refreshVouchers: async () => {
        const state = get();
        await state.fetchVouchers(1, true);
      },

      // 设置筛选条件
      setFilters: (newFilters: Partial<VoucherFilters>) => {
        const currentFilters = get().filters;
        const updatedFilters = { ...currentFilters, ...newFilters };
        
        set({ 
          filters: updatedFilters,
          pagination: { ...get().pagination, current: 1 }
        });
        
        // 重新获取数据
        get().fetchVouchers(1, true);
      },

      // 清除筛选条件
      clearFilters: () => {
        set({ 
          filters: {},
          pagination: { ...get().pagination, current: 1 }
        });
        
        // 重新获取数据
        get().fetchVouchers(1, true);
      },

      // 设置排序方式
      setSortBy: (sortBy: VoucherState['sortBy']) => {
        set({ 
          sortBy,
          pagination: { ...get().pagination, current: 1 }
        });
        
        // 重新获取数据
        get().fetchVouchers(1, true);
      },

      // 设置搜索查询
      setSearchQuery: (query: string) => {
        set({ searchQuery: query });
        
        if (query.trim()) {
          get().searchVouchers(query);
        } else {
          get().fetchVouchers(1, true);
        }
      },

      // 设置视图模式
      setViewMode: (mode: 'grid' | 'list') => {
        set({ viewMode: mode });
      },

      // 设置加载状态
      setLoading: (loading: boolean) => {
        set({ loading });
      },

      // 设置错误状态
      setError: (error: string | null) => {
        set({ error });
      },

      // 切换收藏状态
      toggleFavorite: (voucherId: string) => {
        const favorites = get().favorites;
        const isFavorited = favorites.includes(voucherId);
        
        if (isFavorited) {
          set({ 
            favorites: favorites.filter(id => id !== voucherId)
          });
        } else {
          set({ 
            favorites: [...favorites, voucherId]
          });
        }
      },

      // 获取收藏的提货券
      getFavorites: async () => {
        const favoriteIds = get().favorites;
        
        if (favoriteIds.length === 0) {
          return [];
        }

        try {
          // 这里可以调用API获取收藏的提货券详情
          // 暂时返回当前列表中的收藏项
          const allVouchers = get().vouchers;
          return allVouchers.filter(voucher => favoriteIds.includes(voucher.id));
        } catch (error) {
          console.error('Failed to fetch favorites:', error);
          return [];
        }
      },

      // 添加到浏览历史
      addToRecentViews: (voucherId: string) => {
        const recentViews = get().recentViews;
        const maxRecentViews = 10;
        
        // 移除重复项并添加到开头
        const updatedViews = [
          voucherId,
          ...recentViews.filter(id => id !== voucherId)
        ].slice(0, maxRecentViews);
        
        set({ recentViews: updatedViews });
      },

      // 清除浏览历史
      clearRecentViews: () => {
        set({ recentViews: [] });
      },

      // 检查是否需要刷新
      shouldRefresh: () => {
        const { lastFetch, cacheTimeout } = get();
        if (!lastFetch) return true;
        return Date.now() - lastFetch > cacheTimeout;
      },

      // 清除缓存
      clearCache: () => {
        set({
          vouchers: [],
          currentVoucher: null,
          lastFetch: null,
          pagination: {
            ...get().pagination,
            current: 1,
            total: 0,
            totalPages: 0,
          },
        });
      },

      // 重置所有状态
      reset: () => {
        set({
          ...initialState,
          favorites: get().favorites, // 保留收藏列表
          recentViews: get().recentViews, // 保留浏览历史
        });
      },
    }),
    {
      name: 'voucher-store',
      partialize: (state) => ({
        // 只持久化部分状态
        favorites: state.favorites,
        recentViews: state.recentViews,
        filters: state.filters,
        sortBy: state.sortBy,
        viewMode: state.viewMode,
      }),
    }
  )
); 