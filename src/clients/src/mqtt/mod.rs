use crate::{poll::ClientPool, retry_sleep_time, retry_times};
use common_base::{errors::RobustMQError, log::error};
use inner::{inner_delete_session, inner_update_cache};
use mobc::Manager;
use protocol::broker_server::generate::mqtt::mqtt_broker_service_client::MqttBrokerServiceClient;
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;
use tonic::transport::Channel;

#[derive(Clone)]
pub enum MQTTBrokerService {
    Mqtt,
}

#[derive(Clone, Debug)]
pub enum MQTTBrokerInterface {
    DeleteSession,
    UpdateCache,
}

pub mod call;
pub mod inner;

async fn retry_call(
    service: MQTTBrokerService,
    interface: MQTTBrokerInterface,
    client_poll: Arc<ClientPool>,
    addrs: Vec<String>,
    request: Vec<u8>,
) -> Result<Vec<u8>, RobustMQError> {
    let mut times = 1;
    loop {
        let index = times % addrs.len();
        let addr = addrs.get(index).unwrap().clone();
        let result = match service {
            MQTTBrokerService::Mqtt => {
                mqtt_interface_call(
                    interface.clone(),
                    client_poll.clone(),
                    addr,
                    request.clone(),
                )
                .await
            }
        };

        match result {
            Ok(data) => {
                return Ok(data);
            }
            Err(e) => {
                error(e.to_string());
                if times > retry_times() {
                    return Err(e);
                }
                times = times + 1;
            }
        }
        sleep(Duration::from_secs(retry_sleep_time(times) as u64)).await;
    }
}

async fn mqtt_client(
    client_poll: Arc<ClientPool>,
    addr: String,
) -> Result<MqttBrokerServiceClient<Channel>, RobustMQError> {
    match client_poll.mqtt_broker_mqtt_services_client(addr).await {
        Ok(client) => {
            return Ok(client);
        }
        Err(e) => {
            return Err(e);
        }
    }
}

pub(crate) async fn mqtt_interface_call(
    interface: MQTTBrokerInterface,
    client_poll: Arc<ClientPool>,
    addr: String,
    request: Vec<u8>,
) -> Result<Vec<u8>, RobustMQError> {
    match mqtt_client(client_poll.clone(), addr.clone()).await {
        Ok(client) => {
            let result = match interface {
                MQTTBrokerInterface::DeleteSession => inner_delete_session(client, request).await,
                MQTTBrokerInterface::UpdateCache => inner_update_cache(client, request).await,
                _ => {
                    return Err(RobustMQError::CommmonError(format!(
                        "mqtt service does not support service interfaces [{:?}]",
                        interface
                    )))
                }
            };
            match result {
                Ok(data) => return Ok(data),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Err(e) => {
            return Err(e);
        }
    }
}

#[derive(Clone)]
pub(crate) struct MqttBrokerMqttServiceManager {
    pub addr: String,
}

impl MqttBrokerMqttServiceManager {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }
}

#[tonic::async_trait]
impl Manager for MqttBrokerMqttServiceManager {
    type Connection = MqttBrokerServiceClient<Channel>;
    type Error = RobustMQError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        match MqttBrokerServiceClient::connect(format!("http://{}", self.addr.clone())).await {
            Ok(client) => {
                return Ok(client);
            }
            Err(err) => {
                return Err(RobustMQError::CommmonError(format!(
                    "{},{}",
                    err.to_string(),
                    self.addr.clone()
                )))
            }
        };
    }

    async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
        Ok(conn)
    }
}

#[cfg(test)]
mod tests {}
