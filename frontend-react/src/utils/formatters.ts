/**
 * 格式化货币显示
 * @param amount 金额
 * @param currency 货币类型
 * @param locale 地区
 * @returns 格式化后的货币字符串
 */
export const formatCurrency = (
  amount: number,
  currency: string = 'CNY',
  locale: string = 'zh-CN'
): string => {
  try {
    return new Intl.NumberFormat(locale, {
      style: 'currency',
      currency,
    }).format(amount)
  } catch (error) {
    // 降级处理
    if (currency === 'CNY') {
      return `¥${amount.toFixed(2)}`
    }
    return `${amount.toFixed(2)} ${currency}`
  }
}

/**
 * 格式化数字显示（带千分位分隔符）
 * @param num 数字
 * @param decimals 小数位数
 * @returns 格式化后的数字字符串
 */
export const formatNumber = (num: number, decimals: number = 2): string => {
  return new Intl.NumberFormat('zh-CN', {
    minimumFractionDigits: decimals,
    maximumFractionDigits: decimals,
  }).format(num)
}

/**
 * 格式化百分比
 * @param value 数值（0-1之间或0-100之间）
 * @param isDecimal 是否为小数形式（true: 0-1, false: 0-100）
 * @returns 格式化后的百分比字符串
 */
export const formatPercentage = (value: number, isDecimal: boolean = true): string => {
  const percentage = isDecimal ? value * 100 : value
  return `${percentage.toFixed(1)}%`
}

/**
 * 格式化文件大小
 * @param bytes 字节数
 * @param decimals 小数位数
 * @returns 格式化后的文件大小字符串
 */
export const formatFileSize = (bytes: number, decimals: number = 2): string => {
  if (bytes === 0) return '0 B'

  const k = 1024
  const dm = decimals < 0 ? 0 : decimals
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']

  const i = Math.floor(Math.log(bytes) / Math.log(k))

  return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i]
}

/**
 * 格式化钱包地址（显示前6位和后4位）
 * @param address 钱包地址
 * @param prefixLength 前缀长度
 * @param suffixLength 后缀长度
 * @returns 格式化后的地址
 */
export const formatWalletAddress = (
  address: string,
  prefixLength: number = 6,
  suffixLength: number = 4
): string => {
  if (!address || address.length <= prefixLength + suffixLength) {
    return address
  }
  
  return `${address.slice(0, prefixLength)}...${address.slice(-suffixLength)}`
}

/**
 * 格式化电话号码
 * @param phone 电话号码
 * @returns 格式化后的电话号码
 */
export const formatPhoneNumber = (phone: string): string => {
  const cleaned = phone.replace(/\D/g, '')
  
  if (cleaned.length === 11 && cleaned.startsWith('1')) {
    // 中国手机号格式：138-1234-5678
    return `${cleaned.slice(0, 3)}-${cleaned.slice(3, 7)}-${cleaned.slice(7)}`
  }
  
  return phone
}

/**
 * 格式化评分显示
 * @param rating 评分（0-5或0-100）
 * @param maxRating 最大评分
 * @returns 格式化后的评分字符串
 */
export const formatRating = (rating: number, maxRating: number = 5): string => {
  if (maxRating === 5) {
    return rating.toFixed(1)
  } else if (maxRating === 100) {
    return rating.toFixed(0)
  }
  return rating.toString()
}

/**
 * 格式化倍增器显示
 * @param multiplier 倍增器数值
 * @returns 格式化后的倍增器字符串
 */
export const formatMultiplier = (multiplier: number): string => {
  return `${multiplier.toFixed(1)}x`
}

/**
 * 格式化数量显示（大数字使用K、M、B简化）
 * @param num 数字
 * @returns 格式化后的数量字符串
 */
export const formatCount = (num: number): string => {
  if (num >= 1000000000) {
    return (num / 1000000000).toFixed(1) + 'B'
  } else if (num >= 1000000) {
    return (num / 1000000).toFixed(1) + 'M'
  } else if (num >= 1000) {
    return (num / 1000).toFixed(1) + 'K'
  }
  return num.toString()
}

/**
 * 格式化时长显示
 * @param seconds 秒数
 * @returns 格式化后的时长字符串
 */
export const formatDuration = (seconds: number): string => {
  const hours = Math.floor(seconds / 3600)
  const minutes = Math.floor((seconds % 3600) / 60)
  const remainingSeconds = seconds % 60

  if (hours > 0) {
    return `${hours}:${minutes.toString().padStart(2, '0')}:${remainingSeconds.toString().padStart(2, '0')}`
  } else {
    return `${minutes}:${remainingSeconds.toString().padStart(2, '0')}`
  }
}

/**
 * 格式化距离显示
 * @param meters 米数
 * @returns 格式化后的距离字符串
 */
export const formatDistance = (meters: number): string => {
  if (meters >= 1000) {
    return `${(meters / 1000).toFixed(1)} km`
  }
  return `${meters.toFixed(0)} m`
}

/**
 * 格式化状态显示
 * @param status 状态值
 * @param statusMap 状态映射表
 * @returns 格式化后的状态字符串
 */
export const formatStatus = (status: string, statusMap: Record<string, string>): string => {
  return statusMap[status] || status
}

/**
 * 格式化搜索高亮
 * @param text 原文本
 * @param searchTerm 搜索词
 * @returns 包含高亮标记的HTML字符串
 */
export const formatSearchHighlight = (text: string, searchTerm: string): string => {
  if (!searchTerm) return text
  
  const regex = new RegExp(`(${searchTerm})`, 'gi')
  return text.replace(regex, '<mark>$1</mark>')
}

/**
 * 格式化URL参数
 * @param params 参数对象
 * @returns URL参数字符串
 */
export const formatUrlParams = (params: Record<string, any>): string => {
  const searchParams = new URLSearchParams()
  
  Object.entries(params).forEach(([key, value]) => {
    if (value !== null && value !== undefined && value !== '') {
      searchParams.append(key, String(value))
    }
  })
  
  return searchParams.toString()
} 