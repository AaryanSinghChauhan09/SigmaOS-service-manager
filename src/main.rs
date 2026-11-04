//! SigmaOS-service-manager daemon entry point

use anyhow::Result;
use log::info;
use sigmaos_service_manager::{Component, ServiceManagerComponent};

fn main() -> Result<()> {
    env_logger::init();
    info!("Starting SigmaOS-service-manager daemon...");

    let mut component = ServiceManagerComponent::new();
    component.initialize()?;
    info!("Component initialized: status={:?}", component.status());

    component.start()?;
    info!("Component started: running={}", component.manager().running_count());

    component.stop()?;
    info!("Component stopped cleanly.");

    Ok(())
}
