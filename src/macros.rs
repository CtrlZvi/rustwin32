// Macros to help create types

macro_rules! win32_enum {
    {enum $name:ident ($backing_type:path) { $($variant:ident = $value:expr,)+ } } => {
        pub enum $name {
            $($variant),+
        }

        impl From<$name> for $backing_type {
            fn from(source: $name) -> $backing_type {
                match source {
                    $($name::$variant => $value),+
                }
            }
        }
    };
}