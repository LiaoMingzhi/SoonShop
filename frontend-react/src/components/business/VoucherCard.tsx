import React from 'react';
import { cn } from '@/utils/classnames';
import { Card } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Badge, VoucherBadge, StatusBadge } from '@/components/ui/Badge';
import { GradientIcon } from '@/components/ui/Icon';

export interface VoucherCardProps {
  id: string;
  title: string;
  description?: string;
  value: number;
  type: 'free' | 'discount' | 'cashback' | 'special';
  status: 'active' | 'used' | 'expired' | 'pending';
  validUntil?: string;
  usageCount?: number;
  maxUsage?: number;
  minAmount?: number;
  category?: string;
  gradient?: 'blue' | 'green' | 'orange' | 'pink' | 'purple' | 'primary';
  onUse?: () => void;
  onView?: () => void;
  className?: string;
}

export const VoucherCard: React.FC<VoucherCardProps> = ({
  id,
  title,
  description,
  value,
  type,
  status,
  validUntil,
  usageCount = 0,
  maxUsage,
  minAmount,
  category,
  gradient = 'primary',
  onUse,
  onView,
  className
}) => {
  const gradientClasses = {
    blue: 'bg-gradient-blue',
    green: 'bg-gradient-green',
    orange: 'bg-gradient-orange',
    pink: 'bg-gradient-pink',
    purple: 'bg-gradient-purple',
    primary: 'bg-gradient-primary'
  };

  const typeConfig = {
    free: { icon: 'gift', text: '免费提货券', color: 'green' },
    discount: { icon: 'percent', text: '折扣券', color: 'orange' },
    cashback: { icon: 'coins', text: '返现券', color: 'pink' },
    special: { icon: 'star', text: '特殊优惠', color: 'purple' }
  };

  const config = typeConfig[type];
  const isUsable = status === 'active' && onUse;
  const isExpired = status === 'expired';
  const isUsed = status === 'used';

  return (
    <div className={cn('relative overflow-hidden', className)}>
      {/* 主卡片 */}
      <div
        className={cn(
          'relative rounded-2xl p-6 text-white transition-all duration-300',
          gradientClasses[gradient],
          isUsable && 'hover:shadow-2xl hover:-translate-y-1 cursor-pointer',
          (isExpired || isUsed) && 'opacity-60 grayscale'
        )}
        onClick={isUsable ? onUse : undefined}
      >
        {/* 右上角状态标识 */}
        <div className="absolute top-4 right-4">
          <Badge variant={status === 'active' ? 'success' : status === 'used' ? 'default' : 'error'}>
            {status === 'active' ? '可用' : status === 'used' ? '已使用' : status === 'expired' ? '已过期' : '待处理'}
          </Badge>
        </div>

        {/* 卡片顶部 */}
        <div className="flex items-start justify-between mb-4">
          <div className="flex-1">
            <div className="flex items-center mb-2">
              <GradientIcon 
                name={config.icon}
                size="md"
                className="mr-2 text-white"
              />
              <VoucherBadge 
                text={config.text}
                pulsing={status === 'active'}
              />
            </div>
            <h3 className="text-xl font-bold mb-1">{title}</h3>
            {description && (
              <p className="text-white/80 text-sm">{description}</p>
            )}
          </div>
        </div>

        {/* 价值显示 */}
        <div className="mb-4">
          <div className="text-3xl font-bold">
            {type === 'discount' ? `${value}%` : `¥${value}`}
          </div>
          {minAmount && (
            <div className="text-white/70 text-sm mt-1">
              满¥{minAmount}可用
            </div>
          )}
        </div>

        {/* 使用信息 */}
        {(maxUsage || validUntil) && (
          <div className="flex items-center justify-between text-white/70 text-xs mb-4">
            {maxUsage && (
              <span>已使用 {usageCount}/{maxUsage}</span>
            )}
            {validUntil && (
              <span>有效期至 {validUntil}</span>
            )}
          </div>
        )}

        {/* 操作按钮 */}
        <div className="flex gap-2">
          {isUsable && (
            <Button
              variant="ghost"
              size="sm"
              className="bg-white/20 hover:bg-white/30 text-white border-white/30"
              onClick={(e) => {
                e.stopPropagation();
                onUse();
              }}
            >
              立即使用
            </Button>
          )}
          {onView && (
            <Button
              variant="ghost"
              size="sm"
              className="bg-white/10 hover:bg-white/20 text-white border-white/20"
              onClick={(e) => {
                e.stopPropagation();
                onView();
              }}
            >
              查看详情
            </Button>
          )}
        </div>

        {/* 装饰性元素 */}
        <div className="absolute top-0 right-0 w-32 h-32 bg-white/10 rounded-full -translate-y-16 translate-x-16" />
        <div className="absolute bottom-0 left-0 w-24 h-24 bg-white/5 rounded-full translate-y-12 -translate-x-12" />
      </div>

      {/* 齿轮边缘效果 */}
      <div className="absolute top-0 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-6 h-6 bg-white rounded-full" />
      <div className="absolute bottom-0 left-1/2 transform -translate-x-1/2 translate-y-1/2 w-6 h-6 bg-white rounded-full" />
    </div>
  );
};

// 简单提货券卡片
export interface SimpleVoucherCardProps {
  title: string;
  value: number;
  type: 'amount' | 'percent';
  status: 'active' | 'used' | 'expired';
  validUntil?: string;
  onUse?: () => void;
  className?: string;
}

export const SimpleVoucherCard: React.FC<SimpleVoucherCardProps> = ({
  title,
  value,
  type,
  status,
  validUntil,
  onUse,
  className
}) => {
  const isActive = status === 'active';
  const statusColors = {
    active: 'border-green-500 bg-green-50',
    used: 'border-gray-400 bg-gray-50',
    expired: 'border-red-400 bg-red-50'
  };

  return (
    <Card
      className={cn(
        'border-2 border-dashed transition-all duration-200',
        statusColors[status],
        isActive && 'hover:shadow-lg cursor-pointer',
        className
      )}
      onClick={isActive ? onUse : undefined}
    >
      <div className="text-center">
        <div className="text-2xl font-bold text-gray-800 mb-2">
          {type === 'percent' ? `${value}%` : `¥${value}`}
        </div>
        <div className="text-sm text-gray-600 mb-3">{title}</div>
        {validUntil && (
          <div className="text-xs text-gray-500">
            有效期至 {validUntil}
          </div>
        )}
        <div className="mt-3">
          <Badge variant={status === 'active' ? 'success' : status === 'used' ? 'default' : 'error'}>
            {status === 'active' ? '可用' : status === 'used' ? '已使用' : '已过期'}
          </Badge>
        </div>
      </div>
    </Card>
  );
};

// 提货券列表项
export interface VoucherListItemProps {
  id: string;
  title: string;
  description?: string;
  value: number;
  type: 'free' | 'discount' | 'cashback';
  status: 'active' | 'used' | 'expired';
  validUntil?: string;
  usageCount?: number;
  maxUsage?: number;
  onUse?: () => void;
  onView?: () => void;
  onDelete?: () => void;
  className?: string;
}

export const VoucherListItem: React.FC<VoucherListItemProps> = ({
  id,
  title,
  description,
  value,
  type,
  status,
  validUntil,
  usageCount = 0,
  maxUsage,
  onUse,
  onView,
  onDelete,
  className
}) => {
  const typeConfig = {
    free: { icon: 'gift', color: 'green' as const },
    discount: { icon: 'percent', color: 'orange' as const },
    cashback: { icon: 'coins', color: 'pink' as const }
  };

  const config = typeConfig[type];
  const isUsable = status === 'active' && onUse;

  return (
    <div className={cn(
      'flex items-center justify-between p-4 bg-white rounded-xl border border-gray-200 hover:border-gray-300 transition-colors',
      className
    )}>
      <div className="flex items-center flex-1">
        <GradientIcon 
          name={config.icon}
          gradient={config.color}
          background
          size="md"
          className="mr-4"
        />
        <div className="flex-1">
          <div className="flex items-center gap-2 mb-1">
            <h3 className="font-semibold text-gray-800">{title}</h3>
            <Badge variant={status === 'active' ? 'success' : status === 'used' ? 'default' : 'error'} size="sm">
              {status === 'active' ? '可用' : status === 'used' ? '已使用' : '已过期'}
            </Badge>
          </div>
          {description && (
            <p className="text-gray-600 text-sm mb-1">{description}</p>
          )}
          <div className="flex items-center gap-4 text-sm text-gray-500">
            <span>价值: {type === 'discount' ? `${value}%` : `¥${value}`}</span>
            {maxUsage && (
              <span>使用次数: {usageCount}/{maxUsage}</span>
            )}
            {validUntil && (
              <span>有效期: {validUntil}</span>
            )}
          </div>
        </div>
      </div>

      <div className="flex items-center gap-2">
        {isUsable && (
          <Button
            variant="primary"
            size="sm"
            onClick={onUse}
          >
            使用
          </Button>
        )}
        {onView && (
          <Button
            variant="ghost"
            size="sm"
            onClick={onView}
          >
            查看
          </Button>
        )}
        {onDelete && (
          <Button
            variant="ghost"
            size="sm"
            onClick={onDelete}
            className="text-red-600 hover:text-red-700"
          >
            <GradientIcon name="trash" size="sm" />
          </Button>
        )}
      </div>
    </div>
  );
};

// 提货券统计卡片
export interface VoucherStatsCardProps {
  title: string;
  total: number;
  used: number;
  expired: number;
  active: number;
  onViewAll?: () => void;
  className?: string;
}

export const VoucherStatsCard: React.FC<VoucherStatsCardProps> = ({
  title,
  total,
  used,
  expired,
  active,
  onViewAll,
  className
}) => {
  const usageRate = total > 0 ? (used / total) * 100 : 0;

  return (
    <Card className={cn('hover:shadow-lg transition-shadow', className)}>
      <div className="flex items-center justify-between mb-4">
        <h3 className="text-lg font-semibold text-gray-800">{title}</h3>
        <GradientIcon 
          name="ticket-alt"
          gradient="primary"
          background
          size="md"
        />
      </div>

      <div className="grid grid-cols-2 gap-4 mb-4">
        <div className="text-center">
          <div className="text-2xl font-bold text-gray-800">{total}</div>
          <div className="text-sm text-gray-600">总计</div>
        </div>
        <div className="text-center">
          <div className="text-2xl font-bold text-green-600">{active}</div>
          <div className="text-sm text-gray-600">可用</div>
        </div>
        <div className="text-center">
          <div className="text-2xl font-bold text-blue-600">{used}</div>
          <div className="text-sm text-gray-600">已使用</div>
        </div>
        <div className="text-center">
          <div className="text-2xl font-bold text-red-600">{expired}</div>
          <div className="text-sm text-gray-600">已过期</div>
        </div>
      </div>

      <div className="mb-4">
        <div className="flex justify-between text-sm mb-2">
          <span className="text-gray-600">使用率</span>
          <span className="font-medium">{usageRate.toFixed(1)}%</span>
        </div>
        <div className="w-full bg-gray-200 rounded-full h-2">
          <div
            className="bg-blue-500 h-2 rounded-full transition-all duration-500"
            style={{ width: `${usageRate}%` }}
          />
        </div>
      </div>

      {onViewAll && (
        <Button
          variant="outline"
          size="sm"
          fullWidth
          onClick={onViewAll}
        >
          查看全部
        </Button>
      )}
    </Card>
  );
}; 