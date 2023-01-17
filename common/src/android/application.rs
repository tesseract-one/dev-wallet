extern crate android_log;

use std::sync::Arc;

use jni::objects::{JObject, JString};
use jni::JNIEnv;
use jni_fn::jni_fn;

use crate::android::interop::JavaWrappable;

use super::core::Core;

#[jni_fn("one.tesseract.devwallet.Application")]
pub fn createCore<'a>(env: JNIEnv<'a>, application: JObject<'a>, data_dir: JString<'a>) -> JObject<'a> {
    android_log::init("DevWallet").unwrap();

    let data_dir: String = env.get_string(data_dir).unwrap().into();

    let core = Arc::new(Core::new(&data_dir));

    core.java_ref::<Core>(&env, None).unwrap()
}

