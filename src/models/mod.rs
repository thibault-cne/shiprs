pub mod container;
pub mod image;
pub mod network;

macro_rules! model {
    {
        $( #[$meta:meta] )*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident : $field_ty:ty
            ),* $(,)?
        }
    } => {
        $( #[$meta] )*
        $vis struct $name {
            $(
                $( #[$field_meta] )*
                #[serde(skip_serializing_if = "Option::is_none")]
                $field_vis $field_name: Option<$field_ty>
            ),*
        }
    };
}

use model;
