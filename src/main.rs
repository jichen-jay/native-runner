use anyhow::Context as _;
use tracing::{error, info};
use wasmcloud_provider_sdk::{load_host_data, start_provider, BoxFuture, Context, Provider, RpcError};
use wasmcloud_tracing::configure_observability;

use std::error::Error;
use wasmbus_rpc::actor::prelude::RpcResult;

// Wit-bindgen generated code
wit_bindgen::generate!({
    path: "world.wit",
    world: "runner",
    name: "wasmcloud_runner",
});

use wasmcloud_runner::executor::Executor;

#[derive(Default, Clone)]
struct RunnerProvider {}

#[async_trait::async_trait]
impl Provider for RunnerProvider {
    fn dispatch(
        &self,
        ctx: &Context,
        op: &str,
        msg: &[u8],
        _wasi_ctx: Option<wasmcloud_provider_sdk::WasiParams>,
    ) -> BoxFuture<'static, RpcResult<Vec<u8>>> {
        let this = self.clone();
        let op = op.to_string();
        let msg = msg.to_vec();

        Box::pin(async move {
            match op.as_str() {
                "run" => {
                    let input = wasmcloud_runner::executor::RunParams::decode(&msg)?;
                    let result = Executor::run(&this, input.command, input.payload, input.post_args).await?;
                    Ok(result.into_bytes()?)
                }
                _ => Err(RpcError::MethodNotFound(format!("Operation {} not found", op))),
            }
        })
    }

    fn shutdown(&self) -> BoxFuture<'static, Result<(), Box<dyn Error + Sync + Send>>> {
        Box::pin(async move {
            info!("shutdown");
            Ok(())
        })
    }
}

#[async_trait::async_trait]
impl Executor for RunnerProvider {
    async fn run(
        &self,
        command: String,
        payload: Option<String>,
        post_args: Option<String>,
    ) -> RpcResult<String> {
        match run_native_tool(&command, payload, post_args) {
            Ok(output) => Ok(output),
            Err(e) => {
                error!("failed to execute command: {}", e);
                Err(RpcError::Other(format!("failed to execute command: {}", e)))
            }
        }
    }
}

fn run_native_tool(
    command_and_pre_args: &str,
    payload: Option<String>,
    post_args: Option<String>,
) -> Result<String, std::io::Error> {
    // Implement your function
    Ok("Success".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let provider = RunnerProvider::default();
    let host_data = load_host_data().context("failed to load host data")?;

    let _observability = configure_observability(
        "runner-provider",
        host_data.config.get("FLAMEGRAPH_PATH").map(String::from),
    );

    start_provider(
        provider,
        Some("runner-provider".to_string()),
        move |_link_definition, _| {
            // Handle link definitions if needed
            info!("Received new link definition");
            Ok(())
        },
    )
    .await
    .context("failed to start provider")?;

    Ok(())
}
