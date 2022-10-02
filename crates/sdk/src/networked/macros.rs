macro_rules! networked {
    ($(($struct:ident, $struct_field:ident): $struct_name:literal {
        $($field:ident: $field_ty:ty = $field_name:literal,)*
    },)*) => {
        #[derive(Clone, Copy)]
        enum Class { $($struct,)* }

        impl Class {
            #[inline]
            fn from_bytes(bytes: &[u8]) -> Option<Self> {
                const MAP: phf::Map<&[u8], Class> = phf::phf_map! {
                    $($struct_name => Class::$struct,)*
                };

                MAP.get(bytes).copied()
            }
        }

        mod imp {
            $(#[allow(non_camel_case_types)]
            #[derive(Clone, Copy)]
            pub(super) enum $struct { $($field,)* }

            impl $struct {
                #[inline]
                pub(super) fn from_bytes(bytes: &[u8]) -> Option<Self> {
                    const MAP: phf::Map<&[u8], $struct> = phf::phf_map! {
                        $($field_name => $struct::$field,)*
                    };

                    MAP.get(bytes).copied()
                }
            })*
        }

        $(#[derive(Debug)]
        pub struct $struct {
            $(pub $field: Var<$field_ty>,)*
        })*

        #[derive(Debug)]
        pub struct Networked {
            $(pub $struct_field: $struct,)*
        }

        #[inline]
        fn iterate_table(networked: &mut Networked, table: &'static Table, class: Class, base_offset: usize) {
            for property in table.properties().iter() {
                let offset = base_offset + property.offset as usize;

                if let Some(sub_table) = property.data_table() {
                    iterate_table(
                        networked,
                        sub_table,
                        class,
                        offset,
                    );
                }

                let name = property.name();

                match class {
                    $(Class::$struct => if let Some(var) = imp::$struct::from_bytes(name) {
                        match var {
                            $(imp::$struct::$field => networked.$struct_field.$field = Var::new(offset),)*
                        }
                    },)*
                }
            }
        }

        #[inline]
        pub unsafe fn init(client: &Client) {
            let mut networked = unsafe { MaybeUninit::zeroed().assume_init() };
            let top_level = client.class_iter();

            for class in top_level {
                if let Some(table) = class.table {
                    let name = table.name();

                    if let Some(class) = Class::from_bytes(name) {
                        iterate_table(&mut networked, table, class, 0);
                    }
                }
            }

            $($(if networked.$struct_field.$field.offset == 0 {
                panic(stringify!($struct_field), stringify!($field));
            })*)*

            println!("{networked:?}");

            NETWORKED.store(Some(&mut *Box::into_raw(Box::new(networked))));
        }
    };
}
