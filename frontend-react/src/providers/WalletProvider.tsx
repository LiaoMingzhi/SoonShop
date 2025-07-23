import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react'

// 钱包连接状态类型
interface WalletState {
  connected: boolean
  publicKey: string | null
  balance: number
  loading: boolean
}

// 钱包上下文类型
interface WalletContextType {
  wallet: WalletState
  connect: () => Promise<void>
  disconnect: () => void
  signTransaction: (transaction: any) => Promise<any>
  signMessage: (message: string) => Promise<string>
}

// 创建钱包上下文
const WalletContext = createContext<WalletContextType | undefined>(undefined)

// 钱包提供者组件属性
interface WalletProviderProps {
  children: ReactNode
}

export const WalletProvider: React.FC<WalletProviderProps> = ({ children }) => {
  const [wallet, setWallet] = useState<WalletState>({
    connected: false,
    publicKey: null,
    balance: 0,
    loading: false,
  })

  // 检查钱包是否可用
  const checkWalletAvailable = (): boolean => {
    // 检查是否有 Solana 钱包扩展
    if (typeof window !== 'undefined' && window.solana) {
      return true
    }
    
    // 检查是否有 Phantom 钱包
    if (typeof window !== 'undefined' && window.phantom?.solana) {
      return true
    }
    
    return false
  }

  // 连接钱包
  const connect = async (): Promise<void> => {
    if (!checkWalletAvailable()) {
      alert('请安装 Solana 钱包扩展（如 Phantom）')
      return
    }

    setWallet(prev => ({ ...prev, loading: true }))

    try {
      // 优先使用 Phantom 钱包
      const provider = window.phantom?.solana || window.solana
      
      if (provider) {
        const response = await provider.connect()
        
        if (response.publicKey) {
          setWallet({
            connected: true,
            publicKey: response.publicKey.toString(),
            balance: 0, // 稍后获取余额
            loading: false,
          })
          
          // 获取余额
          await getBalance(response.publicKey.toString())
          
          console.log('钱包连接成功')
        }
      }
    } catch (error) {
      console.error('钱包连接失败:', error)
      alert('钱包连接失败')
      setWallet(prev => ({ ...prev, loading: false }))
    }
  }

  // 断开钱包连接
  const disconnect = (): void => {
    try {
      const provider = window.phantom?.solana || window.solana
      
      if (provider && provider.disconnect) {
        provider.disconnect()
      }
      
      setWallet({
        connected: false,
        publicKey: null,
        balance: 0,
        loading: false,
      })
      
      console.log('钱包已断开连接')
    } catch (error) {
      console.error('断开钱包连接失败:', error)
    }
  }

  // 获取钱包余额
  const getBalance = async (publicKey: string): Promise<void> => {
    try {
      // 这里应该调用实际的 Solana RPC 来获取余额
      // 暂时使用模拟数据
      const mockBalance = Math.random() * 10
      
      setWallet(prev => ({ ...prev, balance: mockBalance }))
    } catch (error) {
      console.error('获取余额失败:', error)
    }
  }

  // 签名交易
  const signTransaction = async (transaction: any): Promise<any> => {
    if (!wallet.connected) {
      throw new Error('钱包未连接')
    }

    try {
      const provider = window.phantom?.solana || window.solana
      
      if (provider && provider.signTransaction) {
        const signedTransaction = await provider.signTransaction(transaction)
        return signedTransaction
      }
      
      throw new Error('钱包不支持交易签名')
    } catch (error) {
      console.error('交易签名失败:', error)
      throw error
    }
  }

  // 签名消息
  const signMessage = async (messageToSign: string): Promise<string> => {
    if (!wallet.connected) {
      throw new Error('钱包未连接')
    }

    try {
      const provider = window.phantom?.solana || window.solana
      
      if (provider && provider.signMessage) {
        const encodedMessage = new TextEncoder().encode(messageToSign)
        const signedMessage = await provider.signMessage(encodedMessage, 'utf8')
        return signedMessage.signature
      }
      
      throw new Error('钱包不支持消息签名')
    } catch (error) {
      console.error('消息签名失败:', error)
      throw error
    }
  }

  // 监听钱包连接状态变化
  useEffect(() => {
    const provider = window.phantom?.solana || window.solana
    
    if (provider) {
      // 检查是否已经连接
      if (provider.isConnected) {
        provider.connect({ onlyIfTrusted: true })
          .then((response: any) => {
            if (response.publicKey) {
              setWallet({
                connected: true,
                publicKey: response.publicKey.toString(),
                balance: 0,
                loading: false,
              })
              getBalance(response.publicKey.toString())
            }
          })
          .catch(() => {
            // 静默处理，用户可能没有授权
          })
      }

      // 监听连接状态变化
      provider.on('connect', (publicKey: any) => {
        setWallet(prev => ({
          ...prev,
          connected: true,
          publicKey: publicKey.toString(),
        }))
        getBalance(publicKey.toString())
      })

      provider.on('disconnect', () => {
        setWallet({
          connected: false,
          publicKey: null,
          balance: 0,
          loading: false,
        })
      })

      // 监听账户变化
      provider.on('accountChanged', (publicKey: any) => {
        if (publicKey) {
          setWallet(prev => ({
            ...prev,
            publicKey: publicKey.toString(),
          }))
          getBalance(publicKey.toString())
        } else {
          disconnect()
        }
      })
    }

    // 清理监听器
    return () => {
      if (provider) {
        provider.removeAllListeners()
      }
    }
  }, [])

  const contextValue: WalletContextType = {
    wallet,
    connect,
    disconnect,
    signTransaction,
    signMessage,
  }

  return (
    <WalletContext.Provider value={contextValue}>
      {children}
    </WalletContext.Provider>
  )
}

// 使用钱包的 Hook
export const useWallet = (): WalletContextType => {
  const context = useContext(WalletContext)
  
  if (!context) {
    throw new Error('useWallet 必须在 WalletProvider 内使用')
  }
  
  return context
}

// 声明全局 window 对象的类型
declare global {
  interface Window {
    solana?: any
    phantom?: {
      solana?: any
    }
  }
} 