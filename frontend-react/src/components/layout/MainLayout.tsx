import React from 'react';
import Footer from './Footer';
import Navigation from './Navigation';

interface MainLayoutProps {
  children: React.ReactNode;
  showFooter?: boolean;
  showNavigation?: boolean;
}

export const MainLayout: React.FC<MainLayoutProps> = ({ 
  children, 
  showFooter = true,
  showNavigation = true
}) => {
  return (
    <div className="min-h-screen bg-gray-50 flex flex-col">
      {/* 导航栏 */}
      {showNavigation && <Navigation />}

      {/* 主要内容区域 */}
      <main className="flex-grow">
        {children}
      </main>

      {/* 页脚 */}
      {showFooter && <Footer />}
    </div>
  );
};

export default MainLayout; 