use std::fmt::{Display, Formatter};

use leptos::*;

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

impl IntoAttribute for Direction {
    fn into_attribute(self) -> Attribute {
        Attribute::String(self.to_string().into())
    }

    fn into_attribute_boxed(self: Box<Self>) -> Attribute {
        self.into_attribute()
    }
}

type DirectionContextValue = MaybeSignal<Direction>;

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
