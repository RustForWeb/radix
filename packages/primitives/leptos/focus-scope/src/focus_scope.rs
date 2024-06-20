use std::ops::Deref;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;
use std::{cell::RefCell, sync::Arc};

use leptos::{html::AnyElement, *};
use once_cell::sync::Lazy;
use radix_leptos_primitive::Primitive;
use web_sys::{
    wasm_bindgen::{closure::Closure, JsCast},
    CustomEvent, CustomEventInit, Event, FocusEvent, KeyboardEvent, MutationObserver, NodeFilter,
};
use web_sys::{MutationObserverInit, MutationRecord};

const AUTOFOCUS_ON_MOUNT: &str = "focusScope.autoFocusOnMount";
const AUTOFOCUS_ON_UNMOUNT: &str = "focusScope.autoFocusOnUnmount";

#[component]
pub fn FocusScope(
    /// When `true`, tabbing from last item will focus first tabbable and shift+tab from first item will focus last tababble. Defaults to `false`.
    #[prop(into, optional)]
    r#loop: MaybeProp<bool>,
    /// When `true`, focus cannot escape the focus scope via keyboard, pointer, or a programmatic focus. Defaults to `false`.
    #[prop(into, optional)]
    trapped: MaybeProp<bool>,
    #[prop(into, optional)] on_mount_auto_focus: MaybeProp<Rc<dyn Fn(Event)>>,
    #[prop(into, optional)] on_unmount_auto_focus: MaybeProp<Rc<dyn Fn(Event)>>,
    #[prop(into, optional)] as_child: MaybeProp<bool>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    children: ChildrenFn,
) -> impl IntoView {
    let r#loop = Signal::derive(move || r#loop.get().unwrap_or(false));
    let trapped = Signal::derive(move || trapped.get().unwrap_or(false));

    let container_ref = create_node_ref::<AnyElement>();
    let last_focused_element = create_rw_signal::<Option<web_sys::HtmlElement>>(None);
    let focus_scope = create_rw_signal(FocusScopeAPI::new());

    let handle_focus_in: Rc<Closure<dyn Fn(FocusEvent)>> =
        Rc::new(Closure::new(move |event: FocusEvent| {
            if focus_scope.get_untracked().paused() {
                return;
            }

            if let Some(container) = container_ref.get_untracked() {
                let target = event
                    .target()
                    .map(|target| target.unchecked_into::<web_sys::HtmlElement>());

                if container.contains(target.as_ref().map(|e| e.unchecked_ref())) {
                    last_focused_element.set(target);
                } else {
                    focus(
                        last_focused_element.get_untracked(),
                        Some(FocusOptions { select: true }),
                    );
                }
            }
        }));

    let handle_focus_out: Rc<Closure<dyn Fn(FocusEvent)>> =
        Rc::new(Closure::new(move |event: FocusEvent| {
            if focus_scope.get_untracked().paused() {
                return;
            }

            if let Some(container) = container_ref.get_untracked() {
                let related_target = event
                    .related_target()
                    .map(|target| target.unchecked_into::<web_sys::HtmlElement>());

                // A `focusout` event with a `None` `related_target` will happen in at least two cases:
                //
                // 1. When the user switches app/tabs/windows/the browser itself loses focus.
                // 2. In Google Chrome, when the focused element is removed from the DOM.
                //
                // We let the browser do its thing here because:
                //
                // 1. The browser already keeps a memory of what's focused for when the page gets refocused.
                // 2. In Google Chrome, if we try to focus the deleted focused element (as per below), it
                //    throws the CPU to 100%, so we avoid doing anything for this reason here too.
                if related_target.is_none() {
                    return;
                }

                // If the focus has moved to an actual legitimate element (`related_target != None`)
                // that is outside the container, we move focus to the last valid focused element inside.
                if !container.contains(related_target.as_ref().map(|e| e.unchecked_ref())) {
                    focus(
                        last_focused_element.get_untracked(),
                        Some(FocusOptions { select: true }),
                    );
                }
            }
        }));

    let mutation_observer: Rc<RefCell<Option<MutationObserver>>> = Rc::new(RefCell::new(None));

    let cleanup_handle_focus_in = handle_focus_in.clone();
    let cleanup_handle_focus_out = handle_focus_out.clone();
    let cleanup_mutation_observer = mutation_observer.clone();

    // Takes care of trapping focus if focus is moved outside programmatically for example
    create_effect(move |_| {
        if trapped.get() {
            document()
                .add_event_listener_with_callback(
                    "focusin",
                    (*handle_focus_in).as_ref().unchecked_ref(),
                )
                .expect("Focus in event listener should be added.");
            document()
                .add_event_listener_with_callback(
                    "focusout",
                    (*handle_focus_out).as_ref().unchecked_ref(),
                )
                .expect("Focus out event listener should be added.");
        }
    });

    create_effect(move |_| {
        if trapped.get() {
            if let Some(container) = container_ref.get() {
                if let Some(mutation_observer) = mutation_observer.take() {
                    mutation_observer.disconnect();
                }

                // When the focused element gets removed from the DOM, browsers move focus
                // back to the document.body. In this case, we move focus to the container
                // to keep focus trapped correctly.
                let handle_mutations: Closure<dyn Fn(Vec<MutationRecord>)> =
                    Closure::new(move |mutations: Vec<MutationRecord>| {
                        let focused_element = document()
                            .active_element()
                            .map(|element| element.unchecked_into::<web_sys::HtmlElement>());
                        if focused_element != document().body() {
                            return;
                        }

                        for mutation in mutations {
                            if mutation.removed_nodes().length() > 0 {
                                focus(container_ref.get_untracked().as_deref().cloned(), None);
                            }
                        }
                    });

                mutation_observer.replace(Some(
                    MutationObserver::new(handle_mutations.into_js_value().unchecked_ref())
                        .expect("Mutation observer should be created."),
                ));

                mutation_observer
                    .borrow()
                    .as_ref()
                    .expect("Mutation observer should exist.")
                    .observe_with_options(
                        &container,
                        MutationObserverInit::new().child_list(true).subtree(true),
                    )
                    .expect("Mutation observer should observe target.");
            }
        }
    });

    type AutoFocusEndFn = Box<dyn Fn()>;
    let auto_focus_end: Rc<RefCell<Option<AutoFocusEndFn>>> = Rc::new(RefCell::new(None));
    let cleanup_auto_focus_end = auto_focus_end.clone();

    create_effect(move |_| {
        if let Some(on_mount_auto_focus_cleanup) = auto_focus_end.take() {
            on_mount_auto_focus_cleanup();
        }

        if let Some(container) = container_ref.get() {
            {
                let mut focus_scope_stack = FOCUS_SCOPE_STACK
                    .lock()
                    .expect("Focus scope stack mutex should lock.");
                focus_scope_stack.add(focus_scope.get());
            }

            let previously_focused_element = document()
                .active_element()
                .map(|element| element.unchecked_into::<web_sys::HtmlElement>());
            let has_focused_candidate = container.contains(
                previously_focused_element
                    .as_ref()
                    .map(|element| element.unchecked_ref()),
            );

            if !has_focused_candidate {
                let on_mount_auto_focus_inner = on_mount_auto_focus.clone();
                let closure: Closure<dyn Fn(Event)> = Closure::new(move |event: Event| {
                    if let Some(on_mount_auto_focus) = on_mount_auto_focus_inner.get_untracked() {
                        on_mount_auto_focus(event);
                    }
                });

                let mount_event = CustomEvent::new_with_event_init_dict(
                    AUTOFOCUS_ON_MOUNT,
                    CustomEventInit::new().bubbles(false).cancelable(true),
                )
                .expect("Auto focus on mount event should be instantiated.");

                container
                    .add_event_listener_with_callback(
                        AUTOFOCUS_ON_MOUNT,
                        closure.as_ref().unchecked_ref(),
                    )
                    .expect("Auto focus on mount event listener should be added.");
                container
                    .dispatch_event(&mount_event)
                    .expect("Auto focus on mount event should be dispatched.");

                if !mount_event.default_prevented() {
                    focus_first(
                        remove_links(get_tabbable_candidates(&container)),
                        Some(FocusOptions { select: true }),
                    );
                    if document().active_element().as_ref() == previously_focused_element.as_deref()
                    {
                        focus(Some(container.deref().clone()), None);
                    }
                }

                let on_unmount_auto_focus = on_unmount_auto_focus.clone();
                auto_focus_end.replace(Some(Box::new(move || {
                    container
                        .remove_event_listener_with_callback(
                            AUTOFOCUS_ON_MOUNT,
                            closure.as_ref().unchecked_ref(),
                        )
                        .expect("Auto focus on mount event listener should be removed.");

                    let on_unmount_auto_focus_inner = on_unmount_auto_focus.clone();
                    let closure: Closure<dyn Fn(Event)> = Closure::new(move |event: Event| {
                        if let Some(on_unmount_auto_focus) =
                            on_unmount_auto_focus_inner.get_untracked()
                        {
                            on_unmount_auto_focus(event);
                        }
                    });

                    let unmount_event = CustomEvent::new_with_event_init_dict(
                        AUTOFOCUS_ON_UNMOUNT,
                        CustomEventInit::new().bubbles(false).cancelable(true),
                    )
                    .expect("Auto focus on unmount event should be instantiated.");

                    container
                        .add_event_listener_with_callback(
                            AUTOFOCUS_ON_UNMOUNT,
                            closure.as_ref().unchecked_ref(),
                        )
                        .expect("Auto focus on unmount event listener should be added.");
                    container
                        .dispatch_event(&unmount_event)
                        .expect("Auto focus on unmount event should be dispatched.");

                    if !unmount_event.default_prevented() {
                        focus(
                            previously_focused_element.clone().or(document().body()),
                            Some(FocusOptions { select: true }),
                        );
                    }

                    container
                        .remove_event_listener_with_callback(
                            AUTOFOCUS_ON_UNMOUNT,
                            closure.as_ref().unchecked_ref(),
                        )
                        .expect("Auto focus on unmount event listener should be removed.");

                    {
                        let mut focus_scope_stack = FOCUS_SCOPE_STACK
                            .lock()
                            .expect("Focus scope stack mutex should lock.");
                        focus_scope_stack.remove(&focus_scope.get_untracked());
                    }
                })));
            }
        }
    });

    on_cleanup(move || {
        document()
            .remove_event_listener_with_callback(
                "focusin",
                (*cleanup_handle_focus_in).as_ref().unchecked_ref(),
            )
            .expect("Focus in event listener should be removed.");
        document()
            .remove_event_listener_with_callback(
                "focusout",
                (*cleanup_handle_focus_out).as_ref().unchecked_ref(),
            )
            .expect("Focus out event listener should be removed.");

        if let Some(mutation_observer) = cleanup_mutation_observer.take() {
            mutation_observer.disconnect();
        }

        if let Some(auto_focus_cleanup) = cleanup_auto_focus_end.take() {
            auto_focus_cleanup();
        }
    });

    // Takes care of looping focus (when tabbing whilst at the edges).
    let handle_key_down = move |event: KeyboardEvent| {
        let r#loop = r#loop.get_untracked();

        if r#loop && !trapped.get_untracked() {
            return;
        }
        if focus_scope.get_untracked().paused() {
            return;
        }

        let is_tab_key =
            event.key() == "Tab" && !event.alt_key() && !event.ctrl_key() && !event.meta_key();
        let focused_element = document()
            .active_element()
            .map(|element| element.unchecked_into::<web_sys::HtmlElement>());

        if is_tab_key {
            if let Some(focused_element) = focused_element {
                let container = event
                    .current_target()
                    .expect("Event should have current target.")
                    .unchecked_into::<web_sys::HtmlElement>();
                let (first, last) = get_tabbable_edges(&container);
                let has_tabbable_elements_inside = first.is_some() && last.is_some();

                if !has_tabbable_elements_inside {
                    if focused_element == container {
                        event.prevent_default();
                    }
                } else {
                    #[allow(clippy::collapsible_else_if)]
                    if !event.shift_key()
                        && &focused_element == last.as_ref().expect("Last option checked above.")
                    {
                        event.prevent_default();

                        if r#loop {
                            focus(first, Some(FocusOptions { select: true }));
                        }
                    } else if event.shift_key()
                        && &focused_element == first.as_ref().expect("First option checked above.")
                    {
                        event.prevent_default();

                        if r#loop {
                            focus(last, Some(FocusOptions { select: true }));
                        }
                    }
                }
            }
        }
    };

    let mut attrs = attrs.clone();
    attrs.extend(vec![("tabindex", "-1".into_attribute())]);

    view! {
        <Primitive
            element=html::div
            as_child=as_child
            on:keydown=handle_key_down
            node_ref=container_ref
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

fn remove_links(items: Vec<web_sys::HtmlElement>) -> Vec<web_sys::HtmlElement> {
    items
        .into_iter()
        .filter(|item| item.tag_name() != "A")
        .collect()
}

static FOCUS_SCOPE_STACK: Lazy<Mutex<FocusScopeStack>> =
    Lazy::new(|| Mutex::new(FocusScopeStack::new()));

#[derive(Clone, Debug)]
struct FocusScopeAPI {
    id: u64,
    paused: Arc<AtomicBool>,
}

impl FocusScopeAPI {
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self {
            id: COUNTER.fetch_add(1, Ordering::Relaxed),
            paused: Arc::new(AtomicBool::new(false)),
        }
    }

    fn paused(&self) -> bool {
        self.paused.load(Ordering::Relaxed)
    }

    fn pause(&mut self) {
        self.paused.store(true, Ordering::Relaxed)
    }

    fn resume(&mut self) {
        self.paused.store(false, Ordering::Relaxed);
    }
}

impl PartialEq for FocusScopeAPI {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// A stack of focus scopes, with the active one at the top.
#[derive(Clone, Debug, PartialEq)]
struct FocusScopeStack {
    stack: Vec<FocusScopeAPI>,
}

impl FocusScopeStack {
    fn new() -> Self {
        Self { stack: vec![] }
    }

    fn add(&mut self, focus_scope: FocusScopeAPI) {
        // Pause the currently active focus scope (at the top of the stack).
        if let Some(active_focus_scope) = self.stack.first_mut() {
            if focus_scope != *active_focus_scope {
                active_focus_scope.pause();
            }
        }

        // Remove in case it already exists (because we'll re-add it at the top of the stack).
        self.remove_without_resume(&focus_scope);
        self.stack.insert(0, focus_scope);

        // This is not in the React implementation, but without the unit tests could never pass.
        if let Some(first_focus_scope) = self.stack.first_mut() {
            first_focus_scope.resume();
        }
    }

    fn remove(&mut self, focus_scope: &FocusScopeAPI) {
        self.remove_without_resume(focus_scope);

        if let Some(first_focus_scope) = self.stack.first_mut() {
            first_focus_scope.resume();
        }
    }

    fn remove_without_resume(&mut self, focus_scope: &FocusScopeAPI) {
        let index = self.stack.iter().position(|f| f == focus_scope);

        if let Some(index) = index {
            self.stack.remove(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focus_scope_api() {
        let mut a = FocusScopeAPI::new();
        let mut b = a.clone();

        assert!(!a.paused());
        assert!(!b.paused());

        a.pause();
        assert!(a.paused());
        assert!(b.paused());

        a.resume();
        assert!(!a.paused());
        assert!(!b.paused());

        b.pause();
        assert!(a.paused());
        assert!(b.paused());

        b.resume();
        assert!(!a.paused());
        assert!(!b.paused());
    }

    #[test]
    fn test_focus_scope_stack() {
        let mut stack = FocusScopeStack::new();

        let a = FocusScopeAPI::new();
        let b = FocusScopeAPI::new();
        let c = FocusScopeAPI::new();

        stack.add(a.clone());
        assert_eq!(vec![a.clone()], stack.stack);
        assert!(!stack.stack[0].paused());

        stack.add(b.clone());
        assert_eq!(vec![b.clone(), a.clone()], stack.stack);
        assert!(!stack.stack[0].paused());
        assert!(stack.stack[1].paused());

        stack.add(c.clone());
        assert_eq!(vec![c.clone(), b.clone(), a.clone()], stack.stack);
        assert!(!stack.stack[0].paused());
        assert!(stack.stack[1].paused());
        assert!(stack.stack[2].paused());

        stack.add(b.clone());
        assert_eq!(vec![b.clone(), c.clone(), a.clone()], stack.stack);
        assert!(!stack.stack[0].paused());
        assert!(stack.stack[1].paused());
        assert!(stack.stack[2].paused());

        stack.remove(&c);
        assert_eq!(vec![b.clone(), a.clone()], stack.stack);
        assert!(!stack.stack[0].paused());
        assert!(stack.stack[1].paused());

        stack.remove(&c);
        assert_eq!(vec![b.clone(), a.clone()], stack.stack);
        assert!(!stack.stack[0].paused());
        assert!(stack.stack[1].paused());

        stack.remove(&b);
        assert_eq!(vec![a.clone()], stack.stack);
        assert!(!stack.stack[0].paused());

        stack.remove(&a);
        assert!(stack.stack.is_empty());
    }
}
