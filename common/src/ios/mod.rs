//pub mod request;

use async_trait::async_trait;

//use crate::request::Request;
use crate::ui::UIProtocol;
use crate::error::Result as DWResult;

pub (crate) trait Request: Send {
}

pub struct UI {
}

#[async_trait]
impl UIProtocol for UI {
    async fn request_user_confirmation<R: Request>(&self, request: R) -> DWResult<bool> {
        todo!()
    }
}

pub use tesseract_utils::*;
pub use tesseract_service::*;