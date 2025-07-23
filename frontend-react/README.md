# SoonShop Frontend - 共产主义商业平台前端

## 项目介绍

SoonShop 是一个基于共产主义经济原理的现代商业平台，旨在实现按需生产、按需消费，促进社会共同富裕。

### 核心理念

- **人民至上** - 以人民利益为出发点
- **按需分配** - 根据实际需求进行资源分配
- **共同富裕** - 通过倍增系统实现财富共享
- **公平公正** - 透明的评估和分配机制

## 技术栈

### 前端框架
- **React 18** - 现代化React开发
- **TypeScript** - 类型安全的JavaScript
- **Vite** - 快速的构建工具
- **React Router** - 前端路由管理

### 状态管理
- **Zustand** - 轻量级状态管理
- **React Query** - 服务端状态管理

### UI组件
- **Ant Design** - 企业级UI组件库
- **Tailwind CSS** - 原子化CSS框架
- **Framer Motion** - 动画库
- **Lucide React** - 图标库

### 区块链集成
- **Solana Web3.js** - Solana区块链交互
- **Wallet Adapter** - 钱包连接适配器

### 开发工具
- **ESLint** - 代码质量检查
- **Prettier** - 代码格式化
- **Husky** - Git钩子管理
- **Storybook** - 组件开发和文档

## 项目结构

```
frontend-react/
├── public/                     # 静态资源
│   ├── favicon.ico
│   ├── logo.png
│   └── manifest.json
├── src/                        # 源代码
│   ├── components/             # 可复用组件
│   │   ├── ui/                # 基础UI组件
│   │   ├── layout/            # 布局组件
│   │   ├── common/            # 通用组件
│   │   └── auth/              # 认证组件
│   ├── features/              # 业务功能模块
│   │   ├── home/              # 首页
│   │   ├── auth/              # 认证模块
│   │   ├── voucher/           # 提货券模块
│   │   ├── b2c/               # B2C电商
│   │   ├── consumption/       # 消费记录
│   │   ├── evaluation/        # 企业评估
│   │   ├── b2b/               # B2B市场
│   │   ├── restaurant/        # 餐饮服务
│   │   ├── healthcare/        # 医疗服务
│   │   ├── housing/           # 住房服务
│   │   ├── analytics/         # 数据分析
│   │   └── admin/             # 系统管理
│   ├── hooks/                 # 自定义React Hooks
│   ├── services/              # API服务
│   ├── stores/                # 状态管理
│   ├── utils/                 # 工具函数
│   ├── types/                 # TypeScript类型定义
│   ├── constants/             # 常量定义
│   ├── config/                # 配置文件
│   ├── assets/                # 静态资源
│   ├── providers/             # React Context提供者
│   ├── App.tsx               # 主应用组件
│   ├── main.tsx              # 应用入口
│   └── index.css             # 全局样式
├── package.json              # 项目配置
├── tsconfig.json             # TypeScript配置
├── tailwind.config.js        # Tailwind CSS配置
├── vite.config.ts            # Vite配置
└── README.md                 # 项目文档
```

## 功能模块

### 1. 认证系统
- 钱包连接登录
- 用户角色管理（消费者、生产者、评估员、管理员）
- 权限控制

### 2. 提货券系统
- 提货券发行和管理
- 提货券消费
- 库存管理
- 生命周期管理

### 3. B2C电商
- 商品展示
- 购物车管理
- 订单处理
- 支付集成

### 4. 消费与奖励
- 消费记录
- 倍增系统
- 奖励分配
- 满意度评价

### 5. 企业评估
- 企业评估表单
- 评估结果展示
- 倍增器计算
- 历史记录

### 6. B2B市场
- 企业间交易
- 供应链管理
- 批量采购

### 7. 场景服务
- 餐饮服务预订
- 医疗服务预约
- 住房服务管理

### 8. 数据分析
- 实时数据仪表板
- 报表生成
- 趋势分析

## 设计原则

### 1. 现代iOS风格
- 简洁优雅的界面设计
- 流畅的交互动画
- 响应式布局
- 深色模式支持

### 2. 用户体验
- 直观的导航结构
- 快速的页面加载
- 友好的错误处理
- 无障碍访问支持

### 3. 性能优化
- 代码分割和懒加载
- 缓存策略
- 图片优化
- 首屏加载优化

### 4. 安全性
- 输入验证
- XSS防护
- CSRF保护
- 敏感数据加密

## 开发指南

### 环境要求
- Node.js >= 18.0.0
- npm >= 8.0.0 或 yarn >= 1.22.0

### 开发环境搭建

1. 克隆项目
```bash
git clone <repository-url>
cd frontend-react
```

2. 安装依赖
```bash
npm install
# 或
yarn install
```

3. 启动开发服务器
```bash
npm run dev
# 或
yarn dev
```

4. 构建生产版本
```bash
npm run build
# 或
yarn build
```

### 代码规范

1. 使用TypeScript进行类型检查
2. 遵循ESLint和Prettier配置
3. 使用语义化的组件命名
4. 编写清晰的注释和文档

### 测试

```bash
# 运行单元测试
npm run test

# 运行端到端测试
npm run test:e2e

# 生成测试覆盖率报告
npm run test:coverage
```

### 部署

```bash
# 构建生产版本
npm run build

# 预览生产版本
npm run preview
```

## 贡献指南

1. Fork项目
2. 创建功能分支
3. 提交更改
4. 推送到分支
5. 创建Pull Request

## 许可证

本项目采用MIT许可证。

## 联系方式

- 项目主页：https://soonshop.com
- 问题反馈：https://github.com/soonshop/soonshop/issues
- 邮箱：contact@soonshop.com

---

**共产主义理想，指引我们前进！** 