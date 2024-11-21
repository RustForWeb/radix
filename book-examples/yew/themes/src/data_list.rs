#[allow(clippy::module_inception)]
mod data_list;
mod data_list_color;
mod data_list_high_contrast;
mod data_list_orientation;
mod data_list_size;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
pub enum DataListRoute {
    #[at("/")]
    Root,
    #[at("/color")]
    Color,
    #[at("/high-contrast")]
    HighContrast,
    #[at("/orientation")]
    Orientation,
    #[at("/size")]
    Size,
}

pub fn render(route: DataListRoute) -> Html {
    match route {
        DataListRoute::Root => html! { <data_list::DataListExample /> },
        DataListRoute::Color => html! { <data_list_color::DataListColorExample /> },
        DataListRoute::HighContrast => {
            html! { <data_list_high_contrast::DataListHighContrastExample /> }
        }
        DataListRoute::Orientation => {
            html! { <data_list_orientation::DataListOrientationExample /> }
        }
        DataListRoute::Size => html! { <data_list_size::DataListSizeExample /> },
    }
}
