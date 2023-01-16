extern crate android_log;

use std::sync::Arc;

use interop_android::pointer::ArcPointer;
use jni::objects::{JObject, JString, JValue};
use jni::JNIEnv;
use jni_fn::jni_fn;

use interop_android::env::AndroidEnv;

mod interop;

mod test_settings;
mod test_settings_provider;

pub use interop::wrappable::{JavaWrappableDesc, JavaWrappable};
pub use interop::convertible::{JavaConvertibleDesc, JavaConvertible};

use crate::test_settings::TestSettingsProvider;

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

#[jni_fn("one.tesseract.devwallet.Application")]
pub fn createTestSettingsProvider<'a>(env: JNIEnv<'a>, application: JObject<'a>, data_dir: JString<'a>) -> JObject<'a> {
    let provider = Arc::new(TestSettingsProvider {});

    let jref = provider.java_ref(&env).unwrap();

    return jref;
}



// impl<T> Drop for T where T: JavaWrappable {
//     fn drop(&mut self) {
//         todo!()
//     }
// }