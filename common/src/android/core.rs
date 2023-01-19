use std::sync::Arc;

use jni::objects::{JObject, JClass, JString};
use jni::JNIEnv;
use jni_fn::jni_fn;

use tesseract_ipc_android::service::Transport;

use super::interop::deresultify;

use crate::Core;
use super::UI;

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

#[jni_fn("one.tesseract.devwallet.rust.Core")]
pub fn create<'a>(env: JNIEnv<'a>, _core_class: JClass<'a>, ui: JObject<'a>, data_dir: JString<'a>) -> JObject<'a> {
    deresultify(&env, || {
        android_log::init("DevWallet")?;

        let ui = UI::from_java(&env, ui)?;
        let data_dir: String = env.get_string(data_dir)?.into();
        let ipc = Transport::default(&env)?;

        let core = Arc::new(Core::new(ui, &data_dir, || ipc));

        Ok(core.java_ref::<Core>(&env, None)?)
    })
}