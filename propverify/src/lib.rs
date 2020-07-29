mod strategy;

pub mod prelude {
    pub use crate::proptest;
    pub use crate::strategy::of;
    pub use crate::strategy::{maybe_ok, maybe_err};
    pub use crate::strategy::Just;
    pub use crate::strategy::Strategy;

    pub mod prop {
        pub use crate::strategy::{uniform0,  uniform1,  uniform2,  uniform3,  uniform4};
        pub use crate::strategy::{uniform5,  uniform6,  uniform7,  uniform8,  uniform9};
        pub use crate::strategy::{uniform10, uniform11, uniform12, uniform13, uniform14};
        pub use crate::strategy::{uniform15, uniform16, uniform17, uniform18, uniform19};
        pub use crate::strategy::{uniform20, uniform21, uniform22, uniform23, uniform24};
        pub use crate::strategy::{uniform25, uniform26, uniform27, uniform28, uniform29};
        pub use crate::strategy::{uniform30, uniform31, uniform32};
        pub mod collection {
            pub use crate::strategy::vec;
            pub use crate::strategy::vec_deque;
            pub use crate::strategy::linked_list;
            pub use crate::strategy::binary_heap;
            pub use crate::strategy::btree_set;
            pub use crate::strategy::btree_map;
        }
    }
}

