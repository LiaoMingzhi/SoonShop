import React, { useState } from 'react';
import { Link, useLocation } from 'react-router-dom';
// 暂时移除@/路径别名导入，改为相对路径
// import { cn } from '@/utils/classnames';
// import { Button } from '@/components/ui/Button';
// import { SearchInput } from '@/components/ui/Input';
// import { Badge, CountBadge } from '@/components/ui/Badge';
// import { GradientIcon, ArrowLeftIcon, UserIcon, ShoppingCartIcon, CogIcon, BellIcon } from '@/components/ui/Icon';

// 简单的cn函数实现
const cn = (...classes: (string | undefined | null | false)[]) => {
  return classes.filter(Boolean).join(' ');
};

// 新的固定导航栏组件
export const StickyNavigation: React.FC = () => {
  return (
    <nav style={{ 
      position: 'fixed', 
      top: 0, 
      left: 0, 
      right: 0, 
      backgroundColor: 'white', 
      boxShadow: '0 2px 4px rgba(0,0,0,0.1)', 
      zIndex: 1000,
      height: '60px',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'space-between',
      padding: '0 20px'
    }}>
      {/* 左侧品牌区域 */}
      <div style={{ display: 'flex', alignItems: 'center' }}>
        <Link to="/" style={{ display: 'flex', alignItems: 'center', textDecoration: 'none', color: 'inherit' }}>
          <div style={{ 
            width: '40px', 
            height: '40px', 
            backgroundColor: '#3B82F6', 
            borderRadius: '8px', 
            display: 'flex', 
            alignItems: 'center', 
            justifyContent: 'center', 
            marginRight: '12px',
            color: 'white'
          }}>
            <i className="fas fa-store"></i>
          </div>
          <div>
            <h1 style={{ margin: 0, fontSize: '18px', fontWeight: 'bold', color: '#1F2937' }}>SoonShop</h1>
            <p style={{ margin: 0, fontSize: '12px', color: '#6B7280' }}>共产主义商业平台</p>
          </div>
        </Link>
      </div>

      {/* 右侧操作区域 */}
      <div style={{ display: 'flex', alignItems: 'center', gap: '16px' }}>
        <Link to="/admin" style={{ 
          display: 'flex', 
          alignItems: 'center', 
          textDecoration: 'none', 
          color: '#6B7280',
          gap: '8px'
        }}>
          <i className="fas fa-cogs"></i>
          <span>系统管理</span>
        </Link>
        <button style={{ 
          background: 'none', 
          border: 'none', 
          color: '#6B7280',
          cursor: 'pointer',
          padding: '8px',
          borderRadius: '4px'
        }} aria-label="用户菜单">
          <i className="fas fa-user"></i>
        </button>
      </div>
    </nav>
  );
};

export interface NavigationProps {
  title?: string;
  subtitle?: string;
  showBackButton?: boolean;
  showSearch?: boolean;
  showCart?: boolean;
  showNotifications?: boolean;
  cartCount?: number;
  notificationCount?: number;
  currentMultiplier?: number;
  onBack?: () => void;
  onSearch?: (value: string) => void;
  onCartClick?: () => void;
  onNotificationClick?: () => void;
  onProfileClick?: () => void;
  rightActions?: React.ReactNode;
  className?: string;
  showLogo?: boolean;
  showUserMenu?: boolean;
  transparent?: boolean;
}

export const Navigation: React.FC<NavigationProps> = ({
  title = 'SoonShop',
  subtitle = '共产主义商业平台',
  showBackButton = false,
  showSearch = false,
  showCart = false,
  showNotifications = false,
  cartCount = 0,
  notificationCount = 0,
  currentMultiplier,
  onBack,
  onSearch,
  onCartClick,
  onNotificationClick,
  onProfileClick,
  rightActions,
  className,
  showLogo = true,
  showUserMenu = true,
  transparent = false
}) => {
  const [isSearchFocused, setIsSearchFocused] = useState(false);
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const location = useLocation();

  const navItems = [
    {
      label: '首页',
      href: '/',
      icon: (
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
        </svg>
      )
    },
    {
      label: '提货券',
      href: '/vouchers',
      icon: (
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 5v2m0 4v2m0 4v2M5 5a2 2 0 00-2 2v3a2 2 0 110 4v3a2 2 0 002 2h14a2 2 0 002-2v-3a2 2 0 110-4V7a2 2 0 00-2-2H5z" />
        </svg>
      ),
      badge: 'HOT'
    },
    {
      label: 'B2C商城',
      href: '/b2c',
      icon: (
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 3h2l.4 2M7 13h10l4-8H5.4m0 0L7 13m0 0l-2.5 5L17 18M9 19.5a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zM20 19.5a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0z" />
        </svg>
      )
    },
    {
      label: '企业评估',
      href: '/evaluation',
      icon: (
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
        </svg>
      )
    },
    {
      label: '数据分析',
      href: '/analytics',
      icon: (
        <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M16 8v8m-4-5v5m-4-2v2m-2 4h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z" />
        </svg>
      )
    }
  ];

  const isActive = (href: string) => {
    if (href === '/') {
      return location.pathname === '/';
    }
    return location.pathname.startsWith(href);
  };

  return (
    <nav className={cn(
      'sticky top-0 z-50',
      transparent ? 'bg-white/80 backdrop-blur-md' : 'bg-white shadow-lg',
      className
    )}>
      <div className="max-w-7xl mx-auto px-4 tablet:px-6 desktop:px-8">
        <div className="flex justify-between items-center h-16">
          {/* Logo和品牌 */}
          {showLogo && (
            <div className="flex items-center">
              <Link to="/" className="flex items-center">
                <div className="w-10 h-10 rounded-lg flex items-center justify-center mr-3 bg-gradient-to-br from-blue-500 to-purple-600">
                  <svg className="w-5 h-5 text-white" fill="currentColor" viewBox="0 0 24 24">
                    <path d="M20 7h-3V6a4 4 0 00-8 0v1H6a1 1 0 00-1 1v11a3 3 0 003 3h8a3 3 0 003-3V8a1 1 0 00-1-1zM11 6a2 2 0 014 0v1h-4V6zm7 13a1 1 0 01-1 1H8a1 1 0 01-1-1V9h2v1a1 1 0 002 0V9h4v1a1 1 0 002 0V9h2v10z"/>
                  </svg>
                </div>
                <div>
                  <h1 className="text-xl font-bold text-gray-800">{title}</h1>
                  {subtitle && (
                    <p className="text-xs text-gray-600">{subtitle}</p>
                  )}
                </div>
              </Link>
            </div>
          )}

          {/* 桌面菜单 */}
          <div className="hidden tablet:flex items-center space-x-8">
            <NavLink to="/" active={isActive('/')}>首页</NavLink>
            <NavLink to="/vouchers" active={isActive('/vouchers')}>提货券管理</NavLink>
            <NavLink to="/consumption" active={isActive('/consumption')}>消费与倍增</NavLink>
            <NavLink to="/b2c" active={isActive('/b2c')}>B2C电商平台</NavLink>
            <NavLink to="/evaluation" active={isActive('/evaluation')}>企业评估</NavLink>
          </div>
          
          {/* 右侧操作区 */}
          <div className="flex items-center space-x-4">
            {/* 当前倍增显示 */}
            {currentMultiplier && (
              <div className="flex items-center space-x-2 bg-green-100 rounded-full px-4 py-2">
                <i className="fas fa-coins text-green-600"></i>
                <span className="text-green-600 text-sm font-medium">
                  当前倍增: {currentMultiplier}x
                </span>
              </div>
            )}

            {/* 购物车 */}
            {showCart && (
              <div className="relative">
                <button
                  onClick={onCartClick}
                  className="text-gray-600 hover:text-gray-800 p-2 rounded-lg hover:bg-gray-100 transition-colors"
                  aria-label="购物车"
                >
                  <i className="fas fa-shopping-cart text-lg"></i>
                </button>
                {cartCount > 0 && (
                  <div className="absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full w-5 h-5 flex items-center justify-center">
                    {cartCount}
                  </div>
                )}
              </div>
            )}

            {/* 通知 */}
            {showNotifications && (
              <div className="relative">
                <button
                  onClick={onNotificationClick}
                  className="text-gray-600 hover:text-gray-800 p-2 rounded-lg hover:bg-gray-100 transition-colors"
                  aria-label="通知"
                >
                  <i className="fas fa-bell text-lg"></i>
                </button>
                {notificationCount > 0 && (
                  <div className="absolute -top-1 -right-1 bg-red-500 text-white text-xs rounded-full w-5 h-5 flex items-center justify-center">
                    {notificationCount}
                  </div>
                )}
              </div>
            )}

            {/* 系统管理链接 */}
            {showUserMenu && (
              <Link 
                to="/admin" 
                className="text-gray-600 hover:text-gray-800 transition-colors duration-200 flex items-center"
              >
                <svg className="w-4 h-4 mr-2" fill="currentColor" viewBox="0 0 24 24">
                  <path d="M12 15.5A3.5 3.5 0 018.5 12A3.5 3.5 0 0112 8.5a3.5 3.5 0 013.5 3.5 3.5 3.5 0 01-3.5 3.5zM19.43 12.98c.04-.32.07-.64.07-.98s-.03-.66-.07-.98l2.11-1.65c.19-.15.24-.42.12-.64l-2-3.46c-.12-.22-.39-.3-.61-.22l-2.49 1c-.52-.4-1.08-.73-1.69-.98l-.38-2.65C14.46 2.18 14.25 2 14 2h-4c-.25 0-.46.18-.49.42l-.38 2.65c-.61.25-1.17.59-1.69.98l-2.49-1c-.23-.09-.49 0-.61.22l-2 3.46c-.13.22-.07.49.12.64l2.11 1.65c-.04.32-.07.65-.07.98s.03.66.07.98l-2.11 1.65c-.19.15-.24.42-.12.64l2 3.46c.12.22.39.3.61.22l2.49-1c.52.4 1.08.73 1.69.98l.38 2.65c.03.24.24.42.49.42h4c.25 0 .46-.18.49-.42l.38-2.65c.61-.25 1.17-.59 1.69-.98l2.49 1c.23.09.49 0 .61-.22l2-3.46c.12-.22.07-.49-.12-.64l-2.11-1.65z"/>
                </svg>
                系统管理
              </Link>
            )}

            {/* 用户头像 */}
            {showUserMenu && (
              <button
                onClick={onProfileClick}
                className="w-8 h-8 bg-gray-200 rounded-full flex items-center justify-center hover:bg-gray-300 transition-colors"
                aria-label="用户菜单"
              >
                <i className="fas fa-user text-gray-600"></i>
              </button>
            )}

            {/* 移动端菜单按钮 */}
            <div className="tablet:hidden">
              <button
                className="p-2 rounded-md text-gray-600 hover:text-gray-800 hover:bg-gray-100"
                onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
              >
                {mobileMenuOpen ? (
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
                  </svg>
                ) : (
                  <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M4 6h16M4 12h16M4 18h16" />
                  </svg>
                )}
              </button>
            </div>
          </div>
        </div>
        
        {/* 移动端菜单 */}
        {mobileMenuOpen && (
          <div className="tablet:hidden bg-white py-4 border-t border-gray-200">
            <div className="space-y-4">
              <MobileNavLink to="/" active={isActive('/')}>首页</MobileNavLink>
              <MobileNavLink to="/vouchers" active={isActive('/vouchers')}>提货券管理</MobileNavLink>
              <MobileNavLink to="/consumption" active={isActive('/consumption')}>消费与倍增</MobileNavLink>
              <MobileNavLink to="/b2c" active={isActive('/b2c')}>B2C电商平台</MobileNavLink>
              <MobileNavLink to="/evaluation" active={isActive('/evaluation')}>企业评估</MobileNavLink>
            </div>
          </div>
        )}
      </div>
    </nav>
  );
};

interface NavLinkProps {
  to: string;
  active?: boolean;
  children: React.ReactNode;
}

const NavLink: React.FC<NavLinkProps> = ({ to, active, children }) => {
  return (
    <Link
      to={to}
      className={cn(
        'font-medium transition-colors duration-200',
        active
          ? 'text-blue-600'
          : 'text-gray-600 hover:text-gray-800'
      )}
    >
      {children}
    </Link>
  );
};

const MobileNavLink: React.FC<NavLinkProps> = ({ to, active, children }) => {
  return (
    <Link
      to={to}
      className={cn(
        'block px-4 py-2 font-medium',
        active
          ? 'bg-blue-50 text-blue-600'
          : 'text-gray-600 hover:bg-gray-50 hover:text-gray-800'
      )}
    >
      {children}
    </Link>
  );
};

// 面包屑导航组件
export interface BreadcrumbItem {
  label: string;
  href?: string;
  current?: boolean;
}

export interface BreadcrumbsProps {
  items: BreadcrumbItem[];
  className?: string;
}

export const Breadcrumbs: React.FC<BreadcrumbsProps> = ({
  items,
  className
}) => {
  return (
    <nav className={cn('flex', className)} aria-label="面包屑导航">
      <ol className="flex items-center space-x-2">
        {items.map((item, index) => (
          <li key={index} className="flex items-center">
            {index > 0 && (
              <svg
                className="flex-shrink-0 h-5 w-5 text-gray-300 mx-2"
                fill="currentColor"
                viewBox="0 0 20 20"
                aria-hidden="true"
              >
                <path d="M5.555 17.776l8-16 .894.448-8 16-.894-.448z" />
              </svg>
            )}
            {item.href && !item.current ? (
              <a
                href={item.href}
                className="text-gray-500 hover:text-gray-700 text-sm font-medium transition-colors"
              >
                {item.label}
              </a>
            ) : (
              <span
                className={cn(
                  'text-sm font-medium',
                  item.current ? 'text-gray-900' : 'text-gray-500'
                )}
                aria-current={item.current ? 'page' : undefined}
              >
                {item.label}
              </span>
            )}
          </li>
        ))}
      </ol>
    </nav>
  );
};

// 侧边栏导航组件
export interface SidebarNavItem {
  id: string;
  label: string;
  icon: string;
  href?: string;
  badge?: string | number;
  active?: boolean;
  children?: SidebarNavItem[];
}

export interface SidebarNavigationProps {
  items: SidebarNavItem[];
  onItemClick?: (item: SidebarNavItem) => void;
  className?: string;
}

export const SidebarNavigation: React.FC<SidebarNavigationProps> = ({
  items,
  onItemClick,
  className
}) => {
  const [expandedItems, setExpandedItems] = useState<string[]>([]);

  const toggleExpanded = (itemId: string) => {
    setExpandedItems(prev => 
      prev.includes(itemId) 
        ? prev.filter(id => id !== itemId)
        : [...prev, itemId]
    );
  };

  const renderNavItem = (item: SidebarNavItem, level: number = 0) => {
    const hasChildren = item.children && item.children.length > 0;
    const isExpanded = expandedItems.includes(item.id);
    
    return (
      <div key={item.id}>
        <div
          className={cn(
            'flex items-center justify-between px-3 py-2 rounded-lg cursor-pointer transition-colors',
            level > 0 ? 'ml-4' : '',
            item.active ? 'bg-blue-50 text-blue-600' : 'text-gray-700 hover:bg-gray-100'
          )}
          onClick={() => {
            if (hasChildren) {
              toggleExpanded(item.id);
            }
            if (onItemClick) {
              onItemClick(item);
            }
          }}
        >
          <div className="flex items-center">
            <i className={`${item.icon} mr-3`}></i>
            <span className="font-medium">{item.label}</span>
            {item.badge && (
              <span className="ml-2 bg-gray-200 text-gray-600 text-xs px-2 py-1 rounded-full">
                {item.badge}
              </span>
            )}
          </div>
          {hasChildren && (
            <i className={`fas fa-chevron-${isExpanded ? 'up' : 'down'} text-gray-400`}></i>
          )}
        </div>
        
        {hasChildren && isExpanded && (
          <div className="mt-1">
            {item.children!.map(child => renderNavItem(child, level + 1))}
          </div>
        )}
      </div>
    );
  };

  return (
    <nav className={cn('space-y-1', className)}>
      {items.map(item => renderNavItem(item))}
    </nav>
  );
};

// 为了向后兼容，添加别名
export const Breadcrumb = Breadcrumbs;

export default Navigation; 