extern crate android_log;

use jni::objects::{JObject, JString};
use jni::JNIEnv;
use jni_fn::jni_fn;

fn test() {
    android_log::init("MyApp").unwrap();
}

fn test2() {
    debug!("test debug");
}

#[jni_fn("one.tesseract.devwallet.Application")]
pub fn hello(env: JNIEnv, application: JObject, hi: JString) {
    android_log::init("MyApp").unwrap();
    debug!("INITED");

    let hi: String = env
            .get_string(hi).unwrap()
            .into();

    debug!("Android says: {}", hi);

    test2()
}