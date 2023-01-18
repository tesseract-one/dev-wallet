use std::sync::Arc;

use jni::objects::{JObject, JValue};
use jni::JNIEnv;
use jni::errors::Result;

use jni_fn::jni_fn;

use interop_android::env::AndroidEnv;
use interop_android::pointer::ArcPointer;

use super::desc::JavaDesc;
use super::error::deresultify;

struct WrappableHandle {
    pointer: i64,
    dropper: Option<Box<dyn FnOnce(i64)>>
}

impl WrappableHandle {
    fn from_arc<T: JavaWrappable>(arc: Arc<T>) -> Self {
        let long_p: i64 = ArcPointer::new(arc).into();

        Self { pointer: long_p, dropper: Some(Box::new(|pointer| {
            let arc_pointer = ArcPointer::<T>::of(pointer);
            arc_pointer.destroy()
        }))}
    }

    fn from_java_ref(object: JObject, env: &JNIEnv) -> Result<Box<Self>> {
        let handle_lp = env
            .call_method(object, "getHandle", "()J", &[])?
            .j()?;

        let handle_p = handle_lp as *mut WrappableHandle;
        Ok(unsafe { Box::from_raw(handle_p) })
    }

    fn arc<T: JavaWrappable>(&self) -> Arc<T> {
        ArcPointer::of(self.pointer).arc()
    }
}

impl Drop for WrappableHandle {
    fn drop(&mut self) {
        let dropper = self.dropper.take();

        if let Some(dropper) = dropper {
            dropper(self.pointer)
        }
    }
}

pub trait JavaWrappableDesc: JavaDesc {
}

pub trait JavaWrappable {
    fn java_ref<'a: 'b, 'b, D: JavaDesc>(self: Arc<Self>, env: &'b JNIEnv<'a>, desc: Option<D>) -> Result<JObject<'a>>;
    fn from_java_ref(object: JObject, env: &JNIEnv) -> Result<Arc<Self>>;
}

impl<T> JavaWrappable for T where T: JavaWrappableDesc {
    fn java_ref<'a: 'b, 'b, D: JavaDesc>(self: Arc<Self>, env: &'b JNIEnv<'a>, desc: Option<D>) -> Result<JObject<'a>> {
        let clazz = match &desc {
            Some(desc) => desc.java_class(),
            None => self.java_class(),
        };

        let clazz = env
            .find_class_android(clazz)?;

        let handle = WrappableHandle::from_arc(self);
        let handle_p = Box::into_raw(Box::new(handle)) as *const () as i64;

        let obj = env.new_object(clazz, "(J)V", &[JValue::from(handle_p)])?;

        Ok(obj)
    }

    fn from_java_ref(object: JObject, env: &JNIEnv) -> Result<Arc<Self>> {
        let handle= Box::leak(WrappableHandle::from_java_ref(object, env)?);
        Ok(handle.arc())
    }
}

#[jni_fn("one.tesseract.devwallet.interop.RustObject")]
pub fn drop(env: JNIEnv, this: JObject) {
    deresultify(&env, || {
        let handle = WrappableHandle::from_java_ref(this, &env)?;
        Ok(drop(handle))
    })
}

