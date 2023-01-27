use async_trait::async_trait;

use crate::request::Request;
use crate::ui::UIProtocol;
use crate::error::Result as DWResult;
use super::sui::SUI;

pub struct UI {
    swift: SUI
}

impl UI {
    pub (crate) fn new(sui: SUI) -> Self {
        UI {
            swift: sui
        }
    }
}

#[async_trait]
impl UIProtocol for UI {
    async fn request_user_confirmation<R: Request>(&self, request: R) -> DWResult<bool> {
        Ok(true)
    }
}