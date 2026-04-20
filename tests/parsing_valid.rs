use rstest::rstest;
use vls::{Comparator, EqualComparatorKind, Vls};

#[rstest]
#[case("<=2", Comparator::LessThanOrEqual, "2")]
#[case("<4.2", Comparator::LessThan, "4.2")]
#[case(">=8.1.5", Comparator::GreaterThanOrEqual, "8.1.5")]
#[case(">1.0.0", Comparator::GreaterThan, "1.0.0")]
#[case("=3.2.1", Comparator::Equal(EqualComparatorKind::Explicit), "3.2.1")]
#[case("1.2.3", Comparator::Equal(EqualComparatorKind::Implicit), "1.2.3")]
#[case("!=5.0", Comparator::NotEqual, "5.0")]
#[case("*", Comparator::Any, "")]
fn parse_single_constraint(#[case] input: &str, #[case] expected_cmp: Comparator, #[case] expected_ver: &str) {
    let v: Vls = input.parse().unwrap();
    assert_eq!(v.len(), 1);
    assert_eq!(v.constraints()[0].comparator, expected_cmp);
    if expected_cmp != Comparator::Any {
        assert_eq!(v.constraints()[0].version, expected_ver);
    }
}

#[test]
fn parse_multiple_constraints() {
    let v: Vls = ">10.9a|!=10.9c|!=10.9f|<=10.9k".parse().unwrap();
    assert_eq!(v.len(), 4);

    assert_eq!(v.constraints()[0].comparator, Comparator::GreaterThan);
    assert_eq!(v.constraints()[0].version, "10.9a");

    assert_eq!(v.constraints()[1].comparator, Comparator::NotEqual);
    assert_eq!(v.constraints()[1].version, "10.9c");

    assert_eq!(v.constraints()[2].comparator, Comparator::NotEqual);
    assert_eq!(v.constraints()[2].version, "10.9f");

    assert_eq!(v.constraints()[3].comparator, Comparator::LessThanOrEqual);
    assert_eq!(v.constraints()[3].version, "10.9k");
}

#[test]
fn parse_complex_date_versions() {
    let s = "<2024-4-pabc0019|>2024-10-pefd0010|<2024-12-pjkl2010|>2024-12-pjkl5010|<=2025-1-pghi1001";
    let v: Vls = s.parse().unwrap();
    assert_eq!(v.len(), 5);
    assert_eq!(v.to_string(), s);
}

#[test]
fn parse_single_any() {
    let v: Vls = "*".parse().unwrap();
    assert_eq!(v.len(), 1);
    assert_eq!(v.constraints()[0].comparator, Comparator::Any);
}

#[test]
fn parse_equal_implicit() {
    let v_implicit: Vls = "1.2.3".parse().unwrap();
    let v_explicit: Vls = "=1.2.3".parse().unwrap();
    assert_eq!(v_implicit.len(), 1);
    assert_eq!(v_explicit.len(), 1);
    assert_eq!(v_implicit.constraints()[0].comparator, Comparator::Equal(EqualComparatorKind::Implicit));
    assert_eq!(v_explicit.constraints()[0].comparator, Comparator::Equal(EqualComparatorKind::Explicit));
    assert_eq!(v_implicit.constraints()[0].version, "1.2.3");
    assert_eq!(v_explicit.constraints()[0].version, "1.2.3");
    assert_eq!(v_implicit, v_implicit);
}


