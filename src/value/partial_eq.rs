use super::Value;
use alloc::string::String;

fn eq_u64(value: &Value, other: u64) -> bool {
    value.as_u64().map_or(false, |i| i == other)
}

fn eq_bool(value: &Value, other: bool) -> bool {
    value.as_bool().map_or(false, |i| i == other)
}

fn eq_str(value: &Value, other: &str) -> bool {
    value.as_str().map_or(false, |i| i == other)
}

impl PartialEq<str> for Value {
    fn eq(&self, other: &str) -> bool {
        eq_str(self, other)
    }
}

impl<'a> PartialEq<&'a str> for Value {
    fn eq(&self, other: &&str) -> bool {
        eq_str(self, *other)
    }
}

impl PartialEq<Value> for str {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, self)
    }
}

impl<'a> PartialEq<Value> for &'a str {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, *self)
    }
}

impl PartialEq<String> for Value {
    fn eq(&self, other: &String) -> bool {
        eq_str(self, other.as_str())
    }
}

impl PartialEq<Value> for String {
    fn eq(&self, other: &Value) -> bool {
        eq_str(other, self.as_str())
    }
}

macro_rules! partialeq_numeric {
    ($($eq:ident [$($ty:ty)*])*) => {
        $($(
            impl PartialEq<$ty> for Value {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(self, *other as _)
                }
            }

            impl PartialEq<Value> for $ty {
                fn eq(&self, other: &Value) -> bool {
                    $eq(other, *self as _)
                }
            }

            impl<'a> PartialEq<$ty> for &'a Value {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(*self, *other as _)
                }
            }

            impl<'a> PartialEq<$ty> for &'a mut Value {
                fn eq(&self, other: &$ty) -> bool {
                    $eq(*self, *other as _)
                }
            }
        )*)*
    }
}

partialeq_numeric! {
    eq_u64[u8 u16 u32 u64 usize]
    eq_bool[bool]
}
