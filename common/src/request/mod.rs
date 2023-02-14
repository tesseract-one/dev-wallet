mod test_sign;
mod test_error;
mod substrate_account;

#[cfg(target_os = "android")]
pub (crate) use crate::android::Request;

#[cfg(target_os = "ios")]
pub (crate) use crate::ios::Request;

pub (crate) use test_sign::TestSign;
pub (crate) use test_error::TestError;
pub (crate) use substrate_account::SubstrateAccount;