use crate::clients::FileSystemClient;
use crate::responses::*;
use azure_core::prelude::*;
use azure_core::{headers::add_optional_header, AppendToUrlQuery};
use http::method::Method;
use http::status::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteFileSystemBuilder<'a> {
    file_system_client: &'a FileSystemClient,
    if_modified_since_condition: Option<IfModifiedSinceCondition>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
}

impl<'a> DeleteFileSystemBuilder<'a> {
    pub(crate) fn new(file_system_client: &'a FileSystemClient) -> Self {
        Self {
            file_system_client,
            if_modified_since_condition: None,
            client_request_id: None,
            timeout: None,
        }
    }

    setters! {
        if_modified_since_condition: IfModifiedSinceCondition => Some(if_modified_since_condition),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
    }

    pub async fn execute(
        &self,
    ) -> Result<DeleteFileSystemResponse, Box<dyn std::error::Error + Sync + Send>> {
        // we clone this so we can add custom
        // query parameters
        let mut url = self.file_system_client.url().clone();

        url.query_pairs_mut().append_pair("resource", "filesystem");
        self.timeout.append_to_url_query(&mut url);

        debug!("url = {}", url);

        let request = self.file_system_client.prepare_request(
            url.as_str(),
            &Method::DELETE,
            &|mut request| {
                request = add_optional_header(&self.if_modified_since_condition, request);
                request = add_optional_header(&self.client_request_id, request);
                request
            },
            None,
        )?;

        debug!("request == {:?}", request);

        let response = self
            .file_system_client
            .http_client()
            .execute_request_check_status(request.0, StatusCode::ACCEPTED)
            .await?;

        Ok((&response).try_into()?)
    }
}
