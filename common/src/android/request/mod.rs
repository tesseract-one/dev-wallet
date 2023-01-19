use super::interop::JavaConvertible;

pub (crate) trait Request: JavaConvertible+Send {
}

mod test_sign;