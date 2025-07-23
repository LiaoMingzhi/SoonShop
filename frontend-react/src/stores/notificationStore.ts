import { create } from 'zustand';
import { persist } from 'zustand/middleware';

export interface Notification {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  message: string;
  timestamp: string;
  read: boolean;
  actions?: Array<{
    label: string;
    action: () => void;
    variant?: 'primary' | 'secondary' | 'danger';
  }>;
  autoClose?: number; // 自动关闭时间（毫秒）
  persistent?: boolean; // 是否持久化显示
  category?: 'system' | 'order' | 'voucher' | 'payment' | 'security';
  metadata?: Record<string, any>;
}

export interface ToastNotification {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  message: string;
  autoClose?: number;
  actions?: Array<{
    label: string;
    action: () => void;
  }>;
}

export interface NotificationSettings {
  emailNotifications: boolean;
  pushNotifications: boolean;
  smsNotifications: boolean;
  categories: {
    system: boolean;
    order: boolean;
    voucher: boolean;
    payment: boolean;
    security: boolean;
  };
  quietHours: {
    enabled: boolean;
    start: string; // HH:MM format
    end: string; // HH:MM format
  };
  sound: boolean;
  desktop: boolean;
}

export interface NotificationState {
  // 通知数据
  notifications: Notification[];
  toasts: ToastNotification[];
  unreadCount: number;
  
  // UI状态
  showNotificationPanel: boolean;
  notificationFilter: 'all' | 'unread' | 'system' | 'order' | 'voucher' | 'payment' | 'security';
  
  // 设置
  settings: NotificationSettings;
  
  // 权限状态
  permissionStatus: 'default' | 'granted' | 'denied';
  
  // 连接状态
  isConnected: boolean;
  lastSync: number | null;
}

export interface NotificationActions {
  // 通知管理
  addNotification: (notification: Omit<Notification, 'id' | 'timestamp' | 'read'>) => void;
  markAsRead: (id: string) => void;
  markAllAsRead: () => void;
  removeNotification: (id: string) => void;
  clearNotifications: () => void;
  
  // Toast管理
  showToast: (toast: Omit<ToastNotification, 'id'>) => void;
  hideToast: (id: string) => void;
  clearToasts: () => void;
  
  // 快捷方法
  showSuccess: (message: string, autoClose?: number) => void;
  showError: (message: string, persistent?: boolean) => void;
  showWarning: (message: string, autoClose?: number) => void;
  showInfo: (message: string, autoClose?: number) => void;
  
  // UI控制
  toggleNotificationPanel: () => void;
  openNotificationPanel: () => void;
  closeNotificationPanel: () => void;
  setNotificationFilter: (filter: NotificationState['notificationFilter']) => void;
  
  // 设置管理
  updateSettings: (settings: Partial<NotificationSettings>) => void;
  requestPermission: () => Promise<boolean>;
  
  // 同步
  syncNotifications: () => Promise<void>;
  
  // 系统通知
  sendDesktopNotification: (title: string, options?: NotificationOptions) => void;
  
  // 清理
  cleanup: () => void;
}

type NotificationStore = NotificationState & NotificationActions;

const DEFAULT_SETTINGS: NotificationSettings = {
  emailNotifications: true,
  pushNotifications: true,
  smsNotifications: false,
  categories: {
    system: true,
    order: true,
    voucher: true,
    payment: true,
    security: true,
  },
  quietHours: {
    enabled: false,
    start: '22:00',
    end: '08:00',
  },
  sound: true,
  desktop: true,
};

const initialState: NotificationState = {
  notifications: [],
  toasts: [],
  unreadCount: 0,
  showNotificationPanel: false,
  notificationFilter: 'all',
  settings: DEFAULT_SETTINGS,
  permissionStatus: 'default',
  isConnected: false,
  lastSync: null,
};

export const useNotificationStore = create<NotificationStore>()(
  persist(
    (set, get) => ({
      ...initialState,

      // 添加通知
      addNotification: (notificationData) => {
        const notification: Notification = {
          ...notificationData,
          id: Date.now().toString() + Math.random().toString(36).substr(2, 9),
          timestamp: new Date().toISOString(),
          read: false,
        };

        set(state => ({
          notifications: [notification, ...state.notifications].slice(0, 100), // 限制最多100条
          unreadCount: state.unreadCount + 1,
        }));

        // 如果启用了桌面通知
        const { settings } = get();
        if (settings.desktop && notification.type !== 'info') {
          get().sendDesktopNotification(notification.title, {
            body: notification.message,
            icon: '/favicon.ico',
            tag: notification.id,
          });
        }

        // 如果设置了自动关闭
        if (notification.autoClose && !notification.persistent) {
          setTimeout(() => {
            get().removeNotification(notification.id);
          }, notification.autoClose);
        }
      },

      // 标记为已读
      markAsRead: (id) => {
        set(state => ({
          notifications: state.notifications.map(notification =>
            notification.id === id 
              ? { ...notification, read: true }
              : notification
          ),
          unreadCount: Math.max(0, state.unreadCount - 1),
        }));
      },

      // 标记所有为已读
      markAllAsRead: () => {
        set(state => ({
          notifications: state.notifications.map(notification => ({
            ...notification,
            read: true,
          })),
          unreadCount: 0,
        }));
      },

      // 删除通知
      removeNotification: (id) => {
        set(state => {
          const notification = state.notifications.find(n => n.id === id);
          const unreadDecrement = notification && !notification.read ? 1 : 0;
          
          return {
            notifications: state.notifications.filter(n => n.id !== id),
            unreadCount: Math.max(0, state.unreadCount - unreadDecrement),
          };
        });
      },

      // 清空所有通知
      clearNotifications: () => {
        set({
          notifications: [],
          unreadCount: 0,
        });
      },

      // 显示Toast
      showToast: (toastData) => {
        const toast: ToastNotification = {
          ...toastData,
          id: Date.now().toString() + Math.random().toString(36).substr(2, 9),
        };

        set(state => ({
          toasts: [...state.toasts, toast].slice(-5), // 最多显示5个toast
        }));

        // 自动关闭
        const autoClose = toast.autoClose || 5000;
        setTimeout(() => {
          get().hideToast(toast.id);
        }, autoClose);
      },

      // 隐藏Toast
      hideToast: (id) => {
        set(state => ({
          toasts: state.toasts.filter(toast => toast.id !== id),
        }));
      },

      // 清空所有Toast
      clearToasts: () => {
        set({ toasts: [] });
      },

      // 快捷方法
      showSuccess: (message, autoClose = 3000) => {
        get().showToast({
          type: 'success',
          message,
          autoClose,
        });
      },

      showError: (message, persistent = false) => {
        get().showToast({
          type: 'error',
          message,
          autoClose: persistent ? undefined : 5000,
        });
      },

      showWarning: (message, autoClose = 4000) => {
        get().showToast({
          type: 'warning',
          message,
          autoClose,
        });
      },

      showInfo: (message, autoClose = 3000) => {
        get().showToast({
          type: 'info',
          message,
          autoClose,
        });
      },

      // UI控制
      toggleNotificationPanel: () => {
        set(state => ({
          showNotificationPanel: !state.showNotificationPanel,
        }));
      },

      openNotificationPanel: () => {
        set({ showNotificationPanel: true });
      },

      closeNotificationPanel: () => {
        set({ showNotificationPanel: false });
      },

      setNotificationFilter: (filter) => {
        set({ notificationFilter: filter });
      },

      // 更新设置
      updateSettings: (newSettings) => {
        set(state => ({
          settings: { ...state.settings, ...newSettings },
        }));
      },

      // 请求通知权限
      requestPermission: async () => {
        if ('Notification' in window) {
          const permission = await Notification.requestPermission();
          set({ permissionStatus: permission });
          return permission === 'granted';
        }
        return false;
      },

      // 同步通知
      syncNotifications: async () => {
        try {
          // 这里应该调用API获取最新通知
          // const response = await notificationService.getLatestNotifications();
          // if (response.success) {
          //   set({
          //     notifications: response.data,
          //     lastSync: Date.now(),
          //   });
          // }
          
          set({ lastSync: Date.now() });
        } catch (error) {
          console.error('Failed to sync notifications:', error);
        }
      },

      // 发送桌面通知
      sendDesktopNotification: (title, options = {}) => {
        const { settings, permissionStatus } = get();
        
        if (!settings.desktop || permissionStatus !== 'granted') {
          return;
        }

        // 检查静音时间
        if (settings.quietHours.enabled) {
          const now = new Date();
          const currentTime = now.getHours() * 60 + now.getMinutes();
          const startTime = parseInt(settings.quietHours.start.split(':')[0]) * 60 + 
                           parseInt(settings.quietHours.start.split(':')[1]);
          const endTime = parseInt(settings.quietHours.end.split(':')[0]) * 60 + 
                         parseInt(settings.quietHours.end.split(':')[1]);

          if (startTime <= endTime) {
            // 同一天内的静音时间
            if (currentTime >= startTime && currentTime <= endTime) {
              return;
            }
          } else {
            // 跨天的静音时间
            if (currentTime >= startTime || currentTime <= endTime) {
              return;
            }
          }
        }

        try {
          const notification = new Notification(title, {
            ...options,
            requireInteraction: false,
          });

          notification.onclick = () => {
            window.focus();
            notification.close();
          };

          // 自动关闭
          setTimeout(() => {
            notification.close();
          }, 5000);

        } catch (error) {
          console.error('Failed to send desktop notification:', error);
        }
      },

      // 清理资源
      cleanup: () => {
        set({
          toasts: [],
          showNotificationPanel: false,
        });
      },
    }),
    {
      name: 'notification-store',
      partialize: (state) => ({
        // 只持久化部分状态
        notifications: state.notifications.slice(0, 50), // 只保存最近50条
        settings: state.settings,
        notificationFilter: state.notificationFilter,
      }),
    }
  )
);

// 导出便捷的Hook
export const useNotifications = () => {
  const store = useNotificationStore();
  
  return {
    notifications: store.notifications,
    unreadCount: store.unreadCount,
    showNotificationPanel: store.showNotificationPanel,
    markAsRead: store.markAsRead,
    markAllAsRead: store.markAllAsRead,
    toggleNotificationPanel: store.toggleNotificationPanel,
    removeNotification: store.removeNotification,
  };
};

export const useToasts = () => {
  const store = useNotificationStore();
  
  return {
    toasts: store.toasts,
    showSuccess: store.showSuccess,
    showError: store.showError,
    showWarning: store.showWarning,
    showInfo: store.showInfo,
    hideToast: store.hideToast,
  };
}; 