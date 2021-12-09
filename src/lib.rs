#[cfg(test)]
mod test;

#[macro_export]
macro_rules! mod_use {
    ($($id:ident)+) => {
        $(
            mod $id;
            pub use $id::*;
        )+
    };
}
