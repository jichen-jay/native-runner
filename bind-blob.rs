#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod bindings {
    #[allow(dead_code)]
    pub mod wasi {
        #[allow(dead_code)]
        pub mod blobstore {
            #[allow(dead_code, clippy::all)]
            pub mod types {
                pub type InputStream = super::super::super::wasi::io::streams::InputStream;
                pub type OutputStream = super::super::super::wasi::io::streams::OutputStream;
                /// name of a container, a collection of objects.
                /// The container name may be any valid UTF-8 string.
                pub type ContainerName = String;
                /// name of an object within a container
                /// The object name may be any valid UTF-8 string.
                pub type ObjectName = String;
                /// TODO: define timestamp to include seconds since
                /// Unix epoch and nanoseconds
                /// https://github.com/WebAssembly/wasi-blob-store/issues/7
                pub type Timestamp = u64;
                /// size of an object, in bytes
                pub type ObjectSize = u64;
                pub type Error = String;
                /// information about a container
                pub struct ContainerMetadata {
                    /// the container's name
                    pub name: ContainerName,
                    /// date and time container was created
                    pub created_at: Timestamp,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ContainerMetadata {
                    #[inline]
                    fn clone(&self) -> ContainerMetadata {
                        ContainerMetadata {
                            name: ::core::clone::Clone::clone(&self.name),
                            created_at: ::core::clone::Clone::clone(&self.created_at),
                        }
                    }
                }
                impl<W> ::wit_bindgen_wrpc::wrpc_transport::Encode<W>
                for &self::ContainerMetadata
                where
                    W: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                        + ::core::marker::Unpin + 'static,
                {
                    type Encoder = container_metadata::Encoder<W>;
                }
                impl<W> ::wit_bindgen_wrpc::wrpc_transport::Encode<W>
                for self::ContainerMetadata
                where
                    W: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                        + ::core::marker::Unpin + 'static,
                {
                    type Encoder = container_metadata::Encoder<W>;
                }
                impl<R> ::wit_bindgen_wrpc::wrpc_transport::Decode<R>
                for self::ContainerMetadata
                where
                    R: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                        + ::core::marker::Unpin + 'static,
                {
                    type Decoder = container_metadata::Decoder<R>;
                    type ListDecoder = ::wit_bindgen_wrpc::wrpc_transport::ListDecoder<
                        Self::Decoder,
                        R,
                    >;
                }
                mod container_metadata {
                    pub struct Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        name: <super::ContainerName as ::wit_bindgen_wrpc::wrpc_transport::Encode<
                            W,
                        >>::Encoder,
                        created_at: <super::Timestamp as ::wit_bindgen_wrpc::wrpc_transport::Encode<
                            W,
                        >>::Encoder,
                    }
                    #[automatically_derived]
                    impl<W> ::core::default::Default for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        fn default() -> Self {
                            Self {
                                name: ::core::default::Default::default(),
                                created_at: ::core::default::Default::default(),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<W> ::wit_bindgen_wrpc::wrpc_transport::Deferred<W>
                    for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        fn take_deferred(
                            &mut self,
                        ) -> ::core::option::Option<
                            ::wit_bindgen_wrpc::wrpc_transport::DeferredFn<W>,
                        > {
                            let f_name = self.name.take_deferred();
                            let f_created_at = self.created_at.take_deferred();
                            if false || f_name.is_some() || f_created_at.is_some() {
                                return Some(
                                    ::std::boxed::Box::new(|w, mut path| {
                                        let f_name = f_name
                                            .map(|f| {
                                                path.push(0);
                                                let w = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &w,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, w)
                                            });
                                        let f_created_at = f_created_at
                                            .map(|f| {
                                                path.push(1);
                                                let w = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &w,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, w)
                                            });
                                        ::std::boxed::Box::pin(async move {
                                            {
                                                use ::tokio::macros::support::{
                                                    maybe_done, poll_fn, Future, Pin,
                                                };
                                                use ::tokio::macros::support::Poll::{Ready, Pending};
                                                let mut futures = (
                                                    maybe_done(async {
                                                        match f_name {
                                                            Some((f, Ok(w))) => f(w, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                    maybe_done(async {
                                                        match f_created_at {
                                                            Some((f, Ok(w))) => f(w, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                );
                                                let mut futures = &mut futures;
                                                let mut skip_next_time: u32 = 0;
                                                poll_fn(move |cx| {
                                                        const COUNT: u32 = 0 + 1 + 1;
                                                        let mut is_pending = false;
                                                        let mut to_run = COUNT;
                                                        let mut skip = skip_next_time;
                                                        skip_next_time = if skip + 1 == COUNT {
                                                            0
                                                        } else {
                                                            skip + 1
                                                        };
                                                        loop {
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (_, fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                        }
                                                        if is_pending {
                                                            Pending
                                                        } else {
                                                            Ready(
                                                                Ok((
                                                                    {
                                                                        let (fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                    {
                                                                        let (_, fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                )),
                                                            )
                                                        }
                                                    })
                                                    .await
                                            }?;
                                            Ok(())
                                        })
                                    }),
                                );
                            }
                            None
                        }
                    }
                    pub struct Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        c_name: <super::ContainerName as ::wit_bindgen_wrpc::wrpc_transport::Decode<
                            R,
                        >>::Decoder,
                        v_name: ::core::option::Option<super::ContainerName>,
                        c_created_at: <super::Timestamp as ::wit_bindgen_wrpc::wrpc_transport::Decode<
                            R,
                        >>::Decoder,
                        v_created_at: ::core::option::Option<super::Timestamp>,
                    }
                    #[automatically_derived]
                    impl<R> ::core::default::Default for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        fn default() -> Self {
                            Self {
                                c_name: ::core::default::Default::default(),
                                v_name: ::core::default::Default::default(),
                                c_created_at: ::core::default::Default::default(),
                                v_created_at: ::core::default::Default::default(),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<
                        R,
                    > ::wit_bindgen_wrpc::wrpc_transport::Deferred<
                        ::wit_bindgen_wrpc::wrpc_transport::Incoming<R>,
                    > for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        fn take_deferred(
                            &mut self,
                        ) -> ::core::option::Option<
                            ::wit_bindgen_wrpc::wrpc_transport::DeferredFn<
                                ::wit_bindgen_wrpc::wrpc_transport::Incoming<R>,
                            >,
                        > {
                            let f_name = self.c_name.take_deferred();
                            let f_created_at = self.c_created_at.take_deferred();
                            if false || f_name.is_some() || f_created_at.is_some() {
                                return Some(
                                    ::std::boxed::Box::new(|r, mut path| {
                                        let f_name = f_name
                                            .map(|f| {
                                                path.push(0);
                                                let r = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &r,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, r)
                                            });
                                        let f_created_at = f_created_at
                                            .map(|f| {
                                                path.push(1);
                                                let r = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &r,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, r)
                                            });
                                        ::std::boxed::Box::pin(async move {
                                            {
                                                use ::tokio::macros::support::{
                                                    maybe_done, poll_fn, Future, Pin,
                                                };
                                                use ::tokio::macros::support::Poll::{Ready, Pending};
                                                let mut futures = (
                                                    maybe_done(async {
                                                        match f_name {
                                                            Some((f, Ok(r))) => f(r, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                    maybe_done(async {
                                                        match f_created_at {
                                                            Some((f, Ok(r))) => f(r, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                );
                                                let mut futures = &mut futures;
                                                let mut skip_next_time: u32 = 0;
                                                poll_fn(move |cx| {
                                                        const COUNT: u32 = 0 + 1 + 1;
                                                        let mut is_pending = false;
                                                        let mut to_run = COUNT;
                                                        let mut skip = skip_next_time;
                                                        skip_next_time = if skip + 1 == COUNT {
                                                            0
                                                        } else {
                                                            skip + 1
                                                        };
                                                        loop {
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (_, fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                        }
                                                        if is_pending {
                                                            Pending
                                                        } else {
                                                            Ready(
                                                                Ok((
                                                                    {
                                                                        let (fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                    {
                                                                        let (_, fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                )),
                                                            )
                                                        }
                                                    })
                                                    .await
                                            }?;
                                            Ok(())
                                        })
                                    }),
                                );
                            }
                            None
                        }
                    }
                    #[automatically_derived]
                    impl<R> ::wit_bindgen_wrpc::tokio_util::codec::Decoder for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        type Item = super::ContainerMetadata;
                        type Error = ::std::io::Error;
                        fn decode(
                            &mut self,
                            src: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::core::result::Result<
                            ::core::option::Option<Self::Item>,
                            Self::Error,
                        > {
                            if self.v_name.is_none() {
                                let Some(v) = self.c_name.decode(src)? else {
                                    return Ok(None)
                                };
                                self.v_name = Some(v);
                            }
                            if self.v_created_at.is_none() {
                                let Some(v) = self.c_created_at.decode(src)? else {
                                    return Ok(None)
                                };
                                self.v_created_at = Some(v);
                            }
                            Ok(
                                Some(Self::Item {
                                    name: self.v_name.take().unwrap(),
                                    created_at: self.v_created_at.take().unwrap(),
                                }),
                            )
                        }
                    }
                    #[automatically_derived]
                    impl<
                        W,
                    > ::wit_bindgen_wrpc::tokio_util::codec::Encoder<
                        super::ContainerMetadata,
                    > for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        type Error = ::std::io::Error;
                        #[allow(unused_mut)]
                        fn encode(
                            &mut self,
                            item: super::ContainerMetadata,
                            mut dst: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::std::io::Result<()> {
                            let super::ContainerMetadata {
                                name: f_name,
                                created_at: f_created_at,
                            } = item;
                            self.name.encode(f_name, &mut dst)?;
                            self.created_at.encode(f_created_at, &mut dst)?;
                            Ok(())
                        }
                    }
                    #[automatically_derived]
                    impl<
                        W,
                    > ::wit_bindgen_wrpc::tokio_util::codec::Encoder<
                        &super::ContainerMetadata,
                    > for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        type Error = ::std::io::Error;
                        #[allow(unused_mut)]
                        fn encode(
                            &mut self,
                            item: &super::ContainerMetadata,
                            mut dst: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::std::io::Result<()> {
                            let super::ContainerMetadata {
                                name: f_name,
                                created_at: f_created_at,
                            } = item;
                            self.name.encode(f_name, &mut dst)?;
                            self.created_at.encode(f_created_at, &mut dst)?;
                            Ok(())
                        }
                    }
                }
                impl ::core::fmt::Debug for ContainerMetadata {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("ContainerMetadata")
                            .field("name", &self.name)
                            .field("created-at", &self.created_at)
                            .finish()
                    }
                }
                /// information about an object
                pub struct ObjectMetadata {
                    /// the object's name
                    pub name: ObjectName,
                    /// the object's parent container
                    pub container: ContainerName,
                    /// date and time the object was created
                    pub created_at: Timestamp,
                    /// size of the object, in bytes
                    pub size: ObjectSize,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ObjectMetadata {
                    #[inline]
                    fn clone(&self) -> ObjectMetadata {
                        ObjectMetadata {
                            name: ::core::clone::Clone::clone(&self.name),
                            container: ::core::clone::Clone::clone(&self.container),
                            created_at: ::core::clone::Clone::clone(&self.created_at),
                            size: ::core::clone::Clone::clone(&self.size),
                        }
                    }
                }
                impl<W> ::wit_bindgen_wrpc::wrpc_transport::Encode<W>
                for &self::ObjectMetadata
                where
                    W: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                        + ::core::marker::Unpin + 'static,
                {
                    type Encoder = object_metadata::Encoder<W>;
                }
                impl<W> ::wit_bindgen_wrpc::wrpc_transport::Encode<W>
                for self::ObjectMetadata
                where
                    W: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                        + ::core::marker::Unpin + 'static,
                {
                    type Encoder = object_metadata::Encoder<W>;
                }
                impl<R> ::wit_bindgen_wrpc::wrpc_transport::Decode<R>
                for self::ObjectMetadata
                where
                    R: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                        + ::core::marker::Unpin + 'static,
                {
                    type Decoder = object_metadata::Decoder<R>;
                    type ListDecoder = ::wit_bindgen_wrpc::wrpc_transport::ListDecoder<
                        Self::Decoder,
                        R,
                    >;
                }
                mod object_metadata {
                    pub struct Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        name: <super::ObjectName as ::wit_bindgen_wrpc::wrpc_transport::Encode<
                            W,
                        >>::Encoder,
                        container: <super::ContainerName as ::wit_bindgen_wrpc::wrpc_transport::Encode<
                            W,
                        >>::Encoder,
                        created_at: <super::Timestamp as ::wit_bindgen_wrpc::wrpc_transport::Encode<
                            W,
                        >>::Encoder,
                        size: <super::ObjectSize as ::wit_bindgen_wrpc::wrpc_transport::Encode<
                            W,
                        >>::Encoder,
                    }
                    #[automatically_derived]
                    impl<W> ::core::default::Default for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        fn default() -> Self {
                            Self {
                                name: ::core::default::Default::default(),
                                container: ::core::default::Default::default(),
                                created_at: ::core::default::Default::default(),
                                size: ::core::default::Default::default(),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<W> ::wit_bindgen_wrpc::wrpc_transport::Deferred<W>
                    for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        fn take_deferred(
                            &mut self,
                        ) -> ::core::option::Option<
                            ::wit_bindgen_wrpc::wrpc_transport::DeferredFn<W>,
                        > {
                            let f_name = self.name.take_deferred();
                            let f_container = self.container.take_deferred();
                            let f_created_at = self.created_at.take_deferred();
                            let f_size = self.size.take_deferred();
                            if false || f_name.is_some() || f_container.is_some()
                                || f_created_at.is_some() || f_size.is_some()
                            {
                                return Some(
                                    ::std::boxed::Box::new(|w, mut path| {
                                        let f_name = f_name
                                            .map(|f| {
                                                path.push(0);
                                                let w = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &w,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, w)
                                            });
                                        let f_container = f_container
                                            .map(|f| {
                                                path.push(1);
                                                let w = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &w,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, w)
                                            });
                                        let f_created_at = f_created_at
                                            .map(|f| {
                                                path.push(2);
                                                let w = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &w,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, w)
                                            });
                                        let f_size = f_size
                                            .map(|f| {
                                                path.push(3);
                                                let w = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &w,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, w)
                                            });
                                        ::std::boxed::Box::pin(async move {
                                            {
                                                use ::tokio::macros::support::{
                                                    maybe_done, poll_fn, Future, Pin,
                                                };
                                                use ::tokio::macros::support::Poll::{Ready, Pending};
                                                let mut futures = (
                                                    maybe_done(async {
                                                        match f_name {
                                                            Some((f, Ok(w))) => f(w, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                    maybe_done(async {
                                                        match f_container {
                                                            Some((f, Ok(w))) => f(w, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                    maybe_done(async {
                                                        match f_created_at {
                                                            Some((f, Ok(w))) => f(w, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                    maybe_done(async {
                                                        match f_size {
                                                            Some((f, Ok(w))) => f(w, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                );
                                                let mut futures = &mut futures;
                                                let mut skip_next_time: u32 = 0;
                                                poll_fn(move |cx| {
                                                        const COUNT: u32 = 0 + 1 + 1 + 1 + 1;
                                                        let mut is_pending = false;
                                                        let mut to_run = COUNT;
                                                        let mut skip = skip_next_time;
                                                        skip_next_time = if skip + 1 == COUNT {
                                                            0
                                                        } else {
                                                            skip + 1
                                                        };
                                                        loop {
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (_, fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (_, _, fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (_, _, _, fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                        }
                                                        if is_pending {
                                                            Pending
                                                        } else {
                                                            Ready(
                                                                Ok((
                                                                    {
                                                                        let (fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                    {
                                                                        let (_, fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                    {
                                                                        let (_, _, fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                    {
                                                                        let (_, _, _, fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                )),
                                                            )
                                                        }
                                                    })
                                                    .await
                                            }?;
                                            Ok(())
                                        })
                                    }),
                                );
                            }
                            None
                        }
                    }
                    pub struct Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        c_name: <super::ObjectName as ::wit_bindgen_wrpc::wrpc_transport::Decode<
                            R,
                        >>::Decoder,
                        v_name: ::core::option::Option<super::ObjectName>,
                        c_container: <super::ContainerName as ::wit_bindgen_wrpc::wrpc_transport::Decode<
                            R,
                        >>::Decoder,
                        v_container: ::core::option::Option<super::ContainerName>,
                        c_created_at: <super::Timestamp as ::wit_bindgen_wrpc::wrpc_transport::Decode<
                            R,
                        >>::Decoder,
                        v_created_at: ::core::option::Option<super::Timestamp>,
                        c_size: <super::ObjectSize as ::wit_bindgen_wrpc::wrpc_transport::Decode<
                            R,
                        >>::Decoder,
                        v_size: ::core::option::Option<super::ObjectSize>,
                    }
                    #[automatically_derived]
                    impl<R> ::core::default::Default for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        fn default() -> Self {
                            Self {
                                c_name: ::core::default::Default::default(),
                                v_name: ::core::default::Default::default(),
                                c_container: ::core::default::Default::default(),
                                v_container: ::core::default::Default::default(),
                                c_created_at: ::core::default::Default::default(),
                                v_created_at: ::core::default::Default::default(),
                                c_size: ::core::default::Default::default(),
                                v_size: ::core::default::Default::default(),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<
                        R,
                    > ::wit_bindgen_wrpc::wrpc_transport::Deferred<
                        ::wit_bindgen_wrpc::wrpc_transport::Incoming<R>,
                    > for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        fn take_deferred(
                            &mut self,
                        ) -> ::core::option::Option<
                            ::wit_bindgen_wrpc::wrpc_transport::DeferredFn<
                                ::wit_bindgen_wrpc::wrpc_transport::Incoming<R>,
                            >,
                        > {
                            let f_name = self.c_name.take_deferred();
                            let f_container = self.c_container.take_deferred();
                            let f_created_at = self.c_created_at.take_deferred();
                            let f_size = self.c_size.take_deferred();
                            if false || f_name.is_some() || f_container.is_some()
                                || f_created_at.is_some() || f_size.is_some()
                            {
                                return Some(
                                    ::std::boxed::Box::new(|r, mut path| {
                                        let f_name = f_name
                                            .map(|f| {
                                                path.push(0);
                                                let r = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &r,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, r)
                                            });
                                        let f_container = f_container
                                            .map(|f| {
                                                path.push(1);
                                                let r = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &r,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, r)
                                            });
                                        let f_created_at = f_created_at
                                            .map(|f| {
                                                path.push(2);
                                                let r = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &r,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, r)
                                            });
                                        let f_size = f_size
                                            .map(|f| {
                                                path.push(3);
                                                let r = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &r,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, r)
                                            });
                                        ::std::boxed::Box::pin(async move {
                                            {
                                                use ::tokio::macros::support::{
                                                    maybe_done, poll_fn, Future, Pin,
                                                };
                                                use ::tokio::macros::support::Poll::{Ready, Pending};
                                                let mut futures = (
                                                    maybe_done(async {
                                                        match f_name {
                                                            Some((f, Ok(r))) => f(r, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                    maybe_done(async {
                                                        match f_container {
                                                            Some((f, Ok(r))) => f(r, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                    maybe_done(async {
                                                        match f_created_at {
                                                            Some((f, Ok(r))) => f(r, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                    maybe_done(async {
                                                        match f_size {
                                                            Some((f, Ok(r))) => f(r, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                );
                                                let mut futures = &mut futures;
                                                let mut skip_next_time: u32 = 0;
                                                poll_fn(move |cx| {
                                                        const COUNT: u32 = 0 + 1 + 1 + 1 + 1;
                                                        let mut is_pending = false;
                                                        let mut to_run = COUNT;
                                                        let mut skip = skip_next_time;
                                                        skip_next_time = if skip + 1 == COUNT {
                                                            0
                                                        } else {
                                                            skip + 1
                                                        };
                                                        loop {
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (_, fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (_, _, fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (_, _, _, fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                        }
                                                        if is_pending {
                                                            Pending
                                                        } else {
                                                            Ready(
                                                                Ok((
                                                                    {
                                                                        let (fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                    {
                                                                        let (_, fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                    {
                                                                        let (_, _, fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                    {
                                                                        let (_, _, _, fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                )),
                                                            )
                                                        }
                                                    })
                                                    .await
                                            }?;
                                            Ok(())
                                        })
                                    }),
                                );
                            }
                            None
                        }
                    }
                    #[automatically_derived]
                    impl<R> ::wit_bindgen_wrpc::tokio_util::codec::Decoder for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        type Item = super::ObjectMetadata;
                        type Error = ::std::io::Error;
                        fn decode(
                            &mut self,
                            src: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::core::result::Result<
                            ::core::option::Option<Self::Item>,
                            Self::Error,
                        > {
                            if self.v_name.is_none() {
                                let Some(v) = self.c_name.decode(src)? else {
                                    return Ok(None)
                                };
                                self.v_name = Some(v);
                            }
                            if self.v_container.is_none() {
                                let Some(v) = self.c_container.decode(src)? else {
                                    return Ok(None)
                                };
                                self.v_container = Some(v);
                            }
                            if self.v_created_at.is_none() {
                                let Some(v) = self.c_created_at.decode(src)? else {
                                    return Ok(None)
                                };
                                self.v_created_at = Some(v);
                            }
                            if self.v_size.is_none() {
                                let Some(v) = self.c_size.decode(src)? else {
                                    return Ok(None)
                                };
                                self.v_size = Some(v);
                            }
                            Ok(
                                Some(Self::Item {
                                    name: self.v_name.take().unwrap(),
                                    container: self.v_container.take().unwrap(),
                                    created_at: self.v_created_at.take().unwrap(),
                                    size: self.v_size.take().unwrap(),
                                }),
                            )
                        }
                    }
                    #[automatically_derived]
                    impl<
                        W,
                    > ::wit_bindgen_wrpc::tokio_util::codec::Encoder<
                        super::ObjectMetadata,
                    > for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        type Error = ::std::io::Error;
                        #[allow(unused_mut)]
                        fn encode(
                            &mut self,
                            item: super::ObjectMetadata,
                            mut dst: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::std::io::Result<()> {
                            let super::ObjectMetadata {
                                name: f_name,
                                container: f_container,
                                created_at: f_created_at,
                                size: f_size,
                            } = item;
                            self.name.encode(f_name, &mut dst)?;
                            self.container.encode(f_container, &mut dst)?;
                            self.created_at.encode(f_created_at, &mut dst)?;
                            self.size.encode(f_size, &mut dst)?;
                            Ok(())
                        }
                    }
                    #[automatically_derived]
                    impl<
                        W,
                    > ::wit_bindgen_wrpc::tokio_util::codec::Encoder<
                        &super::ObjectMetadata,
                    > for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        type Error = ::std::io::Error;
                        #[allow(unused_mut)]
                        fn encode(
                            &mut self,
                            item: &super::ObjectMetadata,
                            mut dst: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::std::io::Result<()> {
                            let super::ObjectMetadata {
                                name: f_name,
                                container: f_container,
                                created_at: f_created_at,
                                size: f_size,
                            } = item;
                            self.name.encode(f_name, &mut dst)?;
                            self.container.encode(f_container, &mut dst)?;
                            self.created_at.encode(f_created_at, &mut dst)?;
                            self.size.encode(f_size, &mut dst)?;
                            Ok(())
                        }
                    }
                }
                impl ::core::fmt::Debug for ObjectMetadata {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("ObjectMetadata")
                            .field("name", &self.name)
                            .field("container", &self.container)
                            .field("created-at", &self.created_at)
                            .field("size", &self.size)
                            .finish()
                    }
                }
                /// identifier for an object that includes its container name
                pub struct ObjectId {
                    pub container: ContainerName,
                    pub object: ObjectName,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ObjectId {
                    #[inline]
                    fn clone(&self) -> ObjectId {
                        ObjectId {
                            container: ::core::clone::Clone::clone(&self.container),
                            object: ::core::clone::Clone::clone(&self.object),
                        }
                    }
                }
                impl<W> ::wit_bindgen_wrpc::wrpc_transport::Encode<W> for &self::ObjectId
                where
                    W: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                        + ::core::marker::Unpin + 'static,
                {
                    type Encoder = object_id::Encoder<W>;
                }
                impl<W> ::wit_bindgen_wrpc::wrpc_transport::Encode<W> for self::ObjectId
                where
                    W: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                        + ::core::marker::Unpin + 'static,
                {
                    type Encoder = object_id::Encoder<W>;
                }
                impl<R> ::wit_bindgen_wrpc::wrpc_transport::Decode<R> for self::ObjectId
                where
                    R: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                        + ::core::marker::Unpin + 'static,
                {
                    type Decoder = object_id::Decoder<R>;
                    type ListDecoder = ::wit_bindgen_wrpc::wrpc_transport::ListDecoder<
                        Self::Decoder,
                        R,
                    >;
                }
                mod object_id {
                    pub struct Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        container: <super::ContainerName as ::wit_bindgen_wrpc::wrpc_transport::Encode<
                            W,
                        >>::Encoder,
                        object: <super::ObjectName as ::wit_bindgen_wrpc::wrpc_transport::Encode<
                            W,
                        >>::Encoder,
                    }
                    #[automatically_derived]
                    impl<W> ::core::default::Default for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        fn default() -> Self {
                            Self {
                                container: ::core::default::Default::default(),
                                object: ::core::default::Default::default(),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<W> ::wit_bindgen_wrpc::wrpc_transport::Deferred<W>
                    for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        fn take_deferred(
                            &mut self,
                        ) -> ::core::option::Option<
                            ::wit_bindgen_wrpc::wrpc_transport::DeferredFn<W>,
                        > {
                            let f_container = self.container.take_deferred();
                            let f_object = self.object.take_deferred();
                            if false || f_container.is_some() || f_object.is_some() {
                                return Some(
                                    ::std::boxed::Box::new(|w, mut path| {
                                        let f_container = f_container
                                            .map(|f| {
                                                path.push(0);
                                                let w = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &w,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, w)
                                            });
                                        let f_object = f_object
                                            .map(|f| {
                                                path.push(1);
                                                let w = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &w,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, w)
                                            });
                                        ::std::boxed::Box::pin(async move {
                                            {
                                                use ::tokio::macros::support::{
                                                    maybe_done, poll_fn, Future, Pin,
                                                };
                                                use ::tokio::macros::support::Poll::{Ready, Pending};
                                                let mut futures = (
                                                    maybe_done(async {
                                                        match f_container {
                                                            Some((f, Ok(w))) => f(w, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                    maybe_done(async {
                                                        match f_object {
                                                            Some((f, Ok(w))) => f(w, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                );
                                                let mut futures = &mut futures;
                                                let mut skip_next_time: u32 = 0;
                                                poll_fn(move |cx| {
                                                        const COUNT: u32 = 0 + 1 + 1;
                                                        let mut is_pending = false;
                                                        let mut to_run = COUNT;
                                                        let mut skip = skip_next_time;
                                                        skip_next_time = if skip + 1 == COUNT {
                                                            0
                                                        } else {
                                                            skip + 1
                                                        };
                                                        loop {
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (_, fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                        }
                                                        if is_pending {
                                                            Pending
                                                        } else {
                                                            Ready(
                                                                Ok((
                                                                    {
                                                                        let (fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                    {
                                                                        let (_, fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                )),
                                                            )
                                                        }
                                                    })
                                                    .await
                                            }?;
                                            Ok(())
                                        })
                                    }),
                                );
                            }
                            None
                        }
                    }
                    pub struct Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        c_container: <super::ContainerName as ::wit_bindgen_wrpc::wrpc_transport::Decode<
                            R,
                        >>::Decoder,
                        v_container: ::core::option::Option<super::ContainerName>,
                        c_object: <super::ObjectName as ::wit_bindgen_wrpc::wrpc_transport::Decode<
                            R,
                        >>::Decoder,
                        v_object: ::core::option::Option<super::ObjectName>,
                    }
                    #[automatically_derived]
                    impl<R> ::core::default::Default for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        fn default() -> Self {
                            Self {
                                c_container: ::core::default::Default::default(),
                                v_container: ::core::default::Default::default(),
                                c_object: ::core::default::Default::default(),
                                v_object: ::core::default::Default::default(),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<
                        R,
                    > ::wit_bindgen_wrpc::wrpc_transport::Deferred<
                        ::wit_bindgen_wrpc::wrpc_transport::Incoming<R>,
                    > for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        fn take_deferred(
                            &mut self,
                        ) -> ::core::option::Option<
                            ::wit_bindgen_wrpc::wrpc_transport::DeferredFn<
                                ::wit_bindgen_wrpc::wrpc_transport::Incoming<R>,
                            >,
                        > {
                            let f_container = self.c_container.take_deferred();
                            let f_object = self.c_object.take_deferred();
                            if false || f_container.is_some() || f_object.is_some() {
                                return Some(
                                    ::std::boxed::Box::new(|r, mut path| {
                                        let f_container = f_container
                                            .map(|f| {
                                                path.push(0);
                                                let r = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &r,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, r)
                                            });
                                        let f_object = f_object
                                            .map(|f| {
                                                path.push(1);
                                                let r = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &r,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, r)
                                            });
                                        ::std::boxed::Box::pin(async move {
                                            {
                                                use ::tokio::macros::support::{
                                                    maybe_done, poll_fn, Future, Pin,
                                                };
                                                use ::tokio::macros::support::Poll::{Ready, Pending};
                                                let mut futures = (
                                                    maybe_done(async {
                                                        match f_container {
                                                            Some((f, Ok(r))) => f(r, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                    maybe_done(async {
                                                        match f_object {
                                                            Some((f, Ok(r))) => f(r, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                );
                                                let mut futures = &mut futures;
                                                let mut skip_next_time: u32 = 0;
                                                poll_fn(move |cx| {
                                                        const COUNT: u32 = 0 + 1 + 1;
                                                        let mut is_pending = false;
                                                        let mut to_run = COUNT;
                                                        let mut skip = skip_next_time;
                                                        skip_next_time = if skip + 1 == COUNT {
                                                            0
                                                        } else {
                                                            skip + 1
                                                        };
                                                        loop {
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (_, fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                        }
                                                        if is_pending {
                                                            Pending
                                                        } else {
                                                            Ready(
                                                                Ok((
                                                                    {
                                                                        let (fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                    {
                                                                        let (_, fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                )),
                                                            )
                                                        }
                                                    })
                                                    .await
                                            }?;
                                            Ok(())
                                        })
                                    }),
                                );
                            }
                            None
                        }
                    }
                    #[automatically_derived]
                    impl<R> ::wit_bindgen_wrpc::tokio_util::codec::Decoder for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        type Item = super::ObjectId;
                        type Error = ::std::io::Error;
                        fn decode(
                            &mut self,
                            src: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::core::result::Result<
                            ::core::option::Option<Self::Item>,
                            Self::Error,
                        > {
                            if self.v_container.is_none() {
                                let Some(v) = self.c_container.decode(src)? else {
                                    return Ok(None)
                                };
                                self.v_container = Some(v);
                            }
                            if self.v_object.is_none() {
                                let Some(v) = self.c_object.decode(src)? else {
                                    return Ok(None)
                                };
                                self.v_object = Some(v);
                            }
                            Ok(
                                Some(Self::Item {
                                    container: self.v_container.take().unwrap(),
                                    object: self.v_object.take().unwrap(),
                                }),
                            )
                        }
                    }
                    #[automatically_derived]
                    impl<
                        W,
                    > ::wit_bindgen_wrpc::tokio_util::codec::Encoder<super::ObjectId>
                    for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        type Error = ::std::io::Error;
                        #[allow(unused_mut)]
                        fn encode(
                            &mut self,
                            item: super::ObjectId,
                            mut dst: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::std::io::Result<()> {
                            let super::ObjectId {
                                container: f_container,
                                object: f_object,
                            } = item;
                            self.container.encode(f_container, &mut dst)?;
                            self.object.encode(f_object, &mut dst)?;
                            Ok(())
                        }
                    }
                    #[automatically_derived]
                    impl<
                        W,
                    > ::wit_bindgen_wrpc::tokio_util::codec::Encoder<&super::ObjectId>
                    for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        type Error = ::std::io::Error;
                        #[allow(unused_mut)]
                        fn encode(
                            &mut self,
                            item: &super::ObjectId,
                            mut dst: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::std::io::Result<()> {
                            let super::ObjectId {
                                container: f_container,
                                object: f_object,
                            } = item;
                            self.container.encode(f_container, &mut dst)?;
                            self.object.encode(f_object, &mut dst)?;
                            Ok(())
                        }
                    }
                }
                impl ::core::fmt::Debug for ObjectId {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("ObjectId")
                            .field("container", &self.container)
                            .field("object", &self.object)
                            .finish()
                    }
                }
                /// A data is the data stored in a data blob. The value can be of any type
                /// that can be represented in a byte array. It provides a way to write the value
                /// to the output-stream defined in the `wasi-io` interface.
                /// Soon: switch to `resource value { ... }`
                #[repr(transparent)]
                pub struct OutgoingValue(());
                /// A incoming-value is a wrapper around a value. It provides a way to read the value
                /// from the input-stream defined in the `wasi-io` interface.
                ///
                /// The incoming-value provides two ways to consume the value:
                /// 1. `incoming-value-consume-sync` consumes the value synchronously and returns the
                /// value as a list of bytes.
                /// 2. `incoming-value-consume-async` consumes the value asynchronously and returns the
                /// value as an input-stream.
                /// Soon: switch to `resource incoming-value { ... }`
                #[repr(transparent)]
                pub struct IncomingValue(());
                pub type IncomingValueAsyncBody = InputStream;
                pub type IncomingValueSyncBody = ::wit_bindgen_wrpc::bytes::Bytes;
                impl OutgoingValue {
                    #[allow(clippy::all)]
                    pub fn new_outgoing_value<
                        'a,
                        C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                    >(
                        wrpc__: &'a C,
                        cx__: C::Context,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::wit_bindgen_wrpc::wrpc_transport::ResourceOwn<
                                OutgoingValue,
                            >,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:blobstore/types@0.2.0-draft",
                                            "outgoing-value.new-outgoing-value",
                                            (),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:blobstore/types@0.2.0-draft.[static]outgoing-value.new-outgoing-value`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl OutgoingValue {
                    #[allow(clippy::all)]
                    pub fn outgoing_value_write_body<
                        'a,
                        C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                    >(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            OutgoingValue,
                        >,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<
                                ::wit_bindgen_wrpc::wrpc_transport::ResourceOwn<
                                    OutputStream,
                                >,
                                (),
                            >,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:blobstore/types@0.2.0-draft",
                                            "outgoing-value.outgoing-value-write-body",
                                            (self_,),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:blobstore/types@0.2.0-draft.[method]outgoing-value.outgoing-value-write-body`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl IncomingValue {
                    #[allow(clippy::all)]
                    pub fn incoming_value_consume_sync<
                        'a,
                        C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                    >(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        this: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceOwn<
                            IncomingValue,
                        >,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<IncomingValueSyncBody, Error>,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:blobstore/types@0.2.0-draft",
                                            "incoming-value.incoming-value-consume-sync",
                                            (this,),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:blobstore/types@0.2.0-draft.[static]incoming-value.incoming-value-consume-sync`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl IncomingValue {
                    #[allow(clippy::all)]
                    pub fn incoming_value_consume_async<
                        'a,
                        C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                    >(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        this: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceOwn<
                            IncomingValue,
                        >,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<
                                ::wit_bindgen_wrpc::wrpc_transport::ResourceOwn<
                                    IncomingValueAsyncBody,
                                >,
                                Error,
                            >,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:blobstore/types@0.2.0-draft",
                                            "incoming-value.incoming-value-consume-async",
                                            (this,),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:blobstore/types@0.2.0-draft.[static]incoming-value.incoming-value-consume-async`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl IncomingValue {
                    #[allow(clippy::all)]
                    pub fn size<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            IncomingValue,
                        >,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<u64>,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:blobstore/types@0.2.0-draft",
                                            "incoming-value.size",
                                            (self_,),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:blobstore/types@0.2.0-draft.[method]incoming-value.size`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
            }
        }
        #[allow(dead_code)]
        pub mod io {
            #[allow(dead_code, clippy::all)]
            pub mod error {
                /// A resource which represents some error information.
                ///
                /// The only method provided by this resource is `to-debug-string`,
                /// which provides some human-readable information about the error.
                ///
                /// In the `wasi:io` package, this resource is returned through the
                /// `wasi:io/streams/stream-error` type.
                ///
                /// To provide more specific error information, other interfaces may
                /// provide functions to further "downcast" this error into more specific
                /// error information. For example, `error`s returned in streams derived
                /// from filesystem types to be described using the filesystem's own
                /// error-code type, using the function
                /// `wasi:filesystem/types/filesystem-error-code`, which takes a parameter
                /// `borrow<error>` and returns
                /// `option<wasi:filesystem/types/error-code>`.
                ///
                /// The set of functions which can "downcast" an `error` into a more
                /// concrete type is open.
                #[repr(transparent)]
                pub struct Error(());
                impl Error {
                    #[allow(clippy::all)]
                    /// Returns a string that is suitable to assist humans in debugging
                    /// this error.
                    ///
                    /// WARNING: The returned string should not be consumed mechanically!
                    /// It may change across platforms, hosts, or other implementation
                    /// details. Parsing this string is a major platform-compatibility
                    /// hazard.
                    pub fn to_debug_string<
                        'a,
                        C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                    >(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            Error,
                        >,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<String>,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/error@0.2.0",
                                            "error.to-debug-string",
                                            (self_,),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/error@0.2.0.[method]error.to-debug-string`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
            }
            #[allow(dead_code, clippy::all)]
            pub mod poll {
                /// `pollable` represents a single I/O event which may be ready, or not.
                #[repr(transparent)]
                pub struct Pollable(());
                impl Pollable {
                    #[allow(clippy::all)]
                    /// Return the readiness of a pollable. This function never blocks.
                    ///
                    /// Returns `true` when the pollable is ready, and `false` otherwise.
                    pub fn ready<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            Pollable,
                        >,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<bool>,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/poll@0.2.0",
                                            "pollable.ready",
                                            (self_,),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/poll@0.2.0.[method]pollable.ready`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl Pollable {
                    #[allow(clippy::all)]
                    /// `block` returns immediately if the pollable is ready, and otherwise
                    /// blocks until ready.
                    ///
                    /// This function is equivalent to calling `poll.poll` on a list
                    /// containing only this pollable.
                    pub fn block<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            Pollable,
                        >,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/poll@0.2.0",
                                            "pollable.block",
                                            (self_,),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/poll@0.2.0.[method]pollable.block`",
                            )?;
                            Ok(wrpc__)
                        }
                    }
                }
                #[allow(clippy::all)]
                /// Poll for completion on a set of pollables.
                ///
                /// This function takes a list of pollables, which identify I/O sources of
                /// interest, and waits until one or more of the events is ready for I/O.
                ///
                /// The result `list<u32>` contains one or more indices of handles in the
                /// argument list that is ready for I/O.
                ///
                /// If the list contains more elements than can be indexed with a `u32`
                /// value, this function traps.
                ///
                /// A timeout can be implemented by adding a pollable from the
                /// wasi-clocks API to the list.
                ///
                /// This function does not return a `result`; polling in itself does not
                /// do any I/O so it doesn't fail. If any of the I/O sources identified by
                /// the pollables has an error, it is indicated by marking the source as
                /// being reaedy for I/O.
                pub fn poll<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    in_: &'a [::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                        Pollable,
                    >],
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<Vec<u32>>,
                > + Send + 'a {
                    async move {
                        let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                        wrpc__,
                                        cx__,
                                        "wasi:io/poll@0.2.0",
                                        "poll",
                                        (in_,),
                                        [[None; 0]; 0],
                                    ),
                                )
                                .await,
                            "failed to invoke `wasi:io/poll@0.2.0.poll`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok(wrpc__)
                    }
                }
            }
            #[allow(dead_code, clippy::all)]
            pub mod streams {
                pub type Error = super::super::super::wasi::io::error::Error;
                pub type Pollable = super::super::super::wasi::io::poll::Pollable;
                /// An error for input-stream and output-stream operations.
                pub enum StreamError {
                    /// An error for input-stream and output-stream operations.
                    LastOperationFailed(
                        ::wit_bindgen_wrpc::wrpc_transport::ResourceOwn<Error>,
                    ),
                    /// An error for input-stream and output-stream operations.
                    Closed,
                }
                impl ::core::fmt::Debug for StreamError {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        match self {
                            StreamError::LastOperationFailed(e) => {
                                f.debug_tuple("StreamError::LastOperationFailed")
                                    .field(e)
                                    .finish()
                            }
                            StreamError::Closed => {
                                f.debug_tuple("StreamError::Closed").finish()
                            }
                        }
                    }
                }
                impl ::core::fmt::Display for StreamError {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.write_fmt(format_args!("{0:?}", self))
                    }
                }
                impl ::std::error::Error for StreamError {}
                impl<W> ::wit_bindgen_wrpc::wrpc_transport::Encode<W>
                for &self::StreamError
                where
                    W: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                        + ::core::marker::Unpin + 'static,
                {
                    type Encoder = stream_error::Encoder<W>;
                }
                impl<W> ::wit_bindgen_wrpc::wrpc_transport::Encode<W>
                for &&self::StreamError
                where
                    W: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                        + ::core::marker::Unpin + 'static,
                {
                    type Encoder = stream_error::Encoder<W>;
                }
                impl<W> ::wit_bindgen_wrpc::wrpc_transport::Encode<W>
                for self::StreamError
                where
                    W: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                        + ::core::marker::Unpin + 'static,
                {
                    type Encoder = stream_error::Encoder<W>;
                }
                impl<R> ::wit_bindgen_wrpc::wrpc_transport::Decode<R>
                for self::StreamError
                where
                    R: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                        + ::core::marker::Unpin + 'static,
                {
                    type Decoder = stream_error::Decoder<R>;
                    type ListDecoder = ::wit_bindgen_wrpc::wrpc_transport::ListDecoder<
                        Self::Decoder,
                        R,
                    >;
                }
                mod stream_error {
                    pub struct Encoder<W>(
                        ::core::option::Option<
                            ::wit_bindgen_wrpc::wrpc_transport::DeferredFn<W>,
                        >,
                    );
                    #[automatically_derived]
                    impl<W> ::core::default::Default for Encoder<W> {
                        fn default() -> Self {
                            Self(None)
                        }
                    }
                    #[automatically_derived]
                    impl<W> ::wit_bindgen_wrpc::wrpc_transport::Deferred<W>
                    for Encoder<W> {
                        fn take_deferred(
                            &mut self,
                        ) -> ::core::option::Option<
                            ::wit_bindgen_wrpc::wrpc_transport::DeferredFn<W>,
                        > {
                            self.0.take()
                        }
                    }
                    #[automatically_derived]
                    impl<
                        W,
                    > ::wit_bindgen_wrpc::tokio_util::codec::Encoder<super::StreamError>
                    for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        type Error = ::std::io::Error;
                        fn encode(
                            &mut self,
                            item: super::StreamError,
                            dst: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::core::result::Result<(), Self::Error> {
                            match item {
                                super::StreamError::LastOperationFailed(payload) => {
                                    ::wit_bindgen_wrpc::wasm_tokio::Leb128Encoder
                                        .encode(0_u32, dst)?;
                                    self.0 = ::wit_bindgen_wrpc::wrpc_transport::Encode::<
                                        W,
                                    >::encode(
                                        payload,
                                        &mut ::core::default::Default::default(),
                                        dst,
                                    )?;
                                    Ok(())
                                }
                                super::StreamError::Closed => {
                                    ::wit_bindgen_wrpc::wasm_tokio::Leb128Encoder
                                        .encode(1_u32, dst)
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<
                        W,
                    > ::wit_bindgen_wrpc::tokio_util::codec::Encoder<&super::StreamError>
                    for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        type Error = ::std::io::Error;
                        fn encode(
                            &mut self,
                            item: &super::StreamError,
                            dst: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::core::result::Result<(), Self::Error> {
                            match item {
                                super::StreamError::LastOperationFailed(payload) => {
                                    ::wit_bindgen_wrpc::wasm_tokio::Leb128Encoder
                                        .encode(0_u32, dst)?;
                                    self.0 = ::wit_bindgen_wrpc::wrpc_transport::Encode::<
                                        W,
                                    >::encode(
                                        payload,
                                        &mut ::core::default::Default::default(),
                                        dst,
                                    )?;
                                    Ok(())
                                }
                                super::StreamError::Closed => {
                                    ::wit_bindgen_wrpc::wasm_tokio::Leb128Encoder
                                        .encode(1_u32, dst)
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<
                        W,
                    > ::wit_bindgen_wrpc::tokio_util::codec::Encoder<
                        &&super::StreamError,
                    > for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        type Error = ::std::io::Error;
                        fn encode(
                            &mut self,
                            item: &&super::StreamError,
                            dst: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::core::result::Result<(), Self::Error> {
                            match item {
                                super::StreamError::LastOperationFailed(payload) => {
                                    ::wit_bindgen_wrpc::wasm_tokio::Leb128Encoder
                                        .encode(0_u32, dst)?;
                                    self.0 = ::wit_bindgen_wrpc::wrpc_transport::Encode::<
                                        W,
                                    >::encode(
                                        payload,
                                        &mut ::core::default::Default::default(),
                                        dst,
                                    )?;
                                    Ok(())
                                }
                                super::StreamError::Closed => {
                                    ::wit_bindgen_wrpc::wasm_tokio::Leb128Encoder
                                        .encode(1_u32, dst)
                                }
                            }
                        }
                    }
                    pub enum PayloadDecoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        LastOperationFailed(
                            <::wit_bindgen_wrpc::wrpc_transport::ResourceOwn<
                                super::Error,
                            > as ::wit_bindgen_wrpc::wrpc_transport::Decode<R>>::Decoder,
                        ),
                    }
                    pub enum Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        Payload(::core::option::Option<PayloadDecoder<R>>),
                        Deferred(
                            ::core::option::Option<
                                ::wit_bindgen_wrpc::wrpc_transport::DeferredFn<
                                    ::wit_bindgen_wrpc::wrpc_transport::Incoming<R>,
                                >,
                            >,
                        ),
                    }
                    #[automatically_derived]
                    impl<R> ::core::default::Default for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        fn default() -> Self {
                            Self::Payload(None)
                        }
                    }
                    #[automatically_derived]
                    impl<
                        R,
                    > ::wit_bindgen_wrpc::wrpc_transport::Deferred<
                        ::wit_bindgen_wrpc::wrpc_transport::Incoming<R>,
                    > for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        fn take_deferred(
                            &mut self,
                        ) -> ::core::option::Option<
                            ::wit_bindgen_wrpc::wrpc_transport::DeferredFn<
                                ::wit_bindgen_wrpc::wrpc_transport::Incoming<R>,
                            >,
                        > {
                            match self {
                                Self::Payload(None) => None,
                                Self::Payload(
                                    Some(PayloadDecoder::LastOperationFailed(ref mut dec)),
                                ) => dec.take_deferred(),
                                Self::Deferred(f) => {
                                    let f = f.take();
                                    *self = Self::Payload(None);
                                    f
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<R> ::wit_bindgen_wrpc::tokio_util::codec::Decoder for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        type Item = super::StreamError;
                        type Error = ::std::io::Error;
                        fn decode(
                            &mut self,
                            src: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::core::result::Result<
                            ::core::option::Option<Self::Item>,
                            Self::Error,
                        > {
                            let state = if let Self::Payload(Some(ref mut state)) = self {
                                state
                            } else {
                                let Some(disc) = ::wit_bindgen_wrpc::wasm_tokio::Leb128DecoderU32
                                    .decode(src)? else { return Ok(None) };
                                match disc {
                                    0 => {
                                        *self = Self::Payload(::core::option::Option::default());
                                        let Self::Payload(ref mut dec) = self else {
                                            ::core::panicking::panic(
                                                "internal error: entered unreachable code",
                                            )
                                        };
                                        dec.insert(
                                            PayloadDecoder::LastOperationFailed(
                                                ::core::default::Default::default(),
                                            ),
                                        )
                                    }
                                    1 => return Ok(Some(super::StreamError::Closed)),
                                    _ => {
                                        return Err(
                                            ::std::io::Error::new(
                                                ::std::io::ErrorKind::InvalidInput,
                                                ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(
                                                        format_args!("unknown variant discriminant `{0}`", disc),
                                                    );
                                                    res
                                                }),
                                            ),
                                        );
                                    }
                                }
                            };
                            match state {
                                PayloadDecoder::LastOperationFailed(dec) => {
                                    let Some(payload) = dec.decode(src)? else {
                                        return Ok(None)
                                    };
                                    *self = Self::Deferred(
                                        ::wit_bindgen_wrpc::wrpc_transport::Deferred::<
                                            ::wit_bindgen_wrpc::wrpc_transport::Incoming<R>,
                                        >::take_deferred(dec),
                                    );
                                    Ok(Some(super::StreamError::LastOperationFailed(payload)))
                                }
                            }
                        }
                    }
                }
                /// An input bytestream.
                ///
                /// `input-stream`s are *non-blocking* to the extent practical on underlying
                /// platforms. I/O operations always return promptly; if fewer bytes are
                /// promptly available than requested, they return the number of bytes promptly
                /// available, which could even be zero. To wait for data to be available,
                /// use the `subscribe` function to obtain a `pollable` which can be polled
                /// for using `wasi:io/poll`.
                #[repr(transparent)]
                pub struct InputStream(());
                /// An output bytestream.
                ///
                /// `output-stream`s are *non-blocking* to the extent practical on
                /// underlying platforms. Except where specified otherwise, I/O operations also
                /// always return promptly, after the number of bytes that can be written
                /// promptly, which could even be zero. To wait for the stream to be ready to
                /// accept data, the `subscribe` function to obtain a `pollable` which can be
                /// polled for using `wasi:io/poll`.
                #[repr(transparent)]
                pub struct OutputStream(());
                impl InputStream {
                    #[allow(clippy::all)]
                    /// Perform a non-blocking read from the stream.
                    ///
                    /// When the source of a `read` is binary data, the bytes from the source
                    /// are returned verbatim. When the source of a `read` is known to the
                    /// implementation to be text, bytes containing the UTF-8 encoding of the
                    /// text are returned.
                    ///
                    /// This function returns a list of bytes containing the read data,
                    /// when successful. The returned list will contain up to `len` bytes;
                    /// it may return fewer than requested, but not more. The list is
                    /// empty when no bytes are available for reading at this time. The
                    /// pollable given by `subscribe` will be ready when more bytes are
                    /// available.
                    ///
                    /// This function fails with a `stream-error` when the operation
                    /// encounters an error, giving `last-operation-failed`, or when the
                    /// stream is closed, giving `closed`.
                    ///
                    /// When the caller gives a `len` of 0, it represents a request to
                    /// read 0 bytes. If the stream is still open, this call should
                    /// succeed and return an empty list, or otherwise fail with `closed`.
                    ///
                    /// The `len` parameter is a `u64`, which could represent a list of u8 which
                    /// is not possible to allocate in wasm32, or not desirable to allocate as
                    /// as a return value by the callee. The callee may return a list of bytes
                    /// less than `len` in size while more bytes are available for reading.
                    pub fn read<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            InputStream,
                        >,
                        len: u64,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<
                                ::wit_bindgen_wrpc::bytes::Bytes,
                                StreamError,
                            >,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "input-stream.read",
                                            (self_, len),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]input-stream.read`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl InputStream {
                    #[allow(clippy::all)]
                    /// Read bytes from a stream, after blocking until at least one byte can
                    /// be read. Except for blocking, behavior is identical to `read`.
                    pub fn blocking_read<
                        'a,
                        C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                    >(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            InputStream,
                        >,
                        len: u64,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<
                                ::wit_bindgen_wrpc::bytes::Bytes,
                                StreamError,
                            >,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "input-stream.blocking-read",
                                            (self_, len),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]input-stream.blocking-read`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl InputStream {
                    #[allow(clippy::all)]
                    /// Skip bytes from a stream. Returns number of bytes skipped.
                    ///
                    /// Behaves identical to `read`, except instead of returning a list
                    /// of bytes, returns the number of bytes consumed from the stream.
                    pub fn skip<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            InputStream,
                        >,
                        len: u64,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<u64, StreamError>,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "input-stream.skip",
                                            (self_, len),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]input-stream.skip`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl InputStream {
                    #[allow(clippy::all)]
                    /// Skip bytes from a stream, after blocking until at least one byte
                    /// can be skipped. Except for blocking behavior, identical to `skip`.
                    pub fn blocking_skip<
                        'a,
                        C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                    >(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            InputStream,
                        >,
                        len: u64,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<u64, StreamError>,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "input-stream.blocking-skip",
                                            (self_, len),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]input-stream.blocking-skip`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl InputStream {
                    #[allow(clippy::all)]
                    /// Create a `pollable` which will resolve once either the specified stream
                    /// has bytes available to read or the other end of the stream has been
                    /// closed.
                    /// The created `pollable` is a child resource of the `input-stream`.
                    /// Implementations may trap if the `input-stream` is dropped before
                    /// all derived `pollable`s created with this function are dropped.
                    pub fn subscribe<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            InputStream,
                        >,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::wit_bindgen_wrpc::wrpc_transport::ResourceOwn<Pollable>,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "input-stream.subscribe",
                                            (self_,),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]input-stream.subscribe`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl OutputStream {
                    #[allow(clippy::all)]
                    /// Check readiness for writing. This function never blocks.
                    ///
                    /// Returns the number of bytes permitted for the next call to `write`,
                    /// or an error. Calling `write` with more bytes than this function has
                    /// permitted will trap.
                    ///
                    /// When this function returns 0 bytes, the `subscribe` pollable will
                    /// become ready when this function will report at least 1 byte, or an
                    /// error.
                    pub fn check_write<
                        'a,
                        C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                    >(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            OutputStream,
                        >,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<u64, StreamError>,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "output-stream.check-write",
                                            (self_,),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]output-stream.check-write`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl OutputStream {
                    #[allow(clippy::all)]
                    /// Perform a write. This function never blocks.
                    ///
                    /// When the destination of a `write` is binary data, the bytes from
                    /// `contents` are written verbatim. When the destination of a `write` is
                    /// known to the implementation to be text, the bytes of `contents` are
                    /// transcoded from UTF-8 into the encoding of the destination and then
                    /// written.
                    ///
                    /// Precondition: check-write gave permit of Ok(n) and contents has a
                    /// length of less than or equal to n. Otherwise, this function will trap.
                    ///
                    /// returns Err(closed) without writing if the stream has closed since
                    /// the last call to check-write provided a permit.
                    pub fn write<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            OutputStream,
                        >,
                        contents: &'a ::wit_bindgen_wrpc::bytes::Bytes,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<(), StreamError>,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "output-stream.write",
                                            (self_, contents),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]output-stream.write`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl OutputStream {
                    #[allow(clippy::all)]
                    /// Perform a write of up to 4096 bytes, and then flush the stream. Block
                    /// until all of these operations are complete, or an error occurs.
                    ///
                    /// This is a convenience wrapper around the use of `check-write`,
                    /// `subscribe`, `write`, and `flush`, and is implemented with the
                    /// following pseudo-code:
                    ///
                    /// ```text
                    /// let pollable = this.subscribe();
                    /// while !contents.is_empty() {
                    /// // Wait for the stream to become writable
                    /// pollable.block();
                    /// let Ok(n) = this.check-write(); // eliding error handling
                    /// let len = min(n, contents.len());
                    /// let (chunk, rest) = contents.split_at(len);
                    /// this.write(chunk  );            // eliding error handling
                    /// contents = rest;
                    /// }
                    /// this.flush();
                    /// // Wait for completion of `flush`
                    /// pollable.block();
                    /// // Check for any errors that arose during `flush`
                    /// let _ = this.check-write();         // eliding error handling
                    /// ```
                    pub fn blocking_write_and_flush<
                        'a,
                        C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                    >(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            OutputStream,
                        >,
                        contents: &'a ::wit_bindgen_wrpc::bytes::Bytes,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<(), StreamError>,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "output-stream.blocking-write-and-flush",
                                            (self_, contents),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]output-stream.blocking-write-and-flush`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl OutputStream {
                    #[allow(clippy::all)]
                    /// Request to flush buffered output. This function never blocks.
                    ///
                    /// This tells the output-stream that the caller intends any buffered
                    /// output to be flushed. the output which is expected to be flushed
                    /// is all that has been passed to `write` prior to this call.
                    ///
                    /// Upon calling this function, the `output-stream` will not accept any
                    /// writes (`check-write` will return `ok(0)`) until the flush has
                    /// completed. The `subscribe` pollable will become ready when the
                    /// flush has completed and the stream can accept more writes.
                    pub fn flush<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            OutputStream,
                        >,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<(), StreamError>,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "output-stream.flush",
                                            (self_,),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]output-stream.flush`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl OutputStream {
                    #[allow(clippy::all)]
                    /// Request to flush buffered output, and block until flush completes
                    /// and stream is ready for writing again.
                    pub fn blocking_flush<
                        'a,
                        C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                    >(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            OutputStream,
                        >,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<(), StreamError>,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "output-stream.blocking-flush",
                                            (self_,),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]output-stream.blocking-flush`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl OutputStream {
                    #[allow(clippy::all)]
                    /// Create a `pollable` which will resolve once the output-stream
                    /// is ready for more writing, or an error has occured. When this
                    /// pollable is ready, `check-write` will return `ok(n)` with n>0, or an
                    /// error.
                    ///
                    /// If the stream is closed, this pollable is always ready immediately.
                    ///
                    /// The created `pollable` is a child resource of the `output-stream`.
                    /// Implementations may trap if the `output-stream` is dropped before
                    /// all derived `pollable`s created with this function are dropped.
                    pub fn subscribe<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            OutputStream,
                        >,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::wit_bindgen_wrpc::wrpc_transport::ResourceOwn<Pollable>,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "output-stream.subscribe",
                                            (self_,),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]output-stream.subscribe`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl OutputStream {
                    #[allow(clippy::all)]
                    /// Write zeroes to a stream.
                    ///
                    /// This should be used precisely like `write` with the exact same
                    /// preconditions (must use check-write first), but instead of
                    /// passing a list of bytes, you simply pass the number of zero-bytes
                    /// that should be written.
                    pub fn write_zeroes<
                        'a,
                        C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                    >(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            OutputStream,
                        >,
                        len: u64,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<(), StreamError>,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "output-stream.write-zeroes",
                                            (self_, len),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]output-stream.write-zeroes`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl OutputStream {
                    #[allow(clippy::all)]
                    /// Perform a write of up to 4096 zeroes, and then flush the stream.
                    /// Block until all of these operations are complete, or an error
                    /// occurs.
                    ///
                    /// This is a convenience wrapper around the use of `check-write`,
                    /// `subscribe`, `write-zeroes`, and `flush`, and is implemented with
                    /// the following pseudo-code:
                    ///
                    /// ```text
                    /// let pollable = this.subscribe();
                    /// while num_zeroes != 0 {
                    /// // Wait for the stream to become writable
                    /// pollable.block();
                    /// let Ok(n) = this.check-write(); // eliding error handling
                    /// let len = min(n, num_zeroes);
                    /// this.write-zeroes(len);         // eliding error handling
                    /// num_zeroes -= len;
                    /// }
                    /// this.flush();
                    /// // Wait for completion of `flush`
                    /// pollable.block();
                    /// // Check for any errors that arose during `flush`
                    /// let _ = this.check-write();         // eliding error handling
                    /// ```
                    pub fn blocking_write_zeroes_and_flush<
                        'a,
                        C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                    >(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            OutputStream,
                        >,
                        len: u64,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<(), StreamError>,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "output-stream.blocking-write-zeroes-and-flush",
                                            (self_, len),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]output-stream.blocking-write-zeroes-and-flush`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl OutputStream {
                    #[allow(clippy::all)]
                    /// Read from one stream and write to another.
                    ///
                    /// The behavior of splice is equivelant to:
                    /// 1. calling `check-write` on the `output-stream`
                    /// 2. calling `read` on the `input-stream` with the smaller of the
                    /// `check-write` permitted length and the `len` provided to `splice`
                    /// 3. calling `write` on the `output-stream` with that read data.
                    ///
                    /// Any error reported by the call to `check-write`, `read`, or
                    /// `write` ends the splice and reports that error.
                    ///
                    /// This function returns the number of bytes transferred; it may be less
                    /// than `len`.
                    pub fn splice<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            OutputStream,
                        >,
                        src: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            InputStream,
                        >,
                        len: u64,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<u64, StreamError>,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "output-stream.splice",
                                            (self_, src, len),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]output-stream.splice`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
                impl OutputStream {
                    #[allow(clippy::all)]
                    /// Read from one stream and write to another, with blocking.
                    ///
                    /// This is similar to `splice`, except that it blocks until the
                    /// `output-stream` is ready for writing, and the `input-stream`
                    /// is ready for reading, before performing the `splice`.
                    pub fn blocking_splice<
                        'a,
                        C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                    >(
                        wrpc__: &'a C,
                        cx__: C::Context,
                        self_: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            OutputStream,
                        >,
                        src: &'a ::wit_bindgen_wrpc::wrpc_transport::ResourceBorrow<
                            InputStream,
                        >,
                        len: u64,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            ::core::result::Result<u64, StreamError>,
                        >,
                    > + Send + 'a {
                        async move {
                            let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                                ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                        ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                            wrpc__,
                                            cx__,
                                            "wasi:io/streams@0.2.0",
                                            "output-stream.blocking-splice",
                                            (self_, src, len),
                                            [[None; 0]; 0],
                                        ),
                                    )
                                    .await,
                                "failed to invoke `wasi:io/streams@0.2.0.[method]output-stream.blocking-splice`",
                            )?;
                            let (wrpc__,) = wrpc__;
                            Ok(wrpc__)
                        }
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod wrpc {
        #[allow(dead_code)]
        pub mod blobstore {
            #[allow(dead_code, clippy::all)]
            pub mod types {
                pub type WasiObjectId = super::super::super::wasi::blobstore::types::ObjectId;
                pub type Timestamp = super::super::super::wasi::blobstore::types::Timestamp;
                pub type ObjectSize = super::super::super::wasi::blobstore::types::ObjectSize;
                /// information about a container
                #[repr(C)]
                pub struct ContainerMetadata {
                    /// date and time container was created
                    pub created_at: Timestamp,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ContainerMetadata {
                    #[inline]
                    fn clone(&self) -> ContainerMetadata {
                        let _: ::core::clone::AssertParamIsClone<Timestamp>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for ContainerMetadata {}
                impl<W> ::wit_bindgen_wrpc::wrpc_transport::Encode<W>
                for &self::ContainerMetadata
                where
                    W: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                        + ::core::marker::Unpin + 'static,
                {
                    type Encoder = container_metadata::Encoder<W>;
                }
                impl<W> ::wit_bindgen_wrpc::wrpc_transport::Encode<W>
                for self::ContainerMetadata
                where
                    W: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                        + ::core::marker::Unpin + 'static,
                {
                    type Encoder = container_metadata::Encoder<W>;
                }
                impl<R> ::wit_bindgen_wrpc::wrpc_transport::Decode<R>
                for self::ContainerMetadata
                where
                    R: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                        + ::core::marker::Unpin + 'static,
                {
                    type Decoder = container_metadata::Decoder<R>;
                    type ListDecoder = ::wit_bindgen_wrpc::wrpc_transport::ListDecoder<
                        Self::Decoder,
                        R,
                    >;
                }
                mod container_metadata {
                    pub struct Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        created_at: <super::Timestamp as ::wit_bindgen_wrpc::wrpc_transport::Encode<
                            W,
                        >>::Encoder,
                    }
                    #[automatically_derived]
                    impl<W> ::core::default::Default for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        fn default() -> Self {
                            Self {
                                created_at: ::core::default::Default::default(),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<W> ::wit_bindgen_wrpc::wrpc_transport::Deferred<W>
                    for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        fn take_deferred(
                            &mut self,
                        ) -> ::core::option::Option<
                            ::wit_bindgen_wrpc::wrpc_transport::DeferredFn<W>,
                        > {
                            let f_created_at = self.created_at.take_deferred();
                            if false || f_created_at.is_some() {
                                return Some(
                                    ::std::boxed::Box::new(|w, mut path| {
                                        let f_created_at = f_created_at
                                            .map(|f| {
                                                path.push(0);
                                                let w = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &w,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, w)
                                            });
                                        ::std::boxed::Box::pin(async move {
                                            {
                                                use ::tokio::macros::support::{
                                                    maybe_done, poll_fn, Future, Pin,
                                                };
                                                use ::tokio::macros::support::Poll::{Ready, Pending};
                                                let mut futures = (
                                                    maybe_done(async {
                                                        match f_created_at {
                                                            Some((f, Ok(w))) => f(w, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                );
                                                let mut futures = &mut futures;
                                                let mut skip_next_time: u32 = 0;
                                                poll_fn(move |cx| {
                                                        const COUNT: u32 = 0 + 1;
                                                        let mut is_pending = false;
                                                        let mut to_run = COUNT;
                                                        let mut skip = skip_next_time;
                                                        skip_next_time = if skip + 1 == COUNT {
                                                            0
                                                        } else {
                                                            skip + 1
                                                        };
                                                        loop {
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                        }
                                                        if is_pending {
                                                            Pending
                                                        } else {
                                                            Ready(
                                                                Ok((
                                                                    {
                                                                        let (fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                )),
                                                            )
                                                        }
                                                    })
                                                    .await
                                            }?;
                                            Ok(())
                                        })
                                    }),
                                );
                            }
                            None
                        }
                    }
                    pub struct Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        c_created_at: <super::Timestamp as ::wit_bindgen_wrpc::wrpc_transport::Decode<
                            R,
                        >>::Decoder,
                        v_created_at: ::core::option::Option<super::Timestamp>,
                    }
                    #[automatically_derived]
                    impl<R> ::core::default::Default for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        fn default() -> Self {
                            Self {
                                c_created_at: ::core::default::Default::default(),
                                v_created_at: ::core::default::Default::default(),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<
                        R,
                    > ::wit_bindgen_wrpc::wrpc_transport::Deferred<
                        ::wit_bindgen_wrpc::wrpc_transport::Incoming<R>,
                    > for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        fn take_deferred(
                            &mut self,
                        ) -> ::core::option::Option<
                            ::wit_bindgen_wrpc::wrpc_transport::DeferredFn<
                                ::wit_bindgen_wrpc::wrpc_transport::Incoming<R>,
                            >,
                        > {
                            let f_created_at = self.c_created_at.take_deferred();
                            if false || f_created_at.is_some() {
                                return Some(
                                    ::std::boxed::Box::new(|r, mut path| {
                                        let f_created_at = f_created_at
                                            .map(|f| {
                                                path.push(0);
                                                let r = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &r,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, r)
                                            });
                                        ::std::boxed::Box::pin(async move {
                                            {
                                                use ::tokio::macros::support::{
                                                    maybe_done, poll_fn, Future, Pin,
                                                };
                                                use ::tokio::macros::support::Poll::{Ready, Pending};
                                                let mut futures = (
                                                    maybe_done(async {
                                                        match f_created_at {
                                                            Some((f, Ok(r))) => f(r, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                );
                                                let mut futures = &mut futures;
                                                let mut skip_next_time: u32 = 0;
                                                poll_fn(move |cx| {
                                                        const COUNT: u32 = 0 + 1;
                                                        let mut is_pending = false;
                                                        let mut to_run = COUNT;
                                                        let mut skip = skip_next_time;
                                                        skip_next_time = if skip + 1 == COUNT {
                                                            0
                                                        } else {
                                                            skip + 1
                                                        };
                                                        loop {
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                        }
                                                        if is_pending {
                                                            Pending
                                                        } else {
                                                            Ready(
                                                                Ok((
                                                                    {
                                                                        let (fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                )),
                                                            )
                                                        }
                                                    })
                                                    .await
                                            }?;
                                            Ok(())
                                        })
                                    }),
                                );
                            }
                            None
                        }
                    }
                    #[automatically_derived]
                    impl<R> ::wit_bindgen_wrpc::tokio_util::codec::Decoder for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        type Item = super::ContainerMetadata;
                        type Error = ::std::io::Error;
                        fn decode(
                            &mut self,
                            src: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::core::result::Result<
                            ::core::option::Option<Self::Item>,
                            Self::Error,
                        > {
                            if self.v_created_at.is_none() {
                                let Some(v) = self.c_created_at.decode(src)? else {
                                    return Ok(None)
                                };
                                self.v_created_at = Some(v);
                            }
                            Ok(
                                Some(Self::Item {
                                    created_at: self.v_created_at.take().unwrap(),
                                }),
                            )
                        }
                    }
                    #[automatically_derived]
                    impl<
                        W,
                    > ::wit_bindgen_wrpc::tokio_util::codec::Encoder<
                        super::ContainerMetadata,
                    > for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        type Error = ::std::io::Error;
                        #[allow(unused_mut)]
                        fn encode(
                            &mut self,
                            item: super::ContainerMetadata,
                            mut dst: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::std::io::Result<()> {
                            let super::ContainerMetadata { created_at: f_created_at } = item;
                            self.created_at.encode(f_created_at, &mut dst)?;
                            Ok(())
                        }
                    }
                    #[automatically_derived]
                    impl<
                        W,
                    > ::wit_bindgen_wrpc::tokio_util::codec::Encoder<
                        &super::ContainerMetadata,
                    > for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        type Error = ::std::io::Error;
                        #[allow(unused_mut)]
                        fn encode(
                            &mut self,
                            item: &super::ContainerMetadata,
                            mut dst: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::std::io::Result<()> {
                            let super::ContainerMetadata { created_at: f_created_at } = item;
                            self.created_at.encode(f_created_at, &mut dst)?;
                            Ok(())
                        }
                    }
                }
                impl ::core::fmt::Debug for ContainerMetadata {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("ContainerMetadata")
                            .field("created-at", &self.created_at)
                            .finish()
                    }
                }
                pub type ObjectId = WasiObjectId;
                /// information about an object
                #[repr(C)]
                pub struct ObjectMetadata {
                    /// date and time the object was created
                    pub created_at: Timestamp,
                    /// size of the object, in bytes
                    pub size: ObjectSize,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ObjectMetadata {
                    #[inline]
                    fn clone(&self) -> ObjectMetadata {
                        let _: ::core::clone::AssertParamIsClone<Timestamp>;
                        let _: ::core::clone::AssertParamIsClone<ObjectSize>;
                        *self
                    }
                }
                #[automatically_derived]
                impl ::core::marker::Copy for ObjectMetadata {}
                impl<W> ::wit_bindgen_wrpc::wrpc_transport::Encode<W>
                for &self::ObjectMetadata
                where
                    W: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                        + ::core::marker::Unpin + 'static,
                {
                    type Encoder = object_metadata::Encoder<W>;
                }
                impl<W> ::wit_bindgen_wrpc::wrpc_transport::Encode<W>
                for self::ObjectMetadata
                where
                    W: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                        + ::core::marker::Unpin + 'static,
                {
                    type Encoder = object_metadata::Encoder<W>;
                }
                impl<R> ::wit_bindgen_wrpc::wrpc_transport::Decode<R>
                for self::ObjectMetadata
                where
                    R: ::core::marker::Send + ::core::marker::Sync
                        + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                        + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                        + ::core::marker::Unpin + 'static,
                {
                    type Decoder = object_metadata::Decoder<R>;
                    type ListDecoder = ::wit_bindgen_wrpc::wrpc_transport::ListDecoder<
                        Self::Decoder,
                        R,
                    >;
                }
                mod object_metadata {
                    pub struct Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        created_at: <super::Timestamp as ::wit_bindgen_wrpc::wrpc_transport::Encode<
                            W,
                        >>::Encoder,
                        size: <super::ObjectSize as ::wit_bindgen_wrpc::wrpc_transport::Encode<
                            W,
                        >>::Encoder,
                    }
                    #[automatically_derived]
                    impl<W> ::core::default::Default for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        fn default() -> Self {
                            Self {
                                created_at: ::core::default::Default::default(),
                                size: ::core::default::Default::default(),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<W> ::wit_bindgen_wrpc::wrpc_transport::Deferred<W>
                    for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        fn take_deferred(
                            &mut self,
                        ) -> ::core::option::Option<
                            ::wit_bindgen_wrpc::wrpc_transport::DeferredFn<W>,
                        > {
                            let f_created_at = self.created_at.take_deferred();
                            let f_size = self.size.take_deferred();
                            if false || f_created_at.is_some() || f_size.is_some() {
                                return Some(
                                    ::std::boxed::Box::new(|w, mut path| {
                                        let f_created_at = f_created_at
                                            .map(|f| {
                                                path.push(0);
                                                let w = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &w,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, w)
                                            });
                                        let f_size = f_size
                                            .map(|f| {
                                                path.push(1);
                                                let w = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &w,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, w)
                                            });
                                        ::std::boxed::Box::pin(async move {
                                            {
                                                use ::tokio::macros::support::{
                                                    maybe_done, poll_fn, Future, Pin,
                                                };
                                                use ::tokio::macros::support::Poll::{Ready, Pending};
                                                let mut futures = (
                                                    maybe_done(async {
                                                        match f_created_at {
                                                            Some((f, Ok(w))) => f(w, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                    maybe_done(async {
                                                        match f_size {
                                                            Some((f, Ok(w))) => f(w, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                );
                                                let mut futures = &mut futures;
                                                let mut skip_next_time: u32 = 0;
                                                poll_fn(move |cx| {
                                                        const COUNT: u32 = 0 + 1 + 1;
                                                        let mut is_pending = false;
                                                        let mut to_run = COUNT;
                                                        let mut skip = skip_next_time;
                                                        skip_next_time = if skip + 1 == COUNT {
                                                            0
                                                        } else {
                                                            skip + 1
                                                        };
                                                        loop {
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (_, fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                        }
                                                        if is_pending {
                                                            Pending
                                                        } else {
                                                            Ready(
                                                                Ok((
                                                                    {
                                                                        let (fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                    {
                                                                        let (_, fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                )),
                                                            )
                                                        }
                                                    })
                                                    .await
                                            }?;
                                            Ok(())
                                        })
                                    }),
                                );
                            }
                            None
                        }
                    }
                    pub struct Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        c_created_at: <super::Timestamp as ::wit_bindgen_wrpc::wrpc_transport::Decode<
                            R,
                        >>::Decoder,
                        v_created_at: ::core::option::Option<super::Timestamp>,
                        c_size: <super::ObjectSize as ::wit_bindgen_wrpc::wrpc_transport::Decode<
                            R,
                        >>::Decoder,
                        v_size: ::core::option::Option<super::ObjectSize>,
                    }
                    #[automatically_derived]
                    impl<R> ::core::default::Default for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        fn default() -> Self {
                            Self {
                                c_created_at: ::core::default::Default::default(),
                                v_created_at: ::core::default::Default::default(),
                                c_size: ::core::default::Default::default(),
                                v_size: ::core::default::Default::default(),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<
                        R,
                    > ::wit_bindgen_wrpc::wrpc_transport::Deferred<
                        ::wit_bindgen_wrpc::wrpc_transport::Incoming<R>,
                    > for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        fn take_deferred(
                            &mut self,
                        ) -> ::core::option::Option<
                            ::wit_bindgen_wrpc::wrpc_transport::DeferredFn<
                                ::wit_bindgen_wrpc::wrpc_transport::Incoming<R>,
                            >,
                        > {
                            let f_created_at = self.c_created_at.take_deferred();
                            let f_size = self.c_size.take_deferred();
                            if false || f_created_at.is_some() || f_size.is_some() {
                                return Some(
                                    ::std::boxed::Box::new(|r, mut path| {
                                        let f_created_at = f_created_at
                                            .map(|f| {
                                                path.push(0);
                                                let r = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &r,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, r)
                                            });
                                        let f_size = f_size
                                            .map(|f| {
                                                path.push(1);
                                                let r = ::wit_bindgen_wrpc::wrpc_transport::Index::index(
                                                    &r,
                                                    &path,
                                                );
                                                path.pop();
                                                (f, r)
                                            });
                                        ::std::boxed::Box::pin(async move {
                                            {
                                                use ::tokio::macros::support::{
                                                    maybe_done, poll_fn, Future, Pin,
                                                };
                                                use ::tokio::macros::support::Poll::{Ready, Pending};
                                                let mut futures = (
                                                    maybe_done(async {
                                                        match f_created_at {
                                                            Some((f, Ok(r))) => f(r, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                    maybe_done(async {
                                                        match f_size {
                                                            Some((f, Ok(r))) => f(r, Vec::default()).await,
                                                            Some((_, Err(err))) => Err(std::io::Error::other(err)),
                                                            None => Ok(()),
                                                        }
                                                    }),
                                                );
                                                let mut futures = &mut futures;
                                                let mut skip_next_time: u32 = 0;
                                                poll_fn(move |cx| {
                                                        const COUNT: u32 = 0 + 1 + 1;
                                                        let mut is_pending = false;
                                                        let mut to_run = COUNT;
                                                        let mut skip = skip_next_time;
                                                        skip_next_time = if skip + 1 == COUNT {
                                                            0
                                                        } else {
                                                            skip + 1
                                                        };
                                                        loop {
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                            if skip == 0 {
                                                                if to_run == 0 {
                                                                    break;
                                                                }
                                                                to_run -= 1;
                                                                let (_, fut, ..) = &mut *futures;
                                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                if fut.as_mut().poll(cx).is_pending() {
                                                                    is_pending = true;
                                                                } else if fut
                                                                    .as_mut()
                                                                    .output_mut()
                                                                    .expect("expected completed future")
                                                                    .is_err()
                                                                {
                                                                    return Ready(
                                                                        Err(
                                                                            fut
                                                                                .take_output()
                                                                                .expect("expected completed future")
                                                                                .err()
                                                                                .unwrap(),
                                                                        ),
                                                                    )
                                                                }
                                                            } else {
                                                                skip -= 1;
                                                            }
                                                        }
                                                        if is_pending {
                                                            Pending
                                                        } else {
                                                            Ready(
                                                                Ok((
                                                                    {
                                                                        let (fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                    {
                                                                        let (_, fut, ..) = &mut futures;
                                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                                        fut.take_output()
                                                                            .expect("expected completed future")
                                                                            .ok()
                                                                            .expect("expected Ok(_)")
                                                                    },
                                                                )),
                                                            )
                                                        }
                                                    })
                                                    .await
                                            }?;
                                            Ok(())
                                        })
                                    }),
                                );
                            }
                            None
                        }
                    }
                    #[automatically_derived]
                    impl<R> ::wit_bindgen_wrpc::tokio_util::codec::Decoder for Decoder<R>
                    where
                        R: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<R>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncRead
                            + ::core::marker::Unpin + 'static,
                    {
                        type Item = super::ObjectMetadata;
                        type Error = ::std::io::Error;
                        fn decode(
                            &mut self,
                            src: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::core::result::Result<
                            ::core::option::Option<Self::Item>,
                            Self::Error,
                        > {
                            if self.v_created_at.is_none() {
                                let Some(v) = self.c_created_at.decode(src)? else {
                                    return Ok(None)
                                };
                                self.v_created_at = Some(v);
                            }
                            if self.v_size.is_none() {
                                let Some(v) = self.c_size.decode(src)? else {
                                    return Ok(None)
                                };
                                self.v_size = Some(v);
                            }
                            Ok(
                                Some(Self::Item {
                                    created_at: self.v_created_at.take().unwrap(),
                                    size: self.v_size.take().unwrap(),
                                }),
                            )
                        }
                    }
                    #[automatically_derived]
                    impl<
                        W,
                    > ::wit_bindgen_wrpc::tokio_util::codec::Encoder<
                        super::ObjectMetadata,
                    > for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        type Error = ::std::io::Error;
                        #[allow(unused_mut)]
                        fn encode(
                            &mut self,
                            item: super::ObjectMetadata,
                            mut dst: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::std::io::Result<()> {
                            let super::ObjectMetadata {
                                created_at: f_created_at,
                                size: f_size,
                            } = item;
                            self.created_at.encode(f_created_at, &mut dst)?;
                            self.size.encode(f_size, &mut dst)?;
                            Ok(())
                        }
                    }
                    #[automatically_derived]
                    impl<
                        W,
                    > ::wit_bindgen_wrpc::tokio_util::codec::Encoder<
                        &super::ObjectMetadata,
                    > for Encoder<W>
                    where
                        W: ::core::marker::Send + ::core::marker::Sync
                            + ::wit_bindgen_wrpc::wrpc_transport::Index<W>
                            + ::wit_bindgen_wrpc::tokio::io::AsyncWrite
                            + ::core::marker::Unpin + 'static,
                    {
                        type Error = ::std::io::Error;
                        #[allow(unused_mut)]
                        fn encode(
                            &mut self,
                            item: &super::ObjectMetadata,
                            mut dst: &mut ::wit_bindgen_wrpc::bytes::BytesMut,
                        ) -> ::std::io::Result<()> {
                            let super::ObjectMetadata {
                                created_at: f_created_at,
                                size: f_size,
                            } = item;
                            self.created_at.encode(f_created_at, &mut dst)?;
                            self.size.encode(f_size, &mut dst)?;
                            Ok(())
                        }
                    }
                }
                impl ::core::fmt::Debug for ObjectMetadata {
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter<'_>,
                    ) -> ::core::fmt::Result {
                        f.debug_struct("ObjectMetadata")
                            .field("created-at", &self.created_at)
                            .field("size", &self.size)
                            .finish()
                    }
                }
            }
            #[allow(dead_code, clippy::all)]
            pub mod blobstore {
                pub type ContainerMetadata = super::super::super::wrpc::blobstore::types::ContainerMetadata;
                pub type ObjectMetadata = super::super::super::wrpc::blobstore::types::ObjectMetadata;
                pub type ObjectId = super::super::super::wrpc::blobstore::types::ObjectId;
                #[allow(clippy::all)]
                pub fn clear_container<
                    'a,
                    C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                >(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    name: &'a str,
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<
                        ::core::result::Result<(), String>,
                    >,
                > + Send + 'a {
                    async move {
                        let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                        wrpc__,
                                        cx__,
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "clear-container",
                                        (name,),
                                        [[None; 0]; 0],
                                    ),
                                )
                                .await,
                            "failed to invoke `wrpc:blobstore/blobstore@0.2.0.clear-container`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok(wrpc__)
                    }
                }
                #[allow(clippy::all)]
                pub fn container_exists<
                    'a,
                    C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                >(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    name: &'a str,
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<
                        ::core::result::Result<bool, String>,
                    >,
                > + Send + 'a {
                    async move {
                        let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                        wrpc__,
                                        cx__,
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "container-exists",
                                        (name,),
                                        [[None; 0]; 0],
                                    ),
                                )
                                .await,
                            "failed to invoke `wrpc:blobstore/blobstore@0.2.0.container-exists`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok(wrpc__)
                    }
                }
                #[allow(clippy::all)]
                pub fn create_container<
                    'a,
                    C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                >(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    name: &'a str,
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<
                        ::core::result::Result<(), String>,
                    >,
                > + Send + 'a {
                    async move {
                        let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                        wrpc__,
                                        cx__,
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "create-container",
                                        (name,),
                                        [[None; 0]; 0],
                                    ),
                                )
                                .await,
                            "failed to invoke `wrpc:blobstore/blobstore@0.2.0.create-container`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok(wrpc__)
                    }
                }
                #[allow(clippy::all)]
                pub fn delete_container<
                    'a,
                    C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                >(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    name: &'a str,
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<
                        ::core::result::Result<(), String>,
                    >,
                > + Send + 'a {
                    async move {
                        let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                        wrpc__,
                                        cx__,
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "delete-container",
                                        (name,),
                                        [[None; 0]; 0],
                                    ),
                                )
                                .await,
                            "failed to invoke `wrpc:blobstore/blobstore@0.2.0.delete-container`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok(wrpc__)
                    }
                }
                #[allow(clippy::all)]
                pub fn get_container_info<
                    'a,
                    C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                >(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    name: &'a str,
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<
                        ::core::result::Result<ContainerMetadata, String>,
                    >,
                > + Send + 'a {
                    async move {
                        let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                        wrpc__,
                                        cx__,
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "get-container-info",
                                        (name,),
                                        [[None; 0]; 0],
                                    ),
                                )
                                .await,
                            "failed to invoke `wrpc:blobstore/blobstore@0.2.0.get-container-info`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok(wrpc__)
                    }
                }
                #[allow(clippy::all)]
                pub fn list_container_objects<
                    'a,
                    C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                >(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    name: &'a str,
                    limit: ::core::option::Option<u64>,
                    offset: ::core::option::Option<u64>,
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<
                        (
                            ::core::result::Result<
                                (
                                    ::core::pin::Pin<
                                        ::std::boxed::Box<
                                            dyn ::wit_bindgen_wrpc::futures::Stream<
                                                Item = Vec<String>,
                                            > + ::core::marker::Send,
                                        >,
                                    >,
                                    ::core::pin::Pin<
                                        ::std::boxed::Box<
                                            dyn ::core::future::Future<
                                                Output = ::core::result::Result<(), String>,
                                            > + ::core::marker::Send,
                                        >,
                                    >,
                                ),
                                String,
                            >,
                            ::core::option::Option<
                                impl ::core::future::Future<
                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                > + ::core::marker::Send + 'static + ::wit_bindgen_wrpc::wrpc_transport::Captures<
                                    'a,
                                >,
                            >,
                        ),
                    >,
                > + Send + 'a {
                    async move {
                        let (wrpc__, io__) = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values(
                                        wrpc__,
                                        cx__,
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "list-container-objects",
                                        (name, limit, offset),
                                        [
                                            [Some(0), Some(0)].as_slice(),
                                            [Some(0), Some(1)].as_slice(),
                                        ],
                                    ),
                                )
                                .await,
                            "failed to invoke `wrpc:blobstore/blobstore@0.2.0.list-container-objects`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok((wrpc__, io__))
                    }
                }
                #[allow(clippy::all)]
                pub fn copy_object<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    src: &'a super::super::super::wasi::blobstore::types::ObjectId,
                    dest: &'a super::super::super::wasi::blobstore::types::ObjectId,
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<
                        ::core::result::Result<(), String>,
                    >,
                > + Send + 'a {
                    async move {
                        let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                        wrpc__,
                                        cx__,
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "copy-object",
                                        (src, dest),
                                        [[None; 0]; 0],
                                    ),
                                )
                                .await,
                            "failed to invoke `wrpc:blobstore/blobstore@0.2.0.copy-object`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok(wrpc__)
                    }
                }
                #[allow(clippy::all)]
                pub fn delete_object<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    id: &'a super::super::super::wasi::blobstore::types::ObjectId,
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<
                        ::core::result::Result<(), String>,
                    >,
                > + Send + 'a {
                    async move {
                        let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                        wrpc__,
                                        cx__,
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "delete-object",
                                        (id,),
                                        [[None; 0]; 0],
                                    ),
                                )
                                .await,
                            "failed to invoke `wrpc:blobstore/blobstore@0.2.0.delete-object`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok(wrpc__)
                    }
                }
                #[allow(clippy::all)]
                pub fn delete_objects<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    container: &'a str,
                    objects: &'a [&'a str],
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<
                        ::core::result::Result<(), String>,
                    >,
                > + Send + 'a {
                    async move {
                        let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                        wrpc__,
                                        cx__,
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "delete-objects",
                                        (container, objects),
                                        [[None; 0]; 0],
                                    ),
                                )
                                .await,
                            "failed to invoke `wrpc:blobstore/blobstore@0.2.0.delete-objects`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok(wrpc__)
                    }
                }
                #[allow(clippy::all)]
                pub fn get_container_data<
                    'a,
                    C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                >(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    id: &'a super::super::super::wasi::blobstore::types::ObjectId,
                    start: u64,
                    end: u64,
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<
                        (
                            ::core::result::Result<
                                (
                                    ::core::pin::Pin<
                                        ::std::boxed::Box<
                                            dyn ::wit_bindgen_wrpc::futures::Stream<
                                                Item = ::wit_bindgen_wrpc::bytes::Bytes,
                                            > + ::core::marker::Send,
                                        >,
                                    >,
                                    ::core::pin::Pin<
                                        ::std::boxed::Box<
                                            dyn ::core::future::Future<
                                                Output = ::core::result::Result<(), String>,
                                            > + ::core::marker::Send,
                                        >,
                                    >,
                                ),
                                String,
                            >,
                            ::core::option::Option<
                                impl ::core::future::Future<
                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                > + ::core::marker::Send + 'static + ::wit_bindgen_wrpc::wrpc_transport::Captures<
                                    'a,
                                >,
                            >,
                        ),
                    >,
                > + Send + 'a {
                    async move {
                        let (wrpc__, io__) = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values(
                                        wrpc__,
                                        cx__,
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "get-container-data",
                                        (id, start, end),
                                        [
                                            [Some(0), Some(0)].as_slice(),
                                            [Some(0), Some(1)].as_slice(),
                                        ],
                                    ),
                                )
                                .await,
                            "failed to invoke `wrpc:blobstore/blobstore@0.2.0.get-container-data`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok((wrpc__, io__))
                    }
                }
                #[allow(clippy::all)]
                pub fn get_object_info<
                    'a,
                    C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                >(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    id: &'a super::super::super::wasi::blobstore::types::ObjectId,
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<
                        ::core::result::Result<ObjectMetadata, String>,
                    >,
                > + Send + 'a {
                    async move {
                        let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                        wrpc__,
                                        cx__,
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "get-object-info",
                                        (id,),
                                        [[None; 0]; 0],
                                    ),
                                )
                                .await,
                            "failed to invoke `wrpc:blobstore/blobstore@0.2.0.get-object-info`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok(wrpc__)
                    }
                }
                #[allow(clippy::all)]
                pub fn has_object<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    id: &'a super::super::super::wasi::blobstore::types::ObjectId,
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<
                        ::core::result::Result<bool, String>,
                    >,
                > + Send + 'a {
                    async move {
                        let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                        wrpc__,
                                        cx__,
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "has-object",
                                        (id,),
                                        [[None; 0]; 0],
                                    ),
                                )
                                .await,
                            "failed to invoke `wrpc:blobstore/blobstore@0.2.0.has-object`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok(wrpc__)
                    }
                }
                #[allow(clippy::all)]
                pub fn move_object<'a, C: ::wit_bindgen_wrpc::wrpc_transport::Invoke>(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    src: &'a super::super::super::wasi::blobstore::types::ObjectId,
                    dest: &'a super::super::super::wasi::blobstore::types::ObjectId,
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<
                        ::core::result::Result<(), String>,
                    >,
                > + Send + 'a {
                    async move {
                        let wrpc__ = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values_blocking(
                                        wrpc__,
                                        cx__,
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "move-object",
                                        (src, dest),
                                        [[None; 0]; 0],
                                    ),
                                )
                                .await,
                            "failed to invoke `wrpc:blobstore/blobstore@0.2.0.move-object`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok(wrpc__)
                    }
                }
                #[allow(clippy::all)]
                pub fn write_container_data<
                    'a,
                    C: ::wit_bindgen_wrpc::wrpc_transport::Invoke,
                >(
                    wrpc__: &'a C,
                    cx__: C::Context,
                    id: &'a super::super::super::wasi::blobstore::types::ObjectId,
                    data: ::core::pin::Pin<
                        ::std::boxed::Box<
                            dyn ::wit_bindgen_wrpc::futures::Stream<
                                Item = ::wit_bindgen_wrpc::bytes::Bytes,
                            > + ::core::marker::Send,
                        >,
                    >,
                ) -> impl ::core::future::Future<
                    Output = ::wit_bindgen_wrpc::anyhow::Result<
                        (
                            ::core::result::Result<
                                ::core::pin::Pin<
                                    ::std::boxed::Box<
                                        dyn ::core::future::Future<
                                            Output = ::core::result::Result<(), String>,
                                        > + ::core::marker::Send,
                                    >,
                                >,
                                String,
                            >,
                            ::core::option::Option<
                                impl ::core::future::Future<
                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                > + ::core::marker::Send + 'static + ::wit_bindgen_wrpc::wrpc_transport::Captures<
                                    'a,
                                >,
                            >,
                        ),
                    >,
                > + Send + 'a {
                    async move {
                        let (wrpc__, io__) = ::wit_bindgen_wrpc::anyhow::Context::context(
                            ::wit_bindgen_wrpc::wrpc_transport::SendFuture::send(
                                    ::wit_bindgen_wrpc::wrpc_transport::InvokeExt::invoke_values(
                                        wrpc__,
                                        cx__,
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "write-container-data",
                                        (id, data),
                                        [[Some(0)].as_slice()],
                                    ),
                                )
                                .await,
                            "failed to invoke `wrpc:blobstore/blobstore@0.2.0.write-container-data`",
                        )?;
                        let (wrpc__,) = wrpc__;
                        Ok((wrpc__, io__))
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod exports {
        #[allow(dead_code)]
        pub mod wrpc {
            #[allow(dead_code)]
            pub mod blobstore {
                #[allow(dead_code, clippy::all)]
                pub mod blobstore {
                    pub type ContainerMetadata = super::super::super::super::wrpc::blobstore::types::ContainerMetadata;
                    pub type ObjectMetadata = super::super::super::super::wrpc::blobstore::types::ObjectMetadata;
                    pub type ObjectId = super::super::super::super::wrpc::blobstore::types::ObjectId;
                    pub trait Handler<Ctx> {
                        fn clear_container(
                            &self,
                            cx: Ctx,
                            name: String,
                        ) -> impl ::core::future::Future<
                            Output = ::wit_bindgen_wrpc::anyhow::Result<
                                ::core::result::Result<(), String>,
                            >,
                        > + ::core::marker::Send;
                        fn container_exists(
                            &self,
                            cx: Ctx,
                            name: String,
                        ) -> impl ::core::future::Future<
                            Output = ::wit_bindgen_wrpc::anyhow::Result<
                                ::core::result::Result<bool, String>,
                            >,
                        > + ::core::marker::Send;
                        fn create_container(
                            &self,
                            cx: Ctx,
                            name: String,
                        ) -> impl ::core::future::Future<
                            Output = ::wit_bindgen_wrpc::anyhow::Result<
                                ::core::result::Result<(), String>,
                            >,
                        > + ::core::marker::Send;
                        fn delete_container(
                            &self,
                            cx: Ctx,
                            name: String,
                        ) -> impl ::core::future::Future<
                            Output = ::wit_bindgen_wrpc::anyhow::Result<
                                ::core::result::Result<(), String>,
                            >,
                        > + ::core::marker::Send;
                        fn get_container_info(
                            &self,
                            cx: Ctx,
                            name: String,
                        ) -> impl ::core::future::Future<
                            Output = ::wit_bindgen_wrpc::anyhow::Result<
                                ::core::result::Result<ContainerMetadata, String>,
                            >,
                        > + ::core::marker::Send;
                        fn list_container_objects(
                            &self,
                            cx: Ctx,
                            name: String,
                            limit: ::core::option::Option<u64>,
                            offset: ::core::option::Option<u64>,
                        ) -> impl ::core::future::Future<
                            Output = ::wit_bindgen_wrpc::anyhow::Result<
                                ::core::result::Result<
                                    (
                                        ::core::pin::Pin<
                                            ::std::boxed::Box<
                                                dyn ::wit_bindgen_wrpc::futures::Stream<
                                                    Item = Vec<String>,
                                                > + ::core::marker::Send,
                                            >,
                                        >,
                                        ::core::pin::Pin<
                                            ::std::boxed::Box<
                                                dyn ::core::future::Future<
                                                    Output = ::core::result::Result<(), String>,
                                                > + ::core::marker::Send,
                                            >,
                                        >,
                                    ),
                                    String,
                                >,
                            >,
                        > + ::core::marker::Send;
                        fn copy_object(
                            &self,
                            cx: Ctx,
                            src: ObjectId,
                            dest: ObjectId,
                        ) -> impl ::core::future::Future<
                            Output = ::wit_bindgen_wrpc::anyhow::Result<
                                ::core::result::Result<(), String>,
                            >,
                        > + ::core::marker::Send;
                        fn delete_object(
                            &self,
                            cx: Ctx,
                            id: ObjectId,
                        ) -> impl ::core::future::Future<
                            Output = ::wit_bindgen_wrpc::anyhow::Result<
                                ::core::result::Result<(), String>,
                            >,
                        > + ::core::marker::Send;
                        fn delete_objects(
                            &self,
                            cx: Ctx,
                            container: String,
                            objects: Vec<String>,
                        ) -> impl ::core::future::Future<
                            Output = ::wit_bindgen_wrpc::anyhow::Result<
                                ::core::result::Result<(), String>,
                            >,
                        > + ::core::marker::Send;
                        fn get_container_data(
                            &self,
                            cx: Ctx,
                            id: ObjectId,
                            start: u64,
                            end: u64,
                        ) -> impl ::core::future::Future<
                            Output = ::wit_bindgen_wrpc::anyhow::Result<
                                ::core::result::Result<
                                    (
                                        ::core::pin::Pin<
                                            ::std::boxed::Box<
                                                dyn ::wit_bindgen_wrpc::futures::Stream<
                                                    Item = ::wit_bindgen_wrpc::bytes::Bytes,
                                                > + ::core::marker::Send,
                                            >,
                                        >,
                                        ::core::pin::Pin<
                                            ::std::boxed::Box<
                                                dyn ::core::future::Future<
                                                    Output = ::core::result::Result<(), String>,
                                                > + ::core::marker::Send,
                                            >,
                                        >,
                                    ),
                                    String,
                                >,
                            >,
                        > + ::core::marker::Send;
                        fn get_object_info(
                            &self,
                            cx: Ctx,
                            id: ObjectId,
                        ) -> impl ::core::future::Future<
                            Output = ::wit_bindgen_wrpc::anyhow::Result<
                                ::core::result::Result<ObjectMetadata, String>,
                            >,
                        > + ::core::marker::Send;
                        fn has_object(
                            &self,
                            cx: Ctx,
                            id: ObjectId,
                        ) -> impl ::core::future::Future<
                            Output = ::wit_bindgen_wrpc::anyhow::Result<
                                ::core::result::Result<bool, String>,
                            >,
                        > + ::core::marker::Send;
                        fn move_object(
                            &self,
                            cx: Ctx,
                            src: ObjectId,
                            dest: ObjectId,
                        ) -> impl ::core::future::Future<
                            Output = ::wit_bindgen_wrpc::anyhow::Result<
                                ::core::result::Result<(), String>,
                            >,
                        > + ::core::marker::Send;
                        fn write_container_data(
                            &self,
                            cx: Ctx,
                            id: ObjectId,
                            data: ::core::pin::Pin<
                                ::std::boxed::Box<
                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                        Item = ::wit_bindgen_wrpc::bytes::Bytes,
                                    > + ::core::marker::Send,
                                >,
                            >,
                        ) -> impl ::core::future::Future<
                            Output = ::wit_bindgen_wrpc::anyhow::Result<
                                ::core::result::Result<
                                    ::core::pin::Pin<
                                        ::std::boxed::Box<
                                            dyn ::core::future::Future<
                                                Output = ::core::result::Result<(), String>,
                                            > + ::core::marker::Send,
                                        >,
                                    >,
                                    String,
                                >,
                            >,
                        > + ::core::marker::Send;
                    }
                    #[allow(clippy::manual_async_fn)]
                    pub fn serve_interface<
                        'a,
                        T: ::wit_bindgen_wrpc::wrpc_transport::Serve,
                    >(
                        wrpc: &'a T,
                        handler: impl Handler<
                            T::Context,
                        > + ::core::marker::Send + ::core::marker::Sync + ::core::clone::Clone + 'static,
                    ) -> impl ::core::future::Future<
                        Output = ::wit_bindgen_wrpc::anyhow::Result<
                            [(
                                &'static str,
                                &'static str,
                                ::core::pin::Pin<
                                    ::std::boxed::Box<
                                        dyn ::wit_bindgen_wrpc::futures::Stream<
                                            Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                ::core::pin::Pin<
                                                    ::std::boxed::Box<
                                                        dyn ::core::future::Future<
                                                            Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                        > + ::core::marker::Send + 'static,
                                                    >,
                                                >,
                                            >,
                                        > + ::core::marker::Send + 'static,
                                    >,
                                >,
                            ); 14],
                        >,
                    > + ::core::marker::Send + ::wit_bindgen_wrpc::wrpc_transport::Captures<
                        'a,
                    > {
                        async move {
                            let (
                                f_clear_container,
                                f_container_exists,
                                f_create_container,
                                f_delete_container,
                                f_get_container_info,
                                f_list_container_objects,
                                f_copy_object,
                                f_delete_object,
                                f_delete_objects,
                                f_get_container_data,
                                f_get_object_info,
                                f_has_object,
                                f_move_object,
                                f_write_container_data,
                            ) = {
                                use ::tokio::macros::support::{
                                    maybe_done, poll_fn, Future, Pin,
                                };
                                use ::tokio::macros::support::Poll::{Ready, Pending};
                                let mut futures = (
                                    maybe_done(async {
                                        ::wit_bindgen_wrpc::anyhow::Context::context(
                                            ::wit_bindgen_wrpc::wrpc_transport::ServeExt::serve_values(
                                                    wrpc,
                                                    "wrpc:blobstore/blobstore@0.2.0",
                                                    "clear-container",
                                                    ::std::sync::Arc::from({
                                                        let paths: [::std::boxed::Box<
                                                            [::core::option::Option<usize>],
                                                        >; 0] = [];
                                                        paths
                                                    }),
                                                )
                                                .await,
                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.clear-container`",
                                        )
                                    }),
                                    maybe_done(async {
                                        ::wit_bindgen_wrpc::anyhow::Context::context(
                                            ::wit_bindgen_wrpc::wrpc_transport::ServeExt::serve_values(
                                                    wrpc,
                                                    "wrpc:blobstore/blobstore@0.2.0",
                                                    "container-exists",
                                                    ::std::sync::Arc::from({
                                                        let paths: [::std::boxed::Box<
                                                            [::core::option::Option<usize>],
                                                        >; 0] = [];
                                                        paths
                                                    }),
                                                )
                                                .await,
                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.container-exists`",
                                        )
                                    }),
                                    maybe_done(async {
                                        ::wit_bindgen_wrpc::anyhow::Context::context(
                                            ::wit_bindgen_wrpc::wrpc_transport::ServeExt::serve_values(
                                                    wrpc,
                                                    "wrpc:blobstore/blobstore@0.2.0",
                                                    "create-container",
                                                    ::std::sync::Arc::from({
                                                        let paths: [::std::boxed::Box<
                                                            [::core::option::Option<usize>],
                                                        >; 0] = [];
                                                        paths
                                                    }),
                                                )
                                                .await,
                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.create-container`",
                                        )
                                    }),
                                    maybe_done(async {
                                        ::wit_bindgen_wrpc::anyhow::Context::context(
                                            ::wit_bindgen_wrpc::wrpc_transport::ServeExt::serve_values(
                                                    wrpc,
                                                    "wrpc:blobstore/blobstore@0.2.0",
                                                    "delete-container",
                                                    ::std::sync::Arc::from({
                                                        let paths: [::std::boxed::Box<
                                                            [::core::option::Option<usize>],
                                                        >; 0] = [];
                                                        paths
                                                    }),
                                                )
                                                .await,
                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.delete-container`",
                                        )
                                    }),
                                    maybe_done(async {
                                        ::wit_bindgen_wrpc::anyhow::Context::context(
                                            ::wit_bindgen_wrpc::wrpc_transport::ServeExt::serve_values(
                                                    wrpc,
                                                    "wrpc:blobstore/blobstore@0.2.0",
                                                    "get-container-info",
                                                    ::std::sync::Arc::from({
                                                        let paths: [::std::boxed::Box<
                                                            [::core::option::Option<usize>],
                                                        >; 0] = [];
                                                        paths
                                                    }),
                                                )
                                                .await,
                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.get-container-info`",
                                        )
                                    }),
                                    maybe_done(async {
                                        ::wit_bindgen_wrpc::anyhow::Context::context(
                                            ::wit_bindgen_wrpc::wrpc_transport::ServeExt::serve_values(
                                                    wrpc,
                                                    "wrpc:blobstore/blobstore@0.2.0",
                                                    "list-container-objects",
                                                    ::std::sync::Arc::from({
                                                        let paths: [::std::boxed::Box<
                                                            [::core::option::Option<usize>],
                                                        >; 0] = [];
                                                        paths
                                                    }),
                                                )
                                                .await,
                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.list-container-objects`",
                                        )
                                    }),
                                    maybe_done(async {
                                        ::wit_bindgen_wrpc::anyhow::Context::context(
                                            ::wit_bindgen_wrpc::wrpc_transport::ServeExt::serve_values(
                                                    wrpc,
                                                    "wrpc:blobstore/blobstore@0.2.0",
                                                    "copy-object",
                                                    ::std::sync::Arc::from({
                                                        let paths: [::std::boxed::Box<
                                                            [::core::option::Option<usize>],
                                                        >; 0] = [];
                                                        paths
                                                    }),
                                                )
                                                .await,
                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.copy-object`",
                                        )
                                    }),
                                    maybe_done(async {
                                        ::wit_bindgen_wrpc::anyhow::Context::context(
                                            ::wit_bindgen_wrpc::wrpc_transport::ServeExt::serve_values(
                                                    wrpc,
                                                    "wrpc:blobstore/blobstore@0.2.0",
                                                    "delete-object",
                                                    ::std::sync::Arc::from({
                                                        let paths: [::std::boxed::Box<
                                                            [::core::option::Option<usize>],
                                                        >; 0] = [];
                                                        paths
                                                    }),
                                                )
                                                .await,
                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.delete-object`",
                                        )
                                    }),
                                    maybe_done(async {
                                        ::wit_bindgen_wrpc::anyhow::Context::context(
                                            ::wit_bindgen_wrpc::wrpc_transport::ServeExt::serve_values(
                                                    wrpc,
                                                    "wrpc:blobstore/blobstore@0.2.0",
                                                    "delete-objects",
                                                    ::std::sync::Arc::from({
                                                        let paths: [::std::boxed::Box<
                                                            [::core::option::Option<usize>],
                                                        >; 0] = [];
                                                        paths
                                                    }),
                                                )
                                                .await,
                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.delete-objects`",
                                        )
                                    }),
                                    maybe_done(async {
                                        ::wit_bindgen_wrpc::anyhow::Context::context(
                                            ::wit_bindgen_wrpc::wrpc_transport::ServeExt::serve_values(
                                                    wrpc,
                                                    "wrpc:blobstore/blobstore@0.2.0",
                                                    "get-container-data",
                                                    ::std::sync::Arc::from({
                                                        let paths: [::std::boxed::Box<
                                                            [::core::option::Option<usize>],
                                                        >; 0] = [];
                                                        paths
                                                    }),
                                                )
                                                .await,
                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.get-container-data`",
                                        )
                                    }),
                                    maybe_done(async {
                                        ::wit_bindgen_wrpc::anyhow::Context::context(
                                            ::wit_bindgen_wrpc::wrpc_transport::ServeExt::serve_values(
                                                    wrpc,
                                                    "wrpc:blobstore/blobstore@0.2.0",
                                                    "get-object-info",
                                                    ::std::sync::Arc::from({
                                                        let paths: [::std::boxed::Box<
                                                            [::core::option::Option<usize>],
                                                        >; 0] = [];
                                                        paths
                                                    }),
                                                )
                                                .await,
                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.get-object-info`",
                                        )
                                    }),
                                    maybe_done(async {
                                        ::wit_bindgen_wrpc::anyhow::Context::context(
                                            ::wit_bindgen_wrpc::wrpc_transport::ServeExt::serve_values(
                                                    wrpc,
                                                    "wrpc:blobstore/blobstore@0.2.0",
                                                    "has-object",
                                                    ::std::sync::Arc::from({
                                                        let paths: [::std::boxed::Box<
                                                            [::core::option::Option<usize>],
                                                        >; 0] = [];
                                                        paths
                                                    }),
                                                )
                                                .await,
                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.has-object`",
                                        )
                                    }),
                                    maybe_done(async {
                                        ::wit_bindgen_wrpc::anyhow::Context::context(
                                            ::wit_bindgen_wrpc::wrpc_transport::ServeExt::serve_values(
                                                    wrpc,
                                                    "wrpc:blobstore/blobstore@0.2.0",
                                                    "move-object",
                                                    ::std::sync::Arc::from({
                                                        let paths: [::std::boxed::Box<
                                                            [::core::option::Option<usize>],
                                                        >; 0] = [];
                                                        paths
                                                    }),
                                                )
                                                .await,
                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.move-object`",
                                        )
                                    }),
                                    maybe_done(async {
                                        ::wit_bindgen_wrpc::anyhow::Context::context(
                                            ::wit_bindgen_wrpc::wrpc_transport::ServeExt::serve_values(
                                                    wrpc,
                                                    "wrpc:blobstore/blobstore@0.2.0",
                                                    "write-container-data",
                                                    ::std::sync::Arc::from([::std::boxed::Box::from([Some(1)])]),
                                                )
                                                .await,
                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.write-container-data`",
                                        )
                                    }),
                                );
                                let mut futures = &mut futures;
                                let mut skip_next_time: u32 = 0;
                                poll_fn(move |cx| {
                                        const COUNT: u32 = 0 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1
                                            + 1 + 1 + 1 + 1;
                                        let mut is_pending = false;
                                        let mut to_run = COUNT;
                                        let mut skip = skip_next_time;
                                        skip_next_time = if skip + 1 == COUNT {
                                            0
                                        } else {
                                            skip + 1
                                        };
                                        loop {
                                            if skip == 0 {
                                                if to_run == 0 {
                                                    break;
                                                }
                                                to_run -= 1;
                                                let (fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                if fut.as_mut().poll(cx).is_pending() {
                                                    is_pending = true;
                                                } else if fut
                                                    .as_mut()
                                                    .output_mut()
                                                    .expect("expected completed future")
                                                    .is_err()
                                                {
                                                    return Ready(
                                                        Err(
                                                            fut
                                                                .take_output()
                                                                .expect("expected completed future")
                                                                .err()
                                                                .unwrap(),
                                                        ),
                                                    )
                                                }
                                            } else {
                                                skip -= 1;
                                            }
                                            if skip == 0 {
                                                if to_run == 0 {
                                                    break;
                                                }
                                                to_run -= 1;
                                                let (_, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                if fut.as_mut().poll(cx).is_pending() {
                                                    is_pending = true;
                                                } else if fut
                                                    .as_mut()
                                                    .output_mut()
                                                    .expect("expected completed future")
                                                    .is_err()
                                                {
                                                    return Ready(
                                                        Err(
                                                            fut
                                                                .take_output()
                                                                .expect("expected completed future")
                                                                .err()
                                                                .unwrap(),
                                                        ),
                                                    )
                                                }
                                            } else {
                                                skip -= 1;
                                            }
                                            if skip == 0 {
                                                if to_run == 0 {
                                                    break;
                                                }
                                                to_run -= 1;
                                                let (_, _, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                if fut.as_mut().poll(cx).is_pending() {
                                                    is_pending = true;
                                                } else if fut
                                                    .as_mut()
                                                    .output_mut()
                                                    .expect("expected completed future")
                                                    .is_err()
                                                {
                                                    return Ready(
                                                        Err(
                                                            fut
                                                                .take_output()
                                                                .expect("expected completed future")
                                                                .err()
                                                                .unwrap(),
                                                        ),
                                                    )
                                                }
                                            } else {
                                                skip -= 1;
                                            }
                                            if skip == 0 {
                                                if to_run == 0 {
                                                    break;
                                                }
                                                to_run -= 1;
                                                let (_, _, _, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                if fut.as_mut().poll(cx).is_pending() {
                                                    is_pending = true;
                                                } else if fut
                                                    .as_mut()
                                                    .output_mut()
                                                    .expect("expected completed future")
                                                    .is_err()
                                                {
                                                    return Ready(
                                                        Err(
                                                            fut
                                                                .take_output()
                                                                .expect("expected completed future")
                                                                .err()
                                                                .unwrap(),
                                                        ),
                                                    )
                                                }
                                            } else {
                                                skip -= 1;
                                            }
                                            if skip == 0 {
                                                if to_run == 0 {
                                                    break;
                                                }
                                                to_run -= 1;
                                                let (_, _, _, _, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                if fut.as_mut().poll(cx).is_pending() {
                                                    is_pending = true;
                                                } else if fut
                                                    .as_mut()
                                                    .output_mut()
                                                    .expect("expected completed future")
                                                    .is_err()
                                                {
                                                    return Ready(
                                                        Err(
                                                            fut
                                                                .take_output()
                                                                .expect("expected completed future")
                                                                .err()
                                                                .unwrap(),
                                                        ),
                                                    )
                                                }
                                            } else {
                                                skip -= 1;
                                            }
                                            if skip == 0 {
                                                if to_run == 0 {
                                                    break;
                                                }
                                                to_run -= 1;
                                                let (_, _, _, _, _, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                if fut.as_mut().poll(cx).is_pending() {
                                                    is_pending = true;
                                                } else if fut
                                                    .as_mut()
                                                    .output_mut()
                                                    .expect("expected completed future")
                                                    .is_err()
                                                {
                                                    return Ready(
                                                        Err(
                                                            fut
                                                                .take_output()
                                                                .expect("expected completed future")
                                                                .err()
                                                                .unwrap(),
                                                        ),
                                                    )
                                                }
                                            } else {
                                                skip -= 1;
                                            }
                                            if skip == 0 {
                                                if to_run == 0 {
                                                    break;
                                                }
                                                to_run -= 1;
                                                let (_, _, _, _, _, _, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                if fut.as_mut().poll(cx).is_pending() {
                                                    is_pending = true;
                                                } else if fut
                                                    .as_mut()
                                                    .output_mut()
                                                    .expect("expected completed future")
                                                    .is_err()
                                                {
                                                    return Ready(
                                                        Err(
                                                            fut
                                                                .take_output()
                                                                .expect("expected completed future")
                                                                .err()
                                                                .unwrap(),
                                                        ),
                                                    )
                                                }
                                            } else {
                                                skip -= 1;
                                            }
                                            if skip == 0 {
                                                if to_run == 0 {
                                                    break;
                                                }
                                                to_run -= 1;
                                                let (_, _, _, _, _, _, _, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                if fut.as_mut().poll(cx).is_pending() {
                                                    is_pending = true;
                                                } else if fut
                                                    .as_mut()
                                                    .output_mut()
                                                    .expect("expected completed future")
                                                    .is_err()
                                                {
                                                    return Ready(
                                                        Err(
                                                            fut
                                                                .take_output()
                                                                .expect("expected completed future")
                                                                .err()
                                                                .unwrap(),
                                                        ),
                                                    )
                                                }
                                            } else {
                                                skip -= 1;
                                            }
                                            if skip == 0 {
                                                if to_run == 0 {
                                                    break;
                                                }
                                                to_run -= 1;
                                                let (_, _, _, _, _, _, _, _, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                if fut.as_mut().poll(cx).is_pending() {
                                                    is_pending = true;
                                                } else if fut
                                                    .as_mut()
                                                    .output_mut()
                                                    .expect("expected completed future")
                                                    .is_err()
                                                {
                                                    return Ready(
                                                        Err(
                                                            fut
                                                                .take_output()
                                                                .expect("expected completed future")
                                                                .err()
                                                                .unwrap(),
                                                        ),
                                                    )
                                                }
                                            } else {
                                                skip -= 1;
                                            }
                                            if skip == 0 {
                                                if to_run == 0 {
                                                    break;
                                                }
                                                to_run -= 1;
                                                let (_, _, _, _, _, _, _, _, _, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                if fut.as_mut().poll(cx).is_pending() {
                                                    is_pending = true;
                                                } else if fut
                                                    .as_mut()
                                                    .output_mut()
                                                    .expect("expected completed future")
                                                    .is_err()
                                                {
                                                    return Ready(
                                                        Err(
                                                            fut
                                                                .take_output()
                                                                .expect("expected completed future")
                                                                .err()
                                                                .unwrap(),
                                                        ),
                                                    )
                                                }
                                            } else {
                                                skip -= 1;
                                            }
                                            if skip == 0 {
                                                if to_run == 0 {
                                                    break;
                                                }
                                                to_run -= 1;
                                                let (_, _, _, _, _, _, _, _, _, _, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                if fut.as_mut().poll(cx).is_pending() {
                                                    is_pending = true;
                                                } else if fut
                                                    .as_mut()
                                                    .output_mut()
                                                    .expect("expected completed future")
                                                    .is_err()
                                                {
                                                    return Ready(
                                                        Err(
                                                            fut
                                                                .take_output()
                                                                .expect("expected completed future")
                                                                .err()
                                                                .unwrap(),
                                                        ),
                                                    )
                                                }
                                            } else {
                                                skip -= 1;
                                            }
                                            if skip == 0 {
                                                if to_run == 0 {
                                                    break;
                                                }
                                                to_run -= 1;
                                                let (_, _, _, _, _, _, _, _, _, _, _, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                if fut.as_mut().poll(cx).is_pending() {
                                                    is_pending = true;
                                                } else if fut
                                                    .as_mut()
                                                    .output_mut()
                                                    .expect("expected completed future")
                                                    .is_err()
                                                {
                                                    return Ready(
                                                        Err(
                                                            fut
                                                                .take_output()
                                                                .expect("expected completed future")
                                                                .err()
                                                                .unwrap(),
                                                        ),
                                                    )
                                                }
                                            } else {
                                                skip -= 1;
                                            }
                                            if skip == 0 {
                                                if to_run == 0 {
                                                    break;
                                                }
                                                to_run -= 1;
                                                let (_, _, _, _, _, _, _, _, _, _, _, _, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                if fut.as_mut().poll(cx).is_pending() {
                                                    is_pending = true;
                                                } else if fut
                                                    .as_mut()
                                                    .output_mut()
                                                    .expect("expected completed future")
                                                    .is_err()
                                                {
                                                    return Ready(
                                                        Err(
                                                            fut
                                                                .take_output()
                                                                .expect("expected completed future")
                                                                .err()
                                                                .unwrap(),
                                                        ),
                                                    )
                                                }
                                            } else {
                                                skip -= 1;
                                            }
                                            if skip == 0 {
                                                if to_run == 0 {
                                                    break;
                                                }
                                                to_run -= 1;
                                                let (_, _, _, _, _, _, _, _, _, _, _, _, _, fut, ..) = &mut *futures;
                                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                if fut.as_mut().poll(cx).is_pending() {
                                                    is_pending = true;
                                                } else if fut
                                                    .as_mut()
                                                    .output_mut()
                                                    .expect("expected completed future")
                                                    .is_err()
                                                {
                                                    return Ready(
                                                        Err(
                                                            fut
                                                                .take_output()
                                                                .expect("expected completed future")
                                                                .err()
                                                                .unwrap(),
                                                        ),
                                                    )
                                                }
                                            } else {
                                                skip -= 1;
                                            }
                                        }
                                        if is_pending {
                                            Pending
                                        } else {
                                            Ready(
                                                Ok((
                                                    {
                                                        let (fut, ..) = &mut futures;
                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                        fut.take_output()
                                                            .expect("expected completed future")
                                                            .ok()
                                                            .expect("expected Ok(_)")
                                                    },
                                                    {
                                                        let (_, fut, ..) = &mut futures;
                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                        fut.take_output()
                                                            .expect("expected completed future")
                                                            .ok()
                                                            .expect("expected Ok(_)")
                                                    },
                                                    {
                                                        let (_, _, fut, ..) = &mut futures;
                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                        fut.take_output()
                                                            .expect("expected completed future")
                                                            .ok()
                                                            .expect("expected Ok(_)")
                                                    },
                                                    {
                                                        let (_, _, _, fut, ..) = &mut futures;
                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                        fut.take_output()
                                                            .expect("expected completed future")
                                                            .ok()
                                                            .expect("expected Ok(_)")
                                                    },
                                                    {
                                                        let (_, _, _, _, fut, ..) = &mut futures;
                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                        fut.take_output()
                                                            .expect("expected completed future")
                                                            .ok()
                                                            .expect("expected Ok(_)")
                                                    },
                                                    {
                                                        let (_, _, _, _, _, fut, ..) = &mut futures;
                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                        fut.take_output()
                                                            .expect("expected completed future")
                                                            .ok()
                                                            .expect("expected Ok(_)")
                                                    },
                                                    {
                                                        let (_, _, _, _, _, _, fut, ..) = &mut futures;
                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                        fut.take_output()
                                                            .expect("expected completed future")
                                                            .ok()
                                                            .expect("expected Ok(_)")
                                                    },
                                                    {
                                                        let (_, _, _, _, _, _, _, fut, ..) = &mut futures;
                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                        fut.take_output()
                                                            .expect("expected completed future")
                                                            .ok()
                                                            .expect("expected Ok(_)")
                                                    },
                                                    {
                                                        let (_, _, _, _, _, _, _, _, fut, ..) = &mut futures;
                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                        fut.take_output()
                                                            .expect("expected completed future")
                                                            .ok()
                                                            .expect("expected Ok(_)")
                                                    },
                                                    {
                                                        let (_, _, _, _, _, _, _, _, _, fut, ..) = &mut futures;
                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                        fut.take_output()
                                                            .expect("expected completed future")
                                                            .ok()
                                                            .expect("expected Ok(_)")
                                                    },
                                                    {
                                                        let (_, _, _, _, _, _, _, _, _, _, fut, ..) = &mut futures;
                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                        fut.take_output()
                                                            .expect("expected completed future")
                                                            .ok()
                                                            .expect("expected Ok(_)")
                                                    },
                                                    {
                                                        let (_, _, _, _, _, _, _, _, _, _, _, fut, ..) = &mut futures;
                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                        fut.take_output()
                                                            .expect("expected completed future")
                                                            .ok()
                                                            .expect("expected Ok(_)")
                                                    },
                                                    {
                                                        let (_, _, _, _, _, _, _, _, _, _, _, _, fut, ..) = &mut futures;
                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                        fut.take_output()
                                                            .expect("expected completed future")
                                                            .ok()
                                                            .expect("expected Ok(_)")
                                                    },
                                                    {
                                                        let (_, _, _, _, _, _, _, _, _, _, _, _, _, fut, ..) = &mut futures;
                                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                                        fut.take_output()
                                                            .expect("expected completed future")
                                                            .ok()
                                                            .expect("expected Ok(_)")
                                                    },
                                                )),
                                            )
                                        }
                                    })
                                    .await
                            }?;
                            ::wit_bindgen_wrpc::anyhow::Ok([
                                {
                                    let handler = handler.clone();
                                    (
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "clear-container",
                                        ::std::boxed::Box::pin(
                                            ::wit_bindgen_wrpc::futures::TryStreamExt::map_ok(
                                                f_clear_container,
                                                move |(cx, (p0,), rx, tx)| {
                                                    let handler = handler.clone();
                                                    ::std::boxed::Box::pin(async move {
                                                        let rx = rx
                                                            .map(
                                                                ::wit_bindgen_wrpc::tracing::Instrument::in_current_span,
                                                            )
                                                            .map(::wit_bindgen_wrpc::tokio::spawn);
                                                        {
                                                            use ::tracing::__macro_support::Callsite as _;
                                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                static META: ::tracing::Metadata<'static> = {
                                                                    ::tracing_core::metadata::Metadata::new(
                                                                        "event src/lib.rs:2",
                                                                        "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ::tracing::Level::TRACE,
                                                                        ::core::option::Option::Some("src/lib.rs"),
                                                                        ::core::option::Option::Some(2u32),
                                                                        ::core::option::Option::Some(
                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ),
                                                                        ::tracing_core::field::FieldSet::new(
                                                                            &["message", "instance", "func"],
                                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                        ),
                                                                        ::tracing::metadata::Kind::EVENT,
                                                                    )
                                                                };
                                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                                            };
                                                            let enabled = ::tracing::Level::TRACE
                                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                && ::tracing::Level::TRACE
                                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                                && {
                                                                    let interest = __CALLSITE.interest();
                                                                    !interest.is_never()
                                                                        && ::tracing::__macro_support::__is_enabled(
                                                                            __CALLSITE.metadata(),
                                                                            interest,
                                                                        )
                                                                };
                                                            if enabled {
                                                                (|value_set: ::tracing::field::ValueSet| {
                                                                    let meta = __CALLSITE.metadata();
                                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                                })({
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("calling handler") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"clear-container" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                });
                                                            } else {
                                                            }
                                                        };
                                                        match Handler::clear_container(&handler, cx, p0).await {
                                                            Ok(results) => {
                                                                match tx((results,)).await {
                                                                    Ok(()) => {
                                                                        if let Some(rx) = rx {
                                                                            {
                                                                                use ::tracing::__macro_support::Callsite as _;
                                                                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                                    static META: ::tracing::Metadata<'static> = {
                                                                                        ::tracing_core::metadata::Metadata::new(
                                                                                            "event src/lib.rs:2",
                                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ::tracing::Level::TRACE,
                                                                                            ::core::option::Option::Some("src/lib.rs"),
                                                                                            ::core::option::Option::Some(2u32),
                                                                                            ::core::option::Option::Some(
                                                                                                "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ),
                                                                                            ::tracing_core::field::FieldSet::new(
                                                                                                &["message", "instance", "func"],
                                                                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                                            ),
                                                                                            ::tracing::metadata::Kind::EVENT,
                                                                                        )
                                                                                    };
                                                                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                                                                };
                                                                                let enabled = ::tracing::Level::TRACE
                                                                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                                    && ::tracing::Level::TRACE
                                                                                        <= ::tracing::level_filters::LevelFilter::current()
                                                                                    && {
                                                                                        let interest = __CALLSITE.interest();
                                                                                        !interest.is_never()
                                                                                            && ::tracing::__macro_support::__is_enabled(
                                                                                                __CALLSITE.metadata(),
                                                                                                interest,
                                                                                            )
                                                                                    };
                                                                                if enabled {
                                                                                    (|value_set: ::tracing::field::ValueSet| {
                                                                                        let meta = __CALLSITE.metadata();
                                                                                        ::tracing::Event::dispatch(meta, &value_set);
                                                                                    })({
                                                                                        #[allow(unused_imports)]
                                                                                        use ::tracing::field::{debug, display, Value};
                                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                        __CALLSITE
                                                                                            .metadata()
                                                                                            .fields()
                                                                                            .value_set(
                                                                                                &[
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &format_args!("receiving async invocation parameters")
                                                                                                                as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"clear-container" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                ],
                                                                                            )
                                                                                    });
                                                                                } else {
                                                                                }
                                                                            };
                                                                            let rx = ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx.await,
                                                                                "`wrpc:blobstore/blobstore@0.2.0.clear-container` async parameter receipt task failed",
                                                                            )?;
                                                                            ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx,
                                                                                "failed to receive `wrpc:blobstore/blobstore@0.2.0.clear-container` async parameters",
                                                                            )
                                                                        } else {
                                                                            ::wit_bindgen_wrpc::anyhow::Ok(())
                                                                        }
                                                                    }
                                                                    Err(err) => {
                                                                        if let Some(rx) = rx {
                                                                            rx.abort();
                                                                        }
                                                                        return ::anyhow::__private::Err({
                                                                            use ::anyhow::__private::kind::*;
                                                                            let error = match err
                                                                                .context(
                                                                                    "failed to transmit `wrpc:blobstore/blobstore@0.2.0.clear-container` invocation results",
                                                                                )
                                                                            {
                                                                                error => (&error).anyhow_kind().new(error),
                                                                            };
                                                                            error
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                if let Some(rx) = rx {
                                                                    rx.abort();
                                                                }
                                                                return ::anyhow::__private::Err({
                                                                    use ::anyhow::__private::kind::*;
                                                                    let error = match err
                                                                        .context(
                                                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.clear-container` invocation",
                                                                        )
                                                                    {
                                                                        error => (&error).anyhow_kind().new(error),
                                                                    };
                                                                    error
                                                                });
                                                            }
                                                        }
                                                    })
                                                        as ::core::pin::Pin<
                                                            ::std::boxed::Box<
                                                                dyn ::core::future::Future<
                                                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                > + ::core::marker::Send + 'static,
                                                            >,
                                                        >
                                                },
                                            ),
                                        )
                                            as ::core::pin::Pin<
                                                ::std::boxed::Box<
                                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                                        Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                            ::core::pin::Pin<
                                                                ::std::boxed::Box<
                                                                    dyn ::core::future::Future<
                                                                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                    > + ::core::marker::Send + 'static,
                                                                >,
                                                            >,
                                                        >,
                                                    > + ::core::marker::Send + 'static,
                                                >,
                                            >,
                                    )
                                },
                                {
                                    let handler = handler.clone();
                                    (
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "container-exists",
                                        ::std::boxed::Box::pin(
                                            ::wit_bindgen_wrpc::futures::TryStreamExt::map_ok(
                                                f_container_exists,
                                                move |(cx, (p0,), rx, tx)| {
                                                    let handler = handler.clone();
                                                    ::std::boxed::Box::pin(async move {
                                                        let rx = rx
                                                            .map(
                                                                ::wit_bindgen_wrpc::tracing::Instrument::in_current_span,
                                                            )
                                                            .map(::wit_bindgen_wrpc::tokio::spawn);
                                                        {
                                                            use ::tracing::__macro_support::Callsite as _;
                                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                static META: ::tracing::Metadata<'static> = {
                                                                    ::tracing_core::metadata::Metadata::new(
                                                                        "event src/lib.rs:2",
                                                                        "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ::tracing::Level::TRACE,
                                                                        ::core::option::Option::Some("src/lib.rs"),
                                                                        ::core::option::Option::Some(2u32),
                                                                        ::core::option::Option::Some(
                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ),
                                                                        ::tracing_core::field::FieldSet::new(
                                                                            &["message", "instance", "func"],
                                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                        ),
                                                                        ::tracing::metadata::Kind::EVENT,
                                                                    )
                                                                };
                                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                                            };
                                                            let enabled = ::tracing::Level::TRACE
                                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                && ::tracing::Level::TRACE
                                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                                && {
                                                                    let interest = __CALLSITE.interest();
                                                                    !interest.is_never()
                                                                        && ::tracing::__macro_support::__is_enabled(
                                                                            __CALLSITE.metadata(),
                                                                            interest,
                                                                        )
                                                                };
                                                            if enabled {
                                                                (|value_set: ::tracing::field::ValueSet| {
                                                                    let meta = __CALLSITE.metadata();
                                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                                })({
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("calling handler") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"container-exists" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                });
                                                            } else {
                                                            }
                                                        };
                                                        match Handler::container_exists(&handler, cx, p0).await {
                                                            Ok(results) => {
                                                                match tx((results,)).await {
                                                                    Ok(()) => {
                                                                        if let Some(rx) = rx {
                                                                            {
                                                                                use ::tracing::__macro_support::Callsite as _;
                                                                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                                    static META: ::tracing::Metadata<'static> = {
                                                                                        ::tracing_core::metadata::Metadata::new(
                                                                                            "event src/lib.rs:2",
                                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ::tracing::Level::TRACE,
                                                                                            ::core::option::Option::Some("src/lib.rs"),
                                                                                            ::core::option::Option::Some(2u32),
                                                                                            ::core::option::Option::Some(
                                                                                                "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ),
                                                                                            ::tracing_core::field::FieldSet::new(
                                                                                                &["message", "instance", "func"],
                                                                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                                            ),
                                                                                            ::tracing::metadata::Kind::EVENT,
                                                                                        )
                                                                                    };
                                                                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                                                                };
                                                                                let enabled = ::tracing::Level::TRACE
                                                                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                                    && ::tracing::Level::TRACE
                                                                                        <= ::tracing::level_filters::LevelFilter::current()
                                                                                    && {
                                                                                        let interest = __CALLSITE.interest();
                                                                                        !interest.is_never()
                                                                                            && ::tracing::__macro_support::__is_enabled(
                                                                                                __CALLSITE.metadata(),
                                                                                                interest,
                                                                                            )
                                                                                    };
                                                                                if enabled {
                                                                                    (|value_set: ::tracing::field::ValueSet| {
                                                                                        let meta = __CALLSITE.metadata();
                                                                                        ::tracing::Event::dispatch(meta, &value_set);
                                                                                    })({
                                                                                        #[allow(unused_imports)]
                                                                                        use ::tracing::field::{debug, display, Value};
                                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                        __CALLSITE
                                                                                            .metadata()
                                                                                            .fields()
                                                                                            .value_set(
                                                                                                &[
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &format_args!("receiving async invocation parameters")
                                                                                                                as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"container-exists" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                ],
                                                                                            )
                                                                                    });
                                                                                } else {
                                                                                }
                                                                            };
                                                                            let rx = ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx.await,
                                                                                "`wrpc:blobstore/blobstore@0.2.0.container-exists` async parameter receipt task failed",
                                                                            )?;
                                                                            ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx,
                                                                                "failed to receive `wrpc:blobstore/blobstore@0.2.0.container-exists` async parameters",
                                                                            )
                                                                        } else {
                                                                            ::wit_bindgen_wrpc::anyhow::Ok(())
                                                                        }
                                                                    }
                                                                    Err(err) => {
                                                                        if let Some(rx) = rx {
                                                                            rx.abort();
                                                                        }
                                                                        return ::anyhow::__private::Err({
                                                                            use ::anyhow::__private::kind::*;
                                                                            let error = match err
                                                                                .context(
                                                                                    "failed to transmit `wrpc:blobstore/blobstore@0.2.0.container-exists` invocation results",
                                                                                )
                                                                            {
                                                                                error => (&error).anyhow_kind().new(error),
                                                                            };
                                                                            error
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                if let Some(rx) = rx {
                                                                    rx.abort();
                                                                }
                                                                return ::anyhow::__private::Err({
                                                                    use ::anyhow::__private::kind::*;
                                                                    let error = match err
                                                                        .context(
                                                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.container-exists` invocation",
                                                                        )
                                                                    {
                                                                        error => (&error).anyhow_kind().new(error),
                                                                    };
                                                                    error
                                                                });
                                                            }
                                                        }
                                                    })
                                                        as ::core::pin::Pin<
                                                            ::std::boxed::Box<
                                                                dyn ::core::future::Future<
                                                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                > + ::core::marker::Send + 'static,
                                                            >,
                                                        >
                                                },
                                            ),
                                        )
                                            as ::core::pin::Pin<
                                                ::std::boxed::Box<
                                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                                        Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                            ::core::pin::Pin<
                                                                ::std::boxed::Box<
                                                                    dyn ::core::future::Future<
                                                                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                    > + ::core::marker::Send + 'static,
                                                                >,
                                                            >,
                                                        >,
                                                    > + ::core::marker::Send + 'static,
                                                >,
                                            >,
                                    )
                                },
                                {
                                    let handler = handler.clone();
                                    (
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "create-container",
                                        ::std::boxed::Box::pin(
                                            ::wit_bindgen_wrpc::futures::TryStreamExt::map_ok(
                                                f_create_container,
                                                move |(cx, (p0,), rx, tx)| {
                                                    let handler = handler.clone();
                                                    ::std::boxed::Box::pin(async move {
                                                        let rx = rx
                                                            .map(
                                                                ::wit_bindgen_wrpc::tracing::Instrument::in_current_span,
                                                            )
                                                            .map(::wit_bindgen_wrpc::tokio::spawn);
                                                        {
                                                            use ::tracing::__macro_support::Callsite as _;
                                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                static META: ::tracing::Metadata<'static> = {
                                                                    ::tracing_core::metadata::Metadata::new(
                                                                        "event src/lib.rs:2",
                                                                        "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ::tracing::Level::TRACE,
                                                                        ::core::option::Option::Some("src/lib.rs"),
                                                                        ::core::option::Option::Some(2u32),
                                                                        ::core::option::Option::Some(
                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ),
                                                                        ::tracing_core::field::FieldSet::new(
                                                                            &["message", "instance", "func"],
                                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                        ),
                                                                        ::tracing::metadata::Kind::EVENT,
                                                                    )
                                                                };
                                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                                            };
                                                            let enabled = ::tracing::Level::TRACE
                                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                && ::tracing::Level::TRACE
                                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                                && {
                                                                    let interest = __CALLSITE.interest();
                                                                    !interest.is_never()
                                                                        && ::tracing::__macro_support::__is_enabled(
                                                                            __CALLSITE.metadata(),
                                                                            interest,
                                                                        )
                                                                };
                                                            if enabled {
                                                                (|value_set: ::tracing::field::ValueSet| {
                                                                    let meta = __CALLSITE.metadata();
                                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                                })({
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("calling handler") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"create-container" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                });
                                                            } else {
                                                            }
                                                        };
                                                        match Handler::create_container(&handler, cx, p0).await {
                                                            Ok(results) => {
                                                                match tx((results,)).await {
                                                                    Ok(()) => {
                                                                        if let Some(rx) = rx {
                                                                            {
                                                                                use ::tracing::__macro_support::Callsite as _;
                                                                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                                    static META: ::tracing::Metadata<'static> = {
                                                                                        ::tracing_core::metadata::Metadata::new(
                                                                                            "event src/lib.rs:2",
                                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ::tracing::Level::TRACE,
                                                                                            ::core::option::Option::Some("src/lib.rs"),
                                                                                            ::core::option::Option::Some(2u32),
                                                                                            ::core::option::Option::Some(
                                                                                                "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ),
                                                                                            ::tracing_core::field::FieldSet::new(
                                                                                                &["message", "instance", "func"],
                                                                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                                            ),
                                                                                            ::tracing::metadata::Kind::EVENT,
                                                                                        )
                                                                                    };
                                                                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                                                                };
                                                                                let enabled = ::tracing::Level::TRACE
                                                                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                                    && ::tracing::Level::TRACE
                                                                                        <= ::tracing::level_filters::LevelFilter::current()
                                                                                    && {
                                                                                        let interest = __CALLSITE.interest();
                                                                                        !interest.is_never()
                                                                                            && ::tracing::__macro_support::__is_enabled(
                                                                                                __CALLSITE.metadata(),
                                                                                                interest,
                                                                                            )
                                                                                    };
                                                                                if enabled {
                                                                                    (|value_set: ::tracing::field::ValueSet| {
                                                                                        let meta = __CALLSITE.metadata();
                                                                                        ::tracing::Event::dispatch(meta, &value_set);
                                                                                    })({
                                                                                        #[allow(unused_imports)]
                                                                                        use ::tracing::field::{debug, display, Value};
                                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                        __CALLSITE
                                                                                            .metadata()
                                                                                            .fields()
                                                                                            .value_set(
                                                                                                &[
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &format_args!("receiving async invocation parameters")
                                                                                                                as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"create-container" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                ],
                                                                                            )
                                                                                    });
                                                                                } else {
                                                                                }
                                                                            };
                                                                            let rx = ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx.await,
                                                                                "`wrpc:blobstore/blobstore@0.2.0.create-container` async parameter receipt task failed",
                                                                            )?;
                                                                            ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx,
                                                                                "failed to receive `wrpc:blobstore/blobstore@0.2.0.create-container` async parameters",
                                                                            )
                                                                        } else {
                                                                            ::wit_bindgen_wrpc::anyhow::Ok(())
                                                                        }
                                                                    }
                                                                    Err(err) => {
                                                                        if let Some(rx) = rx {
                                                                            rx.abort();
                                                                        }
                                                                        return ::anyhow::__private::Err({
                                                                            use ::anyhow::__private::kind::*;
                                                                            let error = match err
                                                                                .context(
                                                                                    "failed to transmit `wrpc:blobstore/blobstore@0.2.0.create-container` invocation results",
                                                                                )
                                                                            {
                                                                                error => (&error).anyhow_kind().new(error),
                                                                            };
                                                                            error
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                if let Some(rx) = rx {
                                                                    rx.abort();
                                                                }
                                                                return ::anyhow::__private::Err({
                                                                    use ::anyhow::__private::kind::*;
                                                                    let error = match err
                                                                        .context(
                                                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.create-container` invocation",
                                                                        )
                                                                    {
                                                                        error => (&error).anyhow_kind().new(error),
                                                                    };
                                                                    error
                                                                });
                                                            }
                                                        }
                                                    })
                                                        as ::core::pin::Pin<
                                                            ::std::boxed::Box<
                                                                dyn ::core::future::Future<
                                                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                > + ::core::marker::Send + 'static,
                                                            >,
                                                        >
                                                },
                                            ),
                                        )
                                            as ::core::pin::Pin<
                                                ::std::boxed::Box<
                                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                                        Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                            ::core::pin::Pin<
                                                                ::std::boxed::Box<
                                                                    dyn ::core::future::Future<
                                                                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                    > + ::core::marker::Send + 'static,
                                                                >,
                                                            >,
                                                        >,
                                                    > + ::core::marker::Send + 'static,
                                                >,
                                            >,
                                    )
                                },
                                {
                                    let handler = handler.clone();
                                    (
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "delete-container",
                                        ::std::boxed::Box::pin(
                                            ::wit_bindgen_wrpc::futures::TryStreamExt::map_ok(
                                                f_delete_container,
                                                move |(cx, (p0,), rx, tx)| {
                                                    let handler = handler.clone();
                                                    ::std::boxed::Box::pin(async move {
                                                        let rx = rx
                                                            .map(
                                                                ::wit_bindgen_wrpc::tracing::Instrument::in_current_span,
                                                            )
                                                            .map(::wit_bindgen_wrpc::tokio::spawn);
                                                        {
                                                            use ::tracing::__macro_support::Callsite as _;
                                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                static META: ::tracing::Metadata<'static> = {
                                                                    ::tracing_core::metadata::Metadata::new(
                                                                        "event src/lib.rs:2",
                                                                        "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ::tracing::Level::TRACE,
                                                                        ::core::option::Option::Some("src/lib.rs"),
                                                                        ::core::option::Option::Some(2u32),
                                                                        ::core::option::Option::Some(
                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ),
                                                                        ::tracing_core::field::FieldSet::new(
                                                                            &["message", "instance", "func"],
                                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                        ),
                                                                        ::tracing::metadata::Kind::EVENT,
                                                                    )
                                                                };
                                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                                            };
                                                            let enabled = ::tracing::Level::TRACE
                                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                && ::tracing::Level::TRACE
                                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                                && {
                                                                    let interest = __CALLSITE.interest();
                                                                    !interest.is_never()
                                                                        && ::tracing::__macro_support::__is_enabled(
                                                                            __CALLSITE.metadata(),
                                                                            interest,
                                                                        )
                                                                };
                                                            if enabled {
                                                                (|value_set: ::tracing::field::ValueSet| {
                                                                    let meta = __CALLSITE.metadata();
                                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                                })({
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("calling handler") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"delete-container" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                });
                                                            } else {
                                                            }
                                                        };
                                                        match Handler::delete_container(&handler, cx, p0).await {
                                                            Ok(results) => {
                                                                match tx((results,)).await {
                                                                    Ok(()) => {
                                                                        if let Some(rx) = rx {
                                                                            {
                                                                                use ::tracing::__macro_support::Callsite as _;
                                                                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                                    static META: ::tracing::Metadata<'static> = {
                                                                                        ::tracing_core::metadata::Metadata::new(
                                                                                            "event src/lib.rs:2",
                                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ::tracing::Level::TRACE,
                                                                                            ::core::option::Option::Some("src/lib.rs"),
                                                                                            ::core::option::Option::Some(2u32),
                                                                                            ::core::option::Option::Some(
                                                                                                "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ),
                                                                                            ::tracing_core::field::FieldSet::new(
                                                                                                &["message", "instance", "func"],
                                                                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                                            ),
                                                                                            ::tracing::metadata::Kind::EVENT,
                                                                                        )
                                                                                    };
                                                                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                                                                };
                                                                                let enabled = ::tracing::Level::TRACE
                                                                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                                    && ::tracing::Level::TRACE
                                                                                        <= ::tracing::level_filters::LevelFilter::current()
                                                                                    && {
                                                                                        let interest = __CALLSITE.interest();
                                                                                        !interest.is_never()
                                                                                            && ::tracing::__macro_support::__is_enabled(
                                                                                                __CALLSITE.metadata(),
                                                                                                interest,
                                                                                            )
                                                                                    };
                                                                                if enabled {
                                                                                    (|value_set: ::tracing::field::ValueSet| {
                                                                                        let meta = __CALLSITE.metadata();
                                                                                        ::tracing::Event::dispatch(meta, &value_set);
                                                                                    })({
                                                                                        #[allow(unused_imports)]
                                                                                        use ::tracing::field::{debug, display, Value};
                                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                        __CALLSITE
                                                                                            .metadata()
                                                                                            .fields()
                                                                                            .value_set(
                                                                                                &[
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &format_args!("receiving async invocation parameters")
                                                                                                                as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"delete-container" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                ],
                                                                                            )
                                                                                    });
                                                                                } else {
                                                                                }
                                                                            };
                                                                            let rx = ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx.await,
                                                                                "`wrpc:blobstore/blobstore@0.2.0.delete-container` async parameter receipt task failed",
                                                                            )?;
                                                                            ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx,
                                                                                "failed to receive `wrpc:blobstore/blobstore@0.2.0.delete-container` async parameters",
                                                                            )
                                                                        } else {
                                                                            ::wit_bindgen_wrpc::anyhow::Ok(())
                                                                        }
                                                                    }
                                                                    Err(err) => {
                                                                        if let Some(rx) = rx {
                                                                            rx.abort();
                                                                        }
                                                                        return ::anyhow::__private::Err({
                                                                            use ::anyhow::__private::kind::*;
                                                                            let error = match err
                                                                                .context(
                                                                                    "failed to transmit `wrpc:blobstore/blobstore@0.2.0.delete-container` invocation results",
                                                                                )
                                                                            {
                                                                                error => (&error).anyhow_kind().new(error),
                                                                            };
                                                                            error
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                if let Some(rx) = rx {
                                                                    rx.abort();
                                                                }
                                                                return ::anyhow::__private::Err({
                                                                    use ::anyhow::__private::kind::*;
                                                                    let error = match err
                                                                        .context(
                                                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.delete-container` invocation",
                                                                        )
                                                                    {
                                                                        error => (&error).anyhow_kind().new(error),
                                                                    };
                                                                    error
                                                                });
                                                            }
                                                        }
                                                    })
                                                        as ::core::pin::Pin<
                                                            ::std::boxed::Box<
                                                                dyn ::core::future::Future<
                                                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                > + ::core::marker::Send + 'static,
                                                            >,
                                                        >
                                                },
                                            ),
                                        )
                                            as ::core::pin::Pin<
                                                ::std::boxed::Box<
                                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                                        Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                            ::core::pin::Pin<
                                                                ::std::boxed::Box<
                                                                    dyn ::core::future::Future<
                                                                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                    > + ::core::marker::Send + 'static,
                                                                >,
                                                            >,
                                                        >,
                                                    > + ::core::marker::Send + 'static,
                                                >,
                                            >,
                                    )
                                },
                                {
                                    let handler = handler.clone();
                                    (
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "get-container-info",
                                        ::std::boxed::Box::pin(
                                            ::wit_bindgen_wrpc::futures::TryStreamExt::map_ok(
                                                f_get_container_info,
                                                move |(cx, (p0,), rx, tx)| {
                                                    let handler = handler.clone();
                                                    ::std::boxed::Box::pin(async move {
                                                        let rx = rx
                                                            .map(
                                                                ::wit_bindgen_wrpc::tracing::Instrument::in_current_span,
                                                            )
                                                            .map(::wit_bindgen_wrpc::tokio::spawn);
                                                        {
                                                            use ::tracing::__macro_support::Callsite as _;
                                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                static META: ::tracing::Metadata<'static> = {
                                                                    ::tracing_core::metadata::Metadata::new(
                                                                        "event src/lib.rs:2",
                                                                        "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ::tracing::Level::TRACE,
                                                                        ::core::option::Option::Some("src/lib.rs"),
                                                                        ::core::option::Option::Some(2u32),
                                                                        ::core::option::Option::Some(
                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ),
                                                                        ::tracing_core::field::FieldSet::new(
                                                                            &["message", "instance", "func"],
                                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                        ),
                                                                        ::tracing::metadata::Kind::EVENT,
                                                                    )
                                                                };
                                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                                            };
                                                            let enabled = ::tracing::Level::TRACE
                                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                && ::tracing::Level::TRACE
                                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                                && {
                                                                    let interest = __CALLSITE.interest();
                                                                    !interest.is_never()
                                                                        && ::tracing::__macro_support::__is_enabled(
                                                                            __CALLSITE.metadata(),
                                                                            interest,
                                                                        )
                                                                };
                                                            if enabled {
                                                                (|value_set: ::tracing::field::ValueSet| {
                                                                    let meta = __CALLSITE.metadata();
                                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                                })({
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("calling handler") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"get-container-info" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                });
                                                            } else {
                                                            }
                                                        };
                                                        match Handler::get_container_info(&handler, cx, p0).await {
                                                            Ok(results) => {
                                                                match tx((results,)).await {
                                                                    Ok(()) => {
                                                                        if let Some(rx) = rx {
                                                                            {
                                                                                use ::tracing::__macro_support::Callsite as _;
                                                                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                                    static META: ::tracing::Metadata<'static> = {
                                                                                        ::tracing_core::metadata::Metadata::new(
                                                                                            "event src/lib.rs:2",
                                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ::tracing::Level::TRACE,
                                                                                            ::core::option::Option::Some("src/lib.rs"),
                                                                                            ::core::option::Option::Some(2u32),
                                                                                            ::core::option::Option::Some(
                                                                                                "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ),
                                                                                            ::tracing_core::field::FieldSet::new(
                                                                                                &["message", "instance", "func"],
                                                                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                                            ),
                                                                                            ::tracing::metadata::Kind::EVENT,
                                                                                        )
                                                                                    };
                                                                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                                                                };
                                                                                let enabled = ::tracing::Level::TRACE
                                                                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                                    && ::tracing::Level::TRACE
                                                                                        <= ::tracing::level_filters::LevelFilter::current()
                                                                                    && {
                                                                                        let interest = __CALLSITE.interest();
                                                                                        !interest.is_never()
                                                                                            && ::tracing::__macro_support::__is_enabled(
                                                                                                __CALLSITE.metadata(),
                                                                                                interest,
                                                                                            )
                                                                                    };
                                                                                if enabled {
                                                                                    (|value_set: ::tracing::field::ValueSet| {
                                                                                        let meta = __CALLSITE.metadata();
                                                                                        ::tracing::Event::dispatch(meta, &value_set);
                                                                                    })({
                                                                                        #[allow(unused_imports)]
                                                                                        use ::tracing::field::{debug, display, Value};
                                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                        __CALLSITE
                                                                                            .metadata()
                                                                                            .fields()
                                                                                            .value_set(
                                                                                                &[
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &format_args!("receiving async invocation parameters")
                                                                                                                as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"get-container-info" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                ],
                                                                                            )
                                                                                    });
                                                                                } else {
                                                                                }
                                                                            };
                                                                            let rx = ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx.await,
                                                                                "`wrpc:blobstore/blobstore@0.2.0.get-container-info` async parameter receipt task failed",
                                                                            )?;
                                                                            ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx,
                                                                                "failed to receive `wrpc:blobstore/blobstore@0.2.0.get-container-info` async parameters",
                                                                            )
                                                                        } else {
                                                                            ::wit_bindgen_wrpc::anyhow::Ok(())
                                                                        }
                                                                    }
                                                                    Err(err) => {
                                                                        if let Some(rx) = rx {
                                                                            rx.abort();
                                                                        }
                                                                        return ::anyhow::__private::Err({
                                                                            use ::anyhow::__private::kind::*;
                                                                            let error = match err
                                                                                .context(
                                                                                    "failed to transmit `wrpc:blobstore/blobstore@0.2.0.get-container-info` invocation results",
                                                                                )
                                                                            {
                                                                                error => (&error).anyhow_kind().new(error),
                                                                            };
                                                                            error
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                if let Some(rx) = rx {
                                                                    rx.abort();
                                                                }
                                                                return ::anyhow::__private::Err({
                                                                    use ::anyhow::__private::kind::*;
                                                                    let error = match err
                                                                        .context(
                                                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.get-container-info` invocation",
                                                                        )
                                                                    {
                                                                        error => (&error).anyhow_kind().new(error),
                                                                    };
                                                                    error
                                                                });
                                                            }
                                                        }
                                                    })
                                                        as ::core::pin::Pin<
                                                            ::std::boxed::Box<
                                                                dyn ::core::future::Future<
                                                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                > + ::core::marker::Send + 'static,
                                                            >,
                                                        >
                                                },
                                            ),
                                        )
                                            as ::core::pin::Pin<
                                                ::std::boxed::Box<
                                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                                        Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                            ::core::pin::Pin<
                                                                ::std::boxed::Box<
                                                                    dyn ::core::future::Future<
                                                                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                    > + ::core::marker::Send + 'static,
                                                                >,
                                                            >,
                                                        >,
                                                    > + ::core::marker::Send + 'static,
                                                >,
                                            >,
                                    )
                                },
                                {
                                    let handler = handler.clone();
                                    (
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "list-container-objects",
                                        ::std::boxed::Box::pin(
                                            ::wit_bindgen_wrpc::futures::TryStreamExt::map_ok(
                                                f_list_container_objects,
                                                move |(cx, (p0, p1, p2), rx, tx)| {
                                                    let handler = handler.clone();
                                                    ::std::boxed::Box::pin(async move {
                                                        let rx = rx
                                                            .map(
                                                                ::wit_bindgen_wrpc::tracing::Instrument::in_current_span,
                                                            )
                                                            .map(::wit_bindgen_wrpc::tokio::spawn);
                                                        {
                                                            use ::tracing::__macro_support::Callsite as _;
                                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                static META: ::tracing::Metadata<'static> = {
                                                                    ::tracing_core::metadata::Metadata::new(
                                                                        "event src/lib.rs:2",
                                                                        "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ::tracing::Level::TRACE,
                                                                        ::core::option::Option::Some("src/lib.rs"),
                                                                        ::core::option::Option::Some(2u32),
                                                                        ::core::option::Option::Some(
                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ),
                                                                        ::tracing_core::field::FieldSet::new(
                                                                            &["message", "instance", "func"],
                                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                        ),
                                                                        ::tracing::metadata::Kind::EVENT,
                                                                    )
                                                                };
                                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                                            };
                                                            let enabled = ::tracing::Level::TRACE
                                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                && ::tracing::Level::TRACE
                                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                                && {
                                                                    let interest = __CALLSITE.interest();
                                                                    !interest.is_never()
                                                                        && ::tracing::__macro_support::__is_enabled(
                                                                            __CALLSITE.metadata(),
                                                                            interest,
                                                                        )
                                                                };
                                                            if enabled {
                                                                (|value_set: ::tracing::field::ValueSet| {
                                                                    let meta = __CALLSITE.metadata();
                                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                                })({
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("calling handler") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"list-container-objects" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                });
                                                            } else {
                                                            }
                                                        };
                                                        match Handler::list_container_objects(
                                                                &handler,
                                                                cx,
                                                                p0,
                                                                p1,
                                                                p2,
                                                            )
                                                            .await
                                                        {
                                                            Ok(results) => {
                                                                match tx((results,)).await {
                                                                    Ok(()) => {
                                                                        if let Some(rx) = rx {
                                                                            {
                                                                                use ::tracing::__macro_support::Callsite as _;
                                                                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                                    static META: ::tracing::Metadata<'static> = {
                                                                                        ::tracing_core::metadata::Metadata::new(
                                                                                            "event src/lib.rs:2",
                                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ::tracing::Level::TRACE,
                                                                                            ::core::option::Option::Some("src/lib.rs"),
                                                                                            ::core::option::Option::Some(2u32),
                                                                                            ::core::option::Option::Some(
                                                                                                "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ),
                                                                                            ::tracing_core::field::FieldSet::new(
                                                                                                &["message", "instance", "func"],
                                                                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                                            ),
                                                                                            ::tracing::metadata::Kind::EVENT,
                                                                                        )
                                                                                    };
                                                                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                                                                };
                                                                                let enabled = ::tracing::Level::TRACE
                                                                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                                    && ::tracing::Level::TRACE
                                                                                        <= ::tracing::level_filters::LevelFilter::current()
                                                                                    && {
                                                                                        let interest = __CALLSITE.interest();
                                                                                        !interest.is_never()
                                                                                            && ::tracing::__macro_support::__is_enabled(
                                                                                                __CALLSITE.metadata(),
                                                                                                interest,
                                                                                            )
                                                                                    };
                                                                                if enabled {
                                                                                    (|value_set: ::tracing::field::ValueSet| {
                                                                                        let meta = __CALLSITE.metadata();
                                                                                        ::tracing::Event::dispatch(meta, &value_set);
                                                                                    })({
                                                                                        #[allow(unused_imports)]
                                                                                        use ::tracing::field::{debug, display, Value};
                                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                        __CALLSITE
                                                                                            .metadata()
                                                                                            .fields()
                                                                                            .value_set(
                                                                                                &[
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &format_args!("receiving async invocation parameters")
                                                                                                                as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"list-container-objects" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                ],
                                                                                            )
                                                                                    });
                                                                                } else {
                                                                                }
                                                                            };
                                                                            let rx = ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx.await,
                                                                                "`wrpc:blobstore/blobstore@0.2.0.list-container-objects` async parameter receipt task failed",
                                                                            )?;
                                                                            ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx,
                                                                                "failed to receive `wrpc:blobstore/blobstore@0.2.0.list-container-objects` async parameters",
                                                                            )
                                                                        } else {
                                                                            ::wit_bindgen_wrpc::anyhow::Ok(())
                                                                        }
                                                                    }
                                                                    Err(err) => {
                                                                        if let Some(rx) = rx {
                                                                            rx.abort();
                                                                        }
                                                                        return ::anyhow::__private::Err({
                                                                            use ::anyhow::__private::kind::*;
                                                                            let error = match err
                                                                                .context(
                                                                                    "failed to transmit `wrpc:blobstore/blobstore@0.2.0.list-container-objects` invocation results",
                                                                                )
                                                                            {
                                                                                error => (&error).anyhow_kind().new(error),
                                                                            };
                                                                            error
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                if let Some(rx) = rx {
                                                                    rx.abort();
                                                                }
                                                                return ::anyhow::__private::Err({
                                                                    use ::anyhow::__private::kind::*;
                                                                    let error = match err
                                                                        .context(
                                                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.list-container-objects` invocation",
                                                                        )
                                                                    {
                                                                        error => (&error).anyhow_kind().new(error),
                                                                    };
                                                                    error
                                                                });
                                                            }
                                                        }
                                                    })
                                                        as ::core::pin::Pin<
                                                            ::std::boxed::Box<
                                                                dyn ::core::future::Future<
                                                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                > + ::core::marker::Send + 'static,
                                                            >,
                                                        >
                                                },
                                            ),
                                        )
                                            as ::core::pin::Pin<
                                                ::std::boxed::Box<
                                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                                        Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                            ::core::pin::Pin<
                                                                ::std::boxed::Box<
                                                                    dyn ::core::future::Future<
                                                                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                    > + ::core::marker::Send + 'static,
                                                                >,
                                                            >,
                                                        >,
                                                    > + ::core::marker::Send + 'static,
                                                >,
                                            >,
                                    )
                                },
                                {
                                    let handler = handler.clone();
                                    (
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "copy-object",
                                        ::std::boxed::Box::pin(
                                            ::wit_bindgen_wrpc::futures::TryStreamExt::map_ok(
                                                f_copy_object,
                                                move |(cx, (p0, p1), rx, tx)| {
                                                    let handler = handler.clone();
                                                    ::std::boxed::Box::pin(async move {
                                                        let rx = rx
                                                            .map(
                                                                ::wit_bindgen_wrpc::tracing::Instrument::in_current_span,
                                                            )
                                                            .map(::wit_bindgen_wrpc::tokio::spawn);
                                                        {
                                                            use ::tracing::__macro_support::Callsite as _;
                                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                static META: ::tracing::Metadata<'static> = {
                                                                    ::tracing_core::metadata::Metadata::new(
                                                                        "event src/lib.rs:2",
                                                                        "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ::tracing::Level::TRACE,
                                                                        ::core::option::Option::Some("src/lib.rs"),
                                                                        ::core::option::Option::Some(2u32),
                                                                        ::core::option::Option::Some(
                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ),
                                                                        ::tracing_core::field::FieldSet::new(
                                                                            &["message", "instance", "func"],
                                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                        ),
                                                                        ::tracing::metadata::Kind::EVENT,
                                                                    )
                                                                };
                                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                                            };
                                                            let enabled = ::tracing::Level::TRACE
                                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                && ::tracing::Level::TRACE
                                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                                && {
                                                                    let interest = __CALLSITE.interest();
                                                                    !interest.is_never()
                                                                        && ::tracing::__macro_support::__is_enabled(
                                                                            __CALLSITE.metadata(),
                                                                            interest,
                                                                        )
                                                                };
                                                            if enabled {
                                                                (|value_set: ::tracing::field::ValueSet| {
                                                                    let meta = __CALLSITE.metadata();
                                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                                })({
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("calling handler") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(&"copy-object" as &dyn Value),
                                                                                ),
                                                                            ],
                                                                        )
                                                                });
                                                            } else {
                                                            }
                                                        };
                                                        match Handler::copy_object(&handler, cx, p0, p1).await {
                                                            Ok(results) => {
                                                                match tx((results,)).await {
                                                                    Ok(()) => {
                                                                        if let Some(rx) = rx {
                                                                            {
                                                                                use ::tracing::__macro_support::Callsite as _;
                                                                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                                    static META: ::tracing::Metadata<'static> = {
                                                                                        ::tracing_core::metadata::Metadata::new(
                                                                                            "event src/lib.rs:2",
                                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ::tracing::Level::TRACE,
                                                                                            ::core::option::Option::Some("src/lib.rs"),
                                                                                            ::core::option::Option::Some(2u32),
                                                                                            ::core::option::Option::Some(
                                                                                                "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ),
                                                                                            ::tracing_core::field::FieldSet::new(
                                                                                                &["message", "instance", "func"],
                                                                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                                            ),
                                                                                            ::tracing::metadata::Kind::EVENT,
                                                                                        )
                                                                                    };
                                                                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                                                                };
                                                                                let enabled = ::tracing::Level::TRACE
                                                                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                                    && ::tracing::Level::TRACE
                                                                                        <= ::tracing::level_filters::LevelFilter::current()
                                                                                    && {
                                                                                        let interest = __CALLSITE.interest();
                                                                                        !interest.is_never()
                                                                                            && ::tracing::__macro_support::__is_enabled(
                                                                                                __CALLSITE.metadata(),
                                                                                                interest,
                                                                                            )
                                                                                    };
                                                                                if enabled {
                                                                                    (|value_set: ::tracing::field::ValueSet| {
                                                                                        let meta = __CALLSITE.metadata();
                                                                                        ::tracing::Event::dispatch(meta, &value_set);
                                                                                    })({
                                                                                        #[allow(unused_imports)]
                                                                                        use ::tracing::field::{debug, display, Value};
                                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                        __CALLSITE
                                                                                            .metadata()
                                                                                            .fields()
                                                                                            .value_set(
                                                                                                &[
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &format_args!("receiving async invocation parameters")
                                                                                                                as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(&"copy-object" as &dyn Value),
                                                                                                    ),
                                                                                                ],
                                                                                            )
                                                                                    });
                                                                                } else {
                                                                                }
                                                                            };
                                                                            let rx = ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx.await,
                                                                                "`wrpc:blobstore/blobstore@0.2.0.copy-object` async parameter receipt task failed",
                                                                            )?;
                                                                            ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx,
                                                                                "failed to receive `wrpc:blobstore/blobstore@0.2.0.copy-object` async parameters",
                                                                            )
                                                                        } else {
                                                                            ::wit_bindgen_wrpc::anyhow::Ok(())
                                                                        }
                                                                    }
                                                                    Err(err) => {
                                                                        if let Some(rx) = rx {
                                                                            rx.abort();
                                                                        }
                                                                        return ::anyhow::__private::Err({
                                                                            use ::anyhow::__private::kind::*;
                                                                            let error = match err
                                                                                .context(
                                                                                    "failed to transmit `wrpc:blobstore/blobstore@0.2.0.copy-object` invocation results",
                                                                                )
                                                                            {
                                                                                error => (&error).anyhow_kind().new(error),
                                                                            };
                                                                            error
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                if let Some(rx) = rx {
                                                                    rx.abort();
                                                                }
                                                                return ::anyhow::__private::Err({
                                                                    use ::anyhow::__private::kind::*;
                                                                    let error = match err
                                                                        .context(
                                                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.copy-object` invocation",
                                                                        )
                                                                    {
                                                                        error => (&error).anyhow_kind().new(error),
                                                                    };
                                                                    error
                                                                });
                                                            }
                                                        }
                                                    })
                                                        as ::core::pin::Pin<
                                                            ::std::boxed::Box<
                                                                dyn ::core::future::Future<
                                                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                > + ::core::marker::Send + 'static,
                                                            >,
                                                        >
                                                },
                                            ),
                                        )
                                            as ::core::pin::Pin<
                                                ::std::boxed::Box<
                                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                                        Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                            ::core::pin::Pin<
                                                                ::std::boxed::Box<
                                                                    dyn ::core::future::Future<
                                                                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                    > + ::core::marker::Send + 'static,
                                                                >,
                                                            >,
                                                        >,
                                                    > + ::core::marker::Send + 'static,
                                                >,
                                            >,
                                    )
                                },
                                {
                                    let handler = handler.clone();
                                    (
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "delete-object",
                                        ::std::boxed::Box::pin(
                                            ::wit_bindgen_wrpc::futures::TryStreamExt::map_ok(
                                                f_delete_object,
                                                move |(cx, (p0,), rx, tx)| {
                                                    let handler = handler.clone();
                                                    ::std::boxed::Box::pin(async move {
                                                        let rx = rx
                                                            .map(
                                                                ::wit_bindgen_wrpc::tracing::Instrument::in_current_span,
                                                            )
                                                            .map(::wit_bindgen_wrpc::tokio::spawn);
                                                        {
                                                            use ::tracing::__macro_support::Callsite as _;
                                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                static META: ::tracing::Metadata<'static> = {
                                                                    ::tracing_core::metadata::Metadata::new(
                                                                        "event src/lib.rs:2",
                                                                        "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ::tracing::Level::TRACE,
                                                                        ::core::option::Option::Some("src/lib.rs"),
                                                                        ::core::option::Option::Some(2u32),
                                                                        ::core::option::Option::Some(
                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ),
                                                                        ::tracing_core::field::FieldSet::new(
                                                                            &["message", "instance", "func"],
                                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                        ),
                                                                        ::tracing::metadata::Kind::EVENT,
                                                                    )
                                                                };
                                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                                            };
                                                            let enabled = ::tracing::Level::TRACE
                                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                && ::tracing::Level::TRACE
                                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                                && {
                                                                    let interest = __CALLSITE.interest();
                                                                    !interest.is_never()
                                                                        && ::tracing::__macro_support::__is_enabled(
                                                                            __CALLSITE.metadata(),
                                                                            interest,
                                                                        )
                                                                };
                                                            if enabled {
                                                                (|value_set: ::tracing::field::ValueSet| {
                                                                    let meta = __CALLSITE.metadata();
                                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                                })({
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("calling handler") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(&"delete-object" as &dyn Value),
                                                                                ),
                                                                            ],
                                                                        )
                                                                });
                                                            } else {
                                                            }
                                                        };
                                                        match Handler::delete_object(&handler, cx, p0).await {
                                                            Ok(results) => {
                                                                match tx((results,)).await {
                                                                    Ok(()) => {
                                                                        if let Some(rx) = rx {
                                                                            {
                                                                                use ::tracing::__macro_support::Callsite as _;
                                                                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                                    static META: ::tracing::Metadata<'static> = {
                                                                                        ::tracing_core::metadata::Metadata::new(
                                                                                            "event src/lib.rs:2",
                                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ::tracing::Level::TRACE,
                                                                                            ::core::option::Option::Some("src/lib.rs"),
                                                                                            ::core::option::Option::Some(2u32),
                                                                                            ::core::option::Option::Some(
                                                                                                "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ),
                                                                                            ::tracing_core::field::FieldSet::new(
                                                                                                &["message", "instance", "func"],
                                                                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                                            ),
                                                                                            ::tracing::metadata::Kind::EVENT,
                                                                                        )
                                                                                    };
                                                                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                                                                };
                                                                                let enabled = ::tracing::Level::TRACE
                                                                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                                    && ::tracing::Level::TRACE
                                                                                        <= ::tracing::level_filters::LevelFilter::current()
                                                                                    && {
                                                                                        let interest = __CALLSITE.interest();
                                                                                        !interest.is_never()
                                                                                            && ::tracing::__macro_support::__is_enabled(
                                                                                                __CALLSITE.metadata(),
                                                                                                interest,
                                                                                            )
                                                                                    };
                                                                                if enabled {
                                                                                    (|value_set: ::tracing::field::ValueSet| {
                                                                                        let meta = __CALLSITE.metadata();
                                                                                        ::tracing::Event::dispatch(meta, &value_set);
                                                                                    })({
                                                                                        #[allow(unused_imports)]
                                                                                        use ::tracing::field::{debug, display, Value};
                                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                        __CALLSITE
                                                                                            .metadata()
                                                                                            .fields()
                                                                                            .value_set(
                                                                                                &[
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &format_args!("receiving async invocation parameters")
                                                                                                                as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(&"delete-object" as &dyn Value),
                                                                                                    ),
                                                                                                ],
                                                                                            )
                                                                                    });
                                                                                } else {
                                                                                }
                                                                            };
                                                                            let rx = ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx.await,
                                                                                "`wrpc:blobstore/blobstore@0.2.0.delete-object` async parameter receipt task failed",
                                                                            )?;
                                                                            ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx,
                                                                                "failed to receive `wrpc:blobstore/blobstore@0.2.0.delete-object` async parameters",
                                                                            )
                                                                        } else {
                                                                            ::wit_bindgen_wrpc::anyhow::Ok(())
                                                                        }
                                                                    }
                                                                    Err(err) => {
                                                                        if let Some(rx) = rx {
                                                                            rx.abort();
                                                                        }
                                                                        return ::anyhow::__private::Err({
                                                                            use ::anyhow::__private::kind::*;
                                                                            let error = match err
                                                                                .context(
                                                                                    "failed to transmit `wrpc:blobstore/blobstore@0.2.0.delete-object` invocation results",
                                                                                )
                                                                            {
                                                                                error => (&error).anyhow_kind().new(error),
                                                                            };
                                                                            error
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                if let Some(rx) = rx {
                                                                    rx.abort();
                                                                }
                                                                return ::anyhow::__private::Err({
                                                                    use ::anyhow::__private::kind::*;
                                                                    let error = match err
                                                                        .context(
                                                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.delete-object` invocation",
                                                                        )
                                                                    {
                                                                        error => (&error).anyhow_kind().new(error),
                                                                    };
                                                                    error
                                                                });
                                                            }
                                                        }
                                                    })
                                                        as ::core::pin::Pin<
                                                            ::std::boxed::Box<
                                                                dyn ::core::future::Future<
                                                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                > + ::core::marker::Send + 'static,
                                                            >,
                                                        >
                                                },
                                            ),
                                        )
                                            as ::core::pin::Pin<
                                                ::std::boxed::Box<
                                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                                        Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                            ::core::pin::Pin<
                                                                ::std::boxed::Box<
                                                                    dyn ::core::future::Future<
                                                                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                    > + ::core::marker::Send + 'static,
                                                                >,
                                                            >,
                                                        >,
                                                    > + ::core::marker::Send + 'static,
                                                >,
                                            >,
                                    )
                                },
                                {
                                    let handler = handler.clone();
                                    (
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "delete-objects",
                                        ::std::boxed::Box::pin(
                                            ::wit_bindgen_wrpc::futures::TryStreamExt::map_ok(
                                                f_delete_objects,
                                                move |(cx, (p0, p1), rx, tx)| {
                                                    let handler = handler.clone();
                                                    ::std::boxed::Box::pin(async move {
                                                        let rx = rx
                                                            .map(
                                                                ::wit_bindgen_wrpc::tracing::Instrument::in_current_span,
                                                            )
                                                            .map(::wit_bindgen_wrpc::tokio::spawn);
                                                        {
                                                            use ::tracing::__macro_support::Callsite as _;
                                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                static META: ::tracing::Metadata<'static> = {
                                                                    ::tracing_core::metadata::Metadata::new(
                                                                        "event src/lib.rs:2",
                                                                        "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ::tracing::Level::TRACE,
                                                                        ::core::option::Option::Some("src/lib.rs"),
                                                                        ::core::option::Option::Some(2u32),
                                                                        ::core::option::Option::Some(
                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ),
                                                                        ::tracing_core::field::FieldSet::new(
                                                                            &["message", "instance", "func"],
                                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                        ),
                                                                        ::tracing::metadata::Kind::EVENT,
                                                                    )
                                                                };
                                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                                            };
                                                            let enabled = ::tracing::Level::TRACE
                                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                && ::tracing::Level::TRACE
                                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                                && {
                                                                    let interest = __CALLSITE.interest();
                                                                    !interest.is_never()
                                                                        && ::tracing::__macro_support::__is_enabled(
                                                                            __CALLSITE.metadata(),
                                                                            interest,
                                                                        )
                                                                };
                                                            if enabled {
                                                                (|value_set: ::tracing::field::ValueSet| {
                                                                    let meta = __CALLSITE.metadata();
                                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                                })({
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("calling handler") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"delete-objects" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                });
                                                            } else {
                                                            }
                                                        };
                                                        match Handler::delete_objects(&handler, cx, p0, p1).await {
                                                            Ok(results) => {
                                                                match tx((results,)).await {
                                                                    Ok(()) => {
                                                                        if let Some(rx) = rx {
                                                                            {
                                                                                use ::tracing::__macro_support::Callsite as _;
                                                                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                                    static META: ::tracing::Metadata<'static> = {
                                                                                        ::tracing_core::metadata::Metadata::new(
                                                                                            "event src/lib.rs:2",
                                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ::tracing::Level::TRACE,
                                                                                            ::core::option::Option::Some("src/lib.rs"),
                                                                                            ::core::option::Option::Some(2u32),
                                                                                            ::core::option::Option::Some(
                                                                                                "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ),
                                                                                            ::tracing_core::field::FieldSet::new(
                                                                                                &["message", "instance", "func"],
                                                                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                                            ),
                                                                                            ::tracing::metadata::Kind::EVENT,
                                                                                        )
                                                                                    };
                                                                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                                                                };
                                                                                let enabled = ::tracing::Level::TRACE
                                                                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                                    && ::tracing::Level::TRACE
                                                                                        <= ::tracing::level_filters::LevelFilter::current()
                                                                                    && {
                                                                                        let interest = __CALLSITE.interest();
                                                                                        !interest.is_never()
                                                                                            && ::tracing::__macro_support::__is_enabled(
                                                                                                __CALLSITE.metadata(),
                                                                                                interest,
                                                                                            )
                                                                                    };
                                                                                if enabled {
                                                                                    (|value_set: ::tracing::field::ValueSet| {
                                                                                        let meta = __CALLSITE.metadata();
                                                                                        ::tracing::Event::dispatch(meta, &value_set);
                                                                                    })({
                                                                                        #[allow(unused_imports)]
                                                                                        use ::tracing::field::{debug, display, Value};
                                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                        __CALLSITE
                                                                                            .metadata()
                                                                                            .fields()
                                                                                            .value_set(
                                                                                                &[
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &format_args!("receiving async invocation parameters")
                                                                                                                as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"delete-objects" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                ],
                                                                                            )
                                                                                    });
                                                                                } else {
                                                                                }
                                                                            };
                                                                            let rx = ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx.await,
                                                                                "`wrpc:blobstore/blobstore@0.2.0.delete-objects` async parameter receipt task failed",
                                                                            )?;
                                                                            ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx,
                                                                                "failed to receive `wrpc:blobstore/blobstore@0.2.0.delete-objects` async parameters",
                                                                            )
                                                                        } else {
                                                                            ::wit_bindgen_wrpc::anyhow::Ok(())
                                                                        }
                                                                    }
                                                                    Err(err) => {
                                                                        if let Some(rx) = rx {
                                                                            rx.abort();
                                                                        }
                                                                        return ::anyhow::__private::Err({
                                                                            use ::anyhow::__private::kind::*;
                                                                            let error = match err
                                                                                .context(
                                                                                    "failed to transmit `wrpc:blobstore/blobstore@0.2.0.delete-objects` invocation results",
                                                                                )
                                                                            {
                                                                                error => (&error).anyhow_kind().new(error),
                                                                            };
                                                                            error
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                if let Some(rx) = rx {
                                                                    rx.abort();
                                                                }
                                                                return ::anyhow::__private::Err({
                                                                    use ::anyhow::__private::kind::*;
                                                                    let error = match err
                                                                        .context(
                                                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.delete-objects` invocation",
                                                                        )
                                                                    {
                                                                        error => (&error).anyhow_kind().new(error),
                                                                    };
                                                                    error
                                                                });
                                                            }
                                                        }
                                                    })
                                                        as ::core::pin::Pin<
                                                            ::std::boxed::Box<
                                                                dyn ::core::future::Future<
                                                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                > + ::core::marker::Send + 'static,
                                                            >,
                                                        >
                                                },
                                            ),
                                        )
                                            as ::core::pin::Pin<
                                                ::std::boxed::Box<
                                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                                        Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                            ::core::pin::Pin<
                                                                ::std::boxed::Box<
                                                                    dyn ::core::future::Future<
                                                                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                    > + ::core::marker::Send + 'static,
                                                                >,
                                                            >,
                                                        >,
                                                    > + ::core::marker::Send + 'static,
                                                >,
                                            >,
                                    )
                                },
                                {
                                    let handler = handler.clone();
                                    (
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "get-container-data",
                                        ::std::boxed::Box::pin(
                                            ::wit_bindgen_wrpc::futures::TryStreamExt::map_ok(
                                                f_get_container_data,
                                                move |(cx, (p0, p1, p2), rx, tx)| {
                                                    let handler = handler.clone();
                                                    ::std::boxed::Box::pin(async move {
                                                        let rx = rx
                                                            .map(
                                                                ::wit_bindgen_wrpc::tracing::Instrument::in_current_span,
                                                            )
                                                            .map(::wit_bindgen_wrpc::tokio::spawn);
                                                        {
                                                            use ::tracing::__macro_support::Callsite as _;
                                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                static META: ::tracing::Metadata<'static> = {
                                                                    ::tracing_core::metadata::Metadata::new(
                                                                        "event src/lib.rs:2",
                                                                        "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ::tracing::Level::TRACE,
                                                                        ::core::option::Option::Some("src/lib.rs"),
                                                                        ::core::option::Option::Some(2u32),
                                                                        ::core::option::Option::Some(
                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ),
                                                                        ::tracing_core::field::FieldSet::new(
                                                                            &["message", "instance", "func"],
                                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                        ),
                                                                        ::tracing::metadata::Kind::EVENT,
                                                                    )
                                                                };
                                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                                            };
                                                            let enabled = ::tracing::Level::TRACE
                                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                && ::tracing::Level::TRACE
                                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                                && {
                                                                    let interest = __CALLSITE.interest();
                                                                    !interest.is_never()
                                                                        && ::tracing::__macro_support::__is_enabled(
                                                                            __CALLSITE.metadata(),
                                                                            interest,
                                                                        )
                                                                };
                                                            if enabled {
                                                                (|value_set: ::tracing::field::ValueSet| {
                                                                    let meta = __CALLSITE.metadata();
                                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                                })({
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("calling handler") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"get-container-data" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                });
                                                            } else {
                                                            }
                                                        };
                                                        match Handler::get_container_data(&handler, cx, p0, p1, p2)
                                                            .await
                                                        {
                                                            Ok(results) => {
                                                                match tx((results,)).await {
                                                                    Ok(()) => {
                                                                        if let Some(rx) = rx {
                                                                            {
                                                                                use ::tracing::__macro_support::Callsite as _;
                                                                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                                    static META: ::tracing::Metadata<'static> = {
                                                                                        ::tracing_core::metadata::Metadata::new(
                                                                                            "event src/lib.rs:2",
                                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ::tracing::Level::TRACE,
                                                                                            ::core::option::Option::Some("src/lib.rs"),
                                                                                            ::core::option::Option::Some(2u32),
                                                                                            ::core::option::Option::Some(
                                                                                                "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ),
                                                                                            ::tracing_core::field::FieldSet::new(
                                                                                                &["message", "instance", "func"],
                                                                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                                            ),
                                                                                            ::tracing::metadata::Kind::EVENT,
                                                                                        )
                                                                                    };
                                                                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                                                                };
                                                                                let enabled = ::tracing::Level::TRACE
                                                                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                                    && ::tracing::Level::TRACE
                                                                                        <= ::tracing::level_filters::LevelFilter::current()
                                                                                    && {
                                                                                        let interest = __CALLSITE.interest();
                                                                                        !interest.is_never()
                                                                                            && ::tracing::__macro_support::__is_enabled(
                                                                                                __CALLSITE.metadata(),
                                                                                                interest,
                                                                                            )
                                                                                    };
                                                                                if enabled {
                                                                                    (|value_set: ::tracing::field::ValueSet| {
                                                                                        let meta = __CALLSITE.metadata();
                                                                                        ::tracing::Event::dispatch(meta, &value_set);
                                                                                    })({
                                                                                        #[allow(unused_imports)]
                                                                                        use ::tracing::field::{debug, display, Value};
                                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                        __CALLSITE
                                                                                            .metadata()
                                                                                            .fields()
                                                                                            .value_set(
                                                                                                &[
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &format_args!("receiving async invocation parameters")
                                                                                                                as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"get-container-data" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                ],
                                                                                            )
                                                                                    });
                                                                                } else {
                                                                                }
                                                                            };
                                                                            let rx = ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx.await,
                                                                                "`wrpc:blobstore/blobstore@0.2.0.get-container-data` async parameter receipt task failed",
                                                                            )?;
                                                                            ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx,
                                                                                "failed to receive `wrpc:blobstore/blobstore@0.2.0.get-container-data` async parameters",
                                                                            )
                                                                        } else {
                                                                            ::wit_bindgen_wrpc::anyhow::Ok(())
                                                                        }
                                                                    }
                                                                    Err(err) => {
                                                                        if let Some(rx) = rx {
                                                                            rx.abort();
                                                                        }
                                                                        return ::anyhow::__private::Err({
                                                                            use ::anyhow::__private::kind::*;
                                                                            let error = match err
                                                                                .context(
                                                                                    "failed to transmit `wrpc:blobstore/blobstore@0.2.0.get-container-data` invocation results",
                                                                                )
                                                                            {
                                                                                error => (&error).anyhow_kind().new(error),
                                                                            };
                                                                            error
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                if let Some(rx) = rx {
                                                                    rx.abort();
                                                                }
                                                                return ::anyhow::__private::Err({
                                                                    use ::anyhow::__private::kind::*;
                                                                    let error = match err
                                                                        .context(
                                                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.get-container-data` invocation",
                                                                        )
                                                                    {
                                                                        error => (&error).anyhow_kind().new(error),
                                                                    };
                                                                    error
                                                                });
                                                            }
                                                        }
                                                    })
                                                        as ::core::pin::Pin<
                                                            ::std::boxed::Box<
                                                                dyn ::core::future::Future<
                                                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                > + ::core::marker::Send + 'static,
                                                            >,
                                                        >
                                                },
                                            ),
                                        )
                                            as ::core::pin::Pin<
                                                ::std::boxed::Box<
                                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                                        Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                            ::core::pin::Pin<
                                                                ::std::boxed::Box<
                                                                    dyn ::core::future::Future<
                                                                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                    > + ::core::marker::Send + 'static,
                                                                >,
                                                            >,
                                                        >,
                                                    > + ::core::marker::Send + 'static,
                                                >,
                                            >,
                                    )
                                },
                                {
                                    let handler = handler.clone();
                                    (
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "get-object-info",
                                        ::std::boxed::Box::pin(
                                            ::wit_bindgen_wrpc::futures::TryStreamExt::map_ok(
                                                f_get_object_info,
                                                move |(cx, (p0,), rx, tx)| {
                                                    let handler = handler.clone();
                                                    ::std::boxed::Box::pin(async move {
                                                        let rx = rx
                                                            .map(
                                                                ::wit_bindgen_wrpc::tracing::Instrument::in_current_span,
                                                            )
                                                            .map(::wit_bindgen_wrpc::tokio::spawn);
                                                        {
                                                            use ::tracing::__macro_support::Callsite as _;
                                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                static META: ::tracing::Metadata<'static> = {
                                                                    ::tracing_core::metadata::Metadata::new(
                                                                        "event src/lib.rs:2",
                                                                        "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ::tracing::Level::TRACE,
                                                                        ::core::option::Option::Some("src/lib.rs"),
                                                                        ::core::option::Option::Some(2u32),
                                                                        ::core::option::Option::Some(
                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ),
                                                                        ::tracing_core::field::FieldSet::new(
                                                                            &["message", "instance", "func"],
                                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                        ),
                                                                        ::tracing::metadata::Kind::EVENT,
                                                                    )
                                                                };
                                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                                            };
                                                            let enabled = ::tracing::Level::TRACE
                                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                && ::tracing::Level::TRACE
                                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                                && {
                                                                    let interest = __CALLSITE.interest();
                                                                    !interest.is_never()
                                                                        && ::tracing::__macro_support::__is_enabled(
                                                                            __CALLSITE.metadata(),
                                                                            interest,
                                                                        )
                                                                };
                                                            if enabled {
                                                                (|value_set: ::tracing::field::ValueSet| {
                                                                    let meta = __CALLSITE.metadata();
                                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                                })({
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("calling handler") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"get-object-info" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                });
                                                            } else {
                                                            }
                                                        };
                                                        match Handler::get_object_info(&handler, cx, p0).await {
                                                            Ok(results) => {
                                                                match tx((results,)).await {
                                                                    Ok(()) => {
                                                                        if let Some(rx) = rx {
                                                                            {
                                                                                use ::tracing::__macro_support::Callsite as _;
                                                                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                                    static META: ::tracing::Metadata<'static> = {
                                                                                        ::tracing_core::metadata::Metadata::new(
                                                                                            "event src/lib.rs:2",
                                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ::tracing::Level::TRACE,
                                                                                            ::core::option::Option::Some("src/lib.rs"),
                                                                                            ::core::option::Option::Some(2u32),
                                                                                            ::core::option::Option::Some(
                                                                                                "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ),
                                                                                            ::tracing_core::field::FieldSet::new(
                                                                                                &["message", "instance", "func"],
                                                                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                                            ),
                                                                                            ::tracing::metadata::Kind::EVENT,
                                                                                        )
                                                                                    };
                                                                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                                                                };
                                                                                let enabled = ::tracing::Level::TRACE
                                                                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                                    && ::tracing::Level::TRACE
                                                                                        <= ::tracing::level_filters::LevelFilter::current()
                                                                                    && {
                                                                                        let interest = __CALLSITE.interest();
                                                                                        !interest.is_never()
                                                                                            && ::tracing::__macro_support::__is_enabled(
                                                                                                __CALLSITE.metadata(),
                                                                                                interest,
                                                                                            )
                                                                                    };
                                                                                if enabled {
                                                                                    (|value_set: ::tracing::field::ValueSet| {
                                                                                        let meta = __CALLSITE.metadata();
                                                                                        ::tracing::Event::dispatch(meta, &value_set);
                                                                                    })({
                                                                                        #[allow(unused_imports)]
                                                                                        use ::tracing::field::{debug, display, Value};
                                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                        __CALLSITE
                                                                                            .metadata()
                                                                                            .fields()
                                                                                            .value_set(
                                                                                                &[
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &format_args!("receiving async invocation parameters")
                                                                                                                as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"get-object-info" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                ],
                                                                                            )
                                                                                    });
                                                                                } else {
                                                                                }
                                                                            };
                                                                            let rx = ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx.await,
                                                                                "`wrpc:blobstore/blobstore@0.2.0.get-object-info` async parameter receipt task failed",
                                                                            )?;
                                                                            ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx,
                                                                                "failed to receive `wrpc:blobstore/blobstore@0.2.0.get-object-info` async parameters",
                                                                            )
                                                                        } else {
                                                                            ::wit_bindgen_wrpc::anyhow::Ok(())
                                                                        }
                                                                    }
                                                                    Err(err) => {
                                                                        if let Some(rx) = rx {
                                                                            rx.abort();
                                                                        }
                                                                        return ::anyhow::__private::Err({
                                                                            use ::anyhow::__private::kind::*;
                                                                            let error = match err
                                                                                .context(
                                                                                    "failed to transmit `wrpc:blobstore/blobstore@0.2.0.get-object-info` invocation results",
                                                                                )
                                                                            {
                                                                                error => (&error).anyhow_kind().new(error),
                                                                            };
                                                                            error
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                if let Some(rx) = rx {
                                                                    rx.abort();
                                                                }
                                                                return ::anyhow::__private::Err({
                                                                    use ::anyhow::__private::kind::*;
                                                                    let error = match err
                                                                        .context(
                                                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.get-object-info` invocation",
                                                                        )
                                                                    {
                                                                        error => (&error).anyhow_kind().new(error),
                                                                    };
                                                                    error
                                                                });
                                                            }
                                                        }
                                                    })
                                                        as ::core::pin::Pin<
                                                            ::std::boxed::Box<
                                                                dyn ::core::future::Future<
                                                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                > + ::core::marker::Send + 'static,
                                                            >,
                                                        >
                                                },
                                            ),
                                        )
                                            as ::core::pin::Pin<
                                                ::std::boxed::Box<
                                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                                        Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                            ::core::pin::Pin<
                                                                ::std::boxed::Box<
                                                                    dyn ::core::future::Future<
                                                                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                    > + ::core::marker::Send + 'static,
                                                                >,
                                                            >,
                                                        >,
                                                    > + ::core::marker::Send + 'static,
                                                >,
                                            >,
                                    )
                                },
                                {
                                    let handler = handler.clone();
                                    (
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "has-object",
                                        ::std::boxed::Box::pin(
                                            ::wit_bindgen_wrpc::futures::TryStreamExt::map_ok(
                                                f_has_object,
                                                move |(cx, (p0,), rx, tx)| {
                                                    let handler = handler.clone();
                                                    ::std::boxed::Box::pin(async move {
                                                        let rx = rx
                                                            .map(
                                                                ::wit_bindgen_wrpc::tracing::Instrument::in_current_span,
                                                            )
                                                            .map(::wit_bindgen_wrpc::tokio::spawn);
                                                        {
                                                            use ::tracing::__macro_support::Callsite as _;
                                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                static META: ::tracing::Metadata<'static> = {
                                                                    ::tracing_core::metadata::Metadata::new(
                                                                        "event src/lib.rs:2",
                                                                        "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ::tracing::Level::TRACE,
                                                                        ::core::option::Option::Some("src/lib.rs"),
                                                                        ::core::option::Option::Some(2u32),
                                                                        ::core::option::Option::Some(
                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ),
                                                                        ::tracing_core::field::FieldSet::new(
                                                                            &["message", "instance", "func"],
                                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                        ),
                                                                        ::tracing::metadata::Kind::EVENT,
                                                                    )
                                                                };
                                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                                            };
                                                            let enabled = ::tracing::Level::TRACE
                                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                && ::tracing::Level::TRACE
                                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                                && {
                                                                    let interest = __CALLSITE.interest();
                                                                    !interest.is_never()
                                                                        && ::tracing::__macro_support::__is_enabled(
                                                                            __CALLSITE.metadata(),
                                                                            interest,
                                                                        )
                                                                };
                                                            if enabled {
                                                                (|value_set: ::tracing::field::ValueSet| {
                                                                    let meta = __CALLSITE.metadata();
                                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                                })({
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("calling handler") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(&"has-object" as &dyn Value),
                                                                                ),
                                                                            ],
                                                                        )
                                                                });
                                                            } else {
                                                            }
                                                        };
                                                        match Handler::has_object(&handler, cx, p0).await {
                                                            Ok(results) => {
                                                                match tx((results,)).await {
                                                                    Ok(()) => {
                                                                        if let Some(rx) = rx {
                                                                            {
                                                                                use ::tracing::__macro_support::Callsite as _;
                                                                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                                    static META: ::tracing::Metadata<'static> = {
                                                                                        ::tracing_core::metadata::Metadata::new(
                                                                                            "event src/lib.rs:2",
                                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ::tracing::Level::TRACE,
                                                                                            ::core::option::Option::Some("src/lib.rs"),
                                                                                            ::core::option::Option::Some(2u32),
                                                                                            ::core::option::Option::Some(
                                                                                                "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ),
                                                                                            ::tracing_core::field::FieldSet::new(
                                                                                                &["message", "instance", "func"],
                                                                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                                            ),
                                                                                            ::tracing::metadata::Kind::EVENT,
                                                                                        )
                                                                                    };
                                                                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                                                                };
                                                                                let enabled = ::tracing::Level::TRACE
                                                                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                                    && ::tracing::Level::TRACE
                                                                                        <= ::tracing::level_filters::LevelFilter::current()
                                                                                    && {
                                                                                        let interest = __CALLSITE.interest();
                                                                                        !interest.is_never()
                                                                                            && ::tracing::__macro_support::__is_enabled(
                                                                                                __CALLSITE.metadata(),
                                                                                                interest,
                                                                                            )
                                                                                    };
                                                                                if enabled {
                                                                                    (|value_set: ::tracing::field::ValueSet| {
                                                                                        let meta = __CALLSITE.metadata();
                                                                                        ::tracing::Event::dispatch(meta, &value_set);
                                                                                    })({
                                                                                        #[allow(unused_imports)]
                                                                                        use ::tracing::field::{debug, display, Value};
                                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                        __CALLSITE
                                                                                            .metadata()
                                                                                            .fields()
                                                                                            .value_set(
                                                                                                &[
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &format_args!("receiving async invocation parameters")
                                                                                                                as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(&"has-object" as &dyn Value),
                                                                                                    ),
                                                                                                ],
                                                                                            )
                                                                                    });
                                                                                } else {
                                                                                }
                                                                            };
                                                                            let rx = ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx.await,
                                                                                "`wrpc:blobstore/blobstore@0.2.0.has-object` async parameter receipt task failed",
                                                                            )?;
                                                                            ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx,
                                                                                "failed to receive `wrpc:blobstore/blobstore@0.2.0.has-object` async parameters",
                                                                            )
                                                                        } else {
                                                                            ::wit_bindgen_wrpc::anyhow::Ok(())
                                                                        }
                                                                    }
                                                                    Err(err) => {
                                                                        if let Some(rx) = rx {
                                                                            rx.abort();
                                                                        }
                                                                        return ::anyhow::__private::Err({
                                                                            use ::anyhow::__private::kind::*;
                                                                            let error = match err
                                                                                .context(
                                                                                    "failed to transmit `wrpc:blobstore/blobstore@0.2.0.has-object` invocation results",
                                                                                )
                                                                            {
                                                                                error => (&error).anyhow_kind().new(error),
                                                                            };
                                                                            error
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                if let Some(rx) = rx {
                                                                    rx.abort();
                                                                }
                                                                return ::anyhow::__private::Err({
                                                                    use ::anyhow::__private::kind::*;
                                                                    let error = match err
                                                                        .context(
                                                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.has-object` invocation",
                                                                        )
                                                                    {
                                                                        error => (&error).anyhow_kind().new(error),
                                                                    };
                                                                    error
                                                                });
                                                            }
                                                        }
                                                    })
                                                        as ::core::pin::Pin<
                                                            ::std::boxed::Box<
                                                                dyn ::core::future::Future<
                                                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                > + ::core::marker::Send + 'static,
                                                            >,
                                                        >
                                                },
                                            ),
                                        )
                                            as ::core::pin::Pin<
                                                ::std::boxed::Box<
                                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                                        Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                            ::core::pin::Pin<
                                                                ::std::boxed::Box<
                                                                    dyn ::core::future::Future<
                                                                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                    > + ::core::marker::Send + 'static,
                                                                >,
                                                            >,
                                                        >,
                                                    > + ::core::marker::Send + 'static,
                                                >,
                                            >,
                                    )
                                },
                                {
                                    let handler = handler.clone();
                                    (
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "move-object",
                                        ::std::boxed::Box::pin(
                                            ::wit_bindgen_wrpc::futures::TryStreamExt::map_ok(
                                                f_move_object,
                                                move |(cx, (p0, p1), rx, tx)| {
                                                    let handler = handler.clone();
                                                    ::std::boxed::Box::pin(async move {
                                                        let rx = rx
                                                            .map(
                                                                ::wit_bindgen_wrpc::tracing::Instrument::in_current_span,
                                                            )
                                                            .map(::wit_bindgen_wrpc::tokio::spawn);
                                                        {
                                                            use ::tracing::__macro_support::Callsite as _;
                                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                static META: ::tracing::Metadata<'static> = {
                                                                    ::tracing_core::metadata::Metadata::new(
                                                                        "event src/lib.rs:2",
                                                                        "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ::tracing::Level::TRACE,
                                                                        ::core::option::Option::Some("src/lib.rs"),
                                                                        ::core::option::Option::Some(2u32),
                                                                        ::core::option::Option::Some(
                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ),
                                                                        ::tracing_core::field::FieldSet::new(
                                                                            &["message", "instance", "func"],
                                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                        ),
                                                                        ::tracing::metadata::Kind::EVENT,
                                                                    )
                                                                };
                                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                                            };
                                                            let enabled = ::tracing::Level::TRACE
                                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                && ::tracing::Level::TRACE
                                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                                && {
                                                                    let interest = __CALLSITE.interest();
                                                                    !interest.is_never()
                                                                        && ::tracing::__macro_support::__is_enabled(
                                                                            __CALLSITE.metadata(),
                                                                            interest,
                                                                        )
                                                                };
                                                            if enabled {
                                                                (|value_set: ::tracing::field::ValueSet| {
                                                                    let meta = __CALLSITE.metadata();
                                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                                })({
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("calling handler") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(&"move-object" as &dyn Value),
                                                                                ),
                                                                            ],
                                                                        )
                                                                });
                                                            } else {
                                                            }
                                                        };
                                                        match Handler::move_object(&handler, cx, p0, p1).await {
                                                            Ok(results) => {
                                                                match tx((results,)).await {
                                                                    Ok(()) => {
                                                                        if let Some(rx) = rx {
                                                                            {
                                                                                use ::tracing::__macro_support::Callsite as _;
                                                                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                                    static META: ::tracing::Metadata<'static> = {
                                                                                        ::tracing_core::metadata::Metadata::new(
                                                                                            "event src/lib.rs:2",
                                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ::tracing::Level::TRACE,
                                                                                            ::core::option::Option::Some("src/lib.rs"),
                                                                                            ::core::option::Option::Some(2u32),
                                                                                            ::core::option::Option::Some(
                                                                                                "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ),
                                                                                            ::tracing_core::field::FieldSet::new(
                                                                                                &["message", "instance", "func"],
                                                                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                                            ),
                                                                                            ::tracing::metadata::Kind::EVENT,
                                                                                        )
                                                                                    };
                                                                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                                                                };
                                                                                let enabled = ::tracing::Level::TRACE
                                                                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                                    && ::tracing::Level::TRACE
                                                                                        <= ::tracing::level_filters::LevelFilter::current()
                                                                                    && {
                                                                                        let interest = __CALLSITE.interest();
                                                                                        !interest.is_never()
                                                                                            && ::tracing::__macro_support::__is_enabled(
                                                                                                __CALLSITE.metadata(),
                                                                                                interest,
                                                                                            )
                                                                                    };
                                                                                if enabled {
                                                                                    (|value_set: ::tracing::field::ValueSet| {
                                                                                        let meta = __CALLSITE.metadata();
                                                                                        ::tracing::Event::dispatch(meta, &value_set);
                                                                                    })({
                                                                                        #[allow(unused_imports)]
                                                                                        use ::tracing::field::{debug, display, Value};
                                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                        __CALLSITE
                                                                                            .metadata()
                                                                                            .fields()
                                                                                            .value_set(
                                                                                                &[
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &format_args!("receiving async invocation parameters")
                                                                                                                as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(&"move-object" as &dyn Value),
                                                                                                    ),
                                                                                                ],
                                                                                            )
                                                                                    });
                                                                                } else {
                                                                                }
                                                                            };
                                                                            let rx = ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx.await,
                                                                                "`wrpc:blobstore/blobstore@0.2.0.move-object` async parameter receipt task failed",
                                                                            )?;
                                                                            ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx,
                                                                                "failed to receive `wrpc:blobstore/blobstore@0.2.0.move-object` async parameters",
                                                                            )
                                                                        } else {
                                                                            ::wit_bindgen_wrpc::anyhow::Ok(())
                                                                        }
                                                                    }
                                                                    Err(err) => {
                                                                        if let Some(rx) = rx {
                                                                            rx.abort();
                                                                        }
                                                                        return ::anyhow::__private::Err({
                                                                            use ::anyhow::__private::kind::*;
                                                                            let error = match err
                                                                                .context(
                                                                                    "failed to transmit `wrpc:blobstore/blobstore@0.2.0.move-object` invocation results",
                                                                                )
                                                                            {
                                                                                error => (&error).anyhow_kind().new(error),
                                                                            };
                                                                            error
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                if let Some(rx) = rx {
                                                                    rx.abort();
                                                                }
                                                                return ::anyhow::__private::Err({
                                                                    use ::anyhow::__private::kind::*;
                                                                    let error = match err
                                                                        .context(
                                                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.move-object` invocation",
                                                                        )
                                                                    {
                                                                        error => (&error).anyhow_kind().new(error),
                                                                    };
                                                                    error
                                                                });
                                                            }
                                                        }
                                                    })
                                                        as ::core::pin::Pin<
                                                            ::std::boxed::Box<
                                                                dyn ::core::future::Future<
                                                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                > + ::core::marker::Send + 'static,
                                                            >,
                                                        >
                                                },
                                            ),
                                        )
                                            as ::core::pin::Pin<
                                                ::std::boxed::Box<
                                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                                        Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                            ::core::pin::Pin<
                                                                ::std::boxed::Box<
                                                                    dyn ::core::future::Future<
                                                                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                    > + ::core::marker::Send + 'static,
                                                                >,
                                                            >,
                                                        >,
                                                    > + ::core::marker::Send + 'static,
                                                >,
                                            >,
                                    )
                                },
                                {
                                    let handler = handler.clone();
                                    (
                                        "wrpc:blobstore/blobstore@0.2.0",
                                        "write-container-data",
                                        ::std::boxed::Box::pin(
                                            ::wit_bindgen_wrpc::futures::TryStreamExt::map_ok(
                                                f_write_container_data,
                                                move |(cx, (p0, p1), rx, tx)| {
                                                    let handler = handler.clone();
                                                    ::std::boxed::Box::pin(async move {
                                                        let rx = rx
                                                            .map(
                                                                ::wit_bindgen_wrpc::tracing::Instrument::in_current_span,
                                                            )
                                                            .map(::wit_bindgen_wrpc::tokio::spawn);
                                                        {
                                                            use ::tracing::__macro_support::Callsite as _;
                                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                static META: ::tracing::Metadata<'static> = {
                                                                    ::tracing_core::metadata::Metadata::new(
                                                                        "event src/lib.rs:2",
                                                                        "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ::tracing::Level::TRACE,
                                                                        ::core::option::Option::Some("src/lib.rs"),
                                                                        ::core::option::Option::Some(2u32),
                                                                        ::core::option::Option::Some(
                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                        ),
                                                                        ::tracing_core::field::FieldSet::new(
                                                                            &["message", "instance", "func"],
                                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                        ),
                                                                        ::tracing::metadata::Kind::EVENT,
                                                                    )
                                                                };
                                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                                            };
                                                            let enabled = ::tracing::Level::TRACE
                                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                && ::tracing::Level::TRACE
                                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                                && {
                                                                    let interest = __CALLSITE.interest();
                                                                    !interest.is_never()
                                                                        && ::tracing::__macro_support::__is_enabled(
                                                                            __CALLSITE.metadata(),
                                                                            interest,
                                                                        )
                                                                };
                                                            if enabled {
                                                                (|value_set: ::tracing::field::ValueSet| {
                                                                    let meta = __CALLSITE.metadata();
                                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                                })({
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &format_args!("calling handler") as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                                (
                                                                                    &::core::iter::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::core::option::Option::Some(
                                                                                        &"write-container-data" as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                });
                                                            } else {
                                                            }
                                                        };
                                                        match Handler::write_container_data(&handler, cx, p0, p1)
                                                            .await
                                                        {
                                                            Ok(results) => {
                                                                match tx((results,)).await {
                                                                    Ok(()) => {
                                                                        if let Some(rx) = rx {
                                                                            {
                                                                                use ::tracing::__macro_support::Callsite as _;
                                                                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                                                    static META: ::tracing::Metadata<'static> = {
                                                                                        ::tracing_core::metadata::Metadata::new(
                                                                                            "event src/lib.rs:2",
                                                                                            "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ::tracing::Level::TRACE,
                                                                                            ::core::option::Option::Some("src/lib.rs"),
                                                                                            ::core::option::Option::Some(2u32),
                                                                                            ::core::option::Option::Some(
                                                                                                "wrpc_interface_blobstore::bindings::exports::wrpc::blobstore::blobstore",
                                                                                            ),
                                                                                            ::tracing_core::field::FieldSet::new(
                                                                                                &["message", "instance", "func"],
                                                                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                                                            ),
                                                                                            ::tracing::metadata::Kind::EVENT,
                                                                                        )
                                                                                    };
                                                                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                                                                };
                                                                                let enabled = ::tracing::Level::TRACE
                                                                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                                                    && ::tracing::Level::TRACE
                                                                                        <= ::tracing::level_filters::LevelFilter::current()
                                                                                    && {
                                                                                        let interest = __CALLSITE.interest();
                                                                                        !interest.is_never()
                                                                                            && ::tracing::__macro_support::__is_enabled(
                                                                                                __CALLSITE.metadata(),
                                                                                                interest,
                                                                                            )
                                                                                    };
                                                                                if enabled {
                                                                                    (|value_set: ::tracing::field::ValueSet| {
                                                                                        let meta = __CALLSITE.metadata();
                                                                                        ::tracing::Event::dispatch(meta, &value_set);
                                                                                    })({
                                                                                        #[allow(unused_imports)]
                                                                                        use ::tracing::field::{debug, display, Value};
                                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                                        __CALLSITE
                                                                                            .metadata()
                                                                                            .fields()
                                                                                            .value_set(
                                                                                                &[
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &format_args!("receiving async invocation parameters")
                                                                                                                as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"wrpc:blobstore/blobstore@0.2.0" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                    (
                                                                                                        &::core::iter::Iterator::next(&mut iter)
                                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                                        ::core::option::Option::Some(
                                                                                                            &"write-container-data" as &dyn Value,
                                                                                                        ),
                                                                                                    ),
                                                                                                ],
                                                                                            )
                                                                                    });
                                                                                } else {
                                                                                }
                                                                            };
                                                                            let rx = ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx.await,
                                                                                "`wrpc:blobstore/blobstore@0.2.0.write-container-data` async parameter receipt task failed",
                                                                            )?;
                                                                            ::wit_bindgen_wrpc::anyhow::Context::context(
                                                                                rx,
                                                                                "failed to receive `wrpc:blobstore/blobstore@0.2.0.write-container-data` async parameters",
                                                                            )
                                                                        } else {
                                                                            ::wit_bindgen_wrpc::anyhow::Ok(())
                                                                        }
                                                                    }
                                                                    Err(err) => {
                                                                        if let Some(rx) = rx {
                                                                            rx.abort();
                                                                        }
                                                                        return ::anyhow::__private::Err({
                                                                            use ::anyhow::__private::kind::*;
                                                                            let error = match err
                                                                                .context(
                                                                                    "failed to transmit `wrpc:blobstore/blobstore@0.2.0.write-container-data` invocation results",
                                                                                )
                                                                            {
                                                                                error => (&error).anyhow_kind().new(error),
                                                                            };
                                                                            error
                                                                        });
                                                                    }
                                                                }
                                                            }
                                                            Err(err) => {
                                                                if let Some(rx) = rx {
                                                                    rx.abort();
                                                                }
                                                                return ::anyhow::__private::Err({
                                                                    use ::anyhow::__private::kind::*;
                                                                    let error = match err
                                                                        .context(
                                                                            "failed to serve `wrpc:blobstore/blobstore@0.2.0.write-container-data` invocation",
                                                                        )
                                                                    {
                                                                        error => (&error).anyhow_kind().new(error),
                                                                    };
                                                                    error
                                                                });
                                                            }
                                                        }
                                                    })
                                                        as ::core::pin::Pin<
                                                            ::std::boxed::Box<
                                                                dyn ::core::future::Future<
                                                                    Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                > + ::core::marker::Send + 'static,
                                                            >,
                                                        >
                                                },
                                            ),
                                        )
                                            as ::core::pin::Pin<
                                                ::std::boxed::Box<
                                                    dyn ::wit_bindgen_wrpc::futures::Stream<
                                                        Item = ::wit_bindgen_wrpc::anyhow::Result<
                                                            ::core::pin::Pin<
                                                                ::std::boxed::Box<
                                                                    dyn ::core::future::Future<
                                                                        Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                                                    > + ::core::marker::Send + 'static,
                                                                >,
                                                            >,
                                                        >,
                                                    > + ::core::marker::Send + 'static,
                                                >,
                                            >,
                                    )
                                },
                            ])
                        }
                    }
                }
            }
        }
    }
    #[allow(clippy::manual_async_fn)]
    pub fn serve<'a, T: ::wit_bindgen_wrpc::wrpc_transport::Serve>(
        wrpc: &'a T,
        handler: impl exports::wrpc::blobstore::blobstore::Handler<
            T::Context,
        > + ::core::marker::Send + ::core::marker::Sync + ::core::clone::Clone + 'static,
    ) -> impl ::core::future::Future<
        Output = ::wit_bindgen_wrpc::anyhow::Result<
            ::std::vec::Vec<
                (
                    &'static str,
                    &'static str,
                    ::core::pin::Pin<
                        ::std::boxed::Box<
                            dyn ::wit_bindgen_wrpc::futures::Stream<
                                Item = ::wit_bindgen_wrpc::anyhow::Result<
                                    ::core::pin::Pin<
                                        ::std::boxed::Box<
                                            dyn ::core::future::Future<
                                                Output = ::wit_bindgen_wrpc::anyhow::Result<()>,
                                            > + ::core::marker::Send + 'static,
                                        >,
                                    >,
                                >,
                            > + ::core::marker::Send + 'static,
                        >,
                    >,
                ),
            >,
        >,
    > + ::core::marker::Send + ::wit_bindgen_wrpc::wrpc_transport::Captures<'a> {
        async move {
            let interfaces = {
                use ::tokio::macros::support::{maybe_done, poll_fn, Future, Pin};
                use ::tokio::macros::support::Poll::{Ready, Pending};
                let mut futures = (
                    maybe_done(
                        exports::wrpc::blobstore::blobstore::serve_interface(
                            wrpc,
                            handler.clone(),
                        ),
                    ),
                );
                let mut futures = &mut futures;
                let mut skip_next_time: u32 = 0;
                poll_fn(move |cx| {
                        const COUNT: u32 = 0 + 1;
                        let mut is_pending = false;
                        let mut to_run = COUNT;
                        let mut skip = skip_next_time;
                        skip_next_time = if skip + 1 == COUNT { 0 } else { skip + 1 };
                        loop {
                            if skip == 0 {
                                if to_run == 0 {
                                    break;
                                }
                                to_run -= 1;
                                let (fut, ..) = &mut *futures;
                                let mut fut = unsafe { Pin::new_unchecked(fut) };
                                if fut.as_mut().poll(cx).is_pending() {
                                    is_pending = true;
                                } else if fut
                                    .as_mut()
                                    .output_mut()
                                    .expect("expected completed future")
                                    .is_err()
                                {
                                    return Ready(
                                        Err(
                                            fut
                                                .take_output()
                                                .expect("expected completed future")
                                                .err()
                                                .unwrap(),
                                        ),
                                    )
                                }
                            } else {
                                skip -= 1;
                            }
                        }
                        if is_pending {
                            Pending
                        } else {
                            Ready(
                                Ok((
                                    {
                                        let (fut, ..) = &mut futures;
                                        let mut fut = unsafe { Pin::new_unchecked(fut) };
                                        fut.take_output()
                                            .expect("expected completed future")
                                            .ok()
                                            .expect("expected Ok(_)")
                                    },
                                )),
                            )
                        }
                    })
                    .await
            }?;
            let mut streams = Vec::new();
            for s in interfaces.0 {
                streams.push(s);
            }
            Ok(streams)
        }
    }
    const _: &[u8] = b"interface types {\n    use wasi:blobstore/types@0.2.0-draft.{\n        container-metadata as wasi-container-metadata,\n        container-name as wasi-container-name,\n        object-id as wasi-object-id,\n        object-metadata as wasi-object-metadata,\n        timestamp,\n        object-size,\n    };\n    \n    // information about a container\n    record container-metadata {\n      // date and time container was created\n      created-at: timestamp,\n    }\n\n    type container-name = wasi-container-name;\n    type object-id = wasi-object-id;\n\n    // information about an object\n    record object-metadata {\n        // date and time the object was created\n        created-at: timestamp,\n        // size of the object, in bytes\n        size: object-size,\n    }\n}\n\n";
    const _: &[u8] = b"interface blobstore {\n    use types.{container-name, container-metadata, object-metadata, object-id};\n\n    clear-container: func(name: string) -> result<_, string>;\n    container-exists: func(name: string) -> result<bool, string>;\n    create-container: func(name: string) -> result<_, string>;\n    delete-container: func(name: string) -> result<_, string>;\n    get-container-info: func(name: string) -> result<container-metadata, string>;\n    list-container-objects: func(name: string, limit: option<u64>, offset: option<u64>) -> result<tuple<stream<string>, future<result<_, string>>>, string>;\n\n    copy-object: func(src: object-id, dest: object-id) -> result<_, string>;\n    delete-object: func(id: object-id) -> result<_, string>;\n    delete-objects: func(container: string, objects: list<string>) -> result<_, string>;\n    get-container-data: func(id: object-id, start: u64, end: u64) -> result<tuple<stream<u8>, future<result<_, string>>>, string>;\n    get-object-info: func(id: object-id) -> result<object-metadata, string>;\n    has-object: func(id: object-id) -> result<bool, string>;\n    move-object: func(src: object-id, dest: object-id) -> result<_, string>;\n    write-container-data: func(id: object-id, data: stream<u8>) -> result<future<result<_, string>>, string>;\n}\n";
    const _: &[u8] = b"package wrpc:blobstore@0.2.0;\n\nworld imports {\n\timport blobstore;\n}\n\nworld interfaces {\n    import blobstore;\n\n    export blobstore;\n}\n";
    const _: &[u8] = b"// Types used by blobstore\ninterface types {\n  use wasi:io/streams@0.2.0.{input-stream, output-stream};\n\n  // name of a container, a collection of objects.\n  // The container name may be any valid UTF-8 string.\n  type container-name = string;\n\n  // name of an object within a container\n  // The object name may be any valid UTF-8 string.\n  type object-name = string;\n\n  // TODO: define timestamp to include seconds since\n  // Unix epoch and nanoseconds\n  // https://github.com/WebAssembly/wasi-blob-store/issues/7\n  type timestamp = u64;\n\n  // size of an object, in bytes\n  type object-size = u64;\n\n  type error = string;\n\n  // information about a container\n  record container-metadata {\n    // the container\'s name\n    name: container-name,\n    // date and time container was created\n    created-at: timestamp,\n  }\n\n  // information about an object\n  record object-metadata {\n    // the object\'s name\n    name: object-name,\n    // the object\'s parent container\n    container: container-name,\n    // date and time the object was created\n    created-at: timestamp,\n    // size of the object, in bytes\n    size: object-size,\n  }\n\n  // identifier for an object that includes its container name\n  record object-id {\n    container: container-name,\n    object: object-name\n  }\n\n  /// A data is the data stored in a data blob. The value can be of any type\n  /// that can be represented in a byte array. It provides a way to write the value\n  /// to the output-stream defined in the `wasi-io` interface.\n  // Soon: switch to `resource value { ... }`\n  resource outgoing-value {\n    new-outgoing-value: static func() -> outgoing-value;\n    outgoing-value-write-body: func() -> result<output-stream>;\n  }\n\n  /// A incoming-value is a wrapper around a value. It provides a way to read the value\n  /// from the input-stream defined in the `wasi-io` interface.\n  ///\n  /// The incoming-value provides two ways to consume the value:\n  /// 1. `incoming-value-consume-sync` consumes the value synchronously and returns the\n  ///    value as a list of bytes.\n  /// 2. `incoming-value-consume-async` consumes the value asynchronously and returns the\n  ///    value as an input-stream.\n  // Soon: switch to `resource incoming-value { ... }`\n  resource incoming-value {\n      incoming-value-consume-sync: static func(this: incoming-value) -> result<incoming-value-sync-body, error>;\n      incoming-value-consume-async: static func(this: incoming-value) -> result<incoming-value-async-body, error>;\n      size: func() -> u64;\n  }\n\n  type incoming-value-async-body = input-stream;\n  type incoming-value-sync-body = list<u8>;\n}\n";
    const _: &[u8] = b"// wasi-cloud Blobstore service definition\ninterface blobstore {\n  use container.{container};\n  use types.{error, container-name, object-id};\n\n  // creates a new empty container\n  create-container: func(name: container-name) -> result<container, error>;\n\n  // retrieves a container by name\n  get-container: func(name: container-name) -> result<container, error>;\n\n  // deletes a container and all objects within it\n  delete-container: func(name: container-name) -> result<_, error>;\n\n  // returns true if the container exists\n  container-exists: func(name: container-name) -> result<bool, error>;\n\n  // copies (duplicates) an object, to the same or a different container.\n  // returns an error if the target container does not exist.\n  // overwrites destination object if it already existed.\n  copy-object: func(src: object-id, dest: object-id) -> result<_, error>;\n\n  // moves or renames an object, to the same or a different container\n  // returns an error if the destination container does not exist.\n  // overwrites destination object if it already existed.\n  move-object: func(src:object-id, dest: object-id) -> result<_, error>;\n}";
    const _: &[u8] = b"package wasi:blobstore@0.2.0-draft;\n\nworld imports {\n\timport blobstore;\n}";
    const _: &[u8] = b"// a Container is a collection of objects\ninterface container {\n  use wasi:io/streams@0.2.0.{\n    input-stream,\n    output-stream,\n  };\n\n  use types.{\n    container-metadata,\n    error,\n    incoming-value,\n    object-metadata,\n    object-name,\n    outgoing-value,\n  };\n\n  // this defines the `container` resource\n  resource container {\n    // returns container name\n    name: func() -> result<string, error>;\n\n    // returns container metadata\n    info: func() -> result<container-metadata, error>;\n\n    // retrieves an object or portion of an object, as a resource.\n    // Start and end offsets are inclusive.\n    // Once a data-blob resource has been created, the underlying bytes are held by the blobstore service for the lifetime\n    // of the data-blob resource, even if the object they came from is later deleted.\n    get-data: func(name: object-name, start: u64, end: u64) -> result<incoming-value, error>;\n\n    // creates or replaces an object with the data blob.\n    write-data: func(name: object-name, data: borrow<outgoing-value>) -> result<_, error>;\n\n    // returns list of objects in the container. Order is undefined.\n    list-objects: func() -> result<stream-object-names, error>;\n\n    // deletes object.\n    // does not return error if object did not exist.\n    delete-object: func(name: object-name) -> result<_, error>;\n\n    // deletes multiple objects in the container\n    delete-objects: func(names: list<object-name>) -> result<_, error>;\n\n    // returns true if the object exists in this container\n    has-object: func(name: object-name) -> result<bool, error>;\n\n    // returns metadata for the object\n    object-info: func(name: object-name) -> result<object-metadata, error>;\n\n    // removes all objects within the container, leaving the container empty.\n    clear: func() -> result<_, error>;\n  }\n\n  // this defines the `stream-object-names` resource which is a representation of stream<object-name>\n  resource stream-object-names {\n    // reads the next number of objects from the stream\n    //\n    // This function returns the list of objects read, and a boolean indicating if the end of the stream was reached.\n    read-stream-object-names: func(len: u64) -> result<tuple<list<object-name>, bool>, error>;\n\n    // skip the next number of objects in the stream\n    //\n    // This function returns the number of objects skipped, and a boolean indicating if the end of the stream was reached.\n    skip-stream-object-names: func(num: u64) -> result<tuple<u64, bool>, error>;\n  }\n}";
    const _: &[u8] = b"package wasi:io@0.2.0;\n\n/// A poll API intended to let users wait for I/O events on multiple handles\n/// at once.\ninterface poll {\n    /// `pollable` represents a single I/O event which may be ready, or not.\n    resource pollable {\n\n      /// Return the readiness of a pollable. This function never blocks.\n      ///\n      /// Returns `true` when the pollable is ready, and `false` otherwise.\n      ready: func() -> bool;\n\n      /// `block` returns immediately if the pollable is ready, and otherwise\n      /// blocks until ready.\n      ///\n      /// This function is equivalent to calling `poll.poll` on a list\n      /// containing only this pollable.\n      block: func();\n    }\n\n    /// Poll for completion on a set of pollables.\n    ///\n    /// This function takes a list of pollables, which identify I/O sources of\n    /// interest, and waits until one or more of the events is ready for I/O.\n    ///\n    /// The result `list<u32>` contains one or more indices of handles in the\n    /// argument list that is ready for I/O.\n    ///\n    /// If the list contains more elements than can be indexed with a `u32`\n    /// value, this function traps.\n    ///\n    /// A timeout can be implemented by adding a pollable from the\n    /// wasi-clocks API to the list.\n    ///\n    /// This function does not return a `result`; polling in itself does not\n    /// do any I/O so it doesn\'t fail. If any of the I/O sources identified by\n    /// the pollables has an error, it is indicated by marking the source as\n    /// being reaedy for I/O.\n    poll: func(in: list<borrow<pollable>>) -> list<u32>;\n}\n";
    const _: &[u8] = b"package wasi:io@0.2.0;\n\n/// WASI I/O is an I/O abstraction API which is currently focused on providing\n/// stream types.\n///\n/// In the future, the component model is expected to add built-in stream types;\n/// when it does, they are expected to subsume this API.\ninterface streams {\n    use error.{error};\n    use poll.{pollable};\n\n    /// An error for input-stream and output-stream operations.\n    variant stream-error {\n        /// The last operation (a write or flush) failed before completion.\n        ///\n        /// More information is available in the `error` payload.\n        last-operation-failed(error),\n        /// The stream is closed: no more input will be accepted by the\n        /// stream. A closed output-stream will return this error on all\n        /// future operations.\n        closed\n    }\n\n    /// An input bytestream.\n    ///\n    /// `input-stream`s are *non-blocking* to the extent practical on underlying\n    /// platforms. I/O operations always return promptly; if fewer bytes are\n    /// promptly available than requested, they return the number of bytes promptly\n    /// available, which could even be zero. To wait for data to be available,\n    /// use the `subscribe` function to obtain a `pollable` which can be polled\n    /// for using `wasi:io/poll`.\n    resource input-stream {\n        /// Perform a non-blocking read from the stream.\n        ///\n        /// When the source of a `read` is binary data, the bytes from the source\n        /// are returned verbatim. When the source of a `read` is known to the\n        /// implementation to be text, bytes containing the UTF-8 encoding of the\n        /// text are returned.\n        ///\n        /// This function returns a list of bytes containing the read data,\n        /// when successful. The returned list will contain up to `len` bytes;\n        /// it may return fewer than requested, but not more. The list is\n        /// empty when no bytes are available for reading at this time. The\n        /// pollable given by `subscribe` will be ready when more bytes are\n        /// available.\n        ///\n        /// This function fails with a `stream-error` when the operation\n        /// encounters an error, giving `last-operation-failed`, or when the\n        /// stream is closed, giving `closed`.\n        ///\n        /// When the caller gives a `len` of 0, it represents a request to\n        /// read 0 bytes. If the stream is still open, this call should\n        /// succeed and return an empty list, or otherwise fail with `closed`.\n        ///\n        /// The `len` parameter is a `u64`, which could represent a list of u8 which\n        /// is not possible to allocate in wasm32, or not desirable to allocate as\n        /// as a return value by the callee. The callee may return a list of bytes\n        /// less than `len` in size while more bytes are available for reading.\n        read: func(\n            /// The maximum number of bytes to read\n            len: u64\n        ) -> result<list<u8>, stream-error>;\n\n        /// Read bytes from a stream, after blocking until at least one byte can\n        /// be read. Except for blocking, behavior is identical to `read`.\n        blocking-read: func(\n            /// The maximum number of bytes to read\n            len: u64\n        ) -> result<list<u8>, stream-error>;\n\n        /// Skip bytes from a stream. Returns number of bytes skipped.\n        ///\n        /// Behaves identical to `read`, except instead of returning a list\n        /// of bytes, returns the number of bytes consumed from the stream.\n        skip: func(\n            /// The maximum number of bytes to skip.\n            len: u64,\n        ) -> result<u64, stream-error>;\n\n        /// Skip bytes from a stream, after blocking until at least one byte\n        /// can be skipped. Except for blocking behavior, identical to `skip`.\n        blocking-skip: func(\n            /// The maximum number of bytes to skip.\n            len: u64,\n        ) -> result<u64, stream-error>;\n\n        /// Create a `pollable` which will resolve once either the specified stream\n        /// has bytes available to read or the other end of the stream has been\n        /// closed.\n        /// The created `pollable` is a child resource of the `input-stream`.\n        /// Implementations may trap if the `input-stream` is dropped before\n        /// all derived `pollable`s created with this function are dropped.\n        subscribe: func() -> pollable;\n    }\n\n\n    /// An output bytestream.\n    ///\n    /// `output-stream`s are *non-blocking* to the extent practical on\n    /// underlying platforms. Except where specified otherwise, I/O operations also\n    /// always return promptly, after the number of bytes that can be written\n    /// promptly, which could even be zero. To wait for the stream to be ready to\n    /// accept data, the `subscribe` function to obtain a `pollable` which can be\n    /// polled for using `wasi:io/poll`.\n    resource output-stream {\n        /// Check readiness for writing. This function never blocks.\n        ///\n        /// Returns the number of bytes permitted for the next call to `write`,\n        /// or an error. Calling `write` with more bytes than this function has\n        /// permitted will trap.\n        ///\n        /// When this function returns 0 bytes, the `subscribe` pollable will\n        /// become ready when this function will report at least 1 byte, or an\n        /// error.\n        check-write: func() -> result<u64, stream-error>;\n\n        /// Perform a write. This function never blocks.\n        ///\n        /// When the destination of a `write` is binary data, the bytes from\n        /// `contents` are written verbatim. When the destination of a `write` is\n        /// known to the implementation to be text, the bytes of `contents` are\n        /// transcoded from UTF-8 into the encoding of the destination and then\n        /// written.\n        ///\n        /// Precondition: check-write gave permit of Ok(n) and contents has a\n        /// length of less than or equal to n. Otherwise, this function will trap.\n        ///\n        /// returns Err(closed) without writing if the stream has closed since\n        /// the last call to check-write provided a permit.\n        write: func(\n            contents: list<u8>\n        ) -> result<_, stream-error>;\n\n        /// Perform a write of up to 4096 bytes, and then flush the stream. Block\n        /// until all of these operations are complete, or an error occurs.\n        ///\n        /// This is a convenience wrapper around the use of `check-write`,\n        /// `subscribe`, `write`, and `flush`, and is implemented with the\n        /// following pseudo-code:\n        ///\n        /// ```text\n        /// let pollable = this.subscribe();\n        /// while !contents.is_empty() {\n        ///     // Wait for the stream to become writable\n        ///     pollable.block();\n        ///     let Ok(n) = this.check-write(); // eliding error handling\n        ///     let len = min(n, contents.len());\n        ///     let (chunk, rest) = contents.split_at(len);\n        ///     this.write(chunk  );            // eliding error handling\n        ///     contents = rest;\n        /// }\n        /// this.flush();\n        /// // Wait for completion of `flush`\n        /// pollable.block();\n        /// // Check for any errors that arose during `flush`\n        /// let _ = this.check-write();         // eliding error handling\n        /// ```\n        blocking-write-and-flush: func(\n            contents: list<u8>\n        ) -> result<_, stream-error>;\n\n        /// Request to flush buffered output. This function never blocks.\n        ///\n        /// This tells the output-stream that the caller intends any buffered\n        /// output to be flushed. the output which is expected to be flushed\n        /// is all that has been passed to `write` prior to this call.\n        ///\n        /// Upon calling this function, the `output-stream` will not accept any\n        /// writes (`check-write` will return `ok(0)`) until the flush has\n        /// completed. The `subscribe` pollable will become ready when the\n        /// flush has completed and the stream can accept more writes.\n        flush: func() -> result<_, stream-error>;\n\n        /// Request to flush buffered output, and block until flush completes\n        /// and stream is ready for writing again.\n        blocking-flush: func() -> result<_, stream-error>;\n\n        /// Create a `pollable` which will resolve once the output-stream\n        /// is ready for more writing, or an error has occured. When this\n        /// pollable is ready, `check-write` will return `ok(n)` with n>0, or an\n        /// error.\n        ///\n        /// If the stream is closed, this pollable is always ready immediately.\n        ///\n        /// The created `pollable` is a child resource of the `output-stream`.\n        /// Implementations may trap if the `output-stream` is dropped before\n        /// all derived `pollable`s created with this function are dropped.\n        subscribe: func() -> pollable;\n\n        /// Write zeroes to a stream.\n        ///\n        /// This should be used precisely like `write` with the exact same\n        /// preconditions (must use check-write first), but instead of\n        /// passing a list of bytes, you simply pass the number of zero-bytes\n        /// that should be written.\n        write-zeroes: func(\n            /// The number of zero-bytes to write\n            len: u64\n        ) -> result<_, stream-error>;\n\n        /// Perform a write of up to 4096 zeroes, and then flush the stream.\n        /// Block until all of these operations are complete, or an error\n        /// occurs.\n        ///\n        /// This is a convenience wrapper around the use of `check-write`,\n        /// `subscribe`, `write-zeroes`, and `flush`, and is implemented with\n        /// the following pseudo-code:\n        ///\n        /// ```text\n        /// let pollable = this.subscribe();\n        /// while num_zeroes != 0 {\n        ///     // Wait for the stream to become writable\n        ///     pollable.block();\n        ///     let Ok(n) = this.check-write(); // eliding error handling\n        ///     let len = min(n, num_zeroes);\n        ///     this.write-zeroes(len);         // eliding error handling\n        ///     num_zeroes -= len;\n        /// }\n        /// this.flush();\n        /// // Wait for completion of `flush`\n        /// pollable.block();\n        /// // Check for any errors that arose during `flush`\n        /// let _ = this.check-write();         // eliding error handling\n        /// ```\n        blocking-write-zeroes-and-flush: func(\n            /// The number of zero-bytes to write\n            len: u64\n        ) -> result<_, stream-error>;\n\n        /// Read from one stream and write to another.\n        ///\n        /// The behavior of splice is equivelant to:\n        /// 1. calling `check-write` on the `output-stream`\n        /// 2. calling `read` on the `input-stream` with the smaller of the\n        /// `check-write` permitted length and the `len` provided to `splice`\n        /// 3. calling `write` on the `output-stream` with that read data.\n        ///\n        /// Any error reported by the call to `check-write`, `read`, or\n        /// `write` ends the splice and reports that error.\n        ///\n        /// This function returns the number of bytes transferred; it may be less\n        /// than `len`.\n        splice: func(\n            /// The stream to read from\n            src: borrow<input-stream>,\n            /// The number of bytes to splice\n            len: u64,\n        ) -> result<u64, stream-error>;\n\n        /// Read from one stream and write to another, with blocking.\n        ///\n        /// This is similar to `splice`, except that it blocks until the\n        /// `output-stream` is ready for writing, and the `input-stream`\n        /// is ready for reading, before performing the `splice`.\n        blocking-splice: func(\n            /// The stream to read from\n            src: borrow<input-stream>,\n            /// The number of bytes to splice\n            len: u64,\n        ) -> result<u64, stream-error>;\n    }\n}\n";
    const _: &[u8] = b"package wasi:io@0.2.0;\n\nworld imports {\n    import streams;\n    import poll;\n}\n";
    const _: &[u8] = b"package wasi:io@0.2.0;\n\n\ninterface error {\n    /// A resource which represents some error information.\n    ///\n    /// The only method provided by this resource is `to-debug-string`,\n    /// which provides some human-readable information about the error.\n    ///\n    /// In the `wasi:io` package, this resource is returned through the\n    /// `wasi:io/streams/stream-error` type.\n    ///\n    /// To provide more specific error information, other interfaces may\n    /// provide functions to further \"downcast\" this error into more specific\n    /// error information. For example, `error`s returned in streams derived\n    /// from filesystem types to be described using the filesystem\'s own\n    /// error-code type, using the function\n    /// `wasi:filesystem/types/filesystem-error-code`, which takes a parameter\n    /// `borrow<error>` and returns\n    /// `option<wasi:filesystem/types/error-code>`.\n    ///\n    /// The set of functions which can \"downcast\" an `error` into a more\n    /// concrete type is open.\n    resource error {\n        /// Returns a string that is suitable to assist humans in debugging\n        /// this error.\n        ///\n        /// WARNING: The returned string should not be consumed mechanically!\n        /// It may change across platforms, hosts, or other implementation\n        /// details. Parsing this string is a major platform-compatibility\n        /// hazard.\n        to-debug-string: func() -> string;\n    }\n}\n";
}
