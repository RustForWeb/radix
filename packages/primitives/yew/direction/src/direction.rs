use std::fmt::{Display, Formatter};

use yew::prelude::*;

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

type DirectionContextValue = Direction;

#[derive(PartialEq, Properties)]
pub struct DirectionProviderProps {
    direction: Direction,
    #[prop_or_default]
    children: Html,
}

#[function_component]
pub fn DirectionProvider(props: &DirectionProviderProps) -> Html {
    html! {
        <ContextProvider<DirectionContextValue> context={props.direction}>{props.children.clone()}</ContextProvider<DirectionContextValue>>
    }
}

#[hook]
pub fn use_direction(local_dir: Option<Direction>) -> Direction {
    let global_dir = use_context::<DirectionContextValue>();

    local_dir.or(global_dir).unwrap_or(Direction::Ltr)
}
