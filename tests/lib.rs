extern crate pushover;

use pushover::priority::Priority;

//#[cfg(test)]

#[test]
fn test_lowest_Priority_stringify(){
    let enum_val = Priority::Lowest;
    assert_eq!("-2".to_string(), enum_val.stringify());
}
#[test]
fn test_low_Priority_stringify(){
    let enum_val = Priority::Low;
    assert_eq!("-1".to_string(), enum_val.stringify());
}
#[test]
fn test_normal_Priority_stringify(){
    let enum_val = Priority::Normal;
    assert_eq!("0".to_string(), enum_val.stringify());
}
#[test]
fn test_high_Priority_stringify(){
    let enum_val = Priority::High;
    assert_eq!("1".to_string(), enum_val.stringify());
}
#[test]
fn test_emergency_Priority_stringify(){
    let enum_val = Priority::Emergency;
    assert_eq!("2".to_string(), enum_val.stringify());
}
