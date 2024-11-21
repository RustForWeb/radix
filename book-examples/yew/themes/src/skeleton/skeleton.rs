use radix_yew_themes::Skeleton;
use yew::prelude::*;

#[function_component]
pub fn SkeletonExample() -> Html {
    html! {
        <Skeleton>{"Loading"}</Skeleton>
    }
}
