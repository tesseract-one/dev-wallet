use jni::{JNIEnv, objects::JValue, errors::Result};

use interop_android::JavaConvertibleDesc;

use crate::request::TestError;

impl JavaConvertibleDesc for TestError {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/devwallet/entity/request/TestError"
    }

    fn fields() -> Vec<(&'static str, &'static str)> {
        [
            ("transaction", "Ljava/lang/String;"),
            ("error", "Ljava/lang/String;"),
        ].into()
    }

    fn into_values<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<Vec<JValue<'a>>> {
        let strings:Vec<String> = [self.transaction, self.error].into();

        strings.iter().map(|string| {
            let jstring = env.new_string(string);
            jstring.map(|jstring| {
                JValue::from(jstring)
            })
        }).try_collect()
    }

    fn from_values<'a: 'b, 'b>(env: &'b JNIEnv<'a>, values: &[JValue<'a>]) -> Result<Self> {
        let transaction = values[0].l()?;
        let error = values[1].l()?;

        let transaction: String = env.get_string(transaction.into())?.into();
        let error: String = env.get_string(error.into())?.into();

        Ok(Self {
            transaction: transaction,
            error: error,
        })
    }
}