use crate::outcome::Kind;
use crate::outcome::Outcome;

fn convert(values: Vec<String>) -> Vec<Outcome> {
    let mut collected: Vec<Outcome> = Vec::new();
    for value in values.iter() {
        let mut outcome = Outcome::new(value.to_string());
        if let Ok(parsed) = ulid::Ulid::from_string(value) {
            let extracted: uuid::Uuid = parsed.into();
            outcome.set_value_out(extracted.into());
            outcome.set_kind(Kind::Ulid2Uuid);
        } else {
            outcome.set_value_out("can't parse the value".to_string());
        }
        collected.push(outcome)
    }
    collected
}

pub fn process(values: Vec<String>) -> usize {
    let outcomes = convert(values);
    for outcome in outcomes.iter() {
        println!("{outcome}");
    }
    outcomes.len()
}

#[cfg(test)]
mod test {
    use crate::outcome::Kind;
    use crate::outcome::Outcome;
    use crate::process::convert;

    #[test]
    fn test_convert_ok() {
        let some_args = vec!["01K59FJBYATAY80B0ZKHEG28B7".to_string()];
        let outcome = convert(some_args).first().unwrap().to_owned();
        let expected = Outcome::new("01K59FJBYATAY80B0ZKHEG28B7".to_string())
            .set_value_out("019952f9-2fca-d2bc-802c-1f9c5d012167".to_string())
            .set_kind(Kind::Ulid2Uuid);
        assert_eq!(outcome, expected);
    }

    #[test]
    fn test_convert_ko() {
        let some_args = vec!["plop_will_fail".to_string()];
        let outcome = convert(some_args).first().unwrap().to_owned();
        let expected = Outcome::new("plop_will_fail".to_string())
            .set_value_out("can't parse the value".to_string());
        assert_eq!(outcome, expected);
    }
}

#[cfg(test)]
mod test_dummy {
    use crate::process::process;

    #[test]
    fn test_process() {
        let some_args = vec!["dummy", "testing", "is_acceptable"];
        let count = process(some_args.iter().map(|x| x.to_string()).collect());
        assert_eq!(count, 3);
    }
}
