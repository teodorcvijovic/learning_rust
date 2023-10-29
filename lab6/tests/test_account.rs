use lab6::account::new_account;

#[test]
fn test_new_account_api() {
    let user = new_account("pera", "123");
    assert_eq!(user.get_password(), "123");
}