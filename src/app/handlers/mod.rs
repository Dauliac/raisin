use std::sync::Arc;
use tokio::sync::RwLock;

use crate::domain::program::{ProgramError, ProgramEvent};

use super::{
    cqrs_es::event::{EventBus, EventHandlers, Events},
    services::logger::Logger,
};

pub async fn subscribe_logger(
    logger: Arc<RwLock<dyn Logger + Send + Sync>>,
    event_bus: &mut dyn EventBus,
) {
    // TODO subscribe all event
    let domain_event = Events::new_domain(ProgramEvent::default());
    event_bus
      .subscribe(domain_event, EventHandlers::Logger(logger.clone()))
      .await;

    let domain_error = Events::new_domain_error(ProgramError::default());
    event_bus
      .subscribe(domain_error, EventHandlers::Logger(logger))
      .await;
}
