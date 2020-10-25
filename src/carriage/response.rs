use crate::carriage::codes::HttpCodes;

#[derive(Debug)]
pub struct Response<'a> {
    pub code: HttpCodes,
    pub body: &'a str
}
