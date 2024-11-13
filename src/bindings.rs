#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
use std::fmt;
use std::process::{Command, Stdio};
use wasmbus_rpc::provider::prelude::*;
use wasmcloud_component::export;
use wit_bindgen::bitflags::*;
use wit_bindgen::bitflags;
type __IncomingHandlerExport = ::wasmcloud_component::http::IncomingHandler<
    ExecutorProvider,
>;
const _: () = {
    #[export_name = "wasi:http/incoming-handler@0.2.2#handle"]
    unsafe extern "C" fn export_handle(arg0: i32, arg1: i32) {
        ::wasmcloud_component::wasi::exports::wasi::http::incoming_handler::_export_handle_cabi::<
            __IncomingHandlerExport,
        >(arg0, arg1)
    }
};
const _: () = {};

struct ExecutorProvider;
impl Executor for ExecutorProvider {
    fn run(
        &mut self,
        command: String,
        payload: Option<String>,
        post_args: Option<String>,
    ) -> Result<String, String> {
        match run_native_tool(&command, payload.as_deref(), post_args.as_deref()) {
            Ok(output) => Ok(output),
            Err(e) => Err(e.to_string()),
        }
    }
}
fn run_native_tool(
    command: &str,
    payload: Option<&str>,
    post_args: Option<&str>,
) -> Result<String, std::io::Error> {
    let mut cmd = Command::new("sh");
    cmd.arg("-c");
    cmd.arg(command);
    if let Some(args) = post_args {
        cmd.arg(args);
    }
    if payload.is_some() {
        cmd.stdin(Stdio::piped());
    }
    let output = if let Some(input) = payload {
        let mut child = cmd.spawn()?;
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(input.as_bytes())?;
        }
        child.wait_with_output()?
    } else {
        cmd.output()?
    };
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!(
                            "Command failed with status {0}: {1}",
                            output.status,
                            String::from_utf8_lossy(&output.stderr),
                        ),
                    );
                    res
                }),
            ),
        )
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = async {
        provider_main(ExecutorProvider).await?;
        Ok(())
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
