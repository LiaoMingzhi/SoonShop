// 监控配置模块
// 这里可以添加监控配置相关的功能

pub struct MonitoringConfig {
    pub enabled: bool,
    pub metrics_port: u16,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_port: 9090,
        }
    }
} 