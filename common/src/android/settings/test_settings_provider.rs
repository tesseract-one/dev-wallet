use jni::{JNIEnv, objects::JObject, errors::Result};
use jni_fn::jni_fn;

use crate::android::interop::{JavaWrappable, JavaConvertible};
use crate::settings::{TestSettings, SettingsProvider, TestSettingsProvider};

#[jni_fn("one.tesseract.devwallet.rust.TestSettingsProvider")]
pub fn load<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> JObject<'a> {
    let provider = SettingsProvider::from_java_ref(this, &env).unwrap();

    let settings = provider.load_test_settings();

    settings.into_java(&env).unwrap()
}

#[jni_fn("one.tesseract.devwallet.rust.TestSettingsProvider")]
pub fn save<'a>(env: JNIEnv<'a>, this: JObject<'a>, settings: JObject<'a>) {
    let provider = SettingsProvider::from_java_ref(this, &env).unwrap();

    let settings = TestSettings::from_java(&env, settings).unwrap();

    provider.save_test_settings(settings);
}