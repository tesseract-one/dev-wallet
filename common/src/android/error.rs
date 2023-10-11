use jni::{JNIEnv, objects::JObject};

use crabdroid::error::{ExceptionConvertible, CompositeErrorInclude, CompositeError};

use tesseract_android::error::TesseractAndroidError;

use crate::Error;

pub auto trait ErrorInclude {
}

impl !ErrorInclude for TesseractAndroidError {
}

impl<T> !ErrorInclude for std::sync::PoisonError<T> {
}

impl<E> From<E> for Error
where
    E: Into<TesseractAndroidError> + ErrorInclude,
 {
    fn from(value: E) -> Self {
        Self::Android(value.into())
    }
}

impl ExceptionConvertible for Error {
    fn to_exception<'a: 'b, 'b>(&self, env: &'b JNIEnv<'a>) -> jni::errors::Result<JObject<'a>> {
        match self {
            Error::Android(e) => e.to_exception(env),
            Error::IO(e) => {
                let description = format!("IOError in Rust: {}", e);
                let description = env.new_string(description)?;

                env.new_object(
                    "java/lang/Exception",
                    "(Ljava/lang/String;)V",
                    &[description.into()])
            },
            e => {
                let description = format!("Some error from Rust: {}", e);
                let description = env.new_string(description)?;

                env.new_object(
                    "java/lang/Exception",
                    "(Ljava/lang/String;)V",
                    &[description.into()])
            },
        }
    }
}

impl !CompositeErrorInclude for Error {
}

impl From<Error> for CompositeError<Error> {
    fn from(value: Error) -> Self {
        Self::Other(value)
    }
}