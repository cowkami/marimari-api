use anyhow::ensure;
use derive_getters::Getters;
use uuid::Uuid;

#[derive(Clone, Debug, Getters)]
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

#[derive(Clone, Debug)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl TryFrom<String> for UserId {
    type Error = anyhow::Error;

    fn try_from(id: String) -> anyhow::Result<UserId> {
        Ok(UserId(Uuid::parse_str(id.as_str()).unwrap()))
    }
}

impl Into<String> for UserId {
    fn into(self) -> String {
        self.0.to_string()
    }
}

#[derive(Clone, Debug)]
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
            "Name: \"{name}\" is too short or long. Use a name between {MIN} and {MAX} characters."
        );
        Ok(())
    }

    fn validate_characters(name: &String) -> anyhow::Result<()> {
        ensure!(
            !name.chars().any(|c| !c.is_ascii_alphanumeric()),
            "Name: \"{name}\" includes invalid characters!"
        );
        Ok(())
    }
}

impl TryFrom<String> for UserName {
    type Error = anyhow::Error;

    fn try_from(name: String) -> anyhow::Result<UserName> {
        UserName::new(name)
    }
}

impl Into<String> for UserName {
    fn into(self) -> String {
        self.0
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
        UserName::new(name.to_string()).unwrap();
    }
}
