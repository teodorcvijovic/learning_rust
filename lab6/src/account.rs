use std::str::FromStr;

pub struct Account {
    pub username: String,
    password: String,
}

impl Account {
    /// Create a new account with a given username and password.
    fn new(username: &str, password: &str) -> Self {
        Account { username: username.to_owned(), password: password.to_owned() }
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }
}

impl FromStr for Account {
    type Err = std::io::Error;

    /// Create a new account by parsing its data from a string in the form "USERNAME:PASSWORD".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split(":");

        let username = splitted.next().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing username")
        })?;
        let password = splitted.next().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing password")
        })?;
        
        Ok(Account::new(username, password))
    }
}

pub fn new_account(username: &str, password: &str) -> Account {
    Account::new(username, password)
}

pub fn parse_account(s: &str) -> Result<Account, std::io::Error> {
    s.parse()
}

mod tests {
    use crate::account::new_account;

    #[test]
    fn test_new_account() {
        let user = new_account("pera", "123");
        assert_eq!(user.password, "123");
    }
}