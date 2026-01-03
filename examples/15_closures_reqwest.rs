use std::str::FromStr;

use http::Method;
use reqwest::{Request, Url};

// Create a request handler that stores multiple validators
struct RequestValidator<'a> {
    validators: Vec<Box<dyn Fn(&Request) -> bool + 'a>>,
}

impl<'a> RequestValidator<'a> {
    fn new() -> Self {
        RequestValidator { validators: vec![] }
    }

    fn add_validator<F>(&mut self, validator: F)
    where
        F: Fn(&Request) -> bool + 'a,
    {
        self.validators.push(Box::new(validator));
    }

    fn validate(&self, req: &Request) -> bool {
        self.validators.iter().all(|v| v(req))
    }
}

fn main() {
    let test_url = "https://fake.site.com/api/v2/user/list";
    let parsed_url = Url::from_str(test_url).unwrap();
    let req = Request::new(Method::GET, parsed_url);
    println!("REQ 1: {req:?}");
    let mut v = RequestValidator::new();
    v.add_validator(|req| req.url().has_host());
    v.add_validator(|req| req.headers().is_empty());
    let check = v.validate(&req);
    println!("Validation result: {check}");
}
