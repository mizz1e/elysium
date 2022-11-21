macro_rules! networked {
    ($(($struct:ident, $struct_field:ident): $struct_name:literal {
        $($field:ident: $field_ty:ty = $field_name:literal,)*
    },)*) => {
        #[derive(Clone, Copy)]
        enum Table { $($struct,)* }

        impl Table {

            fn from_bytes(bytes: &[u8]) -> Option<Self> {
                const MAP: phf::Map<&[u8], Table> = phf::phf_map! {
                    $($struct_name => Table::$struct,)*
                };

                MAP.get(bytes).copied()
            }
        }

        mod imp {
            $(#[allow(non_camel_case_types)]
            #[derive(Clone, Copy)]
            pub(super) enum $struct { $($field,)* }

            impl $struct {

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

        unsafe fn iterate_table(networked: &mut Networked, recv_table: &RecvTable, table: Table, base_offset: usize) {
            for prop in recv_table.props() {
                let offset = base_offset + prop.offset as usize;
                let prop_name = CStr::from_ptr(prop.name).to_bytes();

                if matches!(prop.kind, PropKind::DataTable) {
                    iterate_table(networked, &*prop.data_table, table, offset);
                }

                match table {
                    $(Table::$struct => if let Some(var) = imp::$struct::from_bytes(prop_name) {
                        match var {
                            $(imp::$struct::$field => networked.$struct_field.$field = Var::new(offset),)*
                        }
                    },)*
                }
            }
        }

        pub unsafe fn setup(mut class_list: *const ClientClass) {
            global::with_app_mut(|app| {
                let mut networked = unsafe { MaybeUninit::zeroed().assume_init() };

                while let Some(class) = class_list.as_ref() {
                    let recv_table = &*class.recv_table;
                    let table_name = CStr::from_ptr(recv_table.name).to_bytes();

                    class_list = class.next;

                    if let Some(table) = Table::from_bytes(table_name) {
                        iterate_table(&mut networked, recv_table, table, 0);
                    }
                }

                $($(if networked.$struct_field.$field.offset == 0 {
                    panic(stringify!($struct_field), stringify!($field));
                })*)*

                app.insert_resource(networked);
            });
        }
    };
}
