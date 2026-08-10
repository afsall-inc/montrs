use crate::{Service, ServiceError, ServiceId, ServiceStatus};

/// Manages service lifecycle: start, stop, restart, status.
pub struct ServiceManager;

impl ServiceManager {
    pub fn new() -> Self { Self }

    pub fn services(&self) -> Vec<Service> { Vec::new() }
    pub fn status(&self) -> Vec<ServiceStatus> { Vec::new() }
    pub fn start(&self, _id: &ServiceId) -> Result<(), ServiceError> { Ok(()) }
    pub fn stop(&self, _id: &ServiceId) -> Result<(), ServiceError> { Ok(()) }
    pub fn restart(&self, _id: &ServiceId) -> Result<(), ServiceError> { Ok(()) }
    pub fn enable(&self, _id: &ServiceId) -> Result<(), ServiceError> { Ok(()) }
    pub fn disable(&self, _id: &ServiceId) -> Result<(), ServiceError> { Ok(()) }
}

impl Default for ServiceManager { fn default() -> Self { Self::new() } }