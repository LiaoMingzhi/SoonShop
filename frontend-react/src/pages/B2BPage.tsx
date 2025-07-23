import React, { useState, useEffect } from 'react';
import { MainLayout } from '../components/layout/MainLayout';
import { StatsCard } from '../components/business/StatsCard';
import { Card } from '../components/ui/Card';
import { Button } from '../components/ui/Button';
import Modal from '../components/ui/Modal';
import { Input } from '../components/ui/Input';
import { Badge } from '../components/ui/Badge';
import { useAuthStore } from '../stores/authStore';

interface Supplier {
  id: string;
  name: string;
  category: string;
  rating: number;
  location: string;
  contactPerson: string;
  email: string;
  phone: string;
  totalOrders: number;
  totalValue: number;
  status: 'verified' | 'pending' | 'suspended';
  certifications: string[];
  establishedYear: number;
}

interface BulkOrder {
  id: string;
  orderNumber: string;
  supplierId: string;
  supplierName: string;
  items: Array<{
    voucherId: string;
    voucherName: string;
    quantity: number;
    unitPrice: number;
    totalPrice: number;
  }>;
  total: number;
  status: 'draft' | 'submitted' | 'confirmed' | 'shipped' | 'delivered' | 'cancelled';
  createdAt: string;
  expectedDelivery?: string;
  notes?: string;
}

interface Contract {
  id: string;
  contractNumber: string;
  supplierId: string;
  supplierName: string;
  type: 'framework' | 'specific' | 'service';
  value: number;
  startDate: string;
  endDate: string;
  status: 'draft' | 'pending_signature' | 'active' | 'expired' | 'terminated';
  terms: string[];
  paymentTerms: string;
  deliveryTerms: string;
}

interface B2BStats {
  totalSuppliers: number;
  activeOrders: number;
  totalPurchaseValue: number;
  averageOrderValue: number;
  onTimeDeliveryRate: number;
  supplierSatisfaction: number;
}

const B2BPage: React.FC = () => {
  const { user } = useAuthStore();
  const [activeTab, setActiveTab] = useState<'overview' | 'suppliers' | 'orders' | 'contracts'>('overview');
  const [stats, setStats] = useState<B2BStats>({
    totalSuppliers: 156,
    activeOrders: 23,
    totalPurchaseValue: 2850000,
    averageOrderValue: 12500,
    onTimeDeliveryRate: 94.5,
    supplierSatisfaction: 4.6,
  });
  const [suppliers, setSuppliers] = useState<Supplier[]>([]);
  const [orders, setOrders] = useState<BulkOrder[]>([]);
  const [contracts, setContracts] = useState<Contract[]>([]);
  const [loading, setLoading] = useState(false);
  const [showSupplierModal, setShowSupplierModal] = useState(false);
  const [showOrderModal, setShowOrderModal] = useState(false);
  const [selectedSupplier, setSelectedSupplier] = useState<Supplier | null>(null);

  // 模拟数据加载
  useEffect(() => {
    loadMockData();
  }, []);

  const loadMockData = () => {
    // 模拟供应商数据
    const mockSuppliers: Supplier[] = [
      {
        id: '1',
        name: '华农生产合作社',
        category: '农产品',
        rating: 4.8,
        location: '山东省济南市',
        contactPerson: '张农民',
        email: 'zhang@huanong.com',
        phone: '138-0000-0001',
        totalOrders: 45,
        totalValue: 680000,
        status: 'verified',
        certifications: ['有机认证', 'ISO9001', '绿色食品'],
        establishedYear: 1995,
      },
      {
        id: '2',
        name: '红星机械制造厂',
        category: '工业设备',
        rating: 4.6,
        location: '江苏省苏州市',
        contactPerson: '李工程师',
        email: 'li@hongxing.com',
        phone: '138-0000-0002',
        totalOrders: 12,
        totalValue: 1200000,
        status: 'verified',
        certifications: ['ISO14001', 'CE认证'],
        establishedYear: 1988,
      },
      {
        id: '3',
        name: '新时代服装工厂',
        category: '纺织服装',
        rating: 4.3,
        location: '广东省广州市',
        contactPerson: '王经理',
        email: 'wang@xinshidai.com',
        phone: '138-0000-0003',
        totalOrders: 28,
        totalValue: 420000,
        status: 'pending',
        certifications: ['OEKO-TEX', '社会责任认证'],
        establishedYear: 2005,
      },
    ];

    // 模拟订单数据
    const mockOrders: BulkOrder[] = [
      {
        id: '1',
        orderNumber: 'PO2024020001',
        supplierId: '1',
        supplierName: '华农生产合作社',
        items: [
          {
            voucherId: 'V001',
            voucherName: '优质大米提货券',
            quantity: 1000,
            unitPrice: 25,
            totalPrice: 25000,
          },
          {
            voucherId: 'V002',
            voucherName: '有机蔬菜提货券',
            quantity: 500,
            unitPrice: 15,
            totalPrice: 7500,
          },
        ],
        total: 32500,
        status: 'confirmed',
        createdAt: '2024-02-08',
        expectedDelivery: '2024-02-15',
        notes: '请确保产品新鲜度',
      },
      {
        id: '2',
        orderNumber: 'PO2024020002',
        supplierId: '2',
        supplierName: '红星机械制造厂',
        items: [
          {
            voucherId: 'V003',
            voucherName: '小型农机提货券',
            quantity: 10,
            unitPrice: 8500,
            totalPrice: 85000,
          },
        ],
        total: 85000,
        status: 'shipped',
        createdAt: '2024-02-05',
        expectedDelivery: '2024-02-12',
      },
    ];

    // 模拟合同数据
    const mockContracts: Contract[] = [
      {
        id: '1',
        contractNumber: 'CT2024001',
        supplierId: '1',
        supplierName: '华农生产合作社',
        type: 'framework',
        value: 500000,
        startDate: '2024-01-01',
        endDate: '2024-12-31',
        status: 'active',
        terms: ['年度供应协议', '价格锁定', '质量保证'],
        paymentTerms: '月结30天',
        deliveryTerms: '指定地点交货',
      },
      {
        id: '2',
        contractNumber: 'CT2024002',
        supplierId: '2',
        supplierName: '红星机械制造厂',
        type: 'specific',
        value: 150000,
        startDate: '2024-02-01',
        endDate: '2024-03-31',
        status: 'pending_signature',
        terms: ['设备采购', '安装调试', '培训服务'],
        paymentTerms: '预付30% 余款货到付清',
        deliveryTerms: 'EXW工厂交货',
      },
    ];

    setSuppliers(mockSuppliers);
    setOrders(mockOrders);
    setContracts(mockContracts);
  };

  const renderOverview = () => (
    <div className="space-y-6">
      {/* 统计卡片 */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <StatsCard
          title="供应商总数"
          value={stats.totalSuppliers.toString()}
          icon="handshake"
          gradient="blue"
          subtitle={`${suppliers.filter(s => s.status === 'verified').length} 已认证`}
        />
        <StatsCard
          title="活跃订单"
          value={stats.activeOrders.toString()}
          icon="clipboard-list"
          gradient="green"
          trend={{
            direction: 'up',
            value: '+12%',
            period: '本月'
          }}
        />
        <StatsCard
          title="采购总额"
          value={`¥${(stats.totalPurchaseValue / 10000).toFixed(1)}万`}
          icon="coins"
          gradient="orange"
          subtitle="本年度累计"
        />
        <StatsCard
          title="平均订单价值"
          value={`¥${stats.averageOrderValue.toLocaleString()}`}
          icon="chart-line"
          gradient="purple"
        />
        <StatsCard
          title="准时交付率"
          value={`${stats.onTimeDeliveryRate}%`}
          icon="truck"
          gradient="green"
          trend={{
            direction: 'up',
            value: '+2.3%',
            period: '较上月'
          }}
        />
        <StatsCard
          title="供应商满意度"
          value={`${stats.supplierSatisfaction}/5.0`}
          icon="star"
          gradient="pink"
        />
      </div>

      {/* 最近订单 */}
      <Card title="最近订单" className="p-6">
        <div className="space-y-4">
          {orders.slice(0, 5).map(order => (
            <div key={order.id} className="flex items-center justify-between p-4 bg-gray-50 rounded-lg">
              <div className="flex-1">
                <div className="flex items-center space-x-4">
                  <div>
                    <div className="font-medium">{order.orderNumber}</div>
                    <div className="text-sm text-gray-600">{order.supplierName}</div>
                  </div>
                  <Badge variant={
                    order.status === 'delivered' ? 'success' :
                    order.status === 'shipped' ? 'default' :
                    order.status === 'confirmed' ? 'success' : 'warning'
                  }>
                    {order.status === 'draft' ? '草稿' :
                     order.status === 'submitted' ? '已提交' :
                     order.status === 'confirmed' ? '已确认' :
                     order.status === 'shipped' ? '已发货' :
                     order.status === 'delivered' ? '已交付' : '已取消'}
                  </Badge>
                </div>
              </div>
              <div className="text-right">
                <div className="font-bold">¥{order.total.toLocaleString()}</div>
                <div className="text-sm text-gray-600">{order.createdAt}</div>
              </div>
            </div>
          ))}
        </div>
        <div className="mt-4 text-center">
          <Button variant="outline" onClick={() => setActiveTab('orders')}>
            查看所有订单
          </Button>
        </div>
      </Card>

      {/* 供应商概览 */}
      <Card title="顶级供应商" className="p-6">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {suppliers.slice(0, 3).map(supplier => (
            <div key={supplier.id} className="p-4 border rounded-lg">
              <div className="flex items-center justify-between mb-2">
                <h3 className="font-medium">{supplier.name}</h3>
                <div className="flex items-center">
                  <i className="fas fa-star text-yellow-500 mr-1"></i>
                  <span className="text-sm">{supplier.rating}</span>
                </div>
              </div>
              <p className="text-sm text-gray-600 mb-2">{supplier.category}</p>
              <div className="flex justify-between text-sm">
                <span>订单数: {supplier.totalOrders}</span>
                <span>总额: ¥{(supplier.totalValue / 10000).toFixed(1)}万</span>
              </div>
            </div>
          ))}
        </div>
      </Card>
    </div>
  );

  const renderSuppliers = () => (
    <div className="space-y-6">
      {/* 供应商管理头部 */}
      <div className="flex justify-between items-center">
        <div className="flex space-x-2">
          <Input placeholder="搜索供应商..." className="w-64" />
          <select className="px-3 py-2 border rounded-md">
            <option>所有类别</option>
            <option>农产品</option>
            <option>工业设备</option>
            <option>纺织服装</option>
            <option>食品饮料</option>
          </select>
          <select className="px-3 py-2 border rounded-md">
            <option>所有状态</option>
            <option>已认证</option>
            <option>待审核</option>
            <option>已暂停</option>
          </select>
        </div>
        <Button onClick={() => setShowSupplierModal(true)}>
          <i className="fas fa-plus mr-2"></i>
          添加供应商
        </Button>
      </div>

      {/* 供应商列表 */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {suppliers.map(supplier => (
          <Card key={supplier.id} className="p-6">
            <div className="flex items-start justify-between mb-4">
              <div className="flex-1">
                <div className="flex items-center space-x-2 mb-2">
                  <h3 className="text-lg font-bold">{supplier.name}</h3>
                  <Badge variant={
                    supplier.status === 'verified' ? 'success' :
                    supplier.status === 'pending' ? 'warning' : 'error'
                  }>
                    {supplier.status === 'verified' ? '已认证' :
                     supplier.status === 'pending' ? '待审核' : '已暂停'}
                  </Badge>
                </div>
                <p className="text-gray-600 mb-2">{supplier.category} · {supplier.location}</p>
                <div className="flex items-center mb-2">
                  <div className="flex items-center mr-4">
                    <i className="fas fa-star text-yellow-500 mr-1"></i>
                    <span>{supplier.rating}</span>
                  </div>
                  <span className="text-sm text-gray-600">成立于{supplier.establishedYear}年</span>
                </div>
              </div>
            </div>

            <div className="grid grid-cols-2 gap-4 mb-4">
              <div className="text-center p-3 bg-gray-50 rounded">
                <div className="text-xl font-bold text-blue-600">{supplier.totalOrders}</div>
                <div className="text-sm text-gray-600">总订单数</div>
              </div>
              <div className="text-center p-3 bg-gray-50 rounded">
                <div className="text-xl font-bold text-green-600">
                  ¥{(supplier.totalValue / 10000).toFixed(1)}万
                </div>
                <div className="text-sm text-gray-600">总采购额</div>
              </div>
            </div>

            <div className="mb-4">
              <div className="text-sm text-gray-600 mb-2">认证资质：</div>
              <div className="flex flex-wrap gap-1">
                {supplier.certifications.map((cert, index) => (
                  <Badge key={index} variant="default" className="text-xs">
                    {cert}
                  </Badge>
                ))}
              </div>
            </div>

            <div className="flex space-x-2">
              <Button 
                size="sm" 
                variant="outline" 
                onClick={() => {
                  setSelectedSupplier(supplier);
                  setShowSupplierModal(true);
                }}
              >
                查看详情
              </Button>
              <Button size="sm" onClick={() => setShowOrderModal(true)}>
                创建订单
              </Button>
            </div>
          </Card>
        ))}
      </div>
    </div>
  );

  const renderOrders = () => (
    <div className="space-y-6">
      {/* 订单统计 */}
      <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
        <Card className="p-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-blue-600">{orders.length}</div>
            <div className="text-sm text-gray-600">总订单</div>
          </div>
        </Card>
        <Card className="p-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-green-600">
              {orders.filter(o => o.status === 'confirmed').length}
            </div>
            <div className="text-sm text-gray-600">已确认</div>
          </div>
        </Card>
        <Card className="p-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-orange-600">
              {orders.filter(o => o.status === 'shipped').length}
            </div>
            <div className="text-sm text-gray-600">运输中</div>
          </div>
        </Card>
        <Card className="p-4">
          <div className="text-center">
            <div className="text-2xl font-bold text-purple-600">
              ¥{orders.reduce((sum, o) => sum + o.total, 0).toLocaleString()}
            </div>
            <div className="text-sm text-gray-600">总金额</div>
          </div>
        </Card>
      </div>

      {/* 订单列表 */}
      <Card title="采购订单" className="p-6">
        <div className="mb-4 flex justify-between items-center">
          <div className="flex space-x-2">
            <Input placeholder="搜索订单..." className="w-64" />
            <select className="px-3 py-2 border rounded-md">
              <option>所有状态</option>
              <option>草稿</option>
              <option>已提交</option>
              <option>已确认</option>
              <option>已发货</option>
              <option>已交付</option>
            </select>
          </div>
          <Button onClick={() => setShowOrderModal(true)}>
            <i className="fas fa-plus mr-2"></i>
            创建订单
          </Button>
        </div>

        <div className="space-y-4">
          {orders.map(order => (
            <Card key={order.id} className="p-4">
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center space-x-4 mb-2">
                    <h3 className="font-bold">{order.orderNumber}</h3>
                    <Badge variant={
                      order.status === 'delivered' ? 'success' :
                      order.status === 'shipped' ? 'default' :
                      order.status === 'confirmed' ? 'success' : 'warning'
                    }>
                      {order.status === 'draft' ? '草稿' :
                       order.status === 'submitted' ? '已提交' :
                       order.status === 'confirmed' ? '已确认' :
                       order.status === 'shipped' ? '已发货' :
                       order.status === 'delivered' ? '已交付' : '已取消'}
                    </Badge>
                  </div>
                  <p className="text-gray-600 mb-2">{order.supplierName}</p>
                  <div className="mb-2">
                    <div className="text-sm text-gray-600">订单项目：</div>
                    {order.items.map((item, index) => (
                      <div key={index} className="text-sm">
                        {item.voucherName} × {item.quantity} = ¥{item.totalPrice.toLocaleString()}
                      </div>
                    ))}
                  </div>
                  <div className="flex items-center space-x-4 text-sm text-gray-600">
                    <span>创建时间：{order.createdAt}</span>
                    {order.expectedDelivery && (
                      <span>预计交付：{order.expectedDelivery}</span>
                    )}
                  </div>
                </div>
                <div className="text-right">
                  <div className="text-xl font-bold text-green-600 mb-2">
                    ¥{order.total.toLocaleString()}
                  </div>
                  <div className="flex space-x-2">
                    <Button size="sm" variant="outline">查看</Button>
                    {order.status === 'draft' && (
                      <Button size="sm">编辑</Button>
                    )}
                  </div>
                </div>
              </div>
            </Card>
          ))}
        </div>
      </Card>
    </div>
  );

  const renderContracts = () => (
    <div className="space-y-6">
      <Card title="合同管理" className="p-6">
        <div className="mb-4 flex justify-between items-center">
          <div className="flex space-x-2">
            <Input placeholder="搜索合同..." className="w-64" />
            <select className="px-3 py-2 border rounded-md">
              <option>所有类型</option>
              <option>框架协议</option>
              <option>专项合同</option>
              <option>服务合同</option>
            </select>
          </div>
          <Button>
            <i className="fas fa-plus mr-2"></i>
            创建合同
          </Button>
        </div>

        <div className="space-y-4">
          {contracts.map(contract => (
            <Card key={contract.id} className="p-4">
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center space-x-4 mb-2">
                    <h3 className="font-bold">{contract.contractNumber}</h3>
                    <Badge variant={
                      contract.status === 'active' ? 'success' :
                      contract.status === 'pending_signature' ? 'warning' :
                      contract.status === 'draft' ? 'default' : 'error'
                    }>
                      {contract.status === 'draft' ? '草稿' :
                       contract.status === 'pending_signature' ? '待签署' :
                       contract.status === 'active' ? '生效中' :
                       contract.status === 'expired' ? '已过期' : '已终止'}
                    </Badge>
                    <Badge variant="default">
                      {contract.type === 'framework' ? '框架协议' :
                       contract.type === 'specific' ? '专项合同' : '服务合同'}
                    </Badge>
                  </div>
                  <p className="text-gray-600 mb-2">{contract.supplierName}</p>
                  <div className="grid grid-cols-2 gap-4 mb-2 text-sm">
                    <div>
                      <span className="text-gray-600">合同期限：</span>
                      {contract.startDate} 至 {contract.endDate}
                    </div>
                    <div>
                      <span className="text-gray-600">付款条件：</span>
                      {contract.paymentTerms}
                    </div>
                  </div>
                  <div className="mb-2">
                    <div className="text-sm text-gray-600">合同条款：</div>
                    <div className="flex flex-wrap gap-1">
                      {contract.terms.map((term, index) => (
                        <Badge key={index} variant="default" className="text-xs">
                          {term}
                        </Badge>
                      ))}
                    </div>
                  </div>
                </div>
                <div className="text-right">
                  <div className="text-xl font-bold text-blue-600 mb-2">
                    ¥{contract.value.toLocaleString()}
                  </div>
                  <div className="flex space-x-2">
                    <Button size="sm" variant="outline">查看</Button>
                    {contract.status === 'pending_signature' && (
                      <Button size="sm">签署</Button>
                    )}
                  </div>
                </div>
              </div>
            </Card>
          ))}
        </div>
      </Card>
    </div>
  );

  return (
    <MainLayout>
      <div className="min-h-screen bg-gray-50">
        {/* 页面头部 */}
        <div className="bg-white shadow-sm border-b border-gray-200">
          <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-6">
            <div className="flex items-center justify-between">
              <div>
                <h1 className="text-2xl font-bold text-gray-800 flex items-center">
                  <i className="fas fa-handshake text-blue-600 mr-3"></i>
                  B2B企业采购平台
                </h1>
                <p className="text-gray-600 mt-1">企业级采购管理，优化供应链协作</p>
              </div>
              <div className="flex space-x-2">
                <Button variant="outline">
                  <i className="fas fa-download mr-2"></i>
                  导出报告
                </Button>
                <Button>
                  <i className="fas fa-plus mr-2"></i>
                  快速采购
                </Button>
              </div>
            </div>

            {/* 标签导航 */}
            <div className="mt-6">
              <nav className="flex space-x-8">
                {[
                  { key: 'overview', label: '概览', icon: 'tachometer-alt' },
                  { key: 'suppliers', label: '供应商', icon: 'handshake' },
                  { key: 'orders', label: '采购订单', icon: 'clipboard-list' },
                  { key: 'contracts', label: '合同管理', icon: 'file-contract' },
                ].map(tab => (
                  <button
                    key={tab.key}
                    onClick={() => setActiveTab(tab.key as any)}
                    className={`flex items-center px-3 py-2 text-sm font-medium rounded-md transition-colors ${
                      activeTab === tab.key
                        ? 'bg-blue-100 text-blue-700'
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
          {activeTab === 'overview' && renderOverview()}
          {activeTab === 'suppliers' && renderSuppliers()}
          {activeTab === 'orders' && renderOrders()}
          {activeTab === 'contracts' && renderContracts()}
        </div>
      </div>

      {/* 供应商模态框 */}
      <Modal
        visible={showSupplierModal}
        title={selectedSupplier ? '供应商详情' : '添加供应商'}
        onClose={() => {
          setShowSupplierModal(false);
          setSelectedSupplier(null);
        }}
      >
        <div className="space-y-4">
          <Input label="供应商名称" defaultValue={selectedSupplier?.name} />
          <Input label="联系人" defaultValue={selectedSupplier?.contactPerson} />
          <Input label="邮箱" defaultValue={selectedSupplier?.email} />
          <Input label="电话" defaultValue={selectedSupplier?.phone} />
          <select className="w-full px-3 py-2 border rounded-md">
            <option>选择类别</option>
            <option>农产品</option>
            <option>工业设备</option>
            <option>纺织服装</option>
            <option>食品饮料</option>
          </select>
          <div className="flex justify-end space-x-2">
            <Button variant="outline" onClick={() => setShowSupplierModal(false)}>
              取消
            </Button>
            <Button>
              {selectedSupplier ? '更新' : '添加'}
            </Button>
          </div>
        </div>
      </Modal>

      {/* 订单创建模态框 */}
      <Modal
        visible={showOrderModal}
        title="创建采购订单"
        onClose={() => setShowOrderModal(false)}
      >
        <div className="space-y-4">
          <select className="w-full px-3 py-2 border rounded-md">
            <option>选择供应商</option>
            {suppliers.map(supplier => (
              <option key={supplier.id} value={supplier.id}>
                {supplier.name}
              </option>
            ))}
          </select>
          <Input label="提货券ID" placeholder="输入提货券ID" />
          <Input label="数量" type="number" placeholder="输入采购数量" />
          <Input label="预期交付日期" type="date" />
          <Input label="备注" placeholder="特殊要求或备注" />
          <div className="flex justify-end space-x-2">
            <Button variant="outline" onClick={() => setShowOrderModal(false)}>
              取消
            </Button>
            <Button>
              创建订单
            </Button>
          </div>
        </div>
      </Modal>
    </MainLayout>
  );
};

export default B2BPage; 