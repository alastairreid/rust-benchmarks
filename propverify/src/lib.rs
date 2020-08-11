// Copyright 2020 The Propverify authors
// Based on parts of Proptest which is Copyright 2017, 2018 Jason Lingle
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod strategy;

pub mod prelude {
    // Macros
    pub use crate::proptest;
    pub use crate::prop_oneof;
    pub use crate::prop_compose;

    // Functions and types
    pub use crate::strategy::of;
    pub use crate::strategy::{maybe_ok, maybe_err};
    pub use crate::strategy::prop_is_replay;
    pub use crate::strategy::Just;
    pub use crate::strategy::Strategy;

    // Modules with same name as types
    pub use crate::strategy::{bool, char};

    pub mod prop {
        pub use crate::strategy::prop_is_replay;
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
        pub mod num {
            pub use crate::strategy::{u8, u16, u32, u64, u128, usize};
            pub use crate::strategy::{i8, i16, i32, i64, i128, isize};
            pub use crate::strategy::{f32, f64};
        }
    }
}

