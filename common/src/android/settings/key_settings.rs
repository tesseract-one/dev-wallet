use jni::{JNIEnv, objects::JValue, errors::Result};

use crabdroid::JavaConvertibleDesc;

use crate::settings::KeySettings;

impl JavaConvertibleDesc for KeySettings {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/devwallet/entity/KeySettings"
    }

    fn fields() -> Vec<(&'static str, &'static str)> {
        [
            ("mnemonic", "Ljava/lang/String;"),
        ].into()
    }

    fn into_values<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<Vec<JValue<'a>>> {
        let strings:Vec<String> = [self.mnemonic.unwrap_or_default()].into();

        strings.iter().map(|string| {
            let jstring = env.new_string(string);
            jstring.map(|jstring| {
                JValue::from(jstring)
            })
        }).try_collect()
    }

    fn from_values<'a: 'b, 'b>(env: &'b JNIEnv<'a>, values: &[JValue<'a>]) -> Result<Self> {
        let mnemonic = values[0].l()?;

        let mnemonic: String = env.get_string(mnemonic.into())?.into();

        let mnemonic = if mnemonic == "" {
            None
        } else {
            Some(mnemonic)
        };

        Ok(Self {
            mnemonic: mnemonic,
        })
    }
}