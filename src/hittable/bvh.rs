use crate::geo::aabb::Aabb;
use crate::hittable::triangle::Triangle;
use crate::hittable::{Hittable, Hittables};
use std::rc::Rc;

/// Bounding Volume Hierarchy
pub struct Bvh {
    left: Box<BvhItem>,
    right: Box<BvhItem>,
    b_box: Aabb,
}

enum BvhItem {
    Node(Bvh),
    Leaf(Rc<Triangle>),
}

impl Bvh {
    /// Creates a new hittable object from the given hittable list
    /// The bounding Volume Hierarchy sorts the hittables in a binary tree
    /// where each node has a bounding box.
    /// This is to optimize the ray intersection search when having many hittable objects.
    pub fn new(list: Vec<Rc<Triangle>>) -> Hittables {
        if list.len() == 0 {
            panic!("Cannot create a Bvh with empty list of objects")
        }
        Hittables::Bvh(create_bvh(&list[..], 0, list.len()))
    }
}

fn create_bvh(list: &[Rc<Triangle>], start: usize, end: usize) -> Bvh {
    let num_objects = end - start;

    let (left, right, b_box) = if num_objects == 1 {
        (
            BvhItem::Leaf(list[start].clone()),
            BvhItem::Leaf(list[start].clone()),
            list[start].bounding_box().clone(),
        )
    } else if num_objects == 2 {
        (
            BvhItem::Leaf(list[start].clone()),
            BvhItem::Leaf(list[start + 1].clone()),
            Aabb::combine_aabbs(list[start].bounding_box(), list[start + 1].bounding_box()),
        )
    } else {
        let mid = sort_hittables_slice_by_most_spread_axis(list, start, end);
        let left = create_bvh(list, start, mid);
        let right = create_bvh(list, mid, end);
        (
            BvhItem::Node(left),
            BvhItem::Node(right),
            Aabb::combine_aabbs(&left.b_box, &right.b_box),
        )
    };

    return Bvh {
        left: Box::new(left),
        right: Box::new(right),
        b_box,
    };
}

fn sort_hittables_slice_by_most_spread_axis(p0: &[Rc<Triangle>], p1: usize, p2: usize) -> usize {
    todo!()
}
