pub struct Response <'a, 'b> {
    pub response : &'a str,
    pub page : &'b str,
}

impl <'a, 'b> Response <'a, 'b> {
    pub fn new(response : &'a str, page : &'b str) -> Response<'a, 'b> {
        Response{response , page}
    }
}