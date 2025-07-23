import React from 'react';
import { StickyNavigation } from '../components/layout/StickyNavigation';

const HomePage: React.FC = () => {
  return (
    <div className="min-h-screen bg-gray-50">
      {/* 固定导航栏 */}
      <StickyNavigation />

      {/* 主要内容 - 添加顶部间距避免被导航栏遮挡 */}
      <main style={{ paddingTop: '60px' }} className="content-width py-8">
        {/* Hero区域 */}
        <div className="hero-center mb-16 py-12">
          <div className="max-w-4xl mx-auto px-4 flex flex-col items-center">
            <h2 className="text-5xl font-bold hero-text mb-6 hero-title-center">
              构建共产主义商业平台
            </h2>
            <p className="text-xl text-gray-600 mb-8 max-w-3xl hero-description-center">
              基于共产主义经济原理，通过提货券机制和智能倍增系统，实现按需消费、按需生产，促进社会共同富裕
            </p>
            <div className="hero-button-center">
              <button className="btn-primary text-white px-8 py-4 rounded-xl font-medium text-lg">
                立即体验
              </button>
            </div>
          </div>
        </div>

        {/* 统计数据 */}
        <div className="stats-container mb-16">
          <div className="stats-item stats-animation">
            <div className="icon-gradient-blue stats-circle-icon">
              <i className="fas fa-users"></i>
            </div>
            <div className="text-3xl font-bold text-gray-800 mb-2">12,456</div>
            <div className="text-gray-600">注册用户</div>
          </div>
          
          <div className="stats-item stats-animation" style={{animationDelay: '0.5s'}}>
            <div className="icon-gradient-green stats-circle-icon">
              <i className="fas fa-building"></i>
            </div>
            <div className="text-3xl font-bold text-gray-800 mb-2">2,834</div>
            <div className="text-gray-600">合作企业</div>
          </div>
          
          <div className="stats-item stats-animation" style={{animationDelay: '1s'}}>
            <div className="icon-gradient-orange stats-circle-icon">
              <i className="fas fa-ticket-alt"></i>
            </div>
            <div className="text-3xl font-bold text-gray-800 mb-2">45,678</div>
            <div className="text-gray-600">提货券发放</div>
          </div>
          
          <div className="stats-item stats-animation" style={{animationDelay: '1.5s'}}>
            <div className="icon-gradient-purple stats-circle-icon">
              <i className="fas fa-exchange-alt"></i>
            </div>
            <div className="text-3xl font-bold text-gray-800 mb-2">156,789</div>
            <div className="text-gray-600">累计交易</div>
          </div>
        </div>

        {/* 核心功能模块 */}
        <div className="mb-16">
          <h3 className="font-bold text-gray-800 mb-12 core-modules-title">核心功能模块</h3>
          
          <div className="core-modules-grid">
            {/* 提货券管理 */}
            <div className="feature-card card-hover" onClick={() => window.location.href = '/vouchers'}>
              <div className="flex items-start">
                <div className="icon-gradient-blue w-16 h-16 rounded-2xl flex items-center justify-center mr-6">
                  <i className="fas fa-ticket-alt text-2xl text-white"></i>
                </div>
                <div className="flex-1">
                  <h4 className="text-xl font-bold text-gray-800 mb-3">提货券管理</h4>
                  <p className="text-gray-600 mb-4">企业发布提货券，消费者免费获取，实现按需生产和消费的良性循环</p>
                  <div className="flex items-center text-blue-600 font-medium">
                    <span>立即体验</span>
                    <i className="fas fa-arrow-right ml-2"></i>
                  </div>
                </div>
              </div>
            </div>

            {/* 消费与倍增 */}
            <div className="feature-card card-hover" onClick={() => window.location.href = '/consumption'}>
              <div className="flex items-start">
                <div className="icon-gradient-green w-16 h-16 rounded-2xl flex items-center justify-center mr-6">
                  <i className="fas fa-chart-line text-2xl text-white"></i>
                </div>
                <div className="flex-1">
                  <h4 className="text-xl font-bold text-gray-800 mb-3">消费与倍增</h4>
                  <p className="text-gray-600 mb-4">智能倍增系统，消费完成后生产者获得2-100倍奖励，促进经济增长</p>
                  <div className="flex items-center text-green-600 font-medium">
                    <span>查看详情</span>
                    <i className="fas fa-arrow-right ml-2"></i>
                  </div>
                </div>
              </div>
            </div>

            {/* B2C电商平台 */}
            <div className="feature-card card-hover" onClick={() => window.location.href = '/b2c'}>
              <div className="flex items-start">
                <div className="icon-gradient-pink w-16 h-16 rounded-2xl flex items-center justify-center mr-6">
                  <i className="fas fa-shopping-cart text-2xl text-white"></i>
                </div>
                <div className="flex-1">
                  <h4 className="text-xl font-bold text-gray-800 mb-3">B2C电商平台</h4>
                  <p className="text-gray-600 mb-4">免费获取提货券，享受优质商品服务，完整的购物体验流程</p>
                  <div className="flex items-center text-pink-600 font-medium">
                    <span>开始购物</span>
                    <i className="fas fa-arrow-right ml-2"></i>
                  </div>
                </div>
              </div>
            </div>

            {/* 企业评估 */}
            <div className="feature-card card-hover" onClick={() => window.location.href = '/evaluation'}>
              <div className="flex items-start">
                <div className="icon-gradient-purple w-16 h-16 rounded-2xl flex items-center justify-center mr-6">
                  <i className="fas fa-chart-bar text-2xl text-white"></i>
                </div>
                <div className="flex-1">
                  <h4 className="text-xl font-bold text-gray-800 mb-3">企业评估</h4>
                  <p className="text-gray-600 mb-4">6维度企业评估体系，促进企业健康发展，提升社会责任意识</p>
                  <div className="flex items-center text-purple-600 font-medium">
                    <span>了解评估</span>
                    <i className="fas fa-arrow-right ml-2"></i>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* 应用场景 */}
        <div className="mb-16">
          <h3 className="scenario-title">应用场景</h3>
          
          <div className="scenario-grid">
            {/* 第一行 */}
            <div className="scenario-card">
              <div className="scenario-icon-box icon-gradient-blue">
                <i className="fas fa-utensils"></i>
              </div>
              <h4>餐饮服务</h4>
              <p>餐厅发布餐饮提货券，用户免费用餐</p>
            </div>
            
            <div className="scenario-card">
              <div className="scenario-icon-box icon-gradient-green">
                <i className="fas fa-heartbeat"></i>
              </div>
              <h4>医疗健康</h4>
              <p>医疗机构提供免费健康服务</p>
            </div>
            
            <div className="scenario-card">
              <div className="scenario-icon-box icon-gradient-orange">
                <i className="fas fa-home"></i>
              </div>
              <h4>住房服务</h4>
              <p>房屋租赁和购买服务提货券</p>
            </div>
            
            <div className="scenario-card">
              <div className="scenario-icon-box icon-gradient-purple">
                <i className="fas fa-graduation-cap"></i>
              </div>
              <h4>教育培训</h4>
              <p>教育机构提供免费学习机会</p>
            </div>
            
            {/* 第二行 */}
            <div className="scenario-card">
              <div className="scenario-icon-box icon-gradient-pink">
                <i className="fas fa-car"></i>
              </div>
              <h4>交通出行</h4>
              <p>公共交通和共享出行服务</p>
            </div>
            
            <div className="scenario-card">
              <div className="scenario-icon-box icon-gradient-teal">
                <i className="fas fa-gamepad"></i>
              </div>
              <h4>娱乐休闲</h4>
              <p>影院、KTV、健身等娱乐服务</p>
            </div>
            
            <div className="scenario-card">
              <div className="scenario-icon-box icon-gradient-blue">
                <i className="fas fa-tools"></i>
              </div>
              <h4>生活服务</h4>
              <p>家政、维修、美容等生活服务</p>
            </div>
            
            <div className="scenario-card">
              <div className="scenario-icon-box icon-gradient-green">
                <i className="fas fa-laptop"></i>
              </div>
              <h4>数字产品</h4>
              <p>软件、应用、数字内容服务</p>
            </div>
          </div>
        </div>

        {/* 核心优势 */}
        <div className="advantages-section">
          <h3 className="advantages-title">核心优势</h3>
          
          <div className="advantages-grid">
            <div className="advantage-item">
              <div className="advantage-circle-icon icon-gradient-blue">
                <i className="fas fa-balance-scale"></i>
              </div>
              <h4 className="advantage-title">公平公正</h4>
              <p className="advantage-description">基于共产主义价值观，确保每个人都能获得公平的机会和待遇</p>
            </div>
            
            <div className="advantage-item">
              <div className="advantage-circle-icon icon-gradient-green">
                <i className="fas fa-recycle"></i>
              </div>
              <h4 className="advantage-title">良性循环</h4>
              <p className="advantage-description">消费推动生产，生产满足消费，形成健康的经济循环体系</p>
            </div>
            
            <div className="advantage-item">
              <div className="advantage-circle-icon icon-gradient-purple">
                <i className="fas fa-users"></i>
              </div>
              <h4 className="advantage-title">共同富裕</h4>
              <p className="advantage-description">通过智能倍增机制，实现资源合理分配，促进社会共同富裕</p>
            </div>
          </div>
        </div>

        {/* 技术架构预览 */}
        <div className="tech-section">
          <div className="tech-header">
            <h3 className="tech-title">先进技术架构</h3>
            <p className="tech-subtitle">基于Solana区块链的高性能DeFi协议</p>
          </div>
          
          <div className="tech-grid">
            <div className="tech-item">
              <div className="tech-icon-box">
                <i className="fas fa-cube"></i>
              </div>
              <h4 className="tech-item-title">区块链底层</h4>
              <p className="tech-item-description">Solana高性能区块链</p>
            </div>
            
            <div className="tech-item">
              <div className="tech-icon-box">
                <i className="fas fa-cogs"></i>
              </div>
              <h4 className="tech-item-title">智能合约</h4>
              <p className="tech-item-description">Rust + Anchor框架</p>
            </div>
            
            <div className="tech-item">
              <div className="tech-icon-box">
                <i className="fas fa-layer-group"></i>
              </div>
              <h4 className="tech-item-title">微服务架构</h4>
              <p className="tech-item-description">Docker + Kubernetes</p>
            </div>
            
            <div className="tech-item">
              <div className="tech-icon-box">
                <i className="fas fa-mobile-alt"></i>
              </div>
              <h4 className="tech-item-title">现代前端</h4>
              <p className="tech-item-description">React.js + TypeScript</p>
            </div>
          </div>
        </div>
      </main>

      {/* 页脚 - 背景全宽，内容80% */}
      <footer className="footer-section">
        <div className="footer-content">
          <div className="footer-grid">
            <div className="footer-brand-section">
              <div className="footer-brand">
                <div className="footer-logo">
                  <i className="fas fa-store"></i>
                </div>
                <span className="footer-brand-name">SoonShop</span>
              </div>
              <p className="footer-description">
                构建基于共产主义经济原理的现代商业平台，实现按需生产和消费的理想社会。
              </p>
            </div>
            
            <div>
              <h4 className="footer-section-title">核心功能</h4>
              <ul className="footer-links">
                <li><a href="/vouchers">提货券管理</a></li>
                <li><a href="/consumption">消费倍增</a></li>
                <li><a href="/b2c">B2C电商</a></li>
                <li><a href="/evaluation">企业评估</a></li>
              </ul>
            </div>
            
            <div>
              <h4 className="footer-section-title">应用场景</h4>
              <ul className="footer-links">
                <li><a href="/restaurant">餐饮服务</a></li>
                <li><a href="/healthcare">医疗健康</a></li>
                <li><a href="/housing">住房服务</a></li>
                <li><a href="/education">教育培训</a></li>
              </ul>
            </div>
            
            <div>
              <h4 className="footer-section-title">联系我们</h4>
              <div className="footer-contact-item">
                <i className="fas fa-envelope"></i>
                <span>admin@soonshop.com</span>
              </div>
              <div className="footer-contact-item">
                <i className="fas fa-phone"></i>
                <span>+86 400-888-0000</span>
              </div>
              <div className="footer-contact-item">
                <i className="fas fa-map-marker-alt"></i>
                <span>北京市朝阳区</span>
              </div>
            </div>
          </div>
          
          <div className="footer-divider">
            <p className="footer-copyright">&copy; 2024 SoonShop. 保留所有权利。基于共产主义经济原理构建。</p>
          </div>
        </div>
      </footer>
    </div>
  );
};

export default HomePage; 