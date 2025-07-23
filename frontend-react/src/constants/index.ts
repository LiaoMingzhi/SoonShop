// API端点常量
export const API_ENDPOINTS = {
  AUTH: {
    LOGIN: '/api/v1/auth/wallet-login',
    REFRESH: '/api/v1/auth/refresh',
    LOGOUT: '/api/v1/auth/logout',
    PROFILE: '/api/v1/auth/profile',
  },
  VOUCHERS: {
    LIST: '/api/v1/vouchers',
    CREATE: '/api/v1/vouchers',
    DETAIL: (id: string) => `/api/v1/vouchers/${id}`,
    CLAIM: (id: string) => `/api/v1/vouchers/${id}/claim`,
    CONSUME: (id: string) => `/api/v1/vouchers/${id}/consume`,
  },
  CONSUMPTION: {
    LIST: '/api/v1/consumptions',
    DETAIL: (id: string) => `/api/v1/consumptions/${id}`,
    CONFIRM: (id: string) => `/api/v1/consumptions/${id}/confirm`,
    RATE: (id: string) => `/api/v1/consumptions/${id}/rate`,
  },
  ENTERPRISES: {
    LIST: '/api/v1/enterprises',
    DETAIL: (id: string) => `/api/v1/enterprises/${id}`,
    EVALUATIONS: (id: string) => `/api/v1/enterprises/${id}/evaluations`,
    CREATE_EVALUATION: (id: string) => `/api/v1/enterprises/${id}/evaluations`,
  },
  B2C: {
    ORDERS: '/api/v1/b2c/orders',
    ORDER_DETAIL: (id: string) => `/api/v1/b2c/orders/${id}`,
    PROCESS_ORDER: (id: string) => `/api/v1/b2c/orders/${id}/process`,
  },
  B2B: {
    VOUCHERS: '/api/v1/b2b/vouchers',
    PURCHASE_ORDERS: '/api/v1/b2b/purchase-orders',
    SUPPLY_CHAIN_CONSUME: '/api/v1/b2b/supply-chain-consume',
  },
  SERVICES: {
    RESTAURANT: '/api/v1/restaurant',
    HEALTHCARE: '/api/v1/healthcare',
    HOUSING: '/api/v1/housing',
  },
  ANALYTICS: {
    DASHBOARD: '/api/v1/analytics/dashboard',
    REPORTS: '/api/v1/analytics/reports',
  },
  ADMIN: {
    USERS: '/api/v1/admin/users',
    SYSTEM_CONFIG: '/api/v1/admin/system/config',
  },
} as const

// 应用配置常量
export const APP_CONFIG = {
  NAME: 'SoonShop',
  TITLE: 'SoonShop - 共产主义商业平台',
  DESCRIPTION: '基于共产主义经济原理的现代商业平台，实现按需生产、按需消费，促进社会共同富裕',
  VERSION: '1.0.0',
  AUTHOR: 'SoonShop Team',
  CONTACT: 'contact@soonshop.com',
  WEBSITE: 'https://soonshop.com',
  GITHUB: 'https://github.com/soonshop/soonshop',
} as const

// 本地存储键名
export const STORAGE_KEYS = {
  ACCESS_TOKEN: 'soonshop_access_token',
  REFRESH_TOKEN: 'soonshop_refresh_token',
  USER_PROFILE: 'soonshop_user_profile',
  WALLET_ADDRESS: 'soonshop_wallet_address',
  THEME: 'soonshop_theme',
  LANGUAGE: 'soonshop_language',
  SETTINGS: 'soonshop_settings',
  RECENT_SEARCHES: 'soonshop_recent_searches',
  FAVORITE_VOUCHERS: 'soonshop_favorite_vouchers',
  CART: 'soonshop_cart',
} as const

// 主题色彩常量
export const THEME_COLORS = {
  PRIMARY: {
    50: '#fef2f2',
    100: '#fee2e2',
    200: '#fecaca',
    300: '#fca5a5',
    400: '#f87171',
    500: '#ef4444', // 主要红色
    600: '#dc2626',
    700: '#b91c1c',
    800: '#991b1b',
    900: '#7f1d1d',
  },
  SECONDARY: {
    50: '#fffbeb',
    100: '#fef3c7',
    200: '#fde68a',
    300: '#fcd34d',
    400: '#fbbf24', // 金色
    500: '#f59e0b',
    600: '#d97706',
    700: '#b45309',
    800: '#92400e',
    900: '#78350f',
  },
  SUCCESS: {
    50: '#f0fdf4',
    100: '#dcfce7',
    200: '#bbf7d0',
    300: '#86efac',
    400: '#4ade80',
    500: '#22c55e', // 绿色
    600: '#16a34a',
    700: '#15803d',
    800: '#166534',
    900: '#14532d',
  },
  GRADIENTS: {
    REVOLUTIONARY: 'linear-gradient(135deg, #ef4444 0%, #dc2626 100%)',
    PROSPERITY: 'linear-gradient(135deg, #f59e0b 0%, #d97706 100%)',
    UNITY: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
    PINK_ORANGE: 'linear-gradient(135deg, #fa709a 0%, #fee140 100%)',
    BLUE_PURPLE: 'linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)',
    GREEN_TEAL: 'linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)',
  },
} as const

// 用户类型常量
export const USER_TYPES = {
  CONSUMER: 'consumer',
  PRODUCER: 'producer',
  EVALUATOR: 'evaluator',
  ADMIN: 'admin',
} as const

export const USER_TYPE_LABELS = {
  [USER_TYPES.CONSUMER]: '消费者',
  [USER_TYPES.PRODUCER]: '生产者',
  [USER_TYPES.EVALUATOR]: '评估员',
  [USER_TYPES.ADMIN]: '管理员',
} as const

// 提货券状态常量
export const VOUCHER_STATUS = {
  DRAFT: 'draft',
  ACTIVE: 'active',
  PAUSED: 'paused',
  EXPIRED: 'expired',
  CANCELLED: 'cancelled',
} as const

export const VOUCHER_STATUS_LABELS = {
  [VOUCHER_STATUS.DRAFT]: '草稿',
  [VOUCHER_STATUS.ACTIVE]: '活跃',
  [VOUCHER_STATUS.PAUSED]: '暂停',
  [VOUCHER_STATUS.EXPIRED]: '过期',
  [VOUCHER_STATUS.CANCELLED]: '取消',
} as const

// 消费状态常量
export const CONSUMPTION_STATUS = {
  PENDING: 'pending',
  CONFIRMED: 'confirmed',
  RATED: 'rated',
  DISPUTED: 'disputed',
} as const

export const CONSUMPTION_STATUS_LABELS = {
  [CONSUMPTION_STATUS.PENDING]: '待确认',
  [CONSUMPTION_STATUS.CONFIRMED]: '已确认',
  [CONSUMPTION_STATUS.RATED]: '已评价',
  [CONSUMPTION_STATUS.DISPUTED]: '争议中',
} as const

// 商品分类常量
export const PRODUCT_CATEGORIES = {
  FOOD: 'food',
  DAILY_NECESSITIES: 'daily_necessities',
  CLOTHING: 'clothing',
  ELECTRONICS: 'electronics',
  HOME_APPLIANCES: 'home_appliances',
  BOOKS: 'books',
  HEALTH: 'health',
  BEAUTY: 'beauty',
  SPORTS: 'sports',
  AUTOMOTIVE: 'automotive',
  SERVICES: 'services',
} as const

export const PRODUCT_CATEGORY_LABELS = {
  [PRODUCT_CATEGORIES.FOOD]: '食品',
  [PRODUCT_CATEGORIES.DAILY_NECESSITIES]: '日用品',
  [PRODUCT_CATEGORIES.CLOTHING]: '服装',
  [PRODUCT_CATEGORIES.ELECTRONICS]: '数码',
  [PRODUCT_CATEGORIES.HOME_APPLIANCES]: '家电',
  [PRODUCT_CATEGORIES.BOOKS]: '图书',
  [PRODUCT_CATEGORIES.HEALTH]: '健康',
  [PRODUCT_CATEGORIES.BEAUTY]: '美妆',
  [PRODUCT_CATEGORIES.SPORTS]: '运动',
  [PRODUCT_CATEGORIES.AUTOMOTIVE]: '汽车',
  [PRODUCT_CATEGORIES.SERVICES]: '服务',
} as const

// 分页默认值
export const PAGINATION_DEFAULTS = {
  PAGE: 1,
  LIMIT: 20,
  MAX_LIMIT: 100,
} as const

// 验证规则常量
export const VALIDATION_RULES = {
  EMAIL_REGEX: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
  PHONE_REGEX: /^1[3-9]\d{9}$/,
  WALLET_ADDRESS_REGEX: /^[1-9A-HJ-NP-Za-km-z]{32,44}$/,
  PASSWORD_MIN_LENGTH: 8,
  PASSWORD_MAX_LENGTH: 32,
  USERNAME_MIN_LENGTH: 3,
  USERNAME_MAX_LENGTH: 20,
  DESCRIPTION_MAX_LENGTH: 500,
  REVIEW_MAX_LENGTH: 1000,
} as const

// 错误消息常量
export const ERROR_MESSAGES = {
  NETWORK_ERROR: '网络连接错误，请检查网络后重试',
  UNAUTHORIZED: '未授权访问，请先登录',
  FORBIDDEN: '权限不足，无法访问该资源',
  NOT_FOUND: '请求的资源不存在',
  SERVER_ERROR: '服务器内部错误，请稍后重试',
  VALIDATION_ERROR: '输入数据格式错误',
  WALLET_NOT_CONNECTED: '钱包未连接，请先连接钱包',
  INSUFFICIENT_BALANCE: '余额不足',
  VOUCHER_EXPIRED: '提货券已过期',
  VOUCHER_INSUFFICIENT: '提货券库存不足',
  RATE_LIMIT_EXCEEDED: '请求过于频繁，请稍后重试',
} as const

// 成功消息常量
export const SUCCESS_MESSAGES = {
  LOGIN_SUCCESS: '登录成功',
  LOGOUT_SUCCESS: '退出成功',
  VOUCHER_CLAIMED: '提货券获取成功',
  VOUCHER_CONSUMED: '提货券消费成功',
  EVALUATION_SUBMITTED: '评估提交成功',
  PROFILE_UPDATED: '个人资料更新成功',
  SETTINGS_SAVED: '设置保存成功',
  ORDER_CREATED: '订单创建成功',
  PAYMENT_SUCCESS: '支付成功',
  REVIEW_SUBMITTED: '评价提交成功',
} as const

// 路由路径常量
export const ROUTES = {
  HOME: '/',
  LOGIN: '/login',
  REGISTER: '/register',
  PROFILE: '/profile',
  VOUCHERS: '/vouchers',
  VOUCHER_DETAIL: (id: string) => `/vouchers/${id}`,
  B2C_SHOPPING: '/b2c',
  CONSUMPTION: '/consumption',
  EVALUATION: '/evaluation',
  B2B_MARKETPLACE: '/b2b',
  RESTAURANT: '/restaurant',
  HEALTHCARE: '/healthcare',
  HOUSING: '/housing',
  ANALYTICS: '/analytics',
  ADMIN: '/admin',
} as const

// 共产主义理念标语
export const COMMUNIST_SLOGANS = [
  '人民至上，共同富裕',
  '按需生产，按需分配',
  '劳动创造价值，共享发展成果',
  '团结就是力量，合作共赢',
  '为人民服务，为社会主义建设贡献力量',
  '共产主义理想，指引我们前进',
  '集体利益高于个人利益',
  '共同劳动，共同发展',
  '消除剥削，实现平等',
  '建设美好社会，人人有责',
] as const

// 动画配置
export const ANIMATIONS = {
  FADE_IN: {
    initial: { opacity: 0 },
    animate: { opacity: 1 },
    exit: { opacity: 0 },
    transition: { duration: 0.3 },
  },
  SLIDE_UP: {
    initial: { opacity: 0, y: 20 },
    animate: { opacity: 1, y: 0 },
    exit: { opacity: 0, y: -20 },
    transition: { duration: 0.3 },
  },
  SCALE_UP: {
    initial: { opacity: 0, scale: 0.95 },
    animate: { opacity: 1, scale: 1 },
    exit: { opacity: 0, scale: 0.95 },
    transition: { duration: 0.2 },
  },
} as const

// 环境配置
export const ENVIRONMENT = {
  DEVELOPMENT: 'development',
  STAGING: 'staging',
  PRODUCTION: 'production',
} as const

// 默认配置
export const DEFAULT_CONFIG = {
  API_TIMEOUT: 30000,
  RETRY_COUNT: 3,
  RETRY_DELAY: 1000,
  DEBOUNCE_DELAY: 300,
  THROTTLE_DELAY: 1000,
  CACHE_TTL: 5 * 60 * 1000, // 5分钟
  MAX_FILE_SIZE: 10 * 1024 * 1024, // 10MB
  SUPPORTED_IMAGE_FORMATS: ['jpg', 'jpeg', 'png', 'gif', 'webp'],
  SUPPORTED_VIDEO_FORMATS: ['mp4', 'webm', 'ogg'],
} as const 