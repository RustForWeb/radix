use std::fmt::{Display, Formatter};

use leptos::{attr::AttributeValue, context::Provider, prelude::*, tachys::renderer::Rndr};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Ltr,
    Rtl,
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Ltr => "ltr",
                Direction::Rtl => "rtl",
            }
        )
    }
}

impl AttributeValue for Direction {
    type AsyncOutput = Self;
    type State = (leptos::tachys::renderer::types::Element, Direction);
    type Cloneable = Direction;
    type CloneableOwned = Direction;

    fn html_len(&self) -> usize {
        self.to_string().len()
    }

    fn to_html(self, key: &str, buf: &mut String) {
        <&str as AttributeValue>::to_html(self.to_string().as_str(), key, buf);
    }

    fn to_template(_key: &str, _buf: &mut String) {}

    fn hydrate<const FROM_SERVER: bool>(
        self,
        key: &str,
        el: &leptos::tachys::renderer::types::Element,
    ) -> Self::State {
        let (el, _) =
            <&str as AttributeValue>::hydrate::<FROM_SERVER>(self.to_string().as_str(), key, el);
        (el, self)
    }

    fn build(self, el: &leptos::tachys::renderer::types::Element, key: &str) -> Self::State {
        Rndr::set_attribute(el, key, &self.to_string());
        (el.clone(), self)
    }

    fn rebuild(self, key: &str, state: &mut Self::State) {
        let (el, prev_value) = state;
        if self != *prev_value {
            Rndr::set_attribute(el, key, &self.to_string());
        }
        *prev_value = self;
    }

    fn into_cloneable(self) -> Self::Cloneable {
        self
    }

    fn into_cloneable_owned(self) -> Self::CloneableOwned {
        self
    }

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }
}

type DirectionContextValue = Signal<Direction>;

#[component]
pub fn DirectionProvider(
    #[prop(into)] direction: DirectionContextValue,
    children: Children,
) -> impl IntoView {
    view! {
        <Provider value={direction}>{children()}</Provider>
    }
}

pub fn use_direction(local_dir: MaybeProp<Direction>) -> Signal<Direction> {
    let global_dir = use_context::<DirectionContextValue>();

    Signal::derive(move || {
        local_dir
            .get()
            .or(global_dir.map(|global_dir| global_dir.get()))
            .unwrap_or(Direction::Ltr)
    })
}
