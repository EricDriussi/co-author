use super::Author;

#[test]
fn should_omit_email_presenting_author() {
    let alias = "a";
    let name = "alice";
    let email = "alice@wonderland.not";
    let author = Author::new(alias, name, email);

    assert!(author.to_string().contains(alias));
    assert!(author.to_string().contains(name));
    assert!(!author.to_string().contains(email));
}
