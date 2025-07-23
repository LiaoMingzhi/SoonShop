/**
 * 简单加密数据（Base64）
 * @param data 要加密的数据
 * @returns 加密后的字符串
 */
export const encryptData = (data: string): string => {
  try {
    return btoa(encodeURIComponent(data))
  } catch (error) {
    console.error('Failed to encrypt data:', error)
    return data
  }
}

/**
 * 简单解密数据（Base64）
 * @param encryptedData 加密的数据
 * @returns 解密后的字符串
 */
export const decryptData = (encryptedData: string): string => {
  try {
    return decodeURIComponent(atob(encryptedData))
  } catch (error) {
    console.error('Failed to decrypt data:', error)
    return encryptedData
  }
}

/**
 * 生成哈希值
 * @param data 要哈希的数据
 * @returns 哈希值字符串
 */
export const hashData = async (data: string): Promise<string> => {
  const encoder = new TextEncoder()
  const dataBuffer = encoder.encode(data)
  const hashBuffer = await crypto.subtle.digest('SHA-256', dataBuffer)
  const hashArray = Array.from(new Uint8Array(hashBuffer))
  return hashArray.map(b => b.toString(16).padStart(2, '0')).join('')
} 