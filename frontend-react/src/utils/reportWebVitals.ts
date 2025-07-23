/**
 * 性能监控工具函数
 * @param onPerfEntry 性能指标回调函数
 */
export const reportWebVitals = (onPerfEntry?: (metric: any) => void) => {
  if (onPerfEntry && typeof onPerfEntry === 'function') {
    // 简化版本的性能监控
    // 在实际项目中，可以安装 web-vitals 包来获取更详细的性能指标
    try {
      // 页面加载时间
      if (performance && performance.timing) {
        const loadTime = performance.timing.loadEventEnd - performance.timing.navigationStart
        onPerfEntry({ name: 'page-load-time', value: loadTime })
      }
      
      // 性能观察者（如果支持）
      if ('PerformanceObserver' in window) {
        const observer = new PerformanceObserver((list) => {
          list.getEntries().forEach((entry) => {
            onPerfEntry({ name: entry.name, value: entry.duration || 0 })
          })
        })
        observer.observe({ entryTypes: ['measure', 'navigation'] })
      }
    } catch (error) {
      console.warn('性能监控初始化失败:', error)
    }
  }
} 