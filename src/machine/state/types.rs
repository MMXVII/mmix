use std::mem::transmute;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Byte(u8);

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Wyde(u16);

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Tetra(u32);

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Octa(u64);

macro_rules! type_impl {
    ( $( ($outer:ident, $inner:ty) => ($( $convert:ty ),*), )* ) => {
        $(
            $(
                impl From<$convert> for $outer {
                    fn from(from: $convert) -> Self {
                        let x = unsafe {
                            transmute::<$convert, $inner>(from)
                        };
                        $outer(x)
                    }
                }

                impl Into<$convert> for $outer {
                    fn into(self) -> $convert {
                        let x = self.0;
                        unsafe {
                            transmute::<$inner, $convert>(x)
                        }
                    }
                }
            )*
        )*
    }
}

type_impl! {
    (Byte,   u8) => ( u8,  i8     ),
    (Wyde,  u16) => (u16, i16     ),
    (Tetra, u32) => (u32, i32, f32),
    (Octa,  u64) => (u64, i64, f64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_tetra_u32() {
        let x: Tetra = 3.14f32.into();
        let y: u32 = x.into();
        assert_eq!(0x4048f5c3, y);
    }

    // TODO add more tests

}
