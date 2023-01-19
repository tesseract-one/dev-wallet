mod test_sign;

#[cfg(target_os = "android")]
pub (crate) use crate::android::Request;

#[cfg(target_os = "ios")]
pub (crate) use crate::ios::Request;

pub (crate) use test_sign::TestSign;