use std::sync::Arc;

use btcnode_metrics::BitcoinNode;
use btcnode_metrics_gatherer::MetricsService;

pub struct AppState {
    pub service: Arc<MetricsService<BitcoinNode>>,
}

impl Clone for AppState {
    fn clone(&self) -> Self {
        Self {
            service: Arc::clone(&self.service),
        }
    }
}
