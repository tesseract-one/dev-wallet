//===------------ jui.rs --------------------------------------------===//
//  Copyright 2023, Tesseract Systems, Inc.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//===----------------------------------------------------------------------===//

use jni::{objects::{JObject, JValue}, JNIEnv};
use jni::errors::Result;

use interop_android::future::completion_stage::JCompletionStage;

/// Lifetime'd representation of a `UI`. Just a `JObject` wrapped in a
/// new class.
#[derive(Clone, Copy)]
pub struct JUI<'a: 'b, 'b> {
    internal: JObject<'a>,
    env: &'b JNIEnv<'a>,
}

impl<'a: 'b, 'b> ::std::ops::Deref for JUI<'a, 'b> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a: 'b, 'b> From<JUI<'a, 'b>> for JObject<'a> {
    fn from(other: JUI<'a, 'b>) -> JObject<'a> {
        other.internal
    }
}

impl<'a: 'b, 'b> JUI<'a, 'b> {
    pub fn from_env(env: &'b JNIEnv<'a>, obj: JObject<'a>) -> JUI<'a, 'b> {
        JUI {
            internal: obj,
            env: env,
        }
    }

    pub (crate) fn request_user_confirmation(&self, request: JObject<'a>) -> Result<JCompletionStage> {

        let stage = self.env
            .call_method(
                self.internal,
                "requestUserConfirmation",
                "(Landroid/os/Parcelable;)Ljava/util/concurrent/CompletionStage;",
                &[JValue::from(request)],
            )?
            .l()?;

        Ok(JCompletionStage::from_env(&self.env, stage))
    }
}