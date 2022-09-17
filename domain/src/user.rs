use anyhow::ensure;
use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn new(name: UserName) -> Self {
        Self {
            id: UserId::new(),
            name: name,
        }
    }
}

#[derive(Debug)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: String) -> anyhow::Result<Self> {
        Self::validate_length(&name).unwrap();
        Self::validate_characters(&name).unwrap();
        Ok(Self(name))
    }

    fn validate_length(name: &String) -> anyhow::Result<()> {
        const MIN: usize = 2;
        const MAX: usize = 20;

        ensure!(
            MIN <= name.len() && name.len() <= MAX,
            "\"{name}\" is too short or long. use a name between {MIN} and {MAX} characters."
        );
        Ok(())
    }

    fn validate_characters(name: &String) -> anyhow::Result<()> {
        ensure!(
            !name.chars().any(|c| !c.is_ascii_alphanumeric()),
            "\"{name}\" includes invalid characters!"
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[should_panic]
    #[case("")]
    #[should_panic]
    #[case("a")]
    #[case("ab")]
    #[case("abc")]
    #[should_panic]
    #[case("あいう")]
    #[case("abcdefghijklmnopqrst")]
    #[should_panic]
    #[case("abcdefghijklmnopqrstu")]
    #[should_panic]
    #[case("a b")]
    #[should_panic]
    #[case("ab\n")]
    fn new_user_name(#[case] name: &str) {
        let _ = UserName::new(name.to_string()).unwrap();
    }
}
