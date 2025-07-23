# SoonShop Frontend - 完整目录结构

## 项目根目录
```
frontend-react/
├── public/                      # 静态资源目录
│   ├── favicon.ico             # 网站图标
│   ├── logo.png                # 应用Logo
│   ├── manifest.json           # PWA清单文件
│   └── robots.txt              # 搜索引擎爬虫规则
├── src/                        # 源代码目录
│   ├── components/             # 可复用组件
│   ├── features/               # 业务功能模块
│   ├── hooks/                  # 自定义React Hooks
│   ├── services/               # API服务
│   ├── stores/                 # 状态管理
│   ├── utils/                  # 工具函数
│   ├── types/                  # TypeScript类型定义
│   ├── constants/              # 常量定义
│   ├── config/                 # 配置文件
│   ├── assets/                 # 静态资源
│   ├── providers/              # React Context提供者
│   ├── App.tsx                 # 主应用组件
│   ├── main.tsx                # 应用入口文件
│   └── index.css               # 全局样式
├── .env                        # 环境变量
├── .env.example                # 环境变量示例
├── .gitignore                  # Git忽略文件
├── .eslintrc.js                # ESLint配置
├── .prettierrc                 # Prettier配置
├── package.json                # 项目配置
├── tsconfig.json               # TypeScript配置
├── tsconfig.node.json          # Node.js TypeScript配置
├── tailwind.config.js          # Tailwind CSS配置
├── vite.config.ts              # Vite配置
├── README.md                   # 项目说明文档
└── DIRECTORY_STRUCTURE.md      # 目录结构说明（本文件）
```

## 详细目录结构

### 1. 组件目录 (src/components/)
```
src/components/
├── ui/                         # 基础UI组件
│   ├── Button.tsx             # 按钮组件
│   ├── Card.tsx               # 卡片组件
│   ├── Input.tsx              # 输入框组件
│   ├── Modal.tsx              # 模态框组件
│   ├── Loading.tsx            # 加载组件
│   ├── Toaster.tsx            # 通知组件
│   ├── Dropdown.tsx           # 下拉菜单组件
│   ├── Tabs.tsx               # 标签页组件
│   ├── Table.tsx              # 表格组件
│   ├── Form.tsx               # 表单组件
│   └── index.ts               # 组件导出
├── layout/                     # 布局组件
│   ├── MainLayout.tsx         # 主布局
│   ├── Header.tsx             # 头部组件
│   ├── Sidebar.tsx            # 侧边栏组件
│   ├── Footer.tsx             # 底部组件
│   └── index.ts               # 组件导出
├── common/                     # 通用组件
│   ├── ErrorBoundary.tsx      # 错误边界组件
│   ├── LoadingSpinner.tsx     # 加载动画组件
│   ├── EmptyState.tsx         # 空状态组件
│   ├── Pagination.tsx         # 分页组件
│   ├── SearchBar.tsx          # 搜索栏组件
│   └── index.ts               # 组件导出
├── auth/                       # 认证组件
│   ├── ProtectedRoute.tsx     # 受保护路由组件
│   ├── WalletConnector.tsx    # 钱包连接组件
│   └── index.ts               # 组件导出
└── index.ts                   # 总组件导出
```

### 2. 功能模块目录 (src/features/)
```
src/features/
├── home/                       # 首页模块
│   ├── pages/
│   │   └── HomePage.tsx       # 首页
│   ├── components/
│   │   ├── HeroSection.tsx    # 英雄区域
│   │   ├── FeaturesSection.tsx # 特性介绍
│   │   └── StatsSection.tsx   # 统计数据
│   └── index.ts
├── auth/                       # 认证模块
│   ├── pages/
│   │   ├── LoginPage.tsx      # 登录页
│   │   └── UserProfilePage.tsx # 用户资料页
│   ├── components/
│   │   ├── LoginForm.tsx      # 登录表单
│   │   └── ProfileForm.tsx    # 资料表单
│   └── index.ts
├── voucher/                    # 提货券模块
│   ├── pages/
│   │   ├── VoucherListPage.tsx # 提货券列表
│   │   └── VoucherDetailPage.tsx # 提货券详情
│   ├── components/
│   │   ├── VoucherCard.tsx    # 提货券卡片
│   │   └── VoucherForm.tsx    # 提货券表单
│   └── index.ts
├── b2c/                        # B2C电商模块
│   ├── pages/
│   │   └── B2CShoppingPage.tsx # B2C购物页
│   ├── components/
│   │   ├── ProductCard.tsx    # 商品卡片
│   │   └── ShoppingCart.tsx   # 购物车
│   └── index.ts
├── consumption/                # 消费记录模块
│   ├── pages/
│   │   └── ConsumptionRewardsPage.tsx # 消费奖励页
│   ├── components/
│   │   ├── ConsumptionCard.tsx # 消费卡片
│   │   └── RewardChart.tsx    # 奖励图表
│   └── index.ts
├── evaluation/                 # 企业评估模块
│   ├── pages/
│   │   └── EnterpriseEvaluationPage.tsx # 企业评估页
│   ├── components/
│   │   ├── EvaluationForm.tsx # 评估表单
│   │   └── ScoreCard.tsx      # 评分卡片
│   └── index.ts
├── b2b/                        # B2B市场模块
│   ├── pages/
│   │   └── B2BMarketplacePage.tsx # B2B市场页
│   ├── components/
│   │   ├── B2BProductCard.tsx # B2B商品卡片
│   │   └── OrderForm.tsx      # 订单表单
│   └── index.ts
├── restaurant/                 # 餐饮服务模块
│   ├── pages/
│   │   └── RestaurantServicePage.tsx # 餐饮服务页
│   ├── components/
│   │   ├── RestaurantCard.tsx # 餐厅卡片
│   │   └── ReservationForm.tsx # 预订表单
│   └── index.ts
├── healthcare/                 # 医疗服务模块
│   ├── pages/
│   │   └── HealthcareServicePage.tsx # 医疗服务页
│   ├── components/
│   │   ├── HealthcareCard.tsx # 医疗卡片
│   │   └── AppointmentForm.tsx # 预约表单
│   └── index.ts
├── housing/                    # 住房服务模块
│   ├── pages/
│   │   └── HousingServicePage.tsx # 住房服务页
│   ├── components/
│   │   ├── HousingCard.tsx    # 住房卡片
│   │   └── RentalForm.tsx     # 租赁表单
│   └── index.ts
├── analytics/                  # 数据分析模块
│   ├── pages/
│   │   └── AnalyticsPage.tsx  # 数据分析页
│   ├── components/
│   │   ├── Dashboard.tsx      # 仪表板
│   │   └── ChartCard.tsx      # 图表卡片
│   └── index.ts
└── admin/                      # 系统管理模块
    ├── pages/
    │   └── SystemManagementPage.tsx # 系统管理页
    ├── components/
    │   ├── UserManagement.tsx # 用户管理
    │   └── SystemSettings.tsx # 系统设置
    └── index.ts
```

### 3. 其他核心目录
```
src/hooks/                      # 自定义Hooks
├── useAuth.ts                 # 认证Hook
├── useVoucher.ts              # 提货券Hook
├── useWallet.ts               # 钱包Hook
├── useLocalStorage.ts         # 本地存储Hook
├── useDebounce.ts             # 防抖Hook
├── useApi.ts                  # API调用Hook
└── index.ts                   # Hooks导出

src/services/                   # API服务
├── api.ts                     # API基础配置
├── auth.ts                    # 认证服务
├── voucher.ts                 # 提货券服务
├── consumption.ts             # 消费服务
├── evaluation.ts              # 评估服务
├── b2c.ts                     # B2C服务
├── b2b.ts                     # B2B服务
├── restaurant.ts              # 餐饮服务
├── healthcare.ts              # 医疗服务
├── housing.ts                 # 住房服务
├── analytics.ts               # 数据分析服务
├── admin.ts                   # 管理服务
└── index.ts                   # 服务导出

src/stores/                     # 状态管理
├── authStore.ts               # 认证状态
├── voucherStore.ts            # 提货券状态
├── cartStore.ts               # 购物车状态
├── notificationStore.ts       # 通知状态
├── themeStore.ts              # 主题状态
└── index.ts                   # Store导出

src/providers/                  # Context提供者
├── WalletProvider.tsx         # 钱包提供者
├── ThemeProvider.tsx          # 主题提供者
├── QueryProvider.tsx          # 查询提供者
└── index.ts                   # 提供者导出

src/assets/                     # 静态资源
├── images/                    # 图片资源
│   ├── logo.png
│   ├── hero-bg.jpg
│   └── placeholder.png
├── icons/                     # 图标资源
│   ├── communist-star.svg
│   ├── voucher-icon.svg
│   └── wallet-icon.svg
├── fonts/                     # 字体资源
│   └── inter.woff2
└── data/                      # 静态数据
    └── mock-data.json
```

## 创建指南

要创建完整的目录结构，请按以下步骤操作：

1. **创建目录结构**：
```bash
# 进入项目目录
cd frontend-react

# 创建所有必需的目录
mkdir -p src/components/ui
mkdir -p src/components/layout
mkdir -p src/components/common
mkdir -p src/components/auth
mkdir -p src/features/home/pages
mkdir -p src/features/home/components
mkdir -p src/features/auth/pages
mkdir -p src/features/auth/components
mkdir -p src/features/voucher/pages
mkdir -p src/features/voucher/components
mkdir -p src/features/b2c/pages
mkdir -p src/features/b2c/components
mkdir -p src/features/consumption/pages
mkdir -p src/features/consumption/components
mkdir -p src/features/evaluation/pages
mkdir -p src/features/evaluation/components
mkdir -p src/features/b2b/pages
mkdir -p src/features/b2b/components
mkdir -p src/features/restaurant/pages
mkdir -p src/features/restaurant/components
mkdir -p src/features/healthcare/pages
mkdir -p src/features/healthcare/components
mkdir -p src/features/housing/pages
mkdir -p src/features/housing/components
mkdir -p src/features/analytics/pages
mkdir -p src/features/analytics/components
mkdir -p src/features/admin/pages
mkdir -p src/features/admin/components
mkdir -p src/hooks
mkdir -p src/services
mkdir -p src/stores
mkdir -p src/providers
mkdir -p src/assets/images
mkdir -p src/assets/icons
mkdir -p src/assets/fonts
mkdir -p src/assets/data
mkdir -p public
```

2. **创建占位符文件**：
每个目录都应该有相应的 `index.ts` 文件用于导出。

3. **配置文件**：
已经创建了基础的配置文件，包括：
- `package.json` - 项目依赖和脚本
- `tsconfig.json` - TypeScript配置
- `tailwind.config.js` - Tailwind CSS配置
- `vite.config.ts` - Vite构建配置

## 下一步

1. 安装依赖：`npm install`
2. 创建组件和页面
3. 配置路由
4. 连接后端API
5. 添加测试
6. 优化性能
7. 部署上线

这个目录结构遵循了现代React应用的最佳实践，支持：
- 模块化开发
- 代码重用
- 类型安全
- 性能优化
- 可维护性
- 可扩展性 