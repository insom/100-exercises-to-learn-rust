#[derive(Debug, PartialEq, Clone)]
pub struct TicketDescription(String);

#[derive(Debug, thiserror::Error)]
pub enum TicketError {
    #[error("The description cannot be longer than 500 bytes")]
    TooLong,
    #[error("The description cannot be empty")]
    Empty,
}

impl TryFrom<String> for TicketDescription {
    type Error = TicketError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 500 {
            return Err(TicketError::TooLong);
        }
        if value.len() == 0 {
            return Err(TicketError::Empty);
        }
        return Ok(TicketDescription { 0: value })
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = TicketError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        TicketDescription::try_from(String::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let description = TicketDescription::try_from("A description".to_string()).unwrap();
        assert_eq!(description.0, "A description");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketDescription::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The description cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let description = "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.".to_string();
        let err = TicketDescription::try_from(description).unwrap_err();
        assert_eq!(
            err.to_string(),
            "The description cannot be longer than 500 bytes"
        );
    }

    #[test]
    fn test_try_from_str() {
        let description = TicketDescription::try_from("A description").unwrap();
        assert_eq!(description.0, "A description");
    }
}
