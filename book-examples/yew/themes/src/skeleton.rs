#[allow(clippy::module_inception)]
mod skeleton;
mod skeleton_size;
mod skeleton_with_children;
mod skeleton_with_text;
mod skeleton_with_text_paragraph;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum SkeletonRoute {
    #[at("/")]
    Root,
    #[at("/size")]
    Size,
    #[at("/with-children")]
    WithChildren,
    #[at("/with-text")]
    WithText,
    #[at("/with-text-paragraph")]
    WithTextParagraph,
}

pub fn render(route: SkeletonRoute) -> Html {
    match route {
        SkeletonRoute::Root => html! { <skeleton::SkeletonExample /> },
        SkeletonRoute::Size => html! { <skeleton_size::SkeletonSizeExample /> },
        SkeletonRoute::WithChildren => {
            html! { <skeleton_with_children::SkeletonWithChildrenExample /> }
        }
        SkeletonRoute::WithText => html! { <skeleton_with_text::SkeletonWithTextExample /> },
        SkeletonRoute::WithTextParagraph => {
            html! { <skeleton_with_text_paragraph::SkeletonWithTextParagraphExample /> }
        }
    }
}
