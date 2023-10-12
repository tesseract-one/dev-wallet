//===------------ substrate.rs --------------------------------------------===//
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

use std::sync::Arc;

use async_trait::async_trait;

use tesseract_protocol_substrate::{AccountType, GetAccountResponse, Substrate};

use crate::request::{SubstrateAccount, SubstrateSign};
use crate::ui::{UI, UIProtocol};
use crate::settings::{SettingsProvider, KeySettingsProvider};

use super::parse::parse_transaction;
use super::wallet::Wallet;
use super::error::{Error, Result, UnsupportedAccountType};

pub(crate) struct SubstrateService {
    ui: Arc<UI>,
    settings_provider: Arc<SettingsProvider>
}

impl SubstrateService {
    pub fn new(ui: Arc<UI>, settings_provider: Arc<SettingsProvider>) -> Self {
        Self { ui: ui, settings_provider: settings_provider }
    }

    fn wallet(&self) -> Result<Wallet> {
        let settings = self.settings_provider.load_key_settings()?;
        let mnemonic = settings.mnemonic.ok_or(Error::MnemonicNotSet)?;

        let wallet = Wallet::new(&mnemonic)?;

        Ok(wallet)
    }

    fn account_type_string(&self, account_type: AccountType) -> Result<&str> {
        use UnsupportedAccountType::*;

        match account_type {
            AccountType::Sr25519 => Ok("Sr25519"),
            AccountType::Ed25519 => Err(Error::UnsupportedAccountType(Ed25519)),
            AccountType::Ecdsa => Err(Error::UnsupportedAccountType(Ecdsa)),
        }
    }

    async fn get_account_impl(self: Arc<Self>, account_type: AccountType) -> Result<GetAccountResponse> {
        let wallet = self.wallet()?;
        let account_type = self.account_type_string(account_type)?;

        let path = "".to_string();

        let key = wallet.derive(&path)?.to_account_id();
        let strkey = key.to_string();

        let request = SubstrateAccount {
            algorithm: account_type.to_owned(),
            path: path.clone(),
            key: strkey
        };

        let allow = self.ui.request_user_confirmation(request).await?;

        if allow {
            let rawkey: &[u8] = key.as_ref();

            Ok(GetAccountResponse {
                public_key: rawkey.into(),
                path: path
            })
        } else {
            Err(Error::Cancelled)
        }
    }

    async fn sign_transaction_impl(
        self: Arc<Self>,
        account_type: AccountType,
        account_path: &str,
        extrinsic_data: &[u8],
        extrinsic_metadata: &[u8],
        extrinsic_types: &[u8],
    ) -> Result<Vec<u8>> {
        debug!("About to sign a transaction");
        let wallet = self.wallet()?;
        debug!("Got wallet");
        let account_type = self.account_type_string(account_type)?;
        debug!("Got account type: {}", &account_type);

        let data = parse_transaction(extrinsic_data, extrinsic_metadata, extrinsic_types)?;

        debug!("Transaction parsed: {}", &data);

        let key = wallet.derive(account_path)?.to_account_id();

        debug!("Got key");

        let strkey = key.to_string();

        debug!("Got strkey");

        let request = SubstrateSign {
            algorithm: account_type.to_owned(),
            path: account_path.to_owned(),
            key: strkey,
            data: data
        };

        //debug!("Formed a request: {:?}", &request);
        debug!("Formed a request");

        let allow = self.ui.request_user_confirmation(request).await?;

        debug!("User allowed signing: {}", &allow);

        if allow {
            let signature = wallet.sign(extrinsic_data)?;

            debug!("Signed the transaction");

            Ok(signature)
        } else {
            Err(Error::Cancelled)
        }
    }
}

#[async_trait]
impl tesseract_protocol_substrate::SubstrateService for SubstrateService {
    async fn get_account(self: Arc<Self>, account_type: AccountType) -> tesseract::Result<GetAccountResponse> {
        self.get_account_impl(account_type).await.map_err(|e| e.into())
    }

    async fn sign_transaction(
        self: Arc<Self>,
        account_type: AccountType,
        account_path: &str,
        extrinsic_data: &[u8],
        extrinsic_metadata: &[u8],
        extrinsic_types: &[u8],
    ) -> tesseract::Result<Vec<u8>> {
        self.sign_transaction_impl(
            account_type,
            account_path,
            extrinsic_data,
            extrinsic_metadata,
            extrinsic_types).await.map_err(|e| e.into())
    }
}

impl tesseract::service::Service for SubstrateService {
    type Protocol = Substrate;

    fn protocol(&self) -> &Substrate {
        &Substrate::Protocol
    }

    fn to_executor(self) -> Box<dyn tesseract::service::Executor + Send + Sync> {
        Box::new(tesseract_protocol_substrate::service::SubstrateExecutor::from_service(
            self,
        ))
    }
}