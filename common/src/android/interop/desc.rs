pub trait JavaDesc {
    fn java_class<'a>(&'a self) -> &'a str;
}