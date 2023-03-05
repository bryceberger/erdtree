use super::node::Node;
use crate::cli;
use std::{cmp::Ordering, convert::From};

/// Order in which to print nodes.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Order {
    Name,
    Dir,
    Size,
    None,
}

impl Order {
    /// Yields function pointer to the appropriate `Node` comparator.
    pub fn comparator(&self) -> Option<fn(a: &Node, b: &Node) -> Ordering> {
        match self {
            Self::Name => Some(Self::name_comparator),
            Self::Dir => Some(Self::name_dir_first_comparator),
            Self::Size => Some(Self::size_comparator),
            _ => None,
        }
    }

    /// Comparator based on `Node` file names.
    fn name_comparator(a: &Node, b: &Node) -> Ordering {
        a.file_name().cmp(b.file_name())
    }

    /// Comparator based on `Node` file names, with directories appearing before files
    fn name_dir_first_comparator(a: &Node, b: &Node) -> Ordering {
        match (a.is_dir(), b.is_dir()) {
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (true, true) | (false, false) => a.file_name().cmp(b.file_name()),
        }
    }

    /// Comparator based on `Node` file sizes
    fn size_comparator(a: &Node, b: &Node) -> Ordering {
        let a_size = a.file_size.unwrap_or(0);
        let b_size = b.file_size.unwrap_or(0);

        b_size.cmp(&a_size)
    }
}

impl From<cli::Order> for Order {
    fn from(ord: cli::Order) -> Self {
        match ord {
            cli::Order::Name => Order::Name,
            cli::Order::Dir => Order::Dir,
            cli::Order::Size => Order::Size,
            cli::Order::None => Order::None,
        }
    }
}
