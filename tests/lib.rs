extern crate pushover;

use pushover::priority::Priority;

//#[cfg(test)]

#[test]
fn test_lowest_priority_stringify(){
    let enum_val = Priority::Lowest;
    assert_eq!("-2", enum_val.stringify());
}
#[test]
fn test_low_priority_stringify(){
    let enum_val = Priority::Low;
    assert_eq!("-1", enum_val.stringify());
}
#[test]
fn test_normal_priority_stringify(){
    let enum_val = Priority::Normal;
    assert_eq!("0", enum_val.stringify());
}
#[test]
fn test_high_priority_stringify(){
    let enum_val = Priority::High;
    assert_eq!("1", enum_val.stringify());
}
#[test]
fn test_emergency_priority_stringify(){
    let enum_val = Priority::Emergency;
    assert_eq!("2", enum_val.stringify());
}
