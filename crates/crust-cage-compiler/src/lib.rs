use quote::ToTokens;

pub trait ToSafeRust{
    fn to_safe(&self) -> impl ToTokens;
}