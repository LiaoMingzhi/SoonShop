import React from 'react';
import { cn } from '@/utils/classnames';
import { Card } from '@/components/ui/Card';
import { GradientIcon } from '@/components/ui/Icon';

export interface StatsCardProps {
  title: string;
  value: string | number;
  subtitle?: string;
  icon?: string;
  gradient?: 'blue' | 'green' | 'orange' | 'pink' | 'purple' | 'primary';
  trend?: {
    direction: 'up' | 'down' | 'neutral';
    value: string;
    period?: string;
  };
  animated?: boolean;
  className?: string;
  onClick?: () => void;
}

export const StatsCard: React.FC<StatsCardProps> = ({
  title,
  value,
  subtitle,
  icon,
  gradient = 'primary',
  trend,
  animated = false,
  className,
  onClick
}) => {
  const gradientClasses = {
    blue: 'bg-gradient-blue',
    green: 'bg-gradient-green',
    orange: 'bg-gradient-orange',
    pink: 'bg-gradient-pink',
    purple: 'bg-gradient-purple',
    primary: 'bg-gradient-primary'
  };

  const trendColors = {
    up: 'text-green-600',
    down: 'text-red-600',
    neutral: 'text-gray-600'
  };

  const trendIcons = {
    up: 'arrow-up',
    down: 'arrow-down',
    neutral: 'minus'
  };

  return (
    <div
      className={cn(
        'rounded-2xl p-6 text-white cursor-pointer transition-all duration-300 hover:shadow-xl hover:-translate-y-1',
        gradientClasses[gradient],
        animated ? 'animate-float' : '',
        className
      )}
      onClick={onClick}
    >
      <div className="flex items-center justify-between">
        <div className="flex-1">
          <p className="text-white/80 text-sm mb-1">{title}</p>
          <p className="text-2xl font-bold mb-2">{value}</p>
          {subtitle && (
            <p className="text-white/70 text-xs">{subtitle}</p>
          )}
          {trend && (
            <div className="flex items-center mt-2">
              <GradientIcon 
                name={trendIcons[trend.direction]}
                size="xs"
                className="mr-1"
              />
              <span className="text-xs text-white/80">
                {trend.value}
                {trend.period && ` ${trend.period}`}
              </span>
            </div>
          )}
        </div>
        {icon && (
          <div className="ml-4">
            <GradientIcon 
              name={icon} 
              size="3xl" 
              className="text-white/60" 
            />
          </div>
        )}
      </div>
    </div>
  );
};

// 简单统计卡片组件
export interface SimpleStatsCardProps {
  label: string;
  value: string | number;
  icon?: string;
  color?: 'blue' | 'green' | 'orange' | 'pink' | 'purple' | 'primary';
  change?: {
    value: number;
    type: 'increase' | 'decrease';
  };
  className?: string;
}

export const SimpleStatsCard: React.FC<SimpleStatsCardProps> = ({
  label,
  value,
  icon,
  color = 'blue',
  change,
  className
}) => {
  const colorClasses = {
    blue: 'text-blue-600',
    green: 'text-green-600',
    orange: 'text-orange-600',
    pink: 'text-pink-600',
    purple: 'text-purple-600',
    primary: 'text-primary-600'
  };

  return (
    <Card className={cn('text-center', className)} hover>
      {icon && (
        <div className="mb-4">
          <GradientIcon 
            name={icon} 
            size="2xl" 
            gradient={color}
            background
          />
        </div>
      )}
      <div className="text-3xl font-bold text-gray-800 mb-2">{value}</div>
      <div className="text-gray-600 text-sm">{label}</div>
      {change && (
        <div className={cn(
          'text-xs mt-2 flex items-center justify-center',
          change.type === 'increase' ? 'text-green-600' : 'text-red-600'
        )}>
          <GradientIcon 
            name={change.type === 'increase' ? 'arrow-up' : 'arrow-down'}
            size="xs"
            className="mr-1"
          />
          {Math.abs(change.value)}%
        </div>
      )}
    </Card>
  );
};

// 带描述的统计卡片
export interface DetailedStatsCardProps {
  title: string;
  value: string | number;
  description: string;
  icon?: string;
  iconColor?: 'blue' | 'green' | 'orange' | 'pink' | 'purple';
  progress?: {
    value: number;
    max: number;
    color?: 'blue' | 'green' | 'orange' | 'red';
  };
  actions?: React.ReactNode;
  className?: string;
}

export const DetailedStatsCard: React.FC<DetailedStatsCardProps> = ({
  title,
  value,
  description,
  icon,
  iconColor = 'blue',
  progress,
  actions,
  className
}) => {
  const progressColors = {
    blue: 'bg-blue-500',
    green: 'bg-green-500',
    orange: 'bg-orange-500',
    red: 'bg-red-500'
  };

  const progressPercentage = progress ? (progress.value / progress.max) * 100 : 0;

  return (
    <Card className={className} hover>
      <div className="flex items-start justify-between mb-4">
        <div className="flex-1">
          <h3 className="text-lg font-semibold text-gray-800 mb-1">{title}</h3>
          <p className="text-3xl font-bold text-gray-900 mb-2">{value}</p>
          <p className="text-sm text-gray-600">{description}</p>
        </div>
        {icon && (
          <GradientIcon 
            name={icon} 
            gradient={iconColor}
            background
            size="lg"
          />
        )}
      </div>

      {progress && (
        <div className="mb-4">
          <div className="flex justify-between text-sm mb-2">
            <span className="text-gray-600">进度</span>
            <span className="font-medium">{progress.value}/{progress.max}</span>
          </div>
          <div className="w-full bg-gray-200 rounded-full h-2">
            <div
              className={cn(
                'h-2 rounded-full transition-all duration-500',
                progressColors[progress.color || 'blue']
              )}
              style={{ width: `${progressPercentage}%` }}
            />
          </div>
        </div>
      )}

      {actions && (
        <div className="pt-4 border-t border-gray-200">
          {actions}
        </div>
      )}
    </Card>
  );
};

// 环形进度统计卡片
export interface CircularStatsCardProps {
  title: string;
  value: number;
  max: number;
  unit?: string;
  color?: 'blue' | 'green' | 'orange' | 'red' | 'purple';
  size?: 'sm' | 'md' | 'lg';
  className?: string;
}

export const CircularStatsCard: React.FC<CircularStatsCardProps> = ({
  title,
  value,
  max,
  unit = '',
  color = 'blue',
  size = 'md',
  className
}) => {
  const percentage = (value / max) * 100;
  const circumference = 2 * Math.PI * 45; // 半径为45的圆周长
  const strokeDasharray = circumference;
  const strokeDashoffset = circumference - (percentage / 100) * circumference;

  const colorClasses = {
    blue: 'stroke-blue-500',
    green: 'stroke-green-500',
    orange: 'stroke-orange-500',
    red: 'stroke-red-500',
    purple: 'stroke-purple-500'
  };

  const sizes = {
    sm: { width: 80, height: 80, textSize: 'text-lg' },
    md: { width: 120, height: 120, textSize: 'text-2xl' },
    lg: { width: 160, height: 160, textSize: 'text-3xl' }
  };

  const sizeConfig = sizes[size];

  return (
    <Card className={cn('text-center', className)} hover>
      <h3 className="text-lg font-semibold text-gray-800 mb-4">{title}</h3>
      
      <div className="relative inline-flex items-center justify-center mb-4">
        <svg width={sizeConfig.width} height={sizeConfig.height} className="transform -rotate-90">
          {/* 背景圆环 */}
          <circle
            cx={sizeConfig.width / 2}
            cy={sizeConfig.height / 2}
            r="45"
            stroke="currentColor"
            strokeWidth="8"
            fill="none"
            className="text-gray-200"
          />
          {/* 进度圆环 */}
          <circle
            cx={sizeConfig.width / 2}
            cy={sizeConfig.height / 2}
            r="45"
            stroke="currentColor"
            strokeWidth="8"
            fill="none"
            strokeLinecap="round"
            className={colorClasses[color]}
            style={{
              strokeDasharray,
              strokeDashoffset,
              transition: 'stroke-dashoffset 0.5s ease-in-out'
            }}
          />
        </svg>
        
        {/* 中心文本 */}
        <div className="absolute inset-0 flex flex-col items-center justify-center">
          <div className={cn('font-bold text-gray-900', sizeConfig.textSize)}>
            {value}
          </div>
          {unit && (
            <div className="text-sm text-gray-600">{unit}</div>
          )}
        </div>
      </div>
      
      <div className="text-sm text-gray-600">
        {percentage.toFixed(1)}% 完成
      </div>
    </Card>
  );
}; 