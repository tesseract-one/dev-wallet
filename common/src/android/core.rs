use std::sync::Arc;

use jni::objects::JObject;
use jni::JNIEnv;
use jni_fn::jni_fn;

use crate::android::settings::SettingsProviderType;
use crate::settings::SettingsProvider;

use crate::android::interop::{JavaDesc, JavaWrappableDesc, JavaWrappable};

pub (super) struct Core {
    settings_provider: Arc<SettingsProvider>
}

impl Core {
    pub (super) fn new(data_dir: &str) -> Self {
        let location = format!("{}/settings.ini", data_dir);
        let settings_provider = Arc::new(SettingsProvider::new(&location));

        Self { settings_provider: settings_provider }
    }
}

impl JavaDesc for Core {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/devwallet/rust/Core"
    }
}

impl JavaWrappableDesc for Core {
}

#[jni_fn("one.tesseract.devwallet.rust.Core")]
pub fn testSettingsProvider<'a>(env: JNIEnv<'a>, this: JObject<'a>) -> JObject<'a> {
    debug!("WTF!!!");
    let this = Core::from_java_ref(this, &env).unwrap();
    debug!("WTF!!!2");
    let res = Arc::clone(&this.settings_provider).java_ref(&env, Some(SettingsProviderType::Test)).unwrap();
    debug!("WTF!!!3");
    res
}