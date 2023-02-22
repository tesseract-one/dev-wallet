use jni::{JNIEnv, objects::JValue, errors::Result};

use interop_android::JavaConvertibleDesc;

use crate::settings::TestSettings;

impl JavaConvertibleDesc for TestSettings {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/devwallet/entity/TestSettings"
    }

    fn fields() -> Vec<(&'static str, &'static str)> {
        [
            ("signature", "Ljava/lang/String;"),
            ("invalidator", "Ljava/lang/String;")
        ].into()
    }

    fn into_values<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<Vec<JValue<'a>>> {
        let strings:Vec<String> = [self.signature, self.invalidator].into();

        strings.iter().map(|string| {
            let jstring = env.new_string(string);
            jstring.map(|jstring| {
                JValue::from(jstring)
            })
        }).try_collect()
    }

    fn from_values<'a: 'b, 'b>(env: &'b JNIEnv<'a>, values: &[JValue<'a>]) -> Result<Self> {
        let signature = values[0].l()?;
        let invalidator = values[1].l()?;

        let signature: String = env.get_string(signature.into())?.into();
        let invalidator: String = env.get_string(invalidator.into())?.into();

        Ok(Self {
            signature: signature,
            invalidator: invalidator,
        })
    }
}