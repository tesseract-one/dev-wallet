use jni::{JNIEnv, objects::JObject};
use jni_fn::jni_fn;

use crabdroid::{JavaWrappable, JavaConvertible, JavaErrorContext};
use crate::settings::{TestSettings, SettingsProvider, TestSettingsProvider};

use crate::Error;

#[jni_fn("one.tesseract.devwallet.rust.TestSettingsProvider")]
pub fn load<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> JObject<'a> {
    Error::java_context(&env, || {
        let provider = SettingsProvider::from_java_ref(this, &env)?;

        let settings = provider.load_test_settings();

        Ok(settings?.into_java(&env)?)
    })
}

#[jni_fn("one.tesseract.devwallet.rust.TestSettingsProvider")]
pub fn save<'a>(env: JNIEnv<'a>, this: JObject<'a>, settings: JObject<'a>) {
    Error::java_context(&env, || {
        let provider = SettingsProvider::from_java_ref(this, &env)?;

        let settings = TestSettings::from_java(&env, settings)?;

        Ok(provider.save_test_settings(settings)?)
    })
}