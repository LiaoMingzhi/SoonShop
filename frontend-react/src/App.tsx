import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { WalletProvider } from './providers/WalletProvider';
import { ErrorBoundary } from './components/common/ErrorBoundary';

// 页面组件
import HomePage from './pages/HomePage';
import VoucherPage from './pages/VoucherPage';
import ConsumptionPage from './pages/ConsumptionPage';
import B2CPage from './pages/B2CPage';
import EvaluationPage from './pages/EvaluationPage';
import RestaurantPage from './pages/RestaurantPage';
import AdminPage from './pages/AdminPage';
import AnalyticsPage from './pages/AnalyticsPage';
import LoginPage from './pages/LoginPage';
import ProfilePage from './pages/ProfilePage';
import B2BPage from './pages/B2BPage';

// 新增医疗和住房页面
import { HealthcareServicePage } from './features/healthcare/pages/HealthcareServicePage';
import { HousingServicePage } from './features/housing/pages/HousingServicePage';

const App: React.FC = () => {
  return (
    <ErrorBoundary>
      <WalletProvider>
        <Router>
          <div className="App">
            <Routes>
              <Route path="/" element={<HomePage />} />
              <Route path="/vouchers" element={<VoucherPage />} />
              <Route path="/consumption" element={<ConsumptionPage />} />
              <Route path="/b2c" element={<B2CPage />} />
              <Route path="/b2b" element={<B2BPage />} />
              <Route path="/evaluation" element={<EvaluationPage />} />
              <Route path="/restaurant" element={<RestaurantPage />} />
              <Route path="/healthcare" element={<HealthcareServicePage />} />
              <Route path="/housing" element={<HousingServicePage />} />
              <Route path="/admin" element={<AdminPage />} />
              <Route path="/analytics" element={<AnalyticsPage />} />
              <Route path="/login" element={<LoginPage />} />
              <Route path="/profile" element={<ProfilePage />} />
            </Routes>
          </div>
        </Router>
      </WalletProvider>
    </ErrorBoundary>
  );
};

export default App; 