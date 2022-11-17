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

        #[derive(Resource)]
        pub struct Networked {
            $(pub $struct_field: $struct,)*
        }

        #[inline]
        unsafe fn iterate_table(networked: &mut Networked, recv_table: &RecvTable, class: Class, base_offset: usize) {
            let props = slice::from_raw_parts_mut(recv_table.props, recv_table.props_len as usize);

            for prop in props {
                let offset = base_offset + prop.offset as usize;
                let prop = CStr::from_ptr(prop.name).to_bytes();

                match class {
                    $(Class::$struct => if let Some(var) = imp::$struct::from_bytes(prop) {
                        match var {
                            $(imp::$struct::$field => networked.$struct_field.$field = Var::new(offset),)*
                        }
                    },)*
                }
            }
        }

        #[inline]
        pub unsafe fn setup(mut class_list: *const ClientClass) {
            let mut networked = unsafe { MaybeUninit::zeroed().assume_init() };

            while let Some(class) = class_list.as_ref() {
                let class_name = CStr::from_ptr(class.network_name).to_bytes();
                let recv_table = &*class.recv_table;

                class_list = class.next;

                if let Some(class) = Class::from_bytes(class_name) {
                    iterate_table(&mut networked, recv_table, class, 0);
                }
            }

            $($(if networked.$struct_field.$field.offset == 0 {
                panic(stringify!($struct_field), stringify!($field));
            })*)*

            global::with_app_mut(|app| app.insert_resource(networked));
        }
    };
}
