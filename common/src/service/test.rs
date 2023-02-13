//===------------ test.rs --------------------------------------------===//
//  Copyright 2022, Tesseract Systems, Inc.
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

use std::sync::Arc;

use async_trait::async_trait;

use tesseract::{Error, ErrorKind};
use tesseract_protocol_test::Test;

use crate::request::{TestSign, TestError};
use crate::ui::{UI, UIProtocol};
use crate::settings::{SettingsProvider, TestSettingsProvider};

pub(crate) struct TestService {
    ui: Arc<UI>,
    settings_provider: Arc<SettingsProvider>
}

impl TestService {
    pub fn new(ui: Arc<UI>, settings_provider: Arc<SettingsProvider>) -> Self {
        Self { ui: ui, settings_provider: settings_provider }
    }
}

impl tesseract::service::Service for TestService {
    type Protocol = Test;

    fn protocol(&self) -> &Test {
        &Test::Protocol
    }

    fn to_executor(self) -> Box<dyn tesseract::service::Executor + Send + Sync> {
        Box::new(tesseract_protocol_test::service::TestExecutor::from_service(
            self,
        ))
    }
}

#[async_trait]
impl tesseract_protocol_test::TestService for TestService {
    async fn sign_transaction(self: Arc<Self>, req: &str) -> tesseract::Result<String> {
        //TODO: change this to showing an error in the wallet itself that it can't load settings
        let settings = self.settings_provider.load_test_settings().map_err(|e| e.into())?;

        if req == settings.invalidator {
            let error = format!("Intentional error. Because your transaction `{}` is set as the invalidator in DevWallet settings", req);

            let request = TestError {
                transaction: req.to_owned(),
                error: error.clone()
            };

            let allow = self.ui.request_user_confirmation(request).await.map_err(|e| e.into())?;

            if allow {
                Err(Error::described(
                    ErrorKind::Weird,
                    &error,
                ))
            } else {
                Err(tesseract::Error::kinded(tesseract::ErrorKind::Cancelled))
            }
        } else {
            let signature = settings.signature.to_owned();
            let signed = format!("{}_{}", req, signature);

            let request = TestSign {
                transaction: req.to_owned(),
                signature: signature,
                result: signed.clone()
            };

            let allow = self.ui.request_user_confirmation(request).await.map_err(|e| e.into())?;

            if allow {
                Ok(signed)
            } else {
                Err(tesseract::Error::kinded(tesseract::ErrorKind::Cancelled))
            }
        }
    }
}