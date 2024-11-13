#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use async_trait::async_trait;
use std::process::{Command, Stdio};
use wasmbus_rpc::provider::prelude::*;
use wit_bindgen;
use wasmcloud_component::error;
use crate::bindings::export::executor::*;
#[provider(upgrade = "1")]
struct RunnerProvider {}
#[automatically_derived]
impl ::core::clone::Clone for RunnerProvider {
    #[inline]
    fn clone(&self) -> RunnerProvider {
        RunnerProvider {}
    }
}
impl wasmbus_rpc::common::MessageDispatch for RunnerProvider {
    #[allow(
        elided_named_lifetimes,
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::needless_arbitrary_self_type,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn dispatch<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        _ctx: &'life1 wasmbus_rpc::common::Context,
        message: wasmbus_rpc::common::Message<'life2>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = std::result::Result<Vec<u8>, wasmbus_rpc::error::RpcError>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                std::result::Result<Vec<u8>, wasmbus_rpc::error::RpcError>,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let __self = self;
            let message = message;
            let __ret: std::result::Result<Vec<u8>, wasmbus_rpc::error::RpcError> = {
                Err(
                    wasmbus_rpc::error::RpcError::MethodNotHandled(
                        message.method.to_string(),
                    ),
                )
            };
            #[allow(unreachable_code)] __ret
        })
    }
}
impl RunnerProvider {
    pub fn new() -> Self {
        RunnerProvider {}
    }
}
impl ProviderHandler for RunnerProvider {
    #[allow(
        elided_named_lifetimes,
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::needless_arbitrary_self_type,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn put_link<'life0, 'life1, 'async_trait>(
        &'life0 self,
        _ld: &'life1 LinkDefinition,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = RpcResult<bool>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                RpcResult<bool>,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let __self = self;
            let __ret: RpcResult<bool> = { Ok(true) };
            #[allow(unreachable_code)] __ret
        })
    }
    #[allow(
        elided_named_lifetimes,
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::needless_arbitrary_self_type,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn delete_link<'life0, 'life1, 'async_trait>(
        &'life0 self,
        _actor_id: &'life1 str,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = RpcResult<bool>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                RpcResult<bool>,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let __self = self;
            let __ret: RpcResult<bool> = { Ok(true) };
            #[allow(unreachable_code)] __ret
        })
    }
    #[allow(
        elided_named_lifetimes,
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::needless_arbitrary_self_type,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn shutdown<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = Result<(), std::convert::Infallible>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                Result<(), std::convert::Infallible>,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let __self = self;
            let __ret: Result<(), std::convert::Infallible> = { Ok(()) };
            #[allow(unreachable_code)] __ret
        })
    }
}
impl Executor for RunnerProvider {
    #[allow(
        elided_named_lifetimes,
        clippy::async_yields_async,
        clippy::diverging_sub_expression,
        clippy::let_unit_value,
        clippy::needless_arbitrary_self_type,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn run<'life0, 'life1, 'async_trait>(
        &'life0 self,
        _ctx: &'life1 Context,
        command: String,
        payload: Option<String>,
        post_args: Option<String>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = RpcResult<String>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                RpcResult<String>,
            > {
                #[allow(unreachable_code)] return __ret;
            }
            let __self = self;
            let command = command;
            let payload = payload;
            let post_args = post_args;
            let __ret: RpcResult<String> = {
                match run_native_tool(
                    &command,
                    payload.as_deref(),
                    post_args.as_deref(),
                ) {
                    Ok(output) => Ok(output),
                    Err(e) => {
                        {
                            ::wasmcloud_component::wasi::logging::logging::log(
                                ::wasmcloud_component::wasi::logging::logging::Level::Error,
                                "",
                                &std::fmt::format(
                                    format_args!("failed to execute command: {0}", e),
                                ),
                            );
                        };
                        Err(
                            RpcError::Other(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("failed to execute command: {0}", e),
                                    );
                                    res
                                }),
                            ),
                        )
                    }
                }
            };
            #[allow(unreachable_code)] __ret
        })
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
    provider_main(RunnerProvider::new())?;
    {
        ::std::io::_eprint(format_args!("Runner Provider exiting\n"));
    };
    Ok(())
}
