use leptos::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Ltr,
    Rtl,
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
        local_dir()
            .or(global_dir.map(|global_dir| global_dir()))
            .unwrap_or(Direction::Ltr)
    })
}
