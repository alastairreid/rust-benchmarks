mod strategy;

pub mod prelude {
    pub use crate::proptest;
    pub use crate::strategy::Just;
    pub use crate::strategy::Strategy;

    pub mod prop {
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

