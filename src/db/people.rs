use db_core::data_interface::ToBytes;

pub struct Person<'a> {
    names: Names,
    emails: Emails<'a>
}

impl<'a> Person<'a> {

}

impl<'a> ToBytes for Person<'a> {
    fn to_data<'b>(&'b self) -> Vec<u8> {
        vec![]
    }
}

pub struct Name {
    words: Vec<&'static str>
}

pub type Names = Vec<Name>;
pub type NameResult<'a> = Result<Name, &'static str>;

impl<'a> Name {
    fn new(full_name: &'static str) -> NameResult<'a> {
        let a = full_name.clone();
        let v: Vec<&'static str> = a.split_whitespace().collect();
        Ok(Name{words: v})
    }
}

#[test]
fn name_new_four_words() {
    let e = Name::new("John Jacob Jingleheimer Schmidt");
    assert!(e.is_ok());
    let eu = e.unwrap();
    assert_eq!(eu.words, vec!["John", "Jacob", "Jingleheimer", "Schmidt"]);
}

#[test]
fn name_new_one_word() {
    let e = Name::new("Bob");
    assert!(e.is_ok());
    let eu = e.unwrap();
    assert_eq!(eu.words, vec!["Bob"]);
}

pub struct Email<'a> {
    local: &'a str,
    domain: &'a str
}

pub type Emails<'a> = Vec<Email<'a>>;

pub type EmailResult<'a> = Result<Email<'a>, &'static str>;

impl<'a> Email<'a> {
    fn new(full_email: &'a str) -> EmailResult<'a> {
        let v: Vec<&str> = full_email.split('@').collect();
        match v.len() {
            2 => Ok(Email{local: v[0], domain: v[1]}),
            _ => Err("Invalid email")
        }
    }

    fn new_from_pieces(local: &'a str, domain: &'a str) ->
            EmailResult<'a> {
        Ok(Email{local: local, domain: domain})
    }

}

#[test]
fn email_new() {
    let e = Email::new("beginning@end.com");
    assert!(e.is_ok());
    let eu = e.unwrap();
    assert_eq!(eu.local, "beginning");
    assert_eq!(eu.domain, "end.com");
}

#[test]
fn email_new_from_pieces() {
    let e = Email::new_from_pieces("beginning", "end.com");
    assert!(e.is_ok());
    let eu = e.unwrap();
    assert_eq!(eu.local, "beginning");
    assert_eq!(eu.domain, "end.com");
}
