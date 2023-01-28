mod crequest;
mod test_sign;
mod test_error;

pub use self::crequest::CRequest;

pub (crate) trait IntoCRequest {
    fn into_crequest(self) -> CRequest;
}

pub (crate) trait Request: IntoCRequest+Send {
}