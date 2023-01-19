use jni::{JNIEnv, objects::JValue, errors::Result};

use crate::android::interop::JavaConvertibleDesc;

use crate::request::TestSign;

impl JavaConvertibleDesc for TestSign {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/devwallet/entity/request/TestSign"
    }

    fn fields() -> Vec<(&'static str, &'static str)> {
        [
            ("transaction", "Ljava/lang/String;"),
            ("signature", "Ljava/lang/String;"),
            ("result", "Ljava/lang/String;"),
        ].into()
    }

    fn into_values<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<Vec<JValue<'a>>> {
        let strings:Vec<String> = [self.transaction, self.signature, self.result].into();

        strings.iter().map(|string| {
            let jstring = env.new_string(string);
            jstring.map(|jstring| {
                JValue::from(jstring)
            })
        }).try_collect()
    }

    fn from_values<'a: 'b, 'b>(env: &'b JNIEnv<'a>, values: &[JValue<'a>]) -> Result<Self> {
        let transaction = values[0].l()?;
        let signature = values[1].l()?;
        let result = values[2].l()?;

        let transaction: String = env.get_string(transaction.into())?.into();
        let signature: String = env.get_string(signature.into())?.into();
        let result: String = env.get_string(result.into())?.into();

        Ok(Self {
            transaction: transaction,
            signature: signature,
            result: result
        })
    }
}