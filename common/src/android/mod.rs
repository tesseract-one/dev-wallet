extern crate android_log;

use std::sync::Arc;

use interop_android::pointer::ArcPointer;
use jni::objects::{JObject, JString, JValue};
use jni::JNIEnv;
use jni_fn::jni_fn;

use interop_android::env::AndroidEnv;

mod wrappable;
mod test_settings;

pub use wrappable::{JavaWrappableDesc, JavaWrappable};

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

    //test2()

    //env.
}



// impl<T> Drop for T where T: JavaWrappable {
//     fn drop(&mut self) {
//         todo!()
//     }
// }