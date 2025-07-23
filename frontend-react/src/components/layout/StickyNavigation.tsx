import React from 'react';
import { Link } from 'react-router-dom';

// 新的固定导航栏组件 - 按照原型设计
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
      justifyContent: 'center',
      padding: '0'
    }}>
      {/* 内容区域 - 与原型HTML一致的max-w-7xl宽度 */}
      <div style={{ 
        maxWidth: '80rem', /* 1280px - 与原型HTML的max-w-7xl一致 */
        width: '100%',
        display: 'flex', 
        alignItems: 'center', 
        justifyContent: 'space-between',
        padding: '0 20px'
      }}>
        {/* 左侧品牌区域 */}
        <div style={{ display: 'flex', alignItems: 'center' }}>
          <Link to="/" style={{ display: 'flex', alignItems: 'center', textDecoration: 'none', color: 'inherit' }}>
            {/* 渐变图标 */}
            <div style={{ 
              width: '40px', 
              height: '40px', 
              background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)', 
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
            background: '#F3F4F6', /* 浅灰色背景 */
            border: 'none', 
            color: '#6B7280',
            cursor: 'pointer',
            padding: '8px',
            borderRadius: '50%', /* 圆形背景 */
            width: '40px',
            height: '40px',
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            transition: 'background-color 0.2s ease'
          }} 
          onMouseEnter={(e) => e.currentTarget.style.backgroundColor = '#E5E7EB'}
          onMouseLeave={(e) => e.currentTarget.style.backgroundColor = '#F3F4F6'}
          aria-label="用户菜单">
            <i className="fas fa-user"></i>
          </button>
        </div>
      </div>
    </nav>
  );
}; 