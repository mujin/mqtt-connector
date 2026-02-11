use std::time::Duration;

use fluvio_connector_common::{connector, secret::SecretString};
use serde::Deserialize;

const DEFAULT_TIMEOUT_VALUE: Duration = Duration::from_secs(60);

#[connector(config, name = "mqtt")]
#[derive(Debug)]
pub(crate) struct MqttConfig {
    #[serde(default = "default_timeout")]
    pub timeout: Duration,
    pub url: SecretString,
    pub topic: String,
    #[serde(default = "default_client_id")]
    pub client_id: String,
    #[serde(default)]
    pub payload_output_type: OutputType,
    /// Delay between MQTT connection and subscription. Works around a race
    /// condition in the NATS MQTT bridge where SUBSCRIBE packets arriving
    /// before async JetStream session setup completes are silently dropped.
    /// See: https://github.com/nats-io/nats-server/issues/6191
    pub subscribe_delay: Option<Duration>,
    /// MQTT QoS level: 0 (AtMostOnce), 1 (AtLeastOnce), 2 (ExactlyOnce).
    /// Default: 0. Use 1+ with a stable client_id and clean_session=false
    /// for durable delivery across reconnects.
    #[serde(default)]
    pub qos: u8,
    /// When false, the broker preserves session state (subscriptions and
    /// pending messages) across disconnects. Requires a stable client_id.
    /// Default: true (no session persistence).
    #[serde(default = "default_clean_session")]
    pub clean_session: bool,
}

fn default_clean_session() -> bool {
    true
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub(crate) enum OutputType {
    #[default]
    Binary,
    Json,
}

fn default_timeout() -> Duration {
    DEFAULT_TIMEOUT_VALUE
}

fn default_client_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
