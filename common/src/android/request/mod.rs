use crabdroid::JavaConvertible;

pub (crate) trait Request: JavaConvertible+Send {
}

mod test_sign;
mod test_error;
mod substrate_account;
mod substrate_sign;