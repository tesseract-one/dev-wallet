use jni::{JNIEnv, objects::JObject};
use jni_fn::jni_fn;

use interop_android::{JavaWrappable, JavaConvertible, deresultify};

use crate::settings::{KeySettings, SettingsProvider, KeySettingsProvider};

#[jni_fn("one.tesseract.devwallet.rust.KeySettingsProvider")]
pub fn load<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> JObject<'a> {
    deresultify(&env, || {
        let provider = SettingsProvider::from_java_ref(this, &env)?;

        let settings = provider.load_key_settings();

        Ok(settings?.into_java(&env)?)
    })
}

#[jni_fn("one.tesseract.devwallet.rust.KeySettingsProvider")]
pub fn save<'a>(env: JNIEnv<'a>, this: JObject<'a>, settings: JObject<'a>) {
    deresultify(&env, || {
        let provider = SettingsProvider::from_java_ref(this, &env)?;

        let settings = KeySettings::from_java(&env, settings)?;

        Ok(provider.save_key_settings(settings)?)
    })
}