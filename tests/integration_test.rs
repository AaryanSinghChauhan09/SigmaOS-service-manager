use anyhow::Result;
use sigmaos_service_manager::{
    Component, ComponentStatus, ServiceManager, ServiceManagerComponent, ServiceUnit, UnitState,
};

#[test]
fn test_service_manager_component_lifecycle_aaa() -> Result<()> {
    // Arrange
    let mut component = ServiceManagerComponent::new();
    assert_eq!(component.status(), ComponentStatus::Uninitialized);

    // Act - Initialize
    component.initialize()?;
    assert_eq!(component.status(), ComponentStatus::Stopped);
    assert_eq!(component.manager().count(), 3);

    // Act - Start
    component.start()?;
    assert_eq!(component.status(), ComponentStatus::Running);
    assert_eq!(component.manager().running_count(), 2); // dbus + display-manager

    // Act - Stop
    component.stop()?;
    assert_eq!(component.status(), ComponentStatus::Stopped);
    assert_eq!(component.manager().running_count(), 0);

    Ok(())
}

#[test]
fn test_restart_service_aaa() -> Result<()> {
    // Arrange
    let mut sm = ServiceManager::new();
    let unit = ServiceUnit::new("network.service", "Network Daemon", "/usr/bin/networkd");
    sm.register(unit);

    // Act - Start & Restart
    sm.start("network.service")?;
    assert_eq!(sm.get("network.service").unwrap().state, UnitState::Running);

    sm.restart("network.service")?;
    assert_eq!(sm.get("network.service").unwrap().state, UnitState::Running);

    // Act - Stop
    sm.stop("network.service")?;
    assert_eq!(sm.get("network.service").unwrap().state, UnitState::Stopped);

    Ok(())
}
