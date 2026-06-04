#[cfg(test)]
mod operator_test {
    use super::super::Operator;
    use serde_json::Value;

    #[test]
    fn test_eq() {
        assert!(Operator::Eq.evaluate(&1, &1).unwrap());
        assert!(!Operator::Eq.evaluate(&1, &2).unwrap());
        assert!(Operator::Eq.evaluate(&Value::Null, &Value::Null).unwrap());
    }

    #[test]
    fn test_neq() {
        assert!(!Operator::Neq.evaluate(&1, &1).unwrap());
        assert!(Operator::Neq.evaluate(&1, &2).unwrap());
        assert!(!Operator::Neq.evaluate(&Value::Null, &Value::Null).unwrap());
    }

    #[test]
    fn test_gte() {
        assert!(!Operator::Gte.evaluate(&1, &2).unwrap());
        assert!(Operator::Gte.evaluate(&2, &2).unwrap());
        assert!(Operator::Gte.evaluate(&3, &2).unwrap());
        assert!(Operator::Gte.evaluate(&"string".to_string(), &2).is_err());
    }

    #[test]
    fn test_lte() {
        assert!(Operator::Lte.evaluate(&1, &2).unwrap());
        assert!(Operator::Lte.evaluate(&2, &2).unwrap());
        assert!(!Operator::Lte.evaluate(&3, &2).unwrap());
        assert!(Operator::Lte.evaluate(&"string".to_string(), &2).is_err());
    }

    #[test]
    fn test_gt() {
        assert!(!Operator::Gt.evaluate(&1, &2).unwrap());
        assert!(!Operator::Gt.evaluate(&2, &2).unwrap());
        assert!(Operator::Gt.evaluate(&3, &2).unwrap());
        assert!(Operator::Gt.evaluate(&"string".to_string(), &2).is_err());
    }

    #[test]
    fn test_lt() {
        assert!(Operator::Lt.evaluate(&1, &2).unwrap());
        assert!(!Operator::Lt.evaluate(&2, &2).unwrap());
        assert!(!Operator::Lt.evaluate(&3, &2).unwrap());
        assert!(Operator::Lt.evaluate(&"foo".to_string(), &2).is_err());
    }

    #[test]
    fn test_contains() {
        assert!(Operator::Contains.evaluate(&[1], &1).unwrap());
        assert!(Operator::Contains.evaluate(&[[1]], &[1]).unwrap());
        assert!(Operator::Contains.evaluate(&["foo"], &"foo".to_string()).unwrap());
        assert!(!Operator::Contains.evaluate(&[1], &2).unwrap());
        assert!(!Operator::Contains.evaluate(&["foo"], &"bar".to_string()).unwrap());
    }

    #[test]
    fn test_not_in() {
        assert!(Operator::NotIn.evaluate(&1, &[2]).unwrap());
        assert!(Operator::NotIn.evaluate(&[1], &[[2]]).unwrap());
        assert!(!Operator::NotIn.evaluate(&[1], &[[1]]).unwrap());
        assert!(!Operator::NotIn.evaluate(&"foo".to_string(), &["foo"]).unwrap());
        assert!(Operator::NotIn.evaluate(&"foo".to_string(), &["bar"]).unwrap());
        assert!(Operator::NotIn.evaluate(&"foo".to_string(), &"foo".to_string()).is_err());
    }

    #[test]
    fn test_any_in() {
        assert!(Operator::AnyIn.evaluate(&[1], &[1, 2]).unwrap());
        assert!(Operator::AnyIn.evaluate(&[[1]], &[[1]]).unwrap());
        assert!(!Operator::AnyIn.evaluate(&[1], &[2]).unwrap());
        assert!(Operator::AnyIn.evaluate(&["foo"], &["foo"]).unwrap());
        assert!(Operator::AnyIn.evaluate(&["foo"], &["foo", "bar"]).unwrap());
        assert!(!Operator::AnyIn.evaluate(&["foo"], &["bar"]).unwrap());
        assert!(Operator::AnyIn.evaluate(&["foo"], &"foo".to_string()).is_err());
    }

    #[test]
    fn test_all_in() {
        assert!(Operator::AllIn.evaluate(&[1, 2, 3], &[1, 2, 3]).unwrap());
        assert!(Operator::AllIn.evaluate(&[[1], [2], [3]], &[[1], [2], [3]]).unwrap());
        assert!(Operator::AllIn.evaluate(&["a", "b", "c"], &["a", "b", "c"]).unwrap());
        assert!(Operator::AllIn.evaluate(&["a", "b"], &["a", "b", "c"]).unwrap());
        assert!(!Operator::AllIn.evaluate(&["a", "b", "c"], &["b", "c"]).unwrap());
        assert!(!Operator::AllIn.evaluate(&["a", "b"], &["b", "c"]).unwrap());
        assert!(Operator::AllIn.evaluate(&["foo"], &"foo".to_string()).is_err());
    }

    #[test]
    fn test_none_in() {
        assert!(Operator::NoneIn.evaluate(&[1], &[2]).unwrap());
        assert!(Operator::NoneIn.evaluate(&[1, 2, 3], &[4, 5, 6]).unwrap());
        assert!(!Operator::NoneIn.evaluate(&[1], &[1]).unwrap());
        assert!(!Operator::NoneIn.evaluate(&[1, 2, 3], &[1]).unwrap());
        assert!(Operator::NoneIn.evaluate(&["foo"], &["bar"]).unwrap());
        assert!(!Operator::NoneIn.evaluate(&["foo"], &["foo"]).unwrap());
        assert!(Operator::NoneIn.evaluate(&["foo"], &"foo".to_string()).is_err());
    }
}