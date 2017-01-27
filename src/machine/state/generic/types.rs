use std::mem::transmute;

pub struct Byte(u8);
pub struct Wyde(u16);
pub struct Tetra(u32);
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
