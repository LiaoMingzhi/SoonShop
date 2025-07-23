import React, { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/Card';
import { Button } from '@/components/ui/Button';
import { Input } from '@/components/ui/Input';
import { 
  CommunistIcon, 
  PeopleIcon, 
  UnityIcon, 
  ProsperityIcon,
  UserIcon,
  LockIcon,
  WalletIcon,
  CheckIcon,
  AlertIcon
} from '@/components/ui/Icon';
import { cn } from '@/utils/classnames';

interface WalletConnectionState {
  isConnecting: boolean;
  isConnected: boolean;
  walletAddress?: string;
  error?: string;
}

interface LoginFormData {
  email: string;
  password: string;
}

export const LoginPage: React.FC = () => {
  const [loginMethod, setLoginMethod] = useState<'email' | 'wallet'>('wallet');
  const [formData, setFormData] = useState<LoginFormData>({ email: '', password: '' });
  const [walletState, setWalletState] = useState<WalletConnectionState>({
    isConnecting: false,
    isConnected: false
  });
  const [isLoading, setIsLoading] = useState(false);

  // 模拟钱包连接
  const handleWalletConnect = async () => {
    setWalletState({ ...walletState, isConnecting: true, error: undefined });
    
    try {
      // 模拟异步钱包连接
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      // 模拟成功连接
      setWalletState({
        isConnecting: false,
        isConnected: true,
        walletAddress: '8K7F9Xb36oFJsjpCKpsXvg4cgBRoZtwNTc3EzG5Ttd2o'
      });
    } catch (error) {
      setWalletState({
        isConnecting: false,
        isConnected: false,
        error: '钱包连接失败，请重试'
      });
    }
  };

  // 处理邮箱登录
  const handleEmailLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);
    
    try {
      // 模拟登录请求
      await new Promise(resolve => setTimeout(resolve, 1500));
      console.log('邮箱登录:', formData);
      // 这里应该调用实际的登录API
    } catch (error) {
      console.error('登录失败:', error);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-primary-50 via-white to-secondary-50 flex items-center justify-center p-4">
      <div className="w-full max-w-md space-y-8">
        {/* 品牌区域 */}
        <div className="text-center space-y-4 animate-fade-in">
          <div className="flex items-center justify-center space-x-3">
            <CommunistIcon size="2xl" className="text-primary-600" />
            <h1 className="text-4xl font-bold text-gray-900">SoonShop</h1>
            <ProsperityIcon size="2xl" className="text-success-600" />
          </div>
          <p className="text-xl text-gray-700 font-medium">共产主义商业平台</p>
          <p className="text-gray-600">人民至上 · 共同富裕 · 集体决策</p>
        </div>
        
        {/* 主登录卡片 */}
        <Card variant="glass" className="shadow-xl animate-slide-up">
          <CardHeader className="text-center pb-6">
            <CardTitle className="text-2xl font-bold text-gray-900">
              欢迎回到人民的平台
            </CardTitle>
            <p className="text-gray-600 mt-2">选择您的登录方式</p>
          </CardHeader>

          <CardContent className="space-y-6">
            {/* 登录方式选择 */}
            <div className="grid grid-cols-2 gap-3">
              <Button
                variant={loginMethod === 'wallet' ? 'gradient' : 'secondary'}
                gradient={loginMethod === 'wallet' ? 'blue' : undefined}
                size="lg"
                fullWidth
                onClick={() => setLoginMethod('wallet')}
                className="relative"
              >
                <WalletIcon size="sm" className="mr-2" />
                区块链钱包
                {loginMethod === 'wallet' && (
                  <CheckIcon size="xs" className="absolute top-2 right-2" />
                )}
              </Button>
              
              <Button
                variant={loginMethod === 'email' ? 'gradient' : 'secondary'}
                gradient={loginMethod === 'email' ? 'green' : undefined}
                size="lg"
                fullWidth
                onClick={() => setLoginMethod('email')}
                className="relative"
              >
                <UserIcon size="sm" className="mr-2" />
                邮箱密码
                {loginMethod === 'email' && (
                  <CheckIcon size="xs" className="absolute top-2 right-2" />
                )}
              </Button>
            </div>

            {/* 钱包连接登录 */}
            {loginMethod === 'wallet' && (
              <div className="space-y-4 animate-fade-in">
                <div className="bg-primary-50 border border-primary-200 rounded-xl p-4">
                  <div className="flex items-start space-x-3">
                    <div className="p-2 bg-primary-100 rounded-lg">
                      <PeopleIcon size="sm" className="text-primary-600" />
                    </div>
                    <div className="flex-1">
                      <h4 className="font-semibold text-primary-900">人民的钱包</h4>
                      <p className="text-sm text-primary-700 mt-1">
                        使用去中心化身份，保护您的隐私和资产安全
                      </p>
                    </div>
                  </div>
                </div>

                {!walletState.isConnected ? (
                  <Button
                    variant="gradient"
                    gradient="blue"
                    size="lg"
                    fullWidth
                    loading={walletState.isConnecting}
                    onClick={handleWalletConnect}
                  >
                    <WalletIcon size="sm" className="mr-2" />
                    {walletState.isConnecting ? '正在连接钱包...' : '连接Solana钱包'}
                  </Button>
                ) : (
                  <div className="space-y-3">
                    <div className="bg-success-50 border border-success-200 rounded-xl p-4">
                      <div className="flex items-center space-x-3">
                        <CheckIcon size="lg" className="text-success-600" />
                        <div>
                          <h4 className="font-semibold text-success-900">钱包已连接</h4>
                          <p className="text-sm text-success-700 font-mono">
                            {walletState.walletAddress?.slice(0, 8)}...{walletState.walletAddress?.slice(-8)}
                          </p>
                        </div>
                      </div>
                    </div>
                    
                    <Button 
                      variant="gradient"
                      gradient="blue"
                      size="lg" 
                      fullWidth
                    >
                      进入人民平台
                    </Button>
                  </div>
                )}

                {walletState.error && (
                  <div className="bg-red-50 border border-red-200 rounded-xl p-4">
                    <div className="flex items-center space-x-3">
                      <AlertIcon size="sm" className="text-red-600" />
                      <p className="text-sm text-red-700">{walletState.error}</p>
                    </div>
                  </div>
                )}
              </div>
            )}

            {/* 邮箱密码登录 */}
            {loginMethod === 'email' && (
              <form onSubmit={handleEmailLogin} className="space-y-4 animate-fade-in">
                <div className="bg-secondary-50 border border-secondary-200 rounded-xl p-4">
                  <div className="flex items-start space-x-3">
                    <div className="p-2 bg-secondary-100 rounded-lg">
                      <UnityIcon size="sm" className="text-secondary-600" />
                    </div>
                    <div className="flex-1">
                      <h4 className="font-semibold text-secondary-900">传统登录</h4>
                      <p className="text-sm text-secondary-700 mt-1">
                        使用邮箱和密码，适合初次体验用户
                      </p>
                    </div>
                  </div>
                </div>

                <div className="space-y-4">
                  <Input
                    type="email"
                    placeholder="输入您的邮箱地址"
                    value={formData.email}
                    onChange={(e) => setFormData({ ...formData, email: e.target.value })}
                    startIcon={<UserIcon size="sm" />}
                    required
                  />
                  
                  <Input
                    type="password"
                    placeholder="输入您的密码"
                    value={formData.password}
                    onChange={(e) => setFormData({ ...formData, password: e.target.value })}
                    startIcon={<LockIcon size="sm" />}
                    required
                  />
                </div>

                <Button
                  variant="gradient"
                  gradient="green"
                  type="submit"
                  size="lg"
                  fullWidth
                  loading={isLoading}
                >
                  登录到人民平台
                </Button>
              </form>
            )}

            {/* 其他选项 */}
            <div className="text-center space-y-3">
              <div className="relative">
                <div className="absolute inset-0 flex items-center">
                  <div className="w-full border-t border-gray-300"></div>
                </div>
                <div className="relative flex justify-center text-sm">
                  <span className="px-2 bg-white text-gray-500">或者</span>
                </div>
              </div>

              <div className="space-y-2 text-sm">
                <a href="/register" className="text-primary-600 hover:text-primary-700 font-medium">
                  注册新账户，加入人民平台
                </a>
                <br />
                <a href="/forgot-password" className="text-gray-500 hover:text-gray-700">
                  忘记密码？
                </a>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* 底部信息 */}
        <div className="text-center space-y-4 animate-slide-up">
          <div className="grid grid-cols-3 gap-4 text-xs text-gray-600">
            <div className="flex items-center justify-center space-x-1">
              <PeopleIcon size="xs" />
              <span>人民当家作主</span>
            </div>
            <div className="flex items-center justify-center space-x-1">
              <UnityIcon size="xs" />
              <span>按需生产分配</span>
            </div>
            <div className="flex items-center justify-center space-x-1">
              <ProsperityIcon size="xs" />
              <span>共同富裕发展</span>
            </div>
          </div>
          
          <p className="text-xs text-gray-500">
            © 2024 SoonShop 共产主义商业平台. 为人民服务，共创美好未来.
          </p>
        </div>
      </div>
    </div>
  );
};

export default LoginPage; 