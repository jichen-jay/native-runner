#![allow(unused_imports)]

mod bindings {
    use wasmcloud_component::export;
    wit_bindgen_wrpc::generate!({ generate_all });
    export!(ExecutorProvider);
}

// use bindings::exports::executor::{Executor, ExecutorData};
use bindings::executor::{Executor, RunArgs, RunResults};

use core::future::Future;
use core::time::Duration;

use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Arc;

use anyhow::{anyhow, bail, Context as _};
use async_trait::async_trait;
use bytes::Bytes;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tracing::{debug, error, info, instrument, warn};
use wasmcloud_provider_sdk::{core::HealthCheckRequest, core::HealthCheckResponse};
use wasmcloud_provider_sdk::{initialize_logging, start_provider};
use wasmcloud_provider_sdk::{
    load_host_data, Context, LinkDefinition, Provider, ProviderInitConfig, ProviderShutdownRequest,
    WasmCloudEntity,
};


#[derive(Clone)]
pub struct ExecutorProvider;

impl ExecutorProvider {
    pub fn new() -> Self {
        ExecutorProvider
    }
}

#[async_trait]
impl ProviderHandler for ExecutorProvider {
    async fn put_link(&self, ld: wasmcloud_provider_sdk::LinkDefinition) -> anyhow::Result<()> {
        // Handle link definitions
        Ok(())
    }

    async fn delete_link(&self, actor_id: &str) -> anyhow::Result<()> {
        // Handle link deletions
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), anyhow::Error> {
        info!("ExecutorProvider shutting down");
        Ok(())
    }
}

impl Executor for ExecutorProvider {
    fn run(
        &self,
        command: String,
        payload: Option<String>,
        post_args: Option<String>,
    ) -> Result<String, String> {
        // Since we're using async code in the function, we need to block on it
        let output = tokio::runtime::Handle::current().block_on(
            self.run_command(command, payload, post_args)
        );
        output.map_err(|e| e.to_string())
    }
}

impl ExecutorProvider {
    #[instrument(level = "debug", skip(self))]
    async fn run_command(
        &self,
        command: String,
        payload: Option<String>,
        post_args: Option<String>,
    ) -> Result<String, anyhow::Error> {
        debug!("Executing command: {}", command);

        let mut cmd = Command::new(command);

        if let Some(args) = post_args {
            cmd.args(args.split_whitespace());
        }

        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        let mut child = cmd.spawn().context("Failed to spawn command")?;

        if let Some(input_data) = payload {
            if let Some(mut stdin) = child.stdin.take() {
                stdin
                    .write_all(input_data.as_bytes())
                    .await
                    .context("Failed to write to stdin")?;
                stdin.flush().await.context("Failed to flush stdin")?;
            } else {
                warn!("Stdin is not available for the command");
            }
        }

        let output = child
            .wait_with_output()
            .await
            .context("Failed to wait for command output")?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(stdout)
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            Err(anyhow!("Command execution failed: {}", stderr))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging
    initialize_logging("executor_provider", None);

    // Create an instance of the provider
    let provider = ExecutorProvider::new();

    // Start the provider runtime
    start_provider(provider, None).await?;

    Ok(())
}