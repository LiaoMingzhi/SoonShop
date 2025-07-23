import React from 'react'
import { cn } from '@/utils/classnames'

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: 'primary' | 'secondary' | 'ghost' | 'outline' | 'success' | 'warning' | 'danger' | 'gradient'
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  gradient?: 'blue' | 'green' | 'purple' | 'orange' | 'pink' | 'teal'
  loading?: boolean
  icon?: React.ReactNode
  iconPosition?: 'left' | 'right'
  fullWidth?: boolean
  children: React.ReactNode
}

export const Button: React.FC<ButtonProps> = ({
  variant = 'primary',
  size = 'md',
  gradient,
  loading = false,
  icon,
  iconPosition = 'left',
  fullWidth = false,
  disabled = false,
  className,
  children,
  ...props
}) => {
  const baseClasses = `
    inline-flex items-center justify-center font-medium rounded-xl transition-all duration-200 ease-out
    focus:outline-none focus:ring-2 focus:ring-offset-2 active:scale-95 disabled:opacity-50 
    disabled:cursor-not-allowed disabled:transform-none
  `;

  const variants = {
    primary: `
      bg-gradient-primary text-white shadow-ios-md hover:shadow-button
      focus:ring-primary-500 btn-hover
    `,
    secondary: `
      bg-white text-gray-900 border border-gray-200 shadow-ios-sm hover:bg-gray-50 
      hover:shadow-ios-md focus:ring-gray-500 btn-hover
    `,
    ghost: `
      bg-transparent text-gray-700 hover:bg-gray-100 hover:text-gray-900
      focus:ring-gray-500 btn-hover
    `,
    outline: `
      bg-transparent border-2 border-current text-primary-600 hover:bg-primary-50
      focus:ring-primary-500 btn-hover
    `,
    success: `
      bg-gradient-green text-white shadow-ios-md hover:shadow-button
      focus:ring-green-500 btn-hover
    `,
    warning: `
      bg-gradient-orange text-white shadow-ios-md hover:shadow-button
      focus:ring-orange-500 btn-hover
    `,
    danger: `
      bg-red-500 text-white shadow-ios-md hover:bg-red-600 hover:shadow-button
      focus:ring-red-500 btn-hover
    `,
    gradient: `
      text-white shadow-ios-md hover:shadow-button
      focus:ring-primary-500 btn-hover
    `
  };

  const sizes = {
    xs: 'h-7 px-3 text-xs',
    sm: 'h-8 px-4 text-sm',
    md: 'h-10 px-6 text-base',
    lg: 'h-12 px-8 text-lg',
    xl: 'h-14 px-10 text-xl'
  };

  const gradients = {
    blue: 'bg-gradient-blue',
    green: 'bg-gradient-green',
    purple: 'bg-gradient-purple',
    orange: 'bg-gradient-orange',
    pink: 'bg-gradient-pink',
    teal: 'bg-gradient-teal'
  };

  const getVariantClasses = () => {
    if (variant === 'gradient' && gradient) {
      return gradients[gradient];
    }
    return variants[variant];
  };

  const fullWidthClass = fullWidth ? 'w-full' : '';
  
  const iconElement = icon && (
    <span className={cn(
      'inline-flex',
      children && iconPosition === 'left' ? 'mr-2' : '',
      children && iconPosition === 'right' ? 'ml-2' : ''
    )}>
      {icon}
    </span>
  );

  const loadingElement = loading && (
    <svg 
      className="animate-spin -ml-1 mr-2 h-4 w-4" 
      fill="none" 
      viewBox="0 0 24 24"
    >
      <circle 
        className="opacity-25" 
        cx="12" 
        cy="12" 
        r="10" 
        stroke="currentColor" 
        strokeWidth="4"
      />
      <path 
        className="opacity-75" 
        fill="currentColor" 
        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
      />
    </svg>
  );

  return (
    <button
      className={cn(
        baseClasses,
        getVariantClasses(),
        sizes[size],
        fullWidthClass,
        className
      )}
      disabled={disabled || loading}
      {...props}
    >
      {loading && loadingElement}
      {!loading && icon && iconPosition === 'left' && iconElement}
      {children}
      {!loading && icon && iconPosition === 'right' && iconElement}
    </button>
  );
};

// 图标按钮组件
export interface IconButtonProps extends Omit<ButtonProps, 'children' | 'icon'> {
  icon: React.ReactNode;
  'aria-label': string;
}

export const IconButton: React.FC<IconButtonProps> = ({
  icon,
  size = 'md',
  className,
  ...props
}) => {
  const sizeClasses = {
    xs: 'w-7 h-7',
    sm: 'w-8 h-8',
    md: 'w-10 h-10',
    lg: 'w-12 h-12',
    xl: 'w-14 h-14'
  };

  return (
    <Button
      className={cn('rounded-full', sizeClasses[size], className)}
      size={size}
      {...props}
    >
      {icon}
    </Button>
  );
};

// 按钮组组件
export interface ButtonGroupProps {
  children: React.ReactNode;
  className?: string;
  orientation?: 'horizontal' | 'vertical';
}

export const ButtonGroup: React.FC<ButtonGroupProps> = ({
  children,
  className,
  orientation = 'horizontal'
}) => {
  const orientationClasses = {
    horizontal: 'flex-row',
    vertical: 'flex-col'
  };

  return (
    <div className={cn(
      'inline-flex',
      orientationClasses[orientation],
      orientation === 'horizontal' ? 'space-x-2' : 'space-y-2',
      className
    )}>
      {children}
    </div>
  );
};

// 浮动操作按钮 (FAB)
export interface FABProps extends Omit<ButtonProps, 'variant' | 'size'> {
  position?: 'bottom-right' | 'bottom-left' | 'top-right' | 'top-left';
  icon: React.ReactNode;
}

export const FAB: React.FC<FABProps> = ({
  position = 'bottom-right',
  icon,
  className,
  ...props
}) => {
  const positions = {
    'bottom-right': 'fixed bottom-6 right-6',
    'bottom-left': 'fixed bottom-6 left-6',
    'top-right': 'fixed top-6 right-6',
    'top-left': 'fixed top-6 left-6'
  };

  return (
    <Button
      variant="gradient"
      gradient="blue"
      className={cn(
        'w-14 h-14 rounded-full shadow-ios-xl hover:shadow-2xl z-50',
        positions[position],
        className
      )}
      {...props}
    >
      {icon}
    </Button>
  );
};

export default Button 