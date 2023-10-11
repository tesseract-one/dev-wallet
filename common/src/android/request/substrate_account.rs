use jni::{JNIEnv, objects::JValue, errors::Result};

use crabdroid::JavaConvertibleDesc;

use crate::request::SubstrateAccount;

impl JavaConvertibleDesc for SubstrateAccount {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/devwallet/entity/request/SubstrateAccount"
    }

    fn fields() -> Vec<(&'static str, &'static str)> {
        [
            ("algorithm", "Ljava/lang/String;"),
            ("path", "Ljava/lang/String;"),
            ("key", "Ljava/lang/String;"),
        ].into()
    }

    fn into_values<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<Vec<JValue<'a>>> {
        let strings:Vec<String> = [self.algorithm, self.path, self.key].into();

        strings.iter().map(|string| {
            let jstring = env.new_string(string);
            jstring.map(|jstring| {
                JValue::from(jstring)
            })
        }).try_collect()
    }

    fn from_values<'a: 'b, 'b>(env: &'b JNIEnv<'a>, values: &[JValue<'a>]) -> Result<Self> {
        let algorithm = values[0].l()?;
        let path = values[1].l()?;
        let key = values[2].l()?;

        let algorithm: String = env.get_string(algorithm.into())?.into();
        let path: String = env.get_string(path.into())?.into();
        let key: String = env.get_string(key.into())?.into();

        Ok(Self {
            algorithm: algorithm,
            path: path,
            key: key
        })
    }
}