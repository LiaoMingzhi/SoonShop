import React from 'react';
import { cn } from '@/utils/classnames';

export interface CardProps extends React.HTMLAttributes<HTMLDivElement> {
  variant?: 'default' | 'glass' | 'gradient' | 'elevated' | 'stats';
  size?: 'sm' | 'md' | 'lg' | 'xl';
  padding?: 'none' | 'sm' | 'md' | 'lg' | 'xl';
  hover?: boolean;
  gradient?: 'blue' | 'green' | 'purple' | 'orange' | 'pink' | 'teal';
  children: React.ReactNode;
}

export const Card: React.FC<CardProps> = ({
  variant = 'default',
  size = 'md',
  padding = 'md',
  hover = false,
  gradient,
  className,
  children,
  ...props
}) => {
  const baseClasses = 'rounded-2xl transition-all duration-300 ease-out';
  
  const variants = {
    default: 'bg-white border border-gray-200 shadow-card',
    glass: 'glass-card backdrop-blur-glass',
    gradient: 'text-white shadow-ios-lg',
    elevated: 'bg-white shadow-ios-lg border border-gray-100',
    stats: 'text-white shadow-ios-md'
  };
  
  const sizes = {
    sm: 'min-h-[120px]',
    md: 'min-h-[160px]',
    lg: 'min-h-[200px]',
    xl: 'min-h-[240px]'
  };
  
  const paddings = {
    none: 'p-0',
    sm: 'p-4',
    md: 'p-6',
    lg: 'p-8',
    xl: 'p-10'
  };
  
  const gradients = {
    blue: 'bg-gradient-blue',
    green: 'bg-gradient-green',
    purple: 'bg-gradient-purple',
    orange: 'bg-gradient-orange',
    pink: 'bg-gradient-pink',
    teal: 'bg-gradient-teal'
  };
  
  const hoverClasses = hover ? 'card-hover cursor-pointer' : '';
  
  const getVariantClasses = () => {
    if (variant === 'gradient' && gradient) {
      return gradients[gradient];
    }
    if (variant === 'stats' && gradient) {
      return gradients[gradient];
    }
    return variants[variant];
  };
  
  return (
    <div
      className={cn(
        baseClasses,
        getVariantClasses(),
        sizes[size],
        paddings[padding],
        hoverClasses,
        className
      )}
      {...props}
    >
      {children}
    </div>
  );
};

// 专门的统计卡片组件
export interface StatsCardProps {
  title: string;
  value: string | number;
  subtitle?: string;
  icon?: React.ReactNode;
  gradient?: 'blue' | 'green' | 'purple' | 'orange' | 'pink' | 'teal';
  trend?: {
    value: number;
    label: string;
  };
  className?: string;
  style?: React.CSSProperties;
}

export const StatsCard: React.FC<StatsCardProps> = ({
  title,
  value,
  subtitle,
  icon,
  gradient = 'blue',
  trend,
  className
}) => {
  return (
    <Card
      variant="stats"
      gradient={gradient}
      padding="lg"
      hover
      className={cn('relative overflow-hidden', className)}
    >
      <div className="flex items-center justify-between">
        <div className="flex-1">
          <p className="text-white/80 text-sm font-medium mb-1">{title}</p>
          <p className="text-3xl font-bold text-white mb-1">{value}</p>
          {subtitle && (
            <p className="text-white/70 text-xs">{subtitle}</p>
          )}
          {trend && (
            <div className="flex items-center mt-2">
              <span className="text-white/90 text-xs font-medium">
                {trend.value > 0 ? '+' : ''}{trend.value}% {trend.label}
              </span>
            </div>
          )}
        </div>
        {icon && (
          <div className="text-white/60 text-3xl ml-4">
            {icon}
          </div>
        )}
      </div>
      
      {/* 装饰性渐变 */}
      <div className="absolute -top-6 -right-6 w-24 h-24 rounded-full bg-white/10 blur-xl"></div>
      <div className="absolute -bottom-4 -left-4 w-16 h-16 rounded-full bg-white/5 blur-lg"></div>
    </Card>
  );
};

// 功能特性卡片组件
export interface FeatureCardProps {
  title: string;
  description: string;
  icon: React.ReactNode;
  gradient?: 'blue' | 'green' | 'purple' | 'orange' | 'pink' | 'teal';
  actionText?: string;
  onAction?: () => void;
  className?: string;
}

export const FeatureCard: React.FC<FeatureCardProps> = ({
  title,
  description,
  icon,
  gradient = 'blue',
  actionText,
  onAction,
  className
}) => {
  const gradientClasses = {
    blue: 'icon-gradient-blue',
    green: 'icon-gradient-green',
    purple: 'icon-gradient-purple',
    orange: 'icon-gradient-orange',
    pink: 'icon-gradient-pink',
    teal: 'icon-gradient-teal'
  };
  
  const textColors = {
    blue: 'text-blue-600',
    green: 'text-green-600',
    purple: 'text-purple-600',
    orange: 'text-orange-600',
    pink: 'text-pink-600',
    teal: 'text-teal-600'
  };
  
  return (
    <Card
      variant="elevated"
      padding="lg"
      hover
      onClick={onAction}
      className={cn('cursor-pointer', className)}
    >
      <div className="flex items-start space-x-6">
        <div className={cn(
          'w-16 h-16 rounded-2xl flex items-center justify-center text-white text-2xl flex-shrink-0',
          gradientClasses[gradient]
        )}>
          {icon}
        </div>
        <div className="flex-1 min-w-0">
          <h3 className="text-xl font-bold text-gray-900 mb-3">{title}</h3>
          <p className="text-gray-600 mb-4 leading-relaxed">{description}</p>
          {actionText && (
            <div className={cn('flex items-center font-medium', textColors[gradient])}>
              <span>{actionText}</span>
              <svg className="w-5 h-5 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
              </svg>
            </div>
          )}
        </div>
      </div>
    </Card>
  );
};

// 产品卡片组件
export interface ProductCardProps {
  name: string;
  description: string;
  image: string;
  price?: string;
  originalPrice?: string;
  badge?: {
    text: string;
    color: 'green' | 'blue' | 'purple' | 'orange' | 'pink';
  };
  stats?: {
    rating: number;
    reviews: number;
    stock: number;
  };
  onAction?: () => void;
  actionText?: string;
  className?: string;
}

export const ProductCard: React.FC<ProductCardProps> = ({
  name,
  description,
  image,
  price,
  originalPrice,
  badge,
  stats,
  onAction,
  actionText = "立即获取",
  className
}) => {
  const badgeColors = {
    green: 'bg-green-500',
    blue: 'bg-blue-500',
    purple: 'bg-purple-500',
    orange: 'bg-orange-500',
    pink: 'bg-pink-500'
  };
  
  return (
    <Card
      variant="elevated"
      padding="none"
      hover
      className={cn('overflow-hidden', className)}
    >
      {/* 产品图片 */}
      <div className="relative aspect-[4/3] overflow-hidden">
        <img
          src={image}
          alt={name}
          className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110"
          loading="lazy"
        />
        
        {/* 渐变遮罩 */}
        <div className="absolute inset-0 bg-gradient-to-t from-black/20 via-transparent to-transparent" />
        
        {/* 徽章 */}
        {badge && (
          <div className="absolute top-4 left-4">
            <span className={cn(
              'px-3 py-1 rounded-full text-white text-sm font-medium shadow-lg',
              badgeColors[badge.color]
            )}>
              {badge.text}
            </span>
          </div>
        )}
      </div>
      
      {/* 产品信息 */}
      <div className="p-6">
        <h3 className="text-lg font-bold text-gray-900 mb-2">{name}</h3>
        <p className="text-gray-600 text-sm mb-4 line-clamp-2">{description}</p>
        
        {/* 价格信息 */}
        {price && (
          <div className="flex items-center mb-4">
            <span className="text-2xl font-bold text-gray-900">{price}</span>
            {originalPrice && (
              <span className="text-sm text-gray-500 line-through ml-2">{originalPrice}</span>
            )}
          </div>
        )}
        
        {/* 统计信息 */}
        {stats && (
          <div className="flex items-center justify-between text-sm text-gray-500 mb-4">
            <div className="flex items-center">
              <span className="text-yellow-400 mr-1">★</span>
              <span>{stats.rating}</span>
              <span className="mx-1">·</span>
              <span>{stats.reviews}评价</span>
            </div>
            <div>
              剩余: {stats.stock}
            </div>
          </div>
        )}
        
        {/* 行动按钮 */}
        {onAction && (
          <button
            onClick={onAction}
            className="w-full bg-gradient-primary text-white py-3 px-4 rounded-xl font-medium 
                     transition-all duration-200 hover:shadow-button active:scale-95
                     focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-offset-2"
          >
            {actionText}
          </button>
        )}
      </div>
    </Card>
  );
};

// 卡片头部组件
export const CardHeader: React.FC<React.HTMLAttributes<HTMLDivElement>> = ({ 
  className, 
  children, 
  ...props 
}) => (
  <div className={cn("flex flex-col space-y-1.5 p-6", className)} {...props}>
    {children}
  </div>
);

// 卡片标题组件
export const CardTitle: React.FC<React.HTMLAttributes<HTMLHeadingElement>> = ({ 
  className, 
  children, 
  ...props 
}) => (
  <h3 className={cn("text-2xl font-semibold leading-none tracking-tight", className)} {...props}>
    {children}
  </h3>
);

// 卡片描述组件
export const CardDescription: React.FC<React.HTMLAttributes<HTMLParagraphElement>> = ({ 
  className, 
  children, 
  ...props 
}) => (
  <p className={cn("text-sm text-gray-600", className)} {...props}>
    {children}
  </p>
);

// 卡片内容组件
export const CardContent: React.FC<React.HTMLAttributes<HTMLDivElement>> = ({ 
  className, 
  children, 
  ...props 
}) => (
  <div className={cn("p-6 pt-0", className)} {...props}>
    {children}
  </div>
);

// 卡片底部组件
export const CardFooter: React.FC<React.HTMLAttributes<HTMLDivElement>> = ({ 
  className, 
  children, 
  ...props 
}) => (
  <div className={cn("flex items-center p-6 pt-0", className)} {...props}>
    {children}
  </div>
);

// 特殊的主题卡片，使用渐变样式
export const ThemeCard: React.FC<Omit<CardProps, 'variant'> & { theme: 'blue' | 'green' | 'purple' | 'orange' | 'pink' | 'teal' }> = ({ 
  children, 
  theme,
  ...props 
}) => (
  <Card variant="gradient" gradient={theme} {...props}>
    <div className="absolute top-2 right-2 w-6 h-6 text-white/60">
      <svg fill="currentColor" viewBox="0 0 24 24">
        <path d="M12 2L13.09 8.26L19 9L13.09 9.74L12 16L10.91 9.74L5 9L10.91 8.26L12 2Z"/>
      </svg>
    </div>
    {children}
  </Card>
);

export default Card; 