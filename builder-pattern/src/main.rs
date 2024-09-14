use chrono::NaiveDate;

#[derive(Debug)]
struct User {
    username: String,
    birthday: NaiveDate,
}

#[derive(Debug)]
struct InvalidUsername;

#[derive(Debug)]
enum IncompleteUserBuild {
    NoUsername,
    NoCreatedOn,
}

struct UserBuilder {
    username: Option<String>,
    birthday: Option<NaiveDate>,
}

impl UserBuilder {
    fn new() -> Self {
        Self {
            username: None,
            birthday: None,
        }
    }

    fn set_username(&mut self, username: String) -> Result<&mut Self, InvalidUsername> {
        let valid = username
            .chars()
            .all(|ch| matches!(ch, 'a'..='z' | '0'..='9'));
        if valid {
            self.username = Some(username);
            Ok(self)
        } else {
            Err(InvalidUsername)
        }
    }

    fn set_birthday(&mut self, date: NaiveDate) -> &mut Self {
        self.birthday = Some(date);
        self
    }

    fn build(&self) -> Result<User, IncompleteUserBuild> {
        if let Some(username) = self.username.clone() {
            if let Some(birthday) = self.birthday.clone() {
                Ok(User { username, birthday })
            } else {
                Err(IncompleteUserBuild::NoCreatedOn)
            }
        } else {
            Err(IncompleteUserBuild::NoUsername)
        }
    }
}

fn main() {
    let user = UserBuilder::new()
        .set_username("hello".to_string())
        .unwrap()
        .set_birthday(NaiveDate::default())
        .build()
        .unwrap();

    println!("{}, {}", user.username, user.birthday);
}
