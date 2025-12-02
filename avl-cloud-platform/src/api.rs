//! REST API implementation

use crate::error::Result;
use crate::{compute, storage, network, billing, auth, monitoring};

pub struct CloudApi {
    compute: compute::ComputeManager,
    storage: storage::StorageService,
    network: network::NetworkManager,
    billing: billing::BillingManager,
    auth: auth::AuthManager,
    monitoring: monitoring::MonitoringService,
}

impl CloudApi {
    pub fn new(
        compute: compute::ComputeManager,
        storage: storage::StorageService,
        network: network::NetworkManager,
        billing: billing::BillingManager,
        auth: auth::AuthManager,
        monitoring: monitoring::MonitoringService,
    ) -> Self {
        Self {
            compute,
            storage,
            network,
            billing,
            auth,
            monitoring,
        }
    }

    pub async fn serve(self, addr: &str) -> Result<()> {
        // TODO: Integrate avx-http Router for production-grade HTTP routing
        println!("🚀 API server would start on {}", addr);
        println!("📚 Endpoints:");
        println!("   POST   /v1/compute/instances");
        println!("   GET    /v1/compute/instances");
        println!("   DELETE /v1/compute/instances/:id");
        println!("   POST   /v1/storage/buckets");
        println!("   PUT    /v1/storage/:bucket/:key");
        println!("   GET    /v1/storage/:bucket/:key");
        println!("   POST   /v1/network/vpcs");
        println!("   GET    /v1/network/vpcs");
        println!("   GET    /v1/billing/usage");
        println!("   GET    /v1/metrics");

        // Keep process running
        tokio::signal::ctrl_c().await?;
        Ok(())
    }
}
