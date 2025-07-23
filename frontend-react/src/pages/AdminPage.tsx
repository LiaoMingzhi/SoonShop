import React, { useState, useEffect } from 'react';
import { MainLayout } from '../components/layout/MainLayout';
import { StatsCard } from '../components/business/StatsCard';
import { Card } from '../components/ui/Card';
import { Button } from '../components/ui/Button';
import Modal from '../components/ui/Modal';
import { Input } from '../components/ui/Input';
import { Badge } from '../components/ui/Badge';
import { useAuthStore } from '../stores/authStore';
import { ProtectedRoute } from '../components/auth/ProtectedRoute';

interface User {
  id: string;
  walletAddress: string;
  userType: 'consumer' | 'producer' | 'evaluator' | 'admin';
  name: string;
  email: string;
  status: 'active' | 'suspended' | 'pending';
  createdAt: string;
  lastLoginAt?: string;
}

interface SystemConfig {
  id: string;
  key: string;
  value: string;
  description: string;
  category: string;
  updatedAt: string;
}

interface AdminStats {
  totalUsers: number;
  activeUsers: number;
  totalVouchers: number;
  totalConsumptions: number;
  systemHealth: 'good' | 'warning' | 'error';
  userGrowth: number;
  voucherGrowth: number;
}

const AdminPage: React.FC = () => {
  const { user } = useAuthStore();
  const [activeTab, setActiveTab] = useState<'dashboard' | 'users' | 'config' | 'logs'>('dashboard');
  const [stats, setStats] = useState<AdminStats>({
    totalUsers: 1250,
    activeUsers: 980,
    totalVouchers: 3200,
    totalConsumptions: 8500,
    systemHealth: 'good',
    userGrowth: 12.5,
    voucherGrowth: 8.3,
  });
  const [users, setUsers] = useState<User[]>([]);
  const [configs, setConfigs] = useState<SystemConfig[]>([]);
  const [loading, setLoading] = useState(false);
  const [selectedUser, setSelectedUser] = useState<User | null>(null);
  const [showUserModal, setShowUserModal] = useState(false);
  const [showConfigModal, setShowConfigModal] = useState(false);

  // 模拟数据加载
  useEffect(() => {
    loadMockData();
  }, []);

  const loadMockData = () => {
    // 模拟用户数据
    const mockUsers: User[] = [
      {
        id: '1',
        walletAddress: '0x1234...5678',
        userType: 'consumer',
        name: '张三',
        email: 'zhangsan@example.com',
        status: 'active',
        createdAt: '2024-01-15',
        lastLoginAt: '2024-02-10',
      },
      {
        id: '2',
        walletAddress: '0x2345...6789',
        userType: 'producer',
        name: '李四生产合作社',
        email: 'lisi@example.com',
        status: 'active',
        createdAt: '2024-01-10',
        lastLoginAt: '2024-02-09',
      },
      {
        id: '3',
        walletAddress: '0x3456...7890',
        userType: 'evaluator',
        name: '王五评估员',
        email: 'wangwu@example.com',
        status: 'pending',
        createdAt: '2024-02-01',
      },
    ];

    // 模拟系统配置
    const mockConfigs: SystemConfig[] = [
      {
        id: '1',
        key: 'max_voucher_per_user',
        value: '50',
        description: '每个用户最大提货券数量',
        category: '业务配置',
        updatedAt: '2024-02-01',
      },
      {
        id: '2',
        key: 'default_multiplier',
        value: '1.2',
        description: '默认倍增系数',
        category: '经济配置',
        updatedAt: '2024-01-30',
      },
      {
        id: '3',
        key: 'session_timeout',
        value: '3600',
        description: '会话超时时间（秒）',
        category: '系统配置',
        updatedAt: '2024-01-25',
      },
    ];

    setUsers(mockUsers);
    setConfigs(mockConfigs);
  };

  const handleUserAction = (action: 'suspend' | 'activate' | 'delete', userId: string) => {
    setUsers(prev => 
      prev.map(user => 
        user.id === userId 
          ? { ...user, status: action === 'suspend' ? 'suspended' : 'active' }
          : user
      )
    );
  };

  const renderDashboard = () => (
    <div className="space-y-6">
      {/* 统计卡片 */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <StatsCard
          title="总用户数"
          value={stats.totalUsers.toLocaleString()}
          icon="users"
          gradient="blue"
          trend={{
            direction: 'up',
            value: `+${stats.userGrowth}%`,
            period: '本月'
          }}
        />
        <StatsCard
          title="活跃用户"
          value={stats.activeUsers.toLocaleString()}
          icon="user-check"
          gradient="green"
          subtitle={`${((stats.activeUsers / stats.totalUsers) * 100).toFixed(1)}% 活跃率`}
        />
        <StatsCard
          title="提货券总数"
          value={stats.totalVouchers.toLocaleString()}
          icon="ticket"
          gradient="orange"
          trend={{
            direction: 'up',
            value: `+${stats.voucherGrowth}%`,
            period: '本月'
          }}
        />
        <StatsCard
          title="总消费次数"
          value={stats.totalConsumptions.toLocaleString()}
          icon="shopping-cart"
          gradient="purple"
        />
      </div>

      {/* 系统健康状态 */}
      <Card title="系统健康状态" className="p-6">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <div className={`w-4 h-4 rounded-full ${
              stats.systemHealth === 'good' ? 'bg-green-500' :
              stats.systemHealth === 'warning' ? 'bg-yellow-500' : 'bg-red-500'
            }`}></div>
            <span className="text-lg font-medium">
              {stats.systemHealth === 'good' ? '系统运行正常' :
               stats.systemHealth === 'warning' ? '系统有轻微问题' : '系统存在严重问题'}
            </span>
          </div>
          <Badge variant={stats.systemHealth === 'good' ? 'success' : 'warning'}>
            {stats.systemHealth === 'good' ? '健康' : '需要关注'}
          </Badge>
        </div>
        
        <div className="mt-6 grid grid-cols-2 md:grid-cols-4 gap-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-green-600">99.9%</div>
            <div className="text-sm text-gray-600">系统可用性</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-blue-600">45ms</div>
            <div className="text-sm text-gray-600">平均响应时间</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-orange-600">2.1GB</div>
            <div className="text-sm text-gray-600">内存使用</div>
          </div>
          <div className="text-center">
            <div className="text-2xl font-bold text-purple-600">156</div>
            <div className="text-sm text-gray-600">活跃连接</div>
          </div>
        </div>
      </Card>

      {/* 快速操作 */}
      <Card title="快速操作" className="p-6">
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <Button variant="outline" className="h-20 flex flex-col">
            <i className="fas fa-user-plus text-xl mb-2"></i>
            添加用户
          </Button>
          <Button variant="outline" className="h-20 flex flex-col">
            <i className="fas fa-cog text-xl mb-2"></i>
            系统配置
          </Button>
          <Button variant="outline" className="h-20 flex flex-col">
            <i className="fas fa-download text-xl mb-2"></i>
            导出数据
          </Button>
          <Button variant="outline" className="h-20 flex flex-col">
            <i className="fas fa-shield-alt text-xl mb-2"></i>
            安全设置
          </Button>
        </div>
      </Card>
    </div>
  );

  const renderUsers = () => (
    <div className="space-y-6">
      {/* 用户统计 */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card className="p-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-blue-600">{users.length}</div>
            <div className="text-sm text-gray-600">总用户</div>
          </div>
        </Card>
        <Card className="p-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-green-600">
              {users.filter(u => u.status === 'active').length}
            </div>
            <div className="text-sm text-gray-600">活跃用户</div>
          </div>
        </Card>
        <Card className="p-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-orange-600">
              {users.filter(u => u.status === 'pending').length}
            </div>
            <div className="text-sm text-gray-600">待审核</div>
          </div>
        </Card>
        <Card className="p-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-red-600">
              {users.filter(u => u.status === 'suspended').length}
            </div>
            <div className="text-sm text-gray-600">已停用</div>
          </div>
        </Card>
      </div>

      {/* 用户列表 */}
      <Card title="用户管理" className="p-6">
        <div className="mb-4 flex justify-between items-center">
          <div className="flex space-x-2">
            <Input 
              placeholder="搜索用户..." 
              className="w-64"
            />
            <Button variant="outline">
              <i className="fas fa-search mr-2"></i>
              搜索
            </Button>
          </div>
          <Button onClick={() => setShowUserModal(true)}>
            <i className="fas fa-plus mr-2"></i>
            添加用户
          </Button>
        </div>

        <div className="overflow-x-auto">
          <table className="w-full">
            <thead>
              <tr className="border-b">
                <th className="text-left py-3 px-4">用户</th>
                <th className="text-left py-3 px-4">类型</th>
                <th className="text-left py-3 px-4">状态</th>
                <th className="text-left py-3 px-4">注册时间</th>
                <th className="text-left py-3 px-4">最后登录</th>
                <th className="text-left py-3 px-4">操作</th>
              </tr>
            </thead>
            <tbody>
              {users.map(user => (
                <tr key={user.id} className="border-b hover:bg-gray-50">
                  <td className="py-3 px-4">
                    <div>
                      <div className="font-medium">{user.name}</div>
                      <div className="text-sm text-gray-600">{user.email}</div>
                      <div className="text-xs text-gray-500">{user.walletAddress}</div>
                    </div>
                  </td>
                  <td className="py-3 px-4">
                    <Badge variant={
                      user.userType === 'admin' ? 'error' :
                      user.userType === 'evaluator' ? 'warning' :
                      user.userType === 'producer' ? 'success' : 'default'
                    }>
                      {user.userType === 'consumer' ? '消费者' :
                       user.userType === 'producer' ? '生产者' :
                       user.userType === 'evaluator' ? '评估员' : '管理员'}
                    </Badge>
                  </td>
                  <td className="py-3 px-4">
                    <Badge variant={
                      user.status === 'active' ? 'success' :
                      user.status === 'pending' ? 'warning' : 'error'
                    }>
                      {user.status === 'active' ? '活跃' :
                       user.status === 'pending' ? '待审核' : '已停用'}
                    </Badge>
                  </td>
                  <td className="py-3 px-4 text-sm">
                    {new Date(user.createdAt).toLocaleDateString()}
                  </td>
                  <td className="py-3 px-4 text-sm">
                    {user.lastLoginAt ? new Date(user.lastLoginAt).toLocaleDateString() : '从未登录'}
                  </td>
                  <td className="py-3 px-4">
                    <div className="flex space-x-2">
                      <Button 
                        size="sm" 
                        variant="outline"
                        onClick={() => {
                          setSelectedUser(user);
                          setShowUserModal(true);
                        }}
                      >
                        编辑
                      </Button>
                      {user.status === 'active' ? (
                        <Button 
                          size="sm" 
                          variant="outline"
                          onClick={() => handleUserAction('suspend', user.id)}
                        >
                          停用
                        </Button>
                      ) : (
                        <Button 
                          size="sm" 
                          variant="outline"
                          onClick={() => handleUserAction('activate', user.id)}
                        >
                          激活
                        </Button>
                      )}
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </Card>
    </div>
  );

  const renderConfig = () => (
    <div className="space-y-6">
      <Card title="系统配置" className="p-6">
        <div className="mb-4 flex justify-between items-center">
          <div className="flex space-x-2">
            <Input 
              placeholder="搜索配置项..." 
              className="w-64"
            />
          </div>
          <Button onClick={() => setShowConfigModal(true)}>
            <i className="fas fa-plus mr-2"></i>
            添加配置
          </Button>
        </div>

        <div className="space-y-4">
          {configs.map(config => (
            <Card key={config.id} className="p-4">
              <div className="flex items-center justify-between">
                <div className="flex-1">
                  <div className="flex items-center space-x-4">
                    <div>
                      <div className="font-medium">{config.key}</div>
                      <div className="text-sm text-gray-600">{config.description}</div>
                    </div>
                    <Badge variant="default">{config.category}</Badge>
                  </div>
                </div>
                <div className="flex items-center space-x-4">
                  <div className="text-right">
                    <div className="font-mono text-lg">{config.value}</div>
                    <div className="text-xs text-gray-500">
                      更新于 {new Date(config.updatedAt).toLocaleDateString()}
                    </div>
                  </div>
                  <Button size="sm" variant="outline">
                    编辑
                  </Button>
                </div>
              </div>
            </Card>
          ))}
        </div>
      </Card>
    </div>
  );

  const renderLogs = () => (
    <div className="space-y-6">
      <Card title="系统日志" className="p-6">
        <div className="mb-4 flex space-x-2">
          <select className="px-3 py-2 border rounded-md">
            <option>所有级别</option>
            <option>错误</option>
            <option>警告</option>
            <option>信息</option>
          </select>
          <Input 
            placeholder="搜索日志..." 
            className="flex-1"
          />
          <Button variant="outline">
            <i className="fas fa-download mr-2"></i>
            导出
          </Button>
        </div>

        <div className="space-y-2 max-h-96 overflow-y-auto">
          {[...Array(20)].map((_, index) => (
            <div key={index} className="text-sm font-mono bg-gray-100 p-3 rounded">
              <span className="text-gray-500">2024-02-10 14:30:{30 + index}</span>
              <span className="text-blue-600 ml-2">[INFO]</span>
              <span className="ml-2">用户 张三 成功登录系统</span>
            </div>
          ))}
        </div>
      </Card>
    </div>
  );

  return (
    <ProtectedRoute requiredRole="admin">
      <MainLayout>
        <div className="min-h-screen bg-gray-50">
          {/* 页面头部 */}
          <div className="bg-white shadow-sm border-b border-gray-200">
            <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
              <div className="flex items-center justify-between">
                <div>
                  <h1 className="text-2xl font-bold text-gray-800 flex items-center">
                    <i className="fas fa-cogs text-purple-600 mr-3"></i>
                    系统管理
                  </h1>
                  <p className="text-gray-600 mt-1">平台系统管理和配置</p>
                </div>
                <div className="flex space-x-2">
                  <Button variant="outline">
                    <i className="fas fa-download mr-2"></i>
                    导出报告
                  </Button>
                  <Button>
                    <i className="fas fa-sync mr-2"></i>
                    刷新数据
                  </Button>
                </div>
              </div>

              {/* 标签导航 */}
              <div className="mt-6">
                <nav className="flex space-x-8">
                  {[
                    { key: 'dashboard', label: '仪表板', icon: 'tachometer-alt' },
                    { key: 'users', label: '用户管理', icon: 'users' },
                    { key: 'config', label: '系统配置', icon: 'cog' },
                    { key: 'logs', label: '系统日志', icon: 'list-alt' },
                  ].map(tab => (
                    <button
                      key={tab.key}
                      onClick={() => setActiveTab(tab.key as any)}
                      className={`flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors ${
                        activeTab === tab.key
                          ? 'bg-purple-100 text-purple-700'
                          : 'text-gray-500 hover:text-gray-700'
                      }`}
                    >
                      <i className={`fas fa-${tab.icon} mr-2`}></i>
                      {tab.label}
                    </button>
                  ))}
                </nav>
              </div>
            </div>
          </div>

          {/* 页面内容 */}
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            {activeTab === 'dashboard' && renderDashboard()}
            {activeTab === 'users' && renderUsers()}
            {activeTab === 'config' && renderConfig()}
            {activeTab === 'logs' && renderLogs()}
          </div>
        </div>

        {/* 用户编辑模态框 */}
        <Modal
          visible={showUserModal}
          title={selectedUser ? '编辑用户' : '添加用户'}
          onClose={() => {
            setShowUserModal(false);
            setSelectedUser(null);
          }}
        >
          <div className="space-y-4">
            <Input label="用户名" defaultValue={selectedUser?.name} />
            <Input label="邮箱" defaultValue={selectedUser?.email} />
            <select className="w-full px-3 py-2 border rounded-md">
              <option value="consumer">消费者</option>
              <option value="producer">生产者</option>
              <option value="evaluator">评估员</option>
              <option value="admin">管理员</option>
            </select>
            <div className="flex justify-end space-x-2">
              <Button variant="outline" onClick={() => setShowUserModal(false)}>
                取消
              </Button>
              <Button>
                {selectedUser ? '更新' : '创建'}
              </Button>
            </div>
          </div>
        </Modal>

        {/* 配置编辑模态框 */}
        <Modal
          visible={showConfigModal}
          title="添加配置"
          onClose={() => setShowConfigModal(false)}
        >
          <div className="space-y-4">
            <Input label="配置键" placeholder="config_key" />
            <Input label="配置值" placeholder="配置值" />
            <Input label="描述" placeholder="配置描述" />
            <select className="w-full px-3 py-2 border rounded-md">
              <option value="业务配置">业务配置</option>
              <option value="系统配置">系统配置</option>
              <option value="经济配置">经济配置</option>
              <option value="安全配置">安全配置</option>
            </select>
            <div className="flex justify-end space-x-2">
              <Button variant="outline" onClick={() => setShowConfigModal(false)}>
                取消
              </Button>
              <Button>
                创建
              </Button>
            </div>
          </div>
        </Modal>
      </MainLayout>
    </ProtectedRoute>
  );
};

export default AdminPage; 