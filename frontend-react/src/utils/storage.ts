/**
 * 本地存储工具类
 */
export const storage = {
  /**
   * 设置本地存储
   * @param key 键名
   * @param value 值
   * @param ttl 过期时间（毫秒）
   */
  set: (key: string, value: any, ttl?: number): void => {
    const item = {
      value,
      timestamp: Date.now(),
      ttl: ttl || 0
    }
    localStorage.setItem(key, JSON.stringify(item))
  },

  /**
   * 获取本地存储
   * @param key 键名
   * @returns 存储的值
   */
  get: (key: string): any => {
    const item = localStorage.getItem(key)
    if (!item) return null

    try {
      const parsed = JSON.parse(item)
      
      // 检查是否过期
      if (parsed.ttl && parsed.ttl > 0) {
        const now = Date.now()
        if (now - parsed.timestamp > parsed.ttl) {
          localStorage.removeItem(key)
          return null
        }
      }
      
      return parsed.value
    } catch (error) {
      console.error('Failed to parse storage item:', error)
      return null
    }
  },

  /**
   * 删除本地存储
   * @param key 键名
   */
  remove: (key: string): void => {
    localStorage.removeItem(key)
  },

  /**
   * 清空所有本地存储
   */
  clear: (): void => {
    localStorage.clear()
  },

  /**
   * 检查是否存在某个键
   * @param key 键名
   * @returns 是否存在
   */
  has: (key: string): boolean => {
    return localStorage.getItem(key) !== null
  }
} 