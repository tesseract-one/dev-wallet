use async_trait::async_trait;

use jni::{JNIEnv, objects::JObject, errors::Result as JResult};

use crabdroid::ContextedGlobal;

use crate::request::Request;
use crate::ui::UIProtocol;
use crate::error::Result as DWResult;

use super::jui::JUI;

pub (crate) struct UI {
    internal: ContextedGlobal
}

impl UI {
    pub (in crate::android) fn from_java<'a: 'b, 'b>(env: &'b JNIEnv<'a>, ui: JObject<'a>) -> JResult<Self> {
        ContextedGlobal::from_local(env, ui).map(|ui| {
            UI {internal: ui}
        })
    }
}

#[async_trait]
impl UIProtocol for UI {
    async fn request_user_confirmation<R: Request>(&self, request: R) -> DWResult<bool> {
        let allow = self.internal.with_async_context(64, |env, ui| {
            let request = request.into_java(&env)?;
            let jui = JUI::from_env(&env, ui);
            jui.request_user_confirmation(request)
        }).await?;

        Ok(allow.with_safe_context_rret(64, |env, jallow| {
            env.call_method(jallow, "booleanValue", "()Z", &[])?.z()
        })?)
    }
}