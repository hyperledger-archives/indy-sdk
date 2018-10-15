pub trait ReplyType {
    fn get_type<'a>() -> &'a str;
}
