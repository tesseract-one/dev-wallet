use async_trait::async_trait;

use crate::request::Request;
use crate::error::Result;

#[async_trait]
pub (crate) trait UIProtocol {
    async fn request_user_confirmation<R: Request>(&self, request: R) -> Result<bool>;
}

#[cfg(target_os = "android")]
pub (crate) use crate::android::UI;

#[cfg(target_os = "ios")]
pub (crate) use crate::ios::UI;