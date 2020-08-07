
#[derive(Debug)]
pub struct Response<'a> {
    pub code: &'a str,
    pub body: &'a str
}
