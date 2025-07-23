import React, { Component, ErrorInfo, ReactNode } from 'react'

interface Props {
  children?: ReactNode
  fallback?: ReactNode
}

interface State {
  hasError: boolean
  error?: Error
  errorInfo?: ErrorInfo
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
  }

  public static getDerivedStateFromError(error: Error): State {
    // 更新 state 使下一次渲染能够显示降级后的 UI
    return { hasError: true, error }
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    // 你同样可以将错误日志上报给服务器
    console.error('错误边界捕获到错误:', error, errorInfo)
    
    this.setState({
      error,
      errorInfo,
    })

    // 发送错误报告到监控服务
    this.reportError(error, errorInfo)
  }

  private reportError = (error: Error, errorInfo: ErrorInfo) => {
    // 这里可以集成错误监控服务，如 Sentry
    try {
      const errorReport = {
        message: error.message,
        stack: error.stack,
        componentStack: errorInfo.componentStack,
        timestamp: new Date().toISOString(),
        url: window.location.href,
        userAgent: navigator.userAgent,
      }

      // 发送到错误监控服务
      // 这里暂时只是打印到控制台
      console.error('错误报告:', errorReport)
      
      // 实际项目中可以发送到监控服务
      // fetch('/api/v1/errors', {
      //   method: 'POST',
      //   headers: { 'Content-Type': 'application/json' },
      //   body: JSON.stringify(errorReport),
      // })
    } catch (reportError) {
      console.error('发送错误报告失败:', reportError)
    }
  }

  private handleRetry = () => {
    this.setState({
      hasError: false,
      error: undefined,
      errorInfo: undefined,
    })
  }

  public render() {
    if (this.state.hasError) {
      // 如果有自定义的错误 UI，则使用自定义的
      if (this.props.fallback) {
        return this.props.fallback
      }

      // 默认的错误 UI
      return (
        <div className="min-h-screen flex items-center justify-center bg-gray-50">
          <div className="max-w-md w-full bg-white rounded-lg shadow-lg p-6">
            <div className="text-center">
              <div className="text-6xl mb-4">😵</div>
              <h1 className="text-2xl font-bold text-gray-900 mb-2">
                出错了！
              </h1>
              <p className="text-gray-600 mb-6">
                抱歉，应用程序遇到了一个错误。我们已经记录了这个问题，会尽快修复。
              </p>
              
              <div className="space-y-4">
                <button
                  onClick={this.handleRetry}
                  className="w-full px-4 py-2 bg-red-500 text-white rounded-md hover:bg-red-600 transition-colors"
                >
                  重试
                </button>
                
                <button
                  onClick={() => window.location.reload()}
                  className="w-full px-4 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300 transition-colors"
                >
                  刷新页面
                </button>
              </div>
              
              {/* 开发环境显示错误详情 */}
              {process.env.NODE_ENV === 'development' && this.state.error && (
                <details className="mt-6 text-left">
                  <summary className="cursor-pointer text-sm text-gray-500 hover:text-gray-700">
                    查看错误详情
                  </summary>
                  <div className="mt-2 p-4 bg-red-50 border border-red-200 rounded-md">
                    <pre className="text-xs text-red-700 overflow-auto">
                      {this.state.error.message}
                      {'\n\n'}
                      {this.state.error.stack}
                      {this.state.errorInfo?.componentStack && (
                        <>
                          {'\n\n组件堆栈:'}
                          {this.state.errorInfo.componentStack}
                        </>
                      )}
                    </pre>
                  </div>
                </details>
              )}
            </div>
          </div>
        </div>
      )
    }

    return this.props.children
  }
}

// 函数式组件版本的错误边界（使用 React 18 的新特性）
export const ErrorBoundaryWrapper: React.FC<{ children: ReactNode; fallback?: ReactNode }> = ({
  children,
  fallback,
}) => {
  return (
    <ErrorBoundary fallback={fallback}>
      {children}
    </ErrorBoundary>
  )
} 