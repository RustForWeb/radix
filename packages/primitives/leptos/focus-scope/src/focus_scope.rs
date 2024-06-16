// TODO: remove
#![allow(unused)]

use std::ops::Deref;

use leptos::{html::AnyElement, *};
use radix_leptos_primitive::Primitive;
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    NodeFilter,
};

struct FocusScopeValue {
    pub paused: bool,
}

impl FocusScopeValue {
    pub fn new() -> Self {
        Self { paused: false }
    }

    pub fn pause(&mut self) {
        self.paused = true;
    }

    pub fn resume(&mut self) {
        self.paused = false;
    }
}

#[component]
pub fn FocusScope(
    #[prop(into, optional)] r#loop: MaybeProp<bool>,
    #[prop(into, optional)] trapped: MaybeProp<bool>,
    // TODO: event handlers
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    // let r#loop = move || r#loop.get().unwrap_or(false);
    let trapped = move || trapped.get().unwrap_or(false);

    // let container_ref = create_node_ref::<AnyElement>();
    // let last_focused_element = create_signal::<Option<HtmlElement<AnyElement>>>(None);

    // let focus_scope = create_rw_signal(FocusScopeValue::new());

    create_effect(move |_| {
        if trapped() {
            // TODO
        }
    });

    let mut attrs = attrs.clone();
    attrs.extend(vec![("tabindex", "-1".into_attribute())]);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            attrs=attrs
        >
            {children()}
        </Primitive>
    }
}

#[derive(Clone, Debug, Default)]
struct FocusOptions {
    pub select: bool,
}

/// Attempts focusing the first element in a list of candidates.
/// Stops when focus has actually moved.
fn focus_first(candidates: Vec<web_sys::HtmlElement>, options: Option<FocusOptions>) {
    let previously_focused_element = document().active_element();

    for candidate in candidates {
        focus(Some(candidate), options.clone());
        if document().active_element() != previously_focused_element {
            return;
        }
    }
}

/// Returns the first and last tabbable elements inside a container.
fn get_tabbable_edges(
    container: &web_sys::HtmlElement,
) -> (Option<web_sys::HtmlElement>, Option<web_sys::HtmlElement>) {
    let candidates = get_tabbable_candidates(container);

    let mut reverse_candidates = candidates.clone();
    reverse_candidates.reverse();

    let first = find_visible(candidates, container);
    let last = find_visible(reverse_candidates, container);

    (first, last)
}

/// Returns a list of potential tabbable candidates.
///
/// NOTE: This is only a close approximation. For example it doesn't take into account cases like when
/// elements are not visible. This cannot be worked out easily by just reading a property, but rather
/// necessitate runtime knowledge (computed styles, etc). We deal with these cases separately.
///
/// See: https://developer.mozilla.org/en-US/docs/Web/API/TreeWalker
/// Credit: https://github.com/discord/focus-layers/blob/master/src/util/wrapFocus.tsx#L1
fn get_tabbable_candidates(container: &web_sys::HtmlElement) -> Vec<web_sys::HtmlElement> {
    let mut nodes: Vec<web_sys::HtmlElement> = vec![];

    let accept_node_closure: Closure<dyn Fn(web_sys::Node) -> u32> =
        Closure::new(move |node: web_sys::Node| -> u32 {
            if let Some(html_element) = node.dyn_ref::<web_sys::HtmlElement>() {
                if html_element.hidden() {
                    // NodeFilter.FILTER_SKIP
                    return 3;
                }

                if let Some(input_element) = node.dyn_ref::<web_sys::HtmlInputElement>() {
                    if input_element.disabled() || input_element.type_() == "hidden" {
                        // NodeFilter.FILTER_SKIP
                        return 3;
                    }
                }

                if html_element.tab_index() >= 0 {
                    // NodeFilter.FILTER_ACCEPT
                    return 1;
                }
            }

            // NodeFilter.FILTER_SKIP
            3
        });

    let mut node_filter = NodeFilter::new();
    node_filter.accept_node(accept_node_closure.as_ref().unchecked_ref());

    let walker = document()
        // 0x01 is NodeFilter.SHOW_ELEMENT
        .create_tree_walker_with_what_to_show_and_filter(container, 0x1, Some(&node_filter))
        .expect("Tree walker should be created.");

    while let Some(node) = walker
        .next_node()
        .expect("Tree walker should return a next node.")
    {
        if let Ok(element) = node.dyn_into::<web_sys::HtmlElement>() {
            nodes.push(element);
        }
    }

    // We do not take into account the order of nodes with positive `tabindex` as it
    // hinders accessibility to have tab order different from visual order.
    nodes
}

/// Returns the first visible element in a list.
/// NOTE: Only checks visibility up to the `container`.
fn find_visible(
    elements: Vec<web_sys::HtmlElement>,
    container: &web_sys::HtmlElement,
) -> Option<web_sys::HtmlElement> {
    elements.into_iter().find(|element| {
        !is_hidden(
            element,
            Some(IsHiddenOptions {
                up_to: Some(container),
            }),
        )
    })
}

#[derive(Debug, Default, Clone)]
struct IsHiddenOptions<'a> {
    pub up_to: Option<&'a web_sys::HtmlElement>,
}

fn is_hidden(node: &web_sys::HtmlElement, options: Option<IsHiddenOptions>) -> bool {
    let options = options.unwrap_or_default();

    if window()
        .get_computed_style(node)
        .expect("Element is valid.")
        .expect("Element should have computed style.")
        .get_property_value("visibility")
        .expect("Computed style should have visibility.")
        == "hidden"
    {
        return true;
    }

    let mut node: Option<web_sys::Element> = Some(node.deref().clone());
    while let Some(n) = node.as_ref() {
        if let Some(up_to) = options.up_to.as_ref() {
            // We stop at `upTo` (excluding it).
            let up_to_element: &web_sys::Element = up_to;
            if n == up_to_element {
                return false;
            }

            if window()
                .get_computed_style(n)
                .expect("Element is valid.")
                .expect("Element should have computed style.")
                .get_property_value("visibility")
                .expect("Computed style should have display.")
                == "none"
            {
                return true;
            }

            node = n.parent_element();
        }
    }

    false
}

fn is_selectable_input(element: &web_sys::Element) -> bool {
    web_sys::HtmlInputElement::instanceof(element)
}

fn focus(element: Option<web_sys::HtmlElement>, options: Option<FocusOptions>) {
    let options = options.unwrap_or_default();

    if let Some(element) = element {
        let previously_focused_element = document().active_element();

        // NOTE: We prevent scrolling on focus, to minimize jarring transitions for users.
        // TODO: web_sys does not support passing options. JS: element.focus({ preventScroll: true })
        element.focus().expect("Focus should be successful.");

        // Only select if its not the same element, it supports selection and we need to select.
        let el: &web_sys::Element = &element;
        if Some(el) != previously_focused_element.as_ref()
            && is_selectable_input(el)
            && options.select
        {
            element
                .unchecked_into::<web_sys::HtmlInputElement>()
                .select();
        }
    }
}
