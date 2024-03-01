use serde::Deserialize;

use crate::error::Result;
use crate::transport::Transport;

pub struct Docker {
    transport: Transport,
}

impl Docker {
    pub fn unix<S: Into<String>>(socket: S) -> Result<Self> {
        Ok(Docker {
            transport: Transport::unix(socket)?,
        })
    }

    pub fn request<B>(
        &self,
        req: crate::http::request::Request,
    ) -> Result<crate::http::response::Response<B>>
    where
        for<'de> B: Deserialize<'de>,
    {
        self.transport.request(req)
    }
}
