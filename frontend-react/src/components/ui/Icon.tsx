import React from 'react';
import { cn } from '@/utils/classnames';

interface IconProps extends React.SVGAttributes<SVGElement> {
  name?: string;
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl' | '2xl';
  color?: string;
  gradient?: boolean;
  spin?: boolean;
  pulse?: boolean;
  children?: React.ReactNode;
}

export const Icon: React.FC<IconProps> = ({
  name,
  size = 'md',
  color = 'currentColor',
  gradient = false,
  spin = false,
  pulse = false,
  className,
  children,
  ...props
}) => {
  const sizeClasses = {
    xs: 'w-3 h-3',
    sm: 'w-4 h-4',
    md: 'w-5 h-5',
    lg: 'w-6 h-6',
    xl: 'w-8 h-8',
    '2xl': 'w-10 h-10',
  };

  const animationClasses = cn({
    'animate-spin': spin,
    'animate-pulse': pulse,
  });

  const iconClasses = cn(
    sizeClasses[size],
    animationClasses,
    {
      'text-gradient': gradient,
    },
    className
  );

  return (
    <svg
      className={iconClasses}
      fill="none"
      stroke={gradient ? undefined : color}
      viewBox="0 0 24 24"
      strokeLinecap="round"
      strokeLinejoin="round"
      strokeWidth="2"
      {...props}
    >
      {children}
    </svg>
  );
};

// 共产主义相关图标组件
export const CommunistIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M12 2L13.09 8.26L19 9L13.09 9.74L12 16L10.91 9.74L5 9L10.91 8.26L12 2Z"/>
  </Icon>
);

export const PeopleIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
    <circle cx="12" cy="7" r="4"/>
  </Icon>
);

export const UnityIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M16 4C18.2 4 20 5.8 20 8C20 10.2 18.2 12 16 12C13.8 12 12 10.2 12 8C12 5.8 13.8 4 16 4M8 4C10.2 4 12 5.8 12 8C12 10.2 10.2 12 8 12C5.8 12 4 10.2 4 8C4 5.8 5.8 4 8 4"/>
    <path d="M8 13C5.33 13 0 14.33 0 17V20H24V17C24 14.33 18.67 13 16 13C13.33 13 8 14.33 8 13Z"/>
  </Icon>
);

export const ProsperityIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M12 2L15.09 8.26L22 9L17 14L18.18 21L12 17.77L5.82 21L7 14L2 9L8.91 8.26L12 2Z"/>
  </Icon>
);

// 业务功能图标
export const VoucherIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M21 3H3c-1.1 0-2 .9-2 2v4c1.1 0 2 .9 2 2s-.9 2-2 2v4c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2v-4c-1.1 0-2-.9-2-2s.9-2 2-2V5c0-1.1-.9-2-2-2z"/>
    <path d="M8 12h8"/>
  </Icon>
);

export const ShoppingIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M3 3h2l.4 2M7 13h10l4-8H5.4M7 13L5.4 5M7 13l-2.293 2.293c-.63.63-.184 1.707.707 1.707H17M17 13v6a2 2 0 01-2 2H9a2 2 0 01-2-2v-6.001"/>
  </Icon>
);

export const RestaurantIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M3 2v7c0 1.1.9 2 2 2h4a2 2 0 002-2V2"/>
    <path d="M7 2v20"/>
    <path d="M21 15V2v0a5 5 0 00-5 5v6c0 1.1.9 2 2 2h3z"/>
  </Icon>
);

export const HealthIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M22 12h-4l-3 9L9 3l-3 9H2"/>
  </Icon>
);

export const HousingIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M3 21h18"/>
    <path d="M5 21V7l8-4v18"/>
    <path d="M19 21V11l-6-4"/>
  </Icon>
);

export const AnalyticsIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M3 3v18h18"/>
    <path d="M18.7 8l-5.1 5.2-2.8-2.7L7 14.3"/>
  </Icon>
);

export const AdminIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M12 1L3 5v6c0 5.55 3.84 10.74 9 12 5.16-1.26 9-6.45 9-12V5z"/>
    <path d="M9 12l2 2 4-4"/>
  </Icon>
);

// 常用操作图标
export const HomeIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M3 9l9-7 9 7v11a2 2 0 01-2 2H5a2 2 0 01-2-2z"/>
    <polyline points="9,22 9,12 15,12 15,22"/>
  </Icon>
);

export const SearchIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <circle cx="11" cy="11" r="8"/>
    <path d="M21 21l-4.35-4.35"/>
  </Icon>
);

export const MenuIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <line x1="3" y1="6" x2="21" y2="6"/>
    <line x1="3" y1="12" x2="21" y2="12"/>
    <line x1="3" y1="18" x2="21" y2="18"/>
  </Icon>
);

export const CloseIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <line x1="18" y1="6" x2="6" y2="18"/>
    <line x1="6" y1="6" x2="18" y2="18"/>
  </Icon>
);

export const ChevronRightIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <polyline points="9,18 15,12 9,6"/>
  </Icon>
);

export const ChevronLeftIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <polyline points="15,18 9,12 15,6"/>
  </Icon>
);

export const PlusIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <line x1="12" y1="5" x2="12" y2="19"/>
    <line x1="5" y1="12" x2="19" y2="12"/>
  </Icon>
);

export const HeartIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"/>
  </Icon>
);

export const StarIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <polygon points="12,2 15.09,8.26 22,9.27 17,14.14 18.18,21.02 12,17.77 5.82,21.02 7,14.14 2,9.27 8.91,8.26"/>
  </Icon>
);

export const BellIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"/>
    <path d="M13.73 21a2 2 0 0 1-3.46 0"/>
  </Icon>
);

export const UserIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/>
    <circle cx="12" cy="7" r="4"/>
  </Icon>
);

export const SettingsIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <circle cx="12" cy="12" r="3"/>
    <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1 1.51V3a2 2 0 0 1 2 2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
  </Icon>
);

// 加载和状态图标
export const LoadingIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon spin {...props}>
    <path d="M21 12a9 9 0 11-6.219-8.56"/>
  </Icon>
);

export const CheckIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <polyline points="20,6 9,17 4,12"/>
  </Icon>
);

export const AlertIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"/>
    <line x1="12" y1="9" x2="12" y2="13"/>
    <line x1="12" y1="17" x2="12.01" y2="17"/>
  </Icon>
);

export const InfoIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <circle cx="12" cy="12" r="10"/>
    <line x1="12" y1="16" x2="12" y2="12"/>
    <line x1="12" y1="8" x2="12.01" y2="8"/>
  </Icon>
);

export const LockIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <rect x="3" y="11" width="18" height="11" rx="2" ry="2"/>
    <circle cx="12" cy="16" r="1"/>
    <path d="M7 11V7a5 5 0 0 1 10 0v4"/>
  </Icon>
);

export const WalletIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <path d="M21 12V7H5a2 2 0 0 1 0-4h14v4"/>
    <path d="M3 5v14a2 2 0 0 0 2 2h16v-5"/>
    <path d="M18 12a2 2 0 0 0 0 4h4V8h-4a2 2 0 0 0 0 4Z"/>
  </Icon>
);

export const TrendingUpIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <polyline points="23,6 13.5,15.5 8.5,10.5 1,18"/>
    <polyline points="17,6 23,6 23,12"/>
  </Icon>
);

export const TrendingDownIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <polyline points="23,18 13.5,8.5 8.5,13.5 1,6"/>
    <polyline points="17,18 23,18 23,12"/>
  </Icon>
);

export const FilterIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <polygon points="22,3 2,3 10,12.46 10,19 14,21 14,12.46"/>
  </Icon>
);

export const CalendarIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <rect x="3" y="4" width="18" height="18" rx="2" ry="2"/>
    <line x1="16" y1="2" x2="16" y2="6"/>
    <line x1="8" y1="2" x2="8" y2="6"/>
    <line x1="3" y1="10" x2="21" y2="10"/>
  </Icon>
);

export const ArrowLeftIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <line x1="19" y1="12" x2="5" y2="12"/>
    <polyline points="12,19 5,12 12,5"/>
  </Icon>
);

export const ShoppingCartIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <circle cx="9" cy="21" r="1"/>
    <circle cx="20" cy="21" r="1"/>
    <path d="M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6"/>
  </Icon>
);

export const CogIcon: React.FC<Omit<IconProps, 'children'>> = (props) => (
  <Icon {...props}>
    <circle cx="12" cy="12" r="3"/>
    <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1 1.51V3a2 2 0 0 1 2 2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
  </Icon>
);

// 兼容旧的渐变图标组件
export const GradientIcon: React.FC<{ name: string, size?: string, gradient?: string, background?: boolean, className?: string }> = ({ 
  name, 
  size = 'md', 
  gradient = 'primary', 
  background = false, 
  className,
  ...props 
}) => {
  // 这是一个兼容性组件，用于支持旧的 GradientIcon 用法
  const iconMap: { [key: string]: React.ComponentType<any> } = {
    'home': HomeIcon,
    'user': UserIcon,
    'shopping-cart': ShoppingCartIcon,
    'heart': HeartIcon,
    'star': StarIcon,
    'ticket-alt': VoucherIcon,
    'chart-line': AnalyticsIcon,
    'cogs': CogIcon,
    'bell': BellIcon,
    'search': SearchIcon,
    'plus': PlusIcon,
    'edit': SettingsIcon,
    'trash': CloseIcon,
    'eye': CheckIcon,
    'download': ChevronRightIcon,
    'check': CheckIcon,
    'times': CloseIcon,
    'spinner': LoadingIcon,
    'arrow-left': ArrowLeftIcon,
    'arrow-right': ChevronRightIcon,
    'chevron-down': ChevronRightIcon,
    'chevron-up': ChevronLeftIcon,
    'info-circle': InfoIcon,
    'exclamation-triangle': AlertIcon,
    'exclamation-circle': AlertIcon,
    'check-circle': CheckIcon,
    'coins': ProsperityIcon,
    'gift': ProsperityIcon,
    'chart-bar': AnalyticsIcon,
    'handshake': UnityIcon,
    'chart-pie': AnalyticsIcon
  };

  const IconComponent = iconMap[name] || Icon;
  
  if (background) {
    return (
      <div className={cn(
        `p-2 rounded-full bg-gradient-to-br from-${gradient}-400 to-${gradient}-600 text-white`,
        className
      )}>
        <IconComponent size={size} />
      </div>
    );
  }
  
  return <IconComponent size={size} className={cn(`text-${gradient}-600`, className)} {...props} />;
};

export default Icon; 