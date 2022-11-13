use anyhow::{ensure, Context};
use derive_getters::Getters;
use error::AppError;
use uuid::Uuid;

#[derive(Clone, Debug, Getters, PartialEq)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
}

impl User {
    pub fn new(name: UserName) -> Self {
        Self {
            id: UserId::new(),
            name: name,
        }
    }

    // to convert from DB row
    pub fn reconstruct(id: String, name: String) -> anyhow::Result<User> {
        let id = UserId::try_from(id).with_context(|| {
            AppError::Internal("failed to reconstruct user: invalid id".to_string())
        })?;

        let name = name.try_into().with_context(|| {
            AppError::Internal("failed to reconstruct user: invalid name".to_string())
        })?;

        Ok(User { id, name })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl TryFrom<String> for UserId {
    type Error = anyhow::Error;

    fn try_from(id: String) -> anyhow::Result<UserId> {
        let id = Uuid::parse_str(id.as_str())
            .with_context(|| AppError::InvalidArgument("invalid user id".to_string()))?;
        Ok(UserId(id))
    }
}

impl Into<String> for UserId {
    fn into(self) -> String {
        self.0.to_string()
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserName(String);

impl UserName {
    pub fn new(name: String) -> anyhow::Result<Self> {
        Self::validate_length(&name)
            .with_context(|| AppError::InvalidArgument("invalid name length".to_string()))?;
        Self::validate_characters(&name).with_context(|| {
            AppError::InvalidArgument("name has invalid character(s).".to_string())
        })?;
        Ok(Self(name))
    }

    fn validate_length(name: &String) -> anyhow::Result<()> {
        const MIN: usize = 2;
        const MAX: usize = 20;

        ensure!(
            MIN <= name.len() && name.len() <= MAX,
            AppError::InvalidArgument(format!(
                "name: {name:?} is too short or long. \
                Use a name between {MIN:?} and {MAX:?} characters."
            ))
        );
        Ok(())
    }

    fn validate_characters(name: &String) -> anyhow::Result<()> {
        ensure!(
            !name.chars().any(|c| !c.is_ascii_alphanumeric()),
            AppError::InvalidArgument(format!(
                "name: {name:?} should consist of ascii alphanumerics."
            ))
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

impl std::fmt::Display for UserName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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
