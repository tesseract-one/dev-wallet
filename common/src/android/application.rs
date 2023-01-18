extern crate android_log;

use std::sync::Arc;

use jni::objects::{JObject, JString};
use jni::JNIEnv;
use jni_fn::jni_fn;

use crate::android::interop::{JavaWrappable, deresultify};

use crate::Core;

#[jni_fn("one.tesseract.devwallet.Application")]
pub fn createCore<'a>(env: JNIEnv<'a>, _application: JObject<'a>, data_dir: JString<'a>) -> JObject<'a> {
    deresultify(&env, || {
        android_log::init("DevWallet")?;

        let data_dir: String = env.get_string(data_dir)?.into();

        let core = Arc::new(Core::new(&data_dir));

        Ok(core.java_ref::<Core>(&env, None)?)
    })
}

