use crate::ttrpc_proto::attestation_agent::ExtendRuntimeMeasurementRequest;
use crate::ttrpc_proto::attestation_agent_ttrpc::AttestationAgentServiceClient;
use anyhow::{Context, Result};

const TTRPC_TIMEOUT: i64 = 50 * 1000 * 1000 * 1000;

pub struct AAClient {
    service_client: AttestationAgentServiceClient,
}

use crate::AA_ATTESTATION_SOCKET;

/// Convenience macro to obtain the scope logger
macro_rules! sl {
    () => {
        slog_scope::logger()
    };
}

impl AAClient {
    pub fn new() -> Result<Self> {
        let aa_addr = format!("unix://{AA_ATTESTATION_SOCKET}");
        let ttrpc_client = ttrpc::asynchronous::Client::connect(&aa_addr)
            .context(format!("ttrpc connect to AA addr: {} failed!", aa_addr))?;
        let service_client = AttestationAgentServiceClient::new(ttrpc_client);

        Ok(Self { service_client })
    }

    pub async fn measure_policy(&mut self, policy: &str) -> Result<()> {
        let content = serde_json::to_string(&policy).context("serialize policy body failed!")?;
        let req = ExtendRuntimeMeasurementRequest {
            Domain: "github.com/confidential-containers".into(),
            Operation: "SetPolicy".into(),
            Content: content,
            ..Default::default()
        };
        debug!(sl!(), "call extend_runtime_measurement w/ {:?}", req);
        self.service_client
            .extend_runtime_measurement(ttrpc::context::with_timeout(TTRPC_TIMEOUT), &req)
            .await
            .context("extend_runtime_measurement failed!")?;
        Ok(())
    }
}
