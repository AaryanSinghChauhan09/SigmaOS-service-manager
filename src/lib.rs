//! SigmaOS-service-manager - Service Lifecycle & Process Supervision for SigmaOS

pub mod error;
pub mod manager;
pub mod unit;

pub use error::{Result, ServiceError};
pub use manager::ServiceManager;
pub use unit::{ServiceType, ServiceUnit, UnitState};

use serde::{Deserialize, Serialize};

/// Standard component interface for SigmaOS
pub trait Component {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> anyhow::Result<()>;
    fn start(&mut self) -> anyhow::Result<()>;
    fn stop(&mut self) -> anyhow::Result<()>;
    fn status(&self) -> ComponentStatus;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComponentStatus {
    Uninitialized,
    Initializing,
    Running,
    Stopped,
    Error(String),
}

pub struct ServiceManagerComponent {
    manager: ServiceManager,
    status: ComponentStatus,
}

impl Default for ServiceManagerComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceManagerComponent {
    pub fn new() -> Self {
        Self {
            manager: ServiceManager::new(),
            status: ComponentStatus::Uninitialized,
        }
    }

    pub fn manager(&self) -> &ServiceManager {
        &self.manager
    }

    pub fn manager_mut(&mut self) -> &mut ServiceManager {
        &mut self.manager
    }
}

impl Component for ServiceManagerComponent {
    fn name(&self) -> &str {
        "SigmaOS-service-manager"
    }

    fn version(&self) -> &str {
        env!("CARGO_PKG_VERSION")
    }

    fn initialize(&mut self) -> anyhow::Result<()> {
        self.status = ComponentStatus::Initializing;
        
        // Register default system services
        self.manager.register(ServiceUnit::new(
            "udev.service",
            "Device Event Daemon",
            "/usr/bin/udevd",
        ));
        self.manager.register(ServiceUnit::new(
            "dbus.service",
            "D-Bus Message Broker",
            "/usr/bin/dbus-daemon",
        ));
        self.manager.register(
            ServiceUnit::new(
                "display-manager.service",
                "Display Manager Greeter",
                "/usr/bin/display-manager",
            )
            .with_dependency("dbus.service"),
        );

        self.status = ComponentStatus::Stopped;
        Ok(())
    }

    fn start(&mut self) -> anyhow::Result<()> {
        self.manager.start("display-manager.service")?;
        self.status = ComponentStatus::Running;
        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        let _ = self.manager.stop("display-manager.service");
        let _ = self.manager.stop("dbus.service");
        let _ = self.manager.stop("udev.service");
        self.status = ComponentStatus::Stopped;
        Ok(())
    }

    fn status(&self) -> ComponentStatus {
        self.status.clone()
    }
}
