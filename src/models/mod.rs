pub mod container;
pub mod image;
pub mod network;

macro_rules! model {
    { @enum_final $( #[$meta:meta] )* $vis:vis $name:ident $($variants:tt)* } => {
        $( #[$meta] )*
        pub enum $name {
            $($variants)*
        }
    };
    {
        @enum_parsing $( #[$meta:meta] )* $vis:vis $name:ident
        @variants [$($variants:tt)*]
        @parsing $variant_name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident : $field_ty:ty
            ),* $(,)?
        } $(, $($tt:tt)* )? } => {
        model! {
            @enum_parsing $( #[$meta] )* $vis $name
            @variants [$($variants)* $variant_name {
                $(
                    $(#[$field_meta])*
                    #[serde(skip_serializing_if = "Option::is_none")]
                    $field_vis $field_name : Option<$field_ty>
                ),*
            },]
            @parsing $($($tt)*)?
        }
    };
    {
        @enum_parsing $( #[$meta:meta] )* $vis:vis $name:ident
        @variants [$($variants:tt)*]
        @parsing ,} => { model! { @enum_final $( #[$meta] )* $vis $name $($variants)* } };
    {
        @enum_parsing $( #[$meta:meta] )* $vis:vis $name:ident
        @variants [$($variants:tt)*]
        @parsing } => { model! { @enum_final $( #[$meta] )* $vis $name $($variants)* } };

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
    {
        $( #[$meta:meta] )*
        $vis:vis enum $name:ident {
            $($tt:tt)*
        }
    } => {
        model! { @enum_parsing $( #[$meta] )* $vis $name @variants [] @parsing $($tt)* }
    };
}

use model;
