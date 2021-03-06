use config::Config;
use connection::Connection;
use context_inner::{ContextInner, NewConnectionFuture, NewConnectionHandle};
use error::*;

use std::net::SocketAddr;

use tokio::runtime::TaskExecutor;

use futures::sync::mpsc::UnboundedReceiver;
use futures::{Poll, Stream};

/// The `Picoquic` context. It setups and controls the `UdpSocket`. Every incoming `Connection`
/// can be obtained by polling this context.
pub struct Context {
    recv_con: UnboundedReceiver<Connection>,
    local_addr: SocketAddr,
    new_connection_handle: NewConnectionHandle,
}

impl Context {
    /// Creates a new `Context`.
    ///
    /// name - Will be used as SNI for TLS.
    pub fn new(
        listen_address: &SocketAddr,
        handle: TaskExecutor,
        config: Config,
    ) -> Result<Context, Error> {
        let (inner, recv_con, new_connection_handle) = ContextInner::new(listen_address, config)?;

        let local_addr = inner.local_addr();

        // start the inner future
        handle.spawn(inner);

        Ok(Context {
            recv_con,
            local_addr,
            new_connection_handle,
        })
    }

    /// Returns the local address, this `Context` is bound to.
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    /// Connects to the given address and returns a future that resolves into a `Connection`.
    ///
    /// addr - Address of the server.
    /// server_name - The name of the server that will be used by TLS to verify the certificate.
    pub fn new_connection<T: Into<String>>(
        &mut self,
        addr: SocketAddr,
        server_name: T,
    ) -> NewConnectionFuture {
        self.new_connection_handle.new_connection(addr, server_name)
    }

    /// Returns the handle to create new connections.
    pub fn get_new_connection_handle(&self) -> NewConnectionHandle {
        self.new_connection_handle.clone()
    }
}

impl Stream for Context {
    type Item = Connection;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.recv_con.poll().map_err(|_| ErrorKind::Unknown.into())
    }
}
