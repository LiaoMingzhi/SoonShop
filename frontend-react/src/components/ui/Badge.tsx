import React from 'react';
import { cn } from '@/utils/classnames';

export interface BadgeProps extends React.HTMLAttributes<HTMLSpanElement> {
  variant?: 'default' | 'primary' | 'secondary' | 'success' | 'warning' | 'error' | 'info' | 'voucher';
  size?: 'sm' | 'md' | 'lg';
  animated?: boolean;
  children: React.ReactNode;
}

export const Badge: React.FC<BadgeProps> = ({
  variant = 'default',
  size = 'md',
  animated = false,
  children,
  className,
  ...props
}) => {
  const baseClasses = `
    inline-flex items-center font-medium rounded-full
    transition-all duration-200 ease-out
    ${animated ? 'animate-pulse' : ''}
  `;

  const variants = {
    default: `
      bg-gray-100 text-gray-700 border border-gray-200
    `,
    primary: `
      bg-gradient-primary text-white shadow-sm
    `,
    secondary: `
      bg-gradient-to-r from-secondary-500 to-secondary-600 text-white shadow-sm
    `,
    success: `
      bg-gradient-green text-white shadow-sm
    `,
    warning: `
      bg-gradient-orange text-white shadow-sm
    `,
    error: `
      bg-gradient-to-r from-red-500 to-red-600 text-white shadow-sm
    `,
    info: `
      bg-gradient-blue text-white shadow-sm
    `,
    voucher: `
      bg-gradient-pink text-white shadow-sm
      ${animated ? 'animate-pulse-slow' : ''}
    `
  };

  const sizes = {
    sm: 'px-2.5 py-0.5 text-xs',
    md: 'px-3 py-1 text-sm',
    lg: 'px-4 py-1.5 text-base'
  };

  return (
    <span
      className={cn(
        baseClasses,
        variants[variant],
        sizes[size],
        className
      )}
      {...props}
    >
      {children}
    </span>
  );
};

// 状态徽章组件
export interface StatusBadgeProps extends Omit<BadgeProps, 'variant'> {
  status: 'active' | 'inactive' | 'pending' | 'completed' | 'cancelled' | 'expired';
}

export const StatusBadge: React.FC<StatusBadgeProps> = ({
  status,
  ...props
}) => {
  const statusConfig = {
    active: { variant: 'success' as const, text: '活跃' },
    inactive: { variant: 'default' as const, text: '停用' },
    pending: { variant: 'warning' as const, text: '待处理' },
    completed: { variant: 'success' as const, text: '已完成' },
    cancelled: { variant: 'error' as const, text: '已取消' },
    expired: { variant: 'error' as const, text: '已过期' }
  };

  const config = statusConfig[status];

  return (
    <Badge variant={config.variant} {...props}>
      {config.text}
    </Badge>
  );
};

// 计数徽章组件
export interface CountBadgeProps extends Omit<BadgeProps, 'children'> {
  count: number;
  max?: number;
  showZero?: boolean;
}

export const CountBadge: React.FC<CountBadgeProps> = ({
  count,
  max = 99,
  showZero = false,
  ...props
}) => {
  if (count === 0 && !showZero) {
    return null;
  }

  const displayCount = count > max ? `${max}+` : count.toString();

  return (
    <Badge variant="error" size="sm" {...props}>
      {displayCount}
    </Badge>
  );
};

// 新功能徽章组件
export interface NewBadgeProps extends Omit<BadgeProps, 'children' | 'variant'> {
  text?: string;
}

export const NewBadge: React.FC<NewBadgeProps> = ({
  text = '新',
  animated = true,
  ...props
}) => {
  return (
    <Badge variant="error" size="sm" animated={animated} {...props}>
      {text}
    </Badge>
  );
};

// 提货券徽章组件
export interface VoucherBadgeProps extends Omit<BadgeProps, 'children' | 'variant'> {
  text?: string;
  pulsing?: boolean;
}

export const VoucherBadge: React.FC<VoucherBadgeProps> = ({
  text = '免费提货券',
  pulsing = true,
  ...props
}) => {
  return (
    <Badge 
      variant="voucher" 
      animated={pulsing}
      className="font-semibold"
      {...props}
    >
      {text}
    </Badge>
  );
}; 