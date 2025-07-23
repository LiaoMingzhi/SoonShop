import { useState, useCallback } from 'react';

interface ErrorInfo {
  message: string;
  stack?: string;
  componentStack?: string;
}

interface UseErrorBoundaryReturn {
  error: ErrorInfo | null;
  hasError: boolean;
  resetError: () => void;
  captureError: (error: Error, errorInfo?: any) => void;
  reportError: (error: Error, errorInfo?: any) => void;
}

export const useErrorBoundary = (): UseErrorBoundaryReturn => {
  const [error, setError] = useState<ErrorInfo | null>(null);

  const captureError = useCallback((error: Error, errorInfo?: any) => {
    const errorData: ErrorInfo = {
      message: error.message,
      stack: error.stack,
      componentStack: errorInfo?.componentStack,
    };
    
    setError(errorData);
    
    // 发送错误到监控服务
    console.error('Error captured:', errorData);
    
    // 显示错误提示
    console.error('发生了一个错误，请刷新页面重试');
  }, []);

  const clearError = useCallback(() => {
    setError(null);
  }, []);

  const resetError = useCallback(() => {
    setError(null);
    window.location.reload();
  }, []);

  const reportError = useCallback((error: Error, errorInfo?: any) => {
    console.error('Error reported:', error, errorInfo);
  }, []);

  return {
    error,
    hasError: !!error,
    resetError,
    captureError,
    reportError,
  };
};

function getUserFriendlyErrorMessage(error: Error): string {
  // 根据错误类型返回用户友好的消息
  if (error.message.includes('Network Error')) {
    return '网络连接失败，请检查网络设置';
  }
  
  if (error.message.includes('timeout')) {
    return '请求超时，请稍后重试';
  }
  
  if (error.message.includes('401')) {
    return '登录已过期，请重新登录';
  }
  
  if (error.message.includes('403')) {
    return '权限不足，无法访问此资源';
  }
  
  if (error.message.includes('404')) {
    return '请求的资源不存在';
  }
  
  if (error.message.includes('500')) {
    return '服务器内部错误，请稍后重试';
  }
  
  // 默认错误消息
  return '发生未知错误，请稍后重试';
}

// 错误边界组件Hook
export function useAsyncError() {
  const [, setError] = useState();
  
  return useCallback(
    (error: Error) => {
      setError(() => {
        throw error;
      });
    },
    [setError]
  );
} 