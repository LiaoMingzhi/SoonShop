/**
 * 下载文件
 * @param url 文件URL
 * @param filename 文件名
 */
export const downloadFile = (url: string, filename: string): void => {
  const link = document.createElement('a')
  link.href = url
  link.download = filename
  document.body.appendChild(link)
  link.click()
  document.body.removeChild(link)
}

/**
 * 上传文件
 * @param file 文件对象
 * @param uploadUrl 上传URL
 * @returns Promise<string> 上传后的文件URL
 */
export const uploadFile = async (file: File, uploadUrl: string): Promise<string> => {
  const formData = new FormData()
  formData.append('file', file)
  
  const response = await fetch(uploadUrl, {
    method: 'POST',
    body: formData,
  })
  
  if (!response.ok) {
    throw new Error('Upload failed')
  }
  
  const result = await response.json()
  return result.url || result.data?.url || ''
} 