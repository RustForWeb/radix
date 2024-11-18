use radix_yew_themes::Box;
use yew::prelude::*;
use yew_style::Style;

#[derive(PartialEq, Properties)]
pub struct DecorativeBoxProps {
    #[prop_or_default]
    pub as_child: Option<Callback<DecorativeBoxChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq)]
pub struct DecorativeBoxChildProps {
    pub height: String,
    pub style: Style,
}

impl DecorativeBoxChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            <Box
                height={self.height}
                style={self.style}
            >
                {children}
            </Box>
        }
    }
}

#[function_component]
pub fn DecorativeBox(props: &DecorativeBoxProps) -> Html {
    let child_props = DecorativeBoxChildProps {
        height: "100%".into(),
        style: Style::from([
            ("background-color", "var(--gray-a3)"),
            ("background-clip", "padding-box"),
            ("border", "1px solid var(--gray-a5)"),
            ("border-radius", "var(--radius-1)"),
            ("background-image", "url(\"data:image/svg+xml,%3Csvg width='6' height='6' viewBox='0 0 6 6' xmlns='http://www.w3.org/2000/svg'%3E%3Cg fill='%239C92AC' fill-opacity='0.2' fill-rule='evenodd'%3E%3Cpath d='M5 0h1L0 6V5zM6 5v1H5z'/%3E%3C/g%3E%3C/svg%3E\")"),
        ])
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
