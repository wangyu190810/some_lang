use std::cmp::Ordering;

#[derive(Debug)]
enum BTree<T: Ord> {
	Leaf {
		v: T,
		l: Box<BTree<T>>,
		r: Box<BTree<T>>,
	},
	Empty,
}

impl<T: Ord> BTree<T> {
	fn new() -> BTree<T> {
		BTree::Empty
	}

	fn insert(&mut self, nv: T) {
		match self {
			&mut BTree::Leaf { ref v, ref mut l, ref mut r } => {
				match nv.cmp(v) {
					Ordering::Less => r.insert(nv),
					Ordering::Greater => l.insert(nv),
					_  => return
				}
			},
			&mut BTree::Empty => {
				*self = BTree::Leaf { v: nv, l: Box::new(BTree::Empty), r: Box::new(BTree::Empty) }
			},
		};
	}

	fn is_empty(&self) -> bool {
		match self {
			&BTree::Leaf { .. } => false,
			&BTree::Empty => true,
		}
	}

	fn find(&self, fv: T) -> bool {
		match self {
			&BTree::Leaf { ref v, ref l, ref r } => {
				match fv.cmp(v) {
					Ordering::Less => r.find(fv),
					Ordering::Greater => l.find(fv),
					_  => true
				}
			},
			&BTree::Empty => false,
		}
	}
}