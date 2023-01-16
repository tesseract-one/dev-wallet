use interop_android::env::AndroidEnv;
use jni::{JNIEnv, objects::{JObject, JValue}, errors::Result};

pub trait JavaConvertibleDesc: Sized {
    fn java_class<'a>(&'a self) -> &'a str;

    fn fields() -> Vec<(&'static str, &'static str)>; //(name, type)

    fn into_values<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<Vec<JValue<'a>>>;
    fn from_values<'a: 'b, 'b>(env: &'b JNIEnv<'a>, values: &[JValue<'a>]) -> Result<Self>;
}

pub trait JavaConvertible: Sized {
    fn into_java<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<JObject<'a>>;
    fn from_java<'a: 'b, 'b>(env: &'b JNIEnv<'a>, object: JObject<'a>) -> Result<Self>;
}

impl<T> JavaConvertible for T where T: JavaConvertibleDesc {
    fn into_java<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<JObject<'a>> {
        let clazz = env.find_class_android(self.java_class())?;

        let types: Vec<&str> = Self::fields().into_iter().map(|field| field.1).collect();
        let sig = format!("({})V", types.join(""));

        let value = self.into_values(env)?;

        env.new_object(clazz, sig, &value)
    }

    fn from_java<'a: 'b, 'b>(env: &'b JNIEnv<'a>, object: JObject<'a>) -> Result<Self> {
        fn uppercase_first_letter(s: &str) -> String {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        }

        fn getter_prefix(ret: &str) -> &'static str {
            let mut c = ret.chars();
            match c.next() {
                None => panic!("Put a sig"),
                Some(f) => {
                    match f {
                        'Z' => "is",
                        _ => "get"
                    }
                }
            }
        }

        let fields = Self::fields();

        let values : Vec<JValue> = fields.into_iter().map(|(field_name, ret)| {
            let getter = format!("{}{}", getter_prefix(ret), uppercase_first_letter(field_name));
            let sig = format!("(){}", ret);

            env.call_method(object, &getter, sig, &[])
        }).try_collect()?;

        Self::from_values(env, &values)
    }
}


/*use crate::test_settings::TestSettings;

impl JavaConvertibleDesc for TestSettings {
    fn java_class<'a>(&'a self) -> &'a str {
        "one/tesseract/devwallet/entity/TestSettings"
    }

    fn fields() -> Vec<(&'static str, &'static str)> {
        [
            ("signature", "Ljava/lang/String;"),
            ("invalidator", "Ljava/lang/String;")
        ].into()
    }

    fn into_values<'a: 'b, 'b>(self, env: &'b JNIEnv<'a>) -> Result<Vec<JValue<'a>>> {
        let strings:Vec<String> = [self.signature, self.invalidator].into();

        let vec2 = strings.iter().map(|string| {
            //convert to jvalue
            let jstring = env.new_string(string);
            jstring.map(|jstring| {
                JValue::from(jstring)
            })
        }).try_collect();
        /*.fold(Result::Ok(Vec::<JValue>::new()), |current, element| {
            //folt to a result propagating the first error if happened
            match current {
                Ok(mut vec) => {
                    match element {
                        Ok(value) => {
                            vec.push(value);
                            Ok(vec)
                        },
                        Err(error) => Err(error),
                    }
                },
                Err(_) => current,
            }
        });*/

        vec2
    }

    fn from_values<'a: 'b, 'b>(env: &'b JNIEnv<'a>, values: &[JValue<'a>]) -> Result<Self> {
        let signature = values[0].l()?;
        let invalidator = values[1].l()?;

        let signature: String = env.get_string(signature.into())?.into();
        let invalidator: String = env.get_string(invalidator.into())?.into();

        Ok(Self {
            signature: signature,
            invalidator: invalidator,
        })
    }
}*/
