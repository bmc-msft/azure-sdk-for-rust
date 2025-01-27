use super::super::IncompleteVector;
use crate::container::Container;
use azure_core::RequestId;

#[derive(Debug, Clone)]
pub struct ListContainersResponse {
    pub incomplete_vector: IncompleteVector<Container>,
    pub request_id: RequestId,
}

impl ListContainersResponse {
    pub fn is_complete(&self) -> bool {
        self.incomplete_vector.is_complete()
    }
}
