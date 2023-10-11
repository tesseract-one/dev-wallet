use std::sync::Arc;

use jni::objects::{JObject, JClass, JString};
use jni::JNIEnv;
use jni_fn::jni_fn;

use crabdroid::error::JavaErrorContext;
use crabdroid::{JavaDesc, JavaWrappableDesc, JavaWrappable};

use tesseract_android::service::transport::IPCTransport;

use crate::Core;
use crate::Error;
use super::UI;

use crate::android::settings::SettingsProviderType;

impl JavaDesc for Core {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/devwallet/rust/Core"
    }
}

impl JavaWrappableDesc for Core {
}

#[jni_fn("one.tesseract.devwallet.rust.Core")]
pub fn testSettingsProvider<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> JObject<'a> {
    Error::java_context(&env, || {
        let this = Core::from_java_ref(this, &env)?;
    
        let res = this.settings_provider().java_ref(&env, Some(SettingsProviderType::Test))?;
    
        Ok(res)
    })
}

#[jni_fn("one.tesseract.devwallet.rust.Core")]
pub fn keySettingsProvider<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> JObject<'a> {
    Error::java_context(&env, || {
        let this = Core::from_java_ref(this, &env)?;
    
        let res = this.settings_provider().java_ref(&env, Some(SettingsProviderType::Key))?;
    
        Ok(res)
    })
}

#[jni_fn("one.tesseract.devwallet.rust.Core")]
pub fn create<'a>(env: JNIEnv<'a>, _core_class: JClass<'a>, ui: JObject<'a>, data_dir: JString<'a>) -> JObject<'a> {
    Error::java_context(&env, || {
        android_log::init("DevWallet")?;

        let ui = UI::from_java(&env, ui)?;
        let data_dir: String = env.get_string(data_dir)?.into();
        let ipc = IPCTransport::default(&env)?;

        let core = Arc::new(Core::new(ui, &data_dir, |tesseract| {
            tesseract.transport(ipc)
        }));

        Ok(core.java_ref::<Core>(&env, None)?)
    })
}