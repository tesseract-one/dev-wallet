use std::{default::Default, error::Error};

use jni::JNIEnv;

pub fn deresultify<T, I, F>(env: &JNIEnv, fun: F) -> T
where
    T: Default,
    I: Into<T>,
    F: FnOnce() -> Result<I, Box<dyn Error>>,
{
    match fun() {
        Err(err) => {
            //temporary solution. need a proper conversion to Exception with a class
            let message: &str = &err.to_string();

            match env.throw(message) {
                Ok(_) => T::default(),
                Err(e) => {
                    debug!("Error '{}' occured, but couldn't be thrown as Exception because JNI returned: {}", message, e.to_string());
                    panic!("Error '{}' occured, but couldn't be thrown as Exception because JNI returned: {}", message, e.to_string())
                },
            }
        }
        Ok(value) => value.into()
    }
}

// pub fn deresultify2<T, I, E, F>(env: &JNIEnv, fun: F) -> T
// where
//     T: Default,
//     I: Into<T>,
//     E: Into<Box<dyn Error>>,
//     F: FnOnce() -> Result<I, E>,
// {
//     match fun() {
//         Err(err) => {
//             //temporary solution. need a proper conversion to Exception with a class
//             let err: Box<dyn Error> = err.into();
//             let message: &str = &err.to_string();

//             match env.throw(message) {
//                 Ok(_) => T::default(),
//                 Err(e) => {
//                     debug!("Error '{}' occured, but couldn't be thrown as Exception because JNI returned: {}", message, e.to_string());
//                     panic!("Error '{}' occured, but couldn't be thrown as Exception because JNI returned: {}", message, e.to_string())
//                 },
//             }
//         }
//         Ok(value) => value.into()
//     }
// }