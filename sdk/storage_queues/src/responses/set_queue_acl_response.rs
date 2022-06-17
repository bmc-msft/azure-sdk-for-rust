use azure_core::error::Error;
use azure_storage::core::headers::CommonStorageResponseHeaders;
use bytes::Bytes;
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct SetQueueACLResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
}

impl std::convert::TryFrom<&Response<Bytes>> for SetQueueACLResponse {
    type Error = Error;

    fn try_from(response: &Response<Bytes>) -> azure_core::Result<Self> {
        debug!("response == {:?}", response);

        Ok(SetQueueACLResponse {
            common_storage_response_headers: response.headers().try_into()?,
        })
    }
}
