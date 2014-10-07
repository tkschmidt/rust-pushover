extern crate pushover;

use pushover::priority;

//#[cfg(test)]

#[test]
fn test_lowest_priority_stringify(){
    let enum_val = priority::Lowest;
    assert_eq!("-2".to_string(), enum_val.stringify());
}
#[test]
fn test_low_priority_stringify(){
    let enum_val = priority::Low;
    assert_eq!("-1".to_string(), enum_val.stringify());
}
#[test]
fn test_normal_priority_stringify(){
    let enum_val = priority::Normal;
    assert_eq!("0".to_string(), enum_val.stringify());
}
#[test]
fn test_high_priority_stringify(){
    let enum_val = priority::High;
    assert_eq!("1".to_string(), enum_val.stringify());
}
#[test]
fn test_emergency_priority_stringify(){
    let enum_val = priority::Emergency;
    assert_eq!("2".to_string(), enum_val.stringify());
}
