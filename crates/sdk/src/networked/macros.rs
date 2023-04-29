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

        unsafe fn iterate_table(networked: &mut Networked, recv_table: &source_sys::RecvTable, table: Table, base_offset: usize) {
            let mut index = 0;

            while index < recv_table.m_nProps {
                let prop = recv_table.m_pProps.add(index as usize).read_unaligned();
                let offset = base_offset + prop.m_Offset as usize;
                let prop_name = CStr::from_ptr(prop.m_pVarName).to_bytes();

                if prop.m_RecvType == source_sys::SendPropType_DPT_DataTable {
                    iterate_table(networked, &prop.m_pDataTable.read_unaligned(), table, offset);
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

        pub unsafe fn setup(mut class_list: *const source_sys::ClientClass) {
            global::with_app_mut(|app| {
                let mut networked = unsafe { MaybeUninit::zeroed().assume_init() };

                while let Some(class) = class_list.as_ref() {
                    let recv_table = &class.m_pRecvTable.read_unaligned();
                    let table_name = CStr::from_ptr(recv_table.m_pNetTableName).to_bytes();

                    class_list = class.m_pNext;

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
