#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Ulid2Uuid,
    Uuid2Ulid,
    Error,
}
impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Kind::Ulid2Uuid => "L2U",
            Kind::Uuid2Ulid => "U2L",
            _ => "ERR",
        };
        write!(f, "{repr}")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Outcome {
    kind: Kind,
    value_in: String,
    value_out: String,
}
impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{0}]:: {1:36} :: {2}",
            self.kind, self.value_in, self.value_out
        )
    }
}
impl Outcome {
    pub fn new(value_in: String) -> Self {
        Self {
            kind: Kind::Error,
            value_in,
            value_out: "".to_string(),
        }
    }
    pub fn set_kind(&mut self, kind: Kind) -> Outcome {
        self.kind = kind;
        self.to_owned()
    }
    pub fn set_value_out(&mut self, value_out: String) -> Outcome {
        self.value_out = value_out;
        self.to_owned()
    }
}

#[cfg(test)]
mod test {
    use crate::outcome::{Kind, Outcome};

    #[test]
    fn test_kind() {
        assert_eq!(format!("{}", Kind::Ulid2Uuid), "L2U");
        assert_eq!(format!("{}", Kind::Uuid2Ulid), "U2L");
        assert_eq!(format!("{}", Kind::Error), "ERR");
    }

    #[test]
    fn test_outcome() {
        let mut outcome = Outcome::new("any-how".to_string());
        outcome.set_value_out("any-good".to_string());
        assert_eq!(
            format!("{outcome}"),
            "[ERR]:: any-how                              :: any-good",
        );
    }
}
