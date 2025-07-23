import { useCallback } from 'react';

/**
 * 自定义Hook - 页面导航管理
 * 提供统一的导航方法和路由管理
 */
export const useNavigation = () => {
  // 导航到指定路径
  const navigateTo = useCallback((path: string) => {
    window.location.href = path;
  }, []);

  // 导航到功能页面
  const navigateToFeature = useCallback((featurePath: string) => {
    navigateTo(featurePath);
  }, [navigateTo]);

  // 导航到首页
  const navigateToHome = useCallback(() => {
    navigateTo('/');
  }, [navigateTo]);

  // 导航到提货券页面
  const navigateToVoucher = useCallback(() => {
    navigateTo('/voucher');
  }, [navigateTo]);

  // 导航到B2C商城
  const navigateToB2C = useCallback(() => {
    navigateTo('/b2c');
  }, [navigateTo]);

  // 导航到消费与倍增页面
  const navigateToConsumption = useCallback(() => {
    navigateTo('/consumption');
  }, [navigateTo]);

  // 导航到企业评估页面
  const navigateToEvaluation = useCallback(() => {
    navigateTo('/evaluation');
  }, [navigateTo]);

  // 导航到系统管理页面
  const navigateToAdmin = useCallback(() => {
    navigateTo('/admin');
  }, [navigateTo]);

  // 导航到B2B市场
  const navigateToB2B = useCallback(() => {
    navigateTo('/b2b');
  }, [navigateTo]);

  // 导航到餐饮服务页面
  const navigateToRestaurant = useCallback(() => {
    navigateTo('/restaurant');
  }, [navigateTo]);

  // 导航到分析页面
  const navigateToAnalytics = useCallback(() => {
    navigateTo('/analytics');
  }, [navigateTo]);

  // 导航到个人资料页面
  const navigateToProfile = useCallback(() => {
    navigateTo('/profile');
  }, [navigateTo]);

  // 导航到登录页面
  const navigateToLogin = useCallback(() => {
    navigateTo('/login');
  }, [navigateTo]);

  // 外部链接导航
  const navigateToExternal = useCallback((url: string, newTab = true) => {
    if (newTab) {
      window.open(url, '_blank', 'noopener,noreferrer');
    } else {
      window.location.href = url;
    }
  }, []);

  // 返回上一页
  const goBack = useCallback(() => {
    window.history.back();
  }, []);

  // 前进到下一页
  const goForward = useCallback(() => {
    window.history.forward();
  }, []);

  // 刷新当前页面
  const refresh = useCallback(() => {
    window.location.reload();
  }, []);

  return {
    // 基础导航
    navigateTo,
    navigateToFeature,
    
    // 具体页面导航
    navigateToHome,
    navigateToVoucher,
    navigateToB2C,
    navigateToConsumption,
    navigateToEvaluation,
    navigateToAdmin,
    navigateToB2B,
    navigateToRestaurant,
    navigateToAnalytics,
    navigateToProfile,
    navigateToLogin,
    
    // 特殊导航
    navigateToExternal,
    goBack,
    goForward,
    refresh,
  };
}; 