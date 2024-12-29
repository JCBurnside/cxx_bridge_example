pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

enum X {
    V1(i8),
    V2(i32),
}

impl X {
    fn test(&self) -> i32 {
        match self {
            Self::V1(a) => *a as i32,
            Self::V2(a) => *a,
        }
    }
}

#[cxx::bridge(namespace="test")]
mod ffi {
    extern "Rust" {
        type X;
        fn test(self:&X) -> i32;
    }

    unsafe extern "C++" {
        unsafe fn free_test(x:&X);
    }
}


#[cfg(test)]
mod tests {
    use ffi::free_test;

    use super::*;

    #[test]
    fn it_works() {
        unsafe { free_test(&X::V1(1)) };
        
        assert!(false);
    }
}
