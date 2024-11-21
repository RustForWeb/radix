use radix_yew_themes::Skeleton;
use yew::prelude::*;

#[function_component]
pub fn SkeletonSizeExample() -> Html {
    html! {
        <Skeleton width="48px" height="48px" />
    }
}
