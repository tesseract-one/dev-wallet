use jni::objects::JObject;
use jni::JNIEnv;
use jni_fn::jni_fn;

use super::interop::deresultify;

use crate::Core;

use crate::android::settings::SettingsProviderType;
use crate::android::interop::{JavaDesc, JavaWrappableDesc, JavaWrappable};

impl JavaDesc for Core {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/devwallet/rust/Core"
    }
}

impl JavaWrappableDesc for Core {
}

#[jni_fn("one.tesseract.devwallet.rust.Core")]
pub fn testSettingsProvider<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> JObject<'a> {
    deresultify(&env, || {
        let this = Core::from_java_ref(this, &env)?;
    
        let res = this.settings_provider().java_ref(&env, Some(SettingsProviderType::Test))?;
    
        Ok(res)
    })
}