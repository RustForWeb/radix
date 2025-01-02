use leptos::{
    html::{self, ElementType},
    prelude::*,
    tachys::html::node_ref::NodeRefContainer,
    wasm_bindgen::JsCast,
    web_sys::Element,
};
use leptos_node_ref::prelude::*;
use std::{rc::Rc, iter::IntoIterator};

/// A trait for composable node references that can be combined,
/// while maintaining static dispatch (tuples) and dynamic dispatch (iterables).
pub trait ComposeRefs {
    /// Applies the composition to a given DOM node.
    fn compose_with(&self, node: &Element);
}

// -------------------------------------
// 1. Static Implementations
// -------------------------------------

impl ComposeRefs for AnyNodeRef {
    #[inline(always)]
    fn compose_with(&self, node: &Element) {
        <AnyNodeRef as NodeRefContainer<html::Div>>::load(*self, node);
    }
}

impl<T> ComposeRefs for NodeRef<T>
where
    T: ElementType,
    T::Output: JsCast,
{
    #[inline(always)]
    fn compose_with(&self, node: &Element) {
        <NodeRef<T> as NodeRefContainer<T>>::load(*self, node);
    }
}

// NOTE: See macro ahead, replaces these. These are
//  left for illustration for now.
// impl<A, B> ComposeRefs for (A, B)
// where
//     A: ComposeRefs,
//     B: ComposeRefs,
// {
//     #[inline(always)]
//     fn compose_with(&self, node: &Element) {
//         self.0.compose_with(node);
//         self.1.compose_with(node);
//     }
// }

// impl<A, B, C> ComposeRefs for (A, B, C)
// where
//     A: ComposeRefs,
//     B: ComposeRefs,
//     C: ComposeRefs,
// {
//     #[inline(always)]
//     fn compose_with(&self, node: &Element) {
//         self.0.compose_with(node);
//         self.1.compose_with(node);
//         self.2.compose_with(node);
//     }
// }

macro_rules! impl_compose_refs_tuple {
    ($($idx:tt $type:ident),+) => {
        impl<$($type),+> ComposeRefs for ($($type),+)
        where
            $($type: ComposeRefs),+
        {
            #[inline(always)]
            fn compose_with(&self, node: &Element) {
                $(
                    self.$idx.compose_with(node);
                )+
            }
        }
    }
}

impl_compose_refs_tuple!(0 A, 1 B);
impl_compose_refs_tuple!(0 A, 1 B, 2 C);
impl_compose_refs_tuple!(0 A, 1 B, 2 C, 3 D);
impl_compose_refs_tuple!(0 A, 1 B, 2 C, 3 D, 4 E);
impl_compose_refs_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F);

// -------------------------------------
// 2. Dynamic Implementations
// -------------------------------------

/// Implementation for arrays of any size
impl<T: ComposeRefs, const N: usize> ComposeRefs for [T; N] {
    fn compose_with(&self, node: &Element) {
        for item in self.iter() {
            item.compose_with(node);
        }
    }
}

/// Implementation for slice references
impl<T: ComposeRefs> ComposeRefs for &[T] {
    fn compose_with(&self, node: &Element) {
        for item in (*self).iter() {
            item.compose_with(node);
        }
    }
}

/// Implementation for Vec
impl<T: ComposeRefs> ComposeRefs for Vec<T> {
    fn compose_with(&self, node: &Element) {
        for item in self.iter() {
            item.compose_with(node);
        }
    }
}

// -------------------------------------
// 3. compose_refs + Hook
// -------------------------------------

/// Combines multiple node references into a single reference that, when set,
/// updates all input references to point to the same DOM node.
///
/// - **Static**: Tuples (`(ref1, ref2, ...)`)—no heap allocation.
/// - **Dynamic**: Any iterable (`Vec`, slice, array) of references.
///
/// # Examples
/// ```rust
/// use leptos::{html::Div, html::Button};
/// use leptos::prelude::NodeRef;
/// use leptos_node_ref::prelude::*;
/// use radix_leptos_compose_refs::compose_refs;
///
/// // 1) Static composition (tuples):
/// let div_ref = NodeRef::<Div>::new();
/// let btn_ref = NodeRef::<Button>::new();
/// let composed = compose_refs((div_ref, btn_ref));
///
/// // 2) Dynamic composition (Vec, slice):
/// let refs = vec![div_ref.into_any(), btn_ref.into_any()];
/// let composed = compose_refs(refs);
/// ```
pub fn compose_refs<T>(refs: T) -> AnyNodeRef
where
    T: ComposeRefs + 'static,
{
    let composed_ref = AnyNodeRef::new();
    let refs = Rc::new(refs);

    // Effect will re-run if `composed_ref` changes.
    Effect::new(move |_| {
        if let Some(node) = composed_ref.get() {
            refs.compose_with(&node);
        }
    });

    composed_ref
}

/// Hook-style wrapper around `compose_refs`.
///
/// Identical behavior, just a `use_*` naming convention.
pub fn use_composed_refs<T>(refs: T) -> AnyNodeRef
where
    T: ComposeRefs + 'static,
{
    compose_refs(refs)
}

// -------------------------------------
// 4. Tests
// -------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::html;

    #[test]
    fn test_static_any_node_ref() {
        let any_ref1 = AnyNodeRef::new();
        let any_ref2 = AnyNodeRef::new();
        let composed = compose_refs((any_ref1, any_ref2));
        assert!(composed.get().is_none());
    }

    #[test]
    fn test_static_specific_node_ref() {
        let div_ref = NodeRef::<html::Div>::new();
        let btn_ref = NodeRef::<html::Button>::new();
        let composed = compose_refs((div_ref, btn_ref));
        assert!(composed.get().is_none());
    }

    #[test]
    fn test_triple_static() {
        let r1 = NodeRef::<html::Div>::new();
        let r2 = NodeRef::<html::Button>::new();
        let r3 = AnyNodeRef::new();
        let composed = compose_refs((r1, r2, r3));
        assert!(composed.get().is_none());
    }

    #[test]
    fn test_dynamic_vec_any_node_ref() {
        let refs = vec![AnyNodeRef::new(), AnyNodeRef::new()];
        let composed = compose_refs(refs);
        assert!(composed.get().is_none());
    }

    #[test]
    fn test_dynamic_vec_specific_node_ref() {
        let div_ref = NodeRef::<html::Div>::new();
        let btn_ref = NodeRef::<html::Button>::new();
        let refs = vec![div_ref.into_any(), btn_ref.into_any()];
        let composed = compose_refs(refs);
        assert!(composed.get().is_none());
    }

    #[test]
    fn test_array_of_any_node_ref() {
        let arr = [AnyNodeRef::new(), AnyNodeRef::new()];
        let composed = compose_refs(arr);
        assert!(composed.get().is_none());
    }

    #[test]
    fn test_slice_of_any_node_ref() {
        let s: &[AnyNodeRef] = &[AnyNodeRef::new(), AnyNodeRef::new()];
        let composed = compose_refs(s);
        assert!(composed.get().is_none());
    }

    #[test]
    fn test_compositions() {
        // Array
        let arr = [AnyNodeRef::new(), AnyNodeRef::new()];
        let composed = compose_refs(arr);
        assert!(composed.get().is_none());

        // Slice reference
        let s: &[AnyNodeRef] = &[AnyNodeRef::new(), AnyNodeRef::new()];
        let composed = compose_refs(s);
        assert!(composed.get().is_none());

        // Vec
        let v = vec![AnyNodeRef::new(), AnyNodeRef::new()];
        let composed = compose_refs(v);
        assert!(composed.get().is_none());
    }
}
