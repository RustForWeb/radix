use leptos::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Ltr,
    Rtl,
}

impl From<Direction> for String {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Ltr => "ltr".into(),
            Direction::Rtl => "rtl".into(),
        }
    }
}

impl IntoAttribute for Direction {
    fn into_attribute(self) -> Attribute {
        let s: String = self.into();
        Attribute::String(s.into())
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
