use jni::{JNIEnv, objects::JObject};
use jni_fn::jni_fn;

use super::{JavaWrappableDesc, JavaWrappable};
use crate::test_settings::TestSettingsProvider;

impl JavaWrappableDesc for TestSettingsProvider {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/devwallet/rust/TestSettingsProvider"
    }
}

#[jni_fn("one.tesseract.devwallet.rust.TestSettingsProvider")]
pub fn load<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> JObject<'a> {
    let provider = TestSettingsProvider::from_java_ref(this, &env).unwrap();

    //let settings = provider.load();

    JObject::null()
}

#[jni_fn("one.tesseract.devwallet.rust.TestSettingsProvider")]
pub fn save<'a>(env: JNIEnv<'a>, this: JObject<'a>, settings: JObject<'a>) {

}