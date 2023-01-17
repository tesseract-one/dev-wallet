extern crate android_log;

use std::sync::Arc;

use interop_android::pointer::ArcPointer;
use jni::objects::{JObject, JString, JValue};
use jni::JNIEnv;
use jni_fn::jni_fn;

use interop_android::env::AndroidEnv;

mod interop;

mod settings_provider;
mod test_settings;
mod test_settings_provider;

// pub use interop::wrappable::{JavaWrappableDesc, JavaWrappable};
// pub use interop::convertible::{JavaConvertibleDesc, JavaConvertible};
// pub use interop::desc::{JavaDesc};

use crate::settings::SettingsProvider;

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

pub use interop::{JavaWrappableDesc, JavaWrappable};

use self::settings_provider::SettingsProviderType;

#[jni_fn("one.tesseract.devwallet.Application")]
pub fn createTestSettingsProvider<'a>(env: JNIEnv<'a>, application: JObject<'a>, data_dir: JString<'a>) -> JObject<'a> {
    //android_log::init("MyApp").unwrap();

    let data_dir: String = env.get_string(data_dir).unwrap().into();

    debug!("####Data dir: {}", &data_dir);

    let provider = Arc::new(SettingsProvider::new(&(data_dir + "/settings.ini")));

    debug!("####Try to ref");

    let jref = provider.java_ref(&env, Some(SettingsProviderType::test)).unwrap();

    debug!("####It worked");

    return jref;
}



// impl<T> Drop for T where T: JavaWrappable {
//     fn drop(&mut self) {
//         todo!()
//     }
// }