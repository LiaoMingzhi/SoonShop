/**
 * 生成唯一ID
 * @param prefix 前缀
 * @returns 唯一ID字符串
 */
export const generateId = (prefix: string = 'id'): string => {
  const timestamp = Date.now().toString(36)
  const randomStr = Math.random().toString(36).substr(2, 9)
  return `${prefix}_${timestamp}_${randomStr}`
}

/**
 * 延迟执行
 * @param ms 延迟毫秒数
 * @returns Promise
 */
export const sleep = (ms: number): Promise<void> => {
  return new Promise(resolve => setTimeout(resolve, ms))
}

/**
 * 复制文本到剪贴板
 * @param text 要复制的文本
 * @returns Promise<boolean> 是否复制成功
 */
export const copyToClipboard = async (text: string): Promise<boolean> => {
  try {
    if (navigator.clipboard && window.isSecureContext) {
      await navigator.clipboard.writeText(text)
      return true
    } else {
      // 降级方案
      const textArea = document.createElement('textarea')
      textArea.value = text
      textArea.style.position = 'fixed'
      textArea.style.left = '-999999px'
      textArea.style.top = '-999999px'
      document.body.appendChild(textArea)
      textArea.focus()
      textArea.select()
      
      const success = document.execCommand('copy')
      document.body.removeChild(textArea)
      return success
    }
  } catch (error) {
    console.error('Failed to copy text:', error)
    return false
  }
}

/**
 * 获取设备类型
 * @returns 设备类型
 */
export const getDeviceType = (): 'mobile' | 'tablet' | 'desktop' => {
  const userAgent = navigator.userAgent
  
  if (/Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(userAgent)) {
    if (/iPad/i.test(userAgent) || (userAgent.includes('Android') && !userAgent.includes('Mobile'))) {
      return 'tablet'
    }
    return 'mobile'
  }
  
  return 'desktop'
}

/**
 * 检测是否为iOS设备
 * @returns 是否为iOS设备
 */
export const isIOS = (): boolean => {
  return /iPad|iPhone|iPod/.test(navigator.userAgent)
}

/**
 * 检测是否为移动设备
 * @returns 是否为移动设备
 */
export const isMobile = (): boolean => {
  return getDeviceType() === 'mobile'
}

/**
 * 检测是否支持触摸
 * @returns 是否支持触摸
 */
export const isTouchDevice = (): boolean => {
  return 'ontouchstart' in window || navigator.maxTouchPoints > 0
}

/**
 * 获取浏览器信息
 * @returns 浏览器信息对象
 */
export const getBrowserInfo = () => {
  const userAgent = navigator.userAgent
  
  let browserName = 'Unknown'
  let version = 'Unknown'
  
  if (userAgent.indexOf('Chrome') > -1) {
    browserName = 'Chrome'
    version = userAgent.match(/Chrome\/(\d+)/)?.[1] || 'Unknown'
  } else if (userAgent.indexOf('Firefox') > -1) {
    browserName = 'Firefox'
    version = userAgent.match(/Firefox\/(\d+)/)?.[1] || 'Unknown'
  } else if (userAgent.indexOf('Safari') > -1) {
    browserName = 'Safari'
    version = userAgent.match(/Safari\/(\d+)/)?.[1] || 'Unknown'
  } else if (userAgent.indexOf('Edge') > -1) {
    browserName = 'Edge'
    version = userAgent.match(/Edge\/(\d+)/)?.[1] || 'Unknown'
  }
  
  return {
    name: browserName,
    version,
    userAgent,
  }
}

/**
 * 滚动到页面顶部
 * @param smooth 是否平滑滚动
 */
export const scrollToTop = (smooth: boolean = true): void => {
  if (smooth) {
    window.scrollTo({ top: 0, behavior: 'smooth' })
  } else {
    window.scrollTo(0, 0)
  }
}

/**
 * 滚动到指定元素
 * @param elementId 元素ID
 * @param offset 偏移量
 */
export const scrollToElement = (elementId: string, offset: number = 0): void => {
  const element = document.getElementById(elementId)
  if (element) {
    const elementTop = element.offsetTop - offset
    window.scrollTo({ top: elementTop, behavior: 'smooth' })
  }
}

/**
 * 检查元素是否在视口中
 * @param element DOM元素
 * @returns 是否在视口中
 */
export const isElementInViewport = (element: Element): boolean => {
  const rect = element.getBoundingClientRect()
  return (
    rect.top >= 0 &&
    rect.left >= 0 &&
    rect.bottom <= (window.innerHeight || document.documentElement.clientHeight) &&
    rect.right <= (window.innerWidth || document.documentElement.clientWidth)
  )
}

/**
 * 获取随机颜色
 * @returns HEX颜色字符串
 */
export const getRandomColor = (): string => {
  const colors = [
    '#ef4444', '#f59e0b', '#22c55e', '#3b82f6', '#8b5cf6',
    '#ec4899', '#f97316', '#84cc16', '#06b6d4', '#6366f1',
  ]
  return colors[Math.floor(Math.random() * colors.length)]
}

/**
 * 获取随机整数
 * @param min 最小值
 * @param max 最大值
 * @returns 随机整数
 */
export const getRandomInt = (min: number, max: number): number => {
  return Math.floor(Math.random() * (max - min + 1)) + min
}

/**
 * 数组洗牌
 * @param array 原数组
 * @returns 洗牌后的新数组
 */
export const shuffleArray = <T>(array: T[]): T[] => {
  const newArray = [...array]
  for (let i = newArray.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1))
    ;[newArray[i], newArray[j]] = [newArray[j], newArray[i]]
  }
  return newArray
}

/**
 * 数组去重
 * @param array 原数组
 * @param key 对象数组的去重键
 * @returns 去重后的数组
 */
export const uniqueArray = <T>(array: T[], key?: keyof T): T[] => {
  if (key) {
    const seen = new Set()
    return array.filter(item => {
      const keyValue = item[key]
      if (seen.has(keyValue)) {
        return false
      }
      seen.add(keyValue)
      return true
    })
  }
  return [...new Set(array)]
}

/**
 * 深拷贝对象
 * @param obj 要拷贝的对象
 * @returns 深拷贝后的对象
 */
export const deepClone = <T>(obj: T): T => {
  if (obj === null || typeof obj !== 'object') return obj
  if (obj instanceof Date) return new Date(obj.getTime()) as unknown as T
  if (obj instanceof Array) return obj.map(item => deepClone(item)) as unknown as T
  if (typeof obj === 'object') {
    const clonedObj = {} as T
    for (const key in obj) {
      if (obj.hasOwnProperty(key)) {
        clonedObj[key] = deepClone(obj[key])
      }
    }
    return clonedObj
  }
  return obj
}

/**
 * 检查对象是否为空
 * @param obj 要检查的对象
 * @returns 是否为空
 */
export const isEmpty = (obj: any): boolean => {
  if (obj == null) return true
  if (Array.isArray(obj) || typeof obj === 'string') return obj.length === 0
  if (typeof obj === 'object') return Object.keys(obj).length === 0
  return false
}

/**
 * 首字母大写
 * @param str 字符串
 * @returns 首字母大写的字符串
 */
export const capitalize = (str: string): string => {
  return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase()
}

/**
 * 驼峰转下划线
 * @param str 驼峰命名字符串
 * @returns 下划线命名字符串
 */
export const camelToSnake = (str: string): string => {
  return str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`)
}

/**
 * 下划线转驼峰
 * @param str 下划线命名字符串
 * @returns 驼峰命名字符串
 */
export const snakeToCamel = (str: string): string => {
  return str.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase())
}

/**
 * 获取文件扩展名
 * @param filename 文件名
 * @returns 文件扩展名
 */
export const getFileExtension = (filename: string): string => {
  return filename.slice((filename.lastIndexOf('.') - 1 >>> 0) + 2)
}

/**
 * 检查URL是否有效
 * @param url URL字符串
 * @returns 是否为有效URL
 */
export const isValidUrl = (url: string): boolean => {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}

/**
 * 获取URL参数
 * @param name 参数名
 * @param url URL字符串（可选，默认当前页面URL）
 * @returns 参数值
 */
export const getUrlParam = (name: string, url?: string): string | null => {
  const searchParams = new URLSearchParams(url ? new URL(url).search : window.location.search)
  return searchParams.get(name)
}

/**
 * 下载数据为文件
 * @param data 数据
 * @param filename 文件名
 * @param type MIME类型
 */
export const downloadData = (data: string, filename: string, type: string = 'text/plain'): void => {
  const blob = new Blob([data], { type })
  const url = window.URL.createObjectURL(blob)
  const link = document.createElement('a')
  link.href = url
  link.download = filename
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
  window.URL.revokeObjectURL(url)
} 