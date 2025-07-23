import React, { createContext, useContext, useState, useCallback, useEffect } from 'react';

// 消息类型
export type ToastType = 'success' | 'error' | 'warning' | 'info';

// 消息接口
export interface Toast {
  id: string;
  type: ToastType;
  title: string;
  message?: string;
  duration?: number;
}

// 上下文类型
interface ToastContextType {
  toasts: Toast[];
  addToast: (toast: Omit<Toast, 'id'>) => void;
  removeToast: (id: string) => void;
}

// 创建上下文
const ToastContext = createContext<ToastContextType | undefined>(undefined);

// 提供者组件
export const ToastProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const addToast = useCallback((toast: Omit<Toast, 'id'>) => {
    const id = Math.random().toString(36).substr(2, 9);
    const newToast = { ...toast, id };
    
    setToasts(prev => [...prev, newToast]);
    
    // 自动移除toast
    const duration = toast.duration || 4000;
    setTimeout(() => {
      removeToast(id);
    }, duration);
  }, []);

  const removeToast = useCallback((id: string) => {
    setToasts(prev => prev.filter(toast => toast.id !== id));
  }, []);

  return (
    <ToastContext.Provider value={{ toasts, addToast, removeToast }}>
      {children}
      <ToastContainer toasts={toasts} removeToast={removeToast} />
    </ToastContext.Provider>
  );
};

// 使用 Toast 的 Hook
export const useToast = () => {
  const context = useContext(ToastContext);
  if (!context) {
    throw new Error('useToast must be used within a ToastProvider');
  }
  return context;
};

const ToastContainer: React.FC<{
  toasts: Toast[];
  removeToast: (id: string) => void;
}> = ({ toasts, removeToast }) => {
  if (toasts.length === 0) return null;

  return (
    <div className="fixed top-4 right-4 z-50 space-y-2">
      {toasts.map(toast => (
        <ToastItem key={toast.id} toast={toast} onRemove={removeToast} />
      ))}
    </div>
  );
};

// 单个 Toast 组件
const ToastItem: React.FC<{
  toast: Toast;
  onRemove: (id: string) => void;
}> = ({ toast, onRemove }) => {
  const typeStyles = {
    success: 'bg-green-50 border-green-200 text-green-800',
    error: 'bg-red-50 border-red-200 text-red-800',
    warning: 'bg-yellow-50 border-yellow-200 text-yellow-800',
    info: 'bg-blue-50 border-blue-200 text-blue-800'
  };

  const iconMap = {
    success: 'fas fa-check-circle',
    error: 'fas fa-exclamation-circle',
    warning: 'fas fa-exclamation-triangle',
    info: 'fas fa-info-circle'
  };

  return (
    <div className={`${typeStyles[toast.type]} border rounded-lg p-4 shadow-lg min-w-80 animate-slide-in`}>
      <div className="flex items-start">
        <i className={`${iconMap[toast.type]} mr-3 mt-0.5`}></i>
        <div className="flex-1">
          <h4 className="font-medium">{toast.title}</h4>
          {toast.message && (
            <p className="mt-1 text-sm opacity-90">{toast.message}</p>
          )}
        </div>
        <button
          onClick={() => onRemove(toast.id)}
          className="ml-2 text-gray-400 hover:text-gray-600 transition-colors"
        >
          <i className="fas fa-times"></i>
        </button>
      </div>
    </div>
  );
};

// 便捷的 Toast 函数
export const toast = {
  success: (title: string, message?: string) => {
    // 这个需要在ToastProvider内部使用
    console.log('Success:', title, message);
  },
  error: (title: string, message?: string) => {
    console.log('Error:', title, message);
  },
  warning: (title: string, message?: string) => {
    console.log('Warning:', title, message);
  },
  info: (title: string, message?: string) => {
    console.log('Info:', title, message);
  }
};

// 创建一个全局的 toast 实例
let globalToastContext: ToastContextType | null = null;

export const setGlobalToastContext = (context: ToastContextType) => {
  globalToastContext = context;
};

// 全局 toast 函数（不依赖 Hook）
export const globalToast = {
  success: (title: string, message?: string) => {
    if (globalToastContext) {
      globalToastContext.addToast({ type: 'success', title, message });
    }
  },
  error: (title: string, message?: string) => {
    if (globalToastContext) {
      globalToastContext.addToast({ type: 'error', title, message });
    }
  },
  warning: (title: string, message?: string) => {
    if (globalToastContext) {
      globalToastContext.addToast({ type: 'warning', title, message });
    }
  },
  info: (title: string, message?: string) => {
    if (globalToastContext) {
      globalToastContext.addToast({ type: 'info', title, message });
    }
  }
};

// 全局 Toast 初始化组件
export const ToastInitializer: React.FC = () => {
  const toastContext = useToast();
  
  useEffect(() => {
    setGlobalToastContext(toastContext);
  }, [toastContext]);
  
  return null;
};

export default ToastProvider; 