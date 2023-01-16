use jni::{JNIEnv, objects::JObject, errors::Result};
use jni_fn::jni_fn;

use super::{JavaWrappableDesc, JavaWrappable, JavaConvertible};
use crate::test_settings::{TestSettings, TestSettingsProvider};

impl JavaWrappableDesc for TestSettingsProvider {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/devwallet/rust/TestSettingsProvider"
    }
}

#[jni_fn("one.tesseract.devwallet.rust.TestSettingsProvider")]
pub fn load<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> JObject<'a> {
    let provider = TestSettingsProvider::from_java_ref(this, &env).unwrap();

    let settings = provider.load();

    settings.into_java(&env).unwrap()
}

#[jni_fn("one.tesseract.devwallet.rust.TestSettingsProvider")]
pub fn save<'a>(env: JNIEnv<'a>, this: JObject<'a>, settings: JObject<'a>) {
    let provider = TestSettingsProvider::from_java_ref(this, &env).unwrap();

    let settings = TestSettings::from_java(&env, settings).unwrap();

    provider.save(settings);
}