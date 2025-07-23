import React from 'react'

// 提货券分类枚举
export enum VoucherCategory {
  FOOD = 'food',
  CLOTHING = 'clothing',
  ELECTRONICS = 'electronics',
  HOME = 'home',
  HEALTH = 'health',
  EDUCATION = 'education',
  ENTERTAINMENT = 'entertainment',
  TRANSPORT = 'transport',
  OTHER = 'other'
}

// 用户相关类型
export interface User {
  id: string
  walletAddress: string
  userType: 'consumer' | 'producer' | 'evaluator' | 'admin'
  name?: string
  profile: {
    displayName: string
    avatarUrl?: string
    bio?: string
    location?: {
      country: string
      province: string
      city: string
    }
    verificationStatus: 'unverified' | 'pending' | 'verified'
  }
  settings: {
    notificationPreferences: {
      email: boolean
      sms: boolean
      push: boolean
    }
    privacySettings: {
      profileVisibility: 'public' | 'private'
      activityVisibility: 'public' | 'private'
    }
  }
  createdAt: string
  updatedAt: string
  status: 'active' | 'suspended' | 'deleted'
}

// 提货券相关类型
export interface Voucher {
  id: string
  voucherTokenAddress: string
  producerId: string
  productInfo: {
    name: string
    description: string
    category: string
    images: string[]
    specifications: Record<string, any>
  }
  pricing: {
    unitPrice: number
    currency: string
    totalSupply: number
    availableSupply: number
  }
  distributionRules: {
    maxPerUser: number
    restrictionPeriod: string
    geographicRestrictions?: {
      type: 'city' | 'province' | 'country'
      values: string[]
    }
    userTypeRestrictions?: string[]
  }
  lifecycle: {
    createdAt: string
    expiresAt: string
    lifecycleDays: number
  }
  blockchainInfo: {
    tokenAddress: string
    creationTransaction: string
    blockHeight: number
  }
  status: 'draft' | 'active' | 'paused' | 'expired' | 'cancelled'
}

// 消费记录类型
export interface Consumption {
  id: string
  voucherId: string
  consumerId: string
  producerId: string
  consumptionDetails: {
    quantity: number
    unitPrice: number
    totalValue: number
    consumptionMethod: 'pickup' | 'delivery' | 'digital'
    location?: {
      latitude: number
      longitude: number
      address: string
    }
  }
  multiplierCalculation: {
    baseMultiplier: number
    qualityScore: number
    serviceScore: number
    finalMultiplier: number
  }
  rewardDistribution: {
    totalReward: number
    producerReward: number
    workerReward: number
    distributionPercentage: number
  }
  satisfactionRating?: {
    overallScore: number
    detailedScores: Record<string, number>
    feedback: string
    photos: string[]
  }
  blockchainTransaction: {
    transactionHash: string
    blockHeight: number
    gasUsed: number
  }
  timestamps: {
    consumedAt: string
    confirmedAt?: string
    ratedAt?: string
  }
  status: 'pending' | 'confirmed' | 'rated' | 'disputed'
}

// 企业评估类型
export interface EnterpriseEvaluation {
  id: string
  enterpriseId: string
  evaluatorId: string
  evaluationPeriod: {
    startDate: string
    endDate: string
  }
  scores: {
    productQualityScore: number
    serviceQualityScore: number
    workerWelfareScore: number
    environmentalScore: number
    safetyScore: number
    ideologyScore: number
  }
  totalScore: number
  recommendedMultiplier: number
  evaluationDetails: {
    productQuality: {
      score: number
      evidence: string[]
      improvementAreas: string[]
    }
    workerWelfare: {
      score: number
      evidence: string[]
      workerBenefits: {
        averageSalary: number
        insuranceCoverage: number
        paidLeaveDays: number
        trainingHours: number
      }
    }
    environmentalProtection: {
      score: number
      carbonFootprint: number
      wasteRecyclingRate: number
      renewableEnergyUsage: number
    }
  }
  evaluatorNotes: string
  effectiveDate: string
  status: 'draft' | 'pending' | 'completed' | 'disputed'
  createdAt: string
}

// API响应类型
export interface ApiResponse<T> {
  code: number
  message: string
  data: T
  error?: {
    errorCode: string
    errorDetails: {
      field?: string
      message: string
      value?: any
    }
    requestId: string
    timestamp: string
  }
}

// 分页类型
export interface PaginationParams {
  page: number
  limit: number
  total?: number
  totalPages?: number
}

export interface PaginatedResponse<T> {
  items: T[]
  pagination: PaginationParams
}

// 搜索和过滤类型
export interface SearchFilters {
  category?: string
  status?: string
  priceRange?: {
    min: number
    max: number
  }
  location?: string
  dateRange?: {
    start: string
    end: string
  }
  sort?: {
    field: string
    order: 'asc' | 'desc'
  }
}

// 钱包相关类型
export interface WalletState {
  connected: boolean
  address: string | null
  balance: number
  chainId: number
  provider: string | null
}

// 通知类型
export interface Notification {
  id: string
  type: 'info' | 'success' | 'warning' | 'error'
  title: string
  message: string
  timestamp: string
  read: boolean
  actions?: {
    label: string
    action: () => void
  }[]
}

// 主题类型
export interface Theme {
  mode: 'light' | 'dark'
  primaryColor: string
  accentColor: string
  fontSize: 'small' | 'medium' | 'large'
}

// 配置类型
export interface AppConfig {
  apiUrl: string
  wsUrl: string
  solanaRpcUrl: string
  environment: 'development' | 'staging' | 'production'
  version: string
  features: {
    enableB2B: boolean
    enableRestaurant: boolean
    enableHealthcare: boolean
    enableHousing: boolean
    enableAnalytics: boolean
  }
}

// 组件属性类型
export interface ButtonProps {
  variant?: 'primary' | 'secondary' | 'success' | 'outline' | 'ghost'
  size?: 'small' | 'medium' | 'large'
  loading?: boolean
  disabled?: boolean
  icon?: React.ReactNode
  onClick?: () => void
  className?: string
  children: React.ReactNode
}

export interface CardProps {
  title?: string
  subtitle?: string
  image?: string
  actions?: React.ReactNode
  hoverable?: boolean
  className?: string
  children: React.ReactNode
}

export interface ModalProps {
  visible: boolean
  title?: string
  width?: number
  onClose: () => void
  onConfirm?: () => void
  confirmText?: string
  cancelText?: string
  children: React.ReactNode
}

// 表单类型
export interface FormFieldProps {
  label: string
  name: string
  type?: 'text' | 'email' | 'password' | 'number' | 'tel' | 'url'
  placeholder?: string
  required?: boolean
  disabled?: boolean
  error?: string
  helpText?: string
  className?: string
}

// 图表数据类型
export interface ChartData {
  labels: string[]
  datasets: {
    label: string
    data: number[]
    backgroundColor?: string
    borderColor?: string
    borderWidth?: number
  }[]
}

// 地图类型
export interface MapLocation {
  latitude: number
  longitude: number
  address: string
  name?: string
  description?: string
}

// API响应基础类型
export interface BaseResponse<T = any> {
  success: boolean
  data?: T
  error?: string
  message?: string
}

// 分页响应类型
export interface PaginationResponse<T> {
  items: T[]
  total: number
  page: number
  limit: number
  totalPages: number
}

// HomePage相关类型定义
export interface StatisticItem {
  id: string;
  icon: string;
  value: string;
  label: string;
  color: 'blue' | 'green' | 'orange' | 'purple';
  animationDelay?: string;
}

export interface FeatureModule {
  id: string;
  title: string;
  description: string;
  icon: string;
  color: 'blue' | 'green' | 'pink' | 'purple';
  actionText: string;
  path: string;
}

export interface ApplicationScenario {
  id: string;
  title: string;
  description: string;
  icon: string;
  color: 'blue' | 'green' | 'orange' | 'purple' | 'pink' | 'teal';
}

export interface CoreAdvantage {
  id: string;
  title: string;
  description: string;
  icon: string;
  color: 'blue' | 'green' | 'purple';
  animationDelay?: string;
}

export interface TechStack {
  id: string;
  title: string;
  description: string;
  icon: string;
}

export interface NavigationItem {
  name: string;
  path: string;
  icon?: string;
  color?: string;
}

// 所有类型已通过 export interface 导出 