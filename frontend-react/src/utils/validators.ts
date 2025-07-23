import { VALIDATION_RULES } from '@constants/index'

/**
 * 验证邮箱格式
 * @param email 邮箱地址
 * @returns 是否为有效邮箱
 */
export const validateEmail = (email: string): boolean => {
  return VALIDATION_RULES.EMAIL_REGEX.test(email)
}

/**
 * 验证手机号格式
 * @param phone 手机号
 * @returns 是否为有效手机号
 */
export const validatePhone = (phone: string): boolean => {
  return VALIDATION_RULES.PHONE_REGEX.test(phone)
}

/**
 * 验证钱包地址格式
 * @param address 钱包地址
 * @returns 是否为有效钱包地址
 */
export const validateWalletAddress = (address: string): boolean => {
  return VALIDATION_RULES.WALLET_ADDRESS_REGEX.test(address)
}

/**
 * 验证密码强度
 * @param password 密码
 * @returns 验证结果对象
 */
export const validatePassword = (password: string): {
  isValid: boolean
  errors: string[]
  strength: 'weak' | 'medium' | 'strong'
} => {
  const errors: string[] = []
  
  if (password.length < VALIDATION_RULES.PASSWORD_MIN_LENGTH) {
    errors.push(`密码长度至少${VALIDATION_RULES.PASSWORD_MIN_LENGTH}位`)
  }
  
  if (password.length > VALIDATION_RULES.PASSWORD_MAX_LENGTH) {
    errors.push(`密码长度最多${VALIDATION_RULES.PASSWORD_MAX_LENGTH}位`)
  }
  
  if (!/[a-z]/.test(password)) {
    errors.push('密码必须包含小写字母')
  }
  
  if (!/[A-Z]/.test(password)) {
    errors.push('密码必须包含大写字母')
  }
  
  if (!/\d/.test(password)) {
    errors.push('密码必须包含数字')
  }
  
  if (!/[!@#$%^&*(),.?":{}|<>]/.test(password)) {
    errors.push('密码必须包含特殊字符')
  }
  
  let strength: 'weak' | 'medium' | 'strong' = 'weak'
  
  if (errors.length === 0) {
    strength = 'strong'
  } else if (errors.length <= 2) {
    strength = 'medium'
  }
  
  return {
    isValid: errors.length === 0,
    errors,
    strength
  }
}

/**
 * 验证用户名格式
 * @param username 用户名
 * @returns 验证结果对象
 */
export const validateUsername = (username: string): {
  isValid: boolean
  errors: string[]
} => {
  const errors: string[] = []
  
  if (username.length < VALIDATION_RULES.USERNAME_MIN_LENGTH) {
    errors.push(`用户名长度至少${VALIDATION_RULES.USERNAME_MIN_LENGTH}位`)
  }
  
  if (username.length > VALIDATION_RULES.USERNAME_MAX_LENGTH) {
    errors.push(`用户名长度最多${VALIDATION_RULES.USERNAME_MAX_LENGTH}位`)
  }
  
  if (!/^[a-zA-Z0-9_]+$/.test(username)) {
    errors.push('用户名只能包含字母、数字和下划线')
  }
  
  return {
    isValid: errors.length === 0,
    errors
  }
}

/**
 * 验证URL格式
 * @param url URL地址
 * @returns 是否为有效URL
 */
export const validateUrl = (url: string): boolean => {
  try {
    new URL(url)
    return true
  } catch {
    return false
  }
}

/**
 * 验证身份证号码格式
 * @param idCard 身份证号码
 * @returns 是否为有效身份证号码
 */
export const validateIdCard = (idCard: string): boolean => {
  const regex = /^[1-9]\d{5}(18|19|20)\d{2}(0[1-9]|1[0-2])(0[1-9]|[12]\d|3[01])\d{3}[\dXx]$/
  return regex.test(idCard)
}

/**
 * 验证银行卡号格式
 * @param cardNumber 银行卡号
 * @returns 是否为有效银行卡号
 */
export const validateBankCard = (cardNumber: string): boolean => {
  const regex = /^[1-9]\d{12,19}$/
  return regex.test(cardNumber)
}

/**
 * 验证价格格式
 * @param price 价格
 * @returns 是否为有效价格
 */
export const validatePrice = (price: string | number): boolean => {
  const priceNum = typeof price === 'string' ? parseFloat(price) : price
  return !isNaN(priceNum) && priceNum >= 0 && priceNum <= 999999999
}

/**
 * 验证数量格式
 * @param quantity 数量
 * @returns 是否为有效数量
 */
export const validateQuantity = (quantity: string | number): boolean => {
  const quantityNum = typeof quantity === 'string' ? parseInt(quantity) : quantity
  return !isNaN(quantityNum) && quantityNum > 0 && quantityNum <= 999999
}

/**
 * 验证日期格式
 * @param date 日期字符串
 * @returns 是否为有效日期
 */
export const validateDate = (date: string): boolean => {
  const dateObj = new Date(date)
  return !isNaN(dateObj.getTime())
}

/**
 * 验证表单字段
 * @param fields 字段对象
 * @param rules 验证规则
 * @returns 验证结果
 */
export const validateForm = (
  fields: Record<string, any>,
  rules: Record<string, any>
): {
  isValid: boolean
  errors: Record<string, string[]>
} => {
  const errors: Record<string, string[]> = {}
  
  for (const [field, value] of Object.entries(fields)) {
    const fieldRules = rules[field]
    if (!fieldRules) continue
    
    const fieldErrors: string[] = []
    
    // 必填验证
    if (fieldRules.required && (!value || value.toString().trim() === '')) {
      fieldErrors.push(`${fieldRules.label || field}是必填项`)
    }
    
    // 类型验证
    if (value && fieldRules.type) {
      switch (fieldRules.type) {
        case 'email':
          if (!validateEmail(value)) {
            fieldErrors.push('邮箱格式不正确')
          }
          break
        case 'phone':
          if (!validatePhone(value)) {
            fieldErrors.push('手机号格式不正确')
          }
          break
        case 'url':
          if (!validateUrl(value)) {
            fieldErrors.push('URL格式不正确')
          }
          break
        case 'number':
          if (isNaN(Number(value))) {
            fieldErrors.push('必须是数字')
          }
          break
      }
    }
    
    // 长度验证
    if (value && fieldRules.minLength && value.length < fieldRules.minLength) {
      fieldErrors.push(`长度至少${fieldRules.minLength}位`)
    }
    
    if (value && fieldRules.maxLength && value.length > fieldRules.maxLength) {
      fieldErrors.push(`长度最多${fieldRules.maxLength}位`)
    }
    
    // 最小值验证
    if (value && fieldRules.min && Number(value) < fieldRules.min) {
      fieldErrors.push(`最小值为${fieldRules.min}`)
    }
    
    // 最大值验证
    if (value && fieldRules.max && Number(value) > fieldRules.max) {
      fieldErrors.push(`最大值为${fieldRules.max}`)
    }
    
    // 自定义验证
    if (value && fieldRules.validator) {
      const customResult = fieldRules.validator(value)
      if (customResult !== true) {
        fieldErrors.push(customResult)
      }
    }
    
    if (fieldErrors.length > 0) {
      errors[field] = fieldErrors
    }
  }
  
  return {
    isValid: Object.keys(errors).length === 0,
    errors
  }
} 