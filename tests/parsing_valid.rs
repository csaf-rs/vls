use rstest::rstest;
use vls::{Comparator, EqualComparatorKind, Vls};

#[test]
fn parse_single_any() {
    let v: Vls = "*".parse().unwrap();
    assert_eq!(v, Vls::Any);
    assert!(v.is_any());
    assert!(v.constraints().is_empty());
    assert_eq!(v.to_string(), "*");
}

#[rstest]
#[case("<=2", Comparator::LessThanOrEqual, "2")]
#[case("<4.2", Comparator::LessThan, "4.2")]
#[case(">=8.1.5", Comparator::GreaterThanOrEqual, "8.1.5")]
#[case(">1.0.0", Comparator::GreaterThan, "1.0.0")]
#[case("=3.2.1", Comparator::Equal(EqualComparatorKind::Explicit), "3.2.1")]
#[case("1.2.3", Comparator::Equal(EqualComparatorKind::Implicit), "1.2.3")]
#[case("!=5.0", Comparator::NotEqual, "5.0")]
fn parse_single_versioned_constraint(
    #[case] input: &str,
    #[case] expected_cmp: Comparator,
    #[case] expected_ver: &str,
) {
    let v: Vls = input
        .parse()
        .expect("Expected valid vls constraints to parse");
    assert!(matches!(v, Vls::Constraints(_)));
    assert_eq!(v.to_string(), input);
    let cs = v.constraints();
    assert_eq!(cs.len(), 1);
    assert_eq!(*cs[0].comparator(), expected_cmp);
    assert_eq!(cs[0].version().as_str(), expected_ver);
}

#[test]
fn parse_multiple_constraints() {
    let input = ">10.9a|!=10.9c|!=10.9f|<=10.9k";

    let v: Vls = input
        .parse()
        .expect("Expected valid vls constraints to parse");
    assert!(matches!(v, Vls::Constraints(_)));

    let cs = v.constraints();
    assert_eq!(cs.len(), 4);

    assert_eq!(*cs[0].comparator(), Comparator::GreaterThan);
    assert_eq!(cs[0].version().as_str(), "10.9a");

    assert_eq!(*cs[1].comparator(), Comparator::NotEqual);
    assert_eq!(cs[1].version().as_str(), "10.9c");

    assert_eq!(*cs[2].comparator(), Comparator::NotEqual);
    assert_eq!(cs[2].version().as_str(), "10.9f");

    assert_eq!(*cs[3].comparator(), Comparator::LessThanOrEqual);
    assert_eq!(cs[3].version().as_str(), "10.9k");

    assert_eq!(v.to_string(), input);
}
