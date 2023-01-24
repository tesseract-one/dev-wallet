use async_trait::async_trait;

use crate::request::Request;
use crate::ui::UIProtocol;
use crate::error::Result as DWResult;

pub struct UI {
}

#[async_trait]
impl UIProtocol for UI {
    async fn request_user_confirmation<R: Request>(&self, request: R) -> DWResult<bool> {
        todo!()
    }
}