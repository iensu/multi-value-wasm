/// Status code for WASM results.
#[derive(Debug, PartialEq)]
#[repr(C)]
enum Status {
    Ok = 0,
    DivByZero = 1,
}

/// WASM calculation result.
#[derive(Debug, PartialEq)]
#[repr(C)]
pub struct Result<T>(Status, T);

#[no_mangle]
pub extern "C" fn add(left: u32, right: u32) -> Result<u32> {
    Result(Status::Ok, left + right)
}

#[no_mangle]
pub extern "C" fn div(top: u32, bottom: u32) -> Result<u32> {
    if bottom == 0 {
        return Result(Status::DivByZero, 0);
    }

    Result(Status::Ok, top / bottom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(40, 2), Result(Status::Ok, 42))
    }

    #[test]
    fn div_works() {
        assert_eq!(div(40, 2), Result(Status::Ok, 20))
    }

    #[test]
    fn div_by_zero_fails() {
        assert_eq!(div(40, 0), Result(Status::DivByZero, 0))
    }
}
