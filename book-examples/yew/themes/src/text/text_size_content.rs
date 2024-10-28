use radix_yew_themes::{AccentColor, Text, TextAs};
use yew::prelude::*;

#[function_component]
pub fn TextSizeContentExample() -> Html {
    html! {
        <>
            <Text r#as={TextAs::P} mb=5 size=4>
                {"The goal of typography is to relate font size, line height, and line width in a proportional way that maximizes beauty and makes reading easier and more pleasant. The question is: What proportion(s) will give us the best results? The golden ratio is often observed in nature where beauty and utility intersect; perhaps we can use this “divine” proportion to enhance these attributes in our typography."}
            </Text>

            <Text r#as={TextAs::P} mb=5 size=3>
                {"The goal of typography is to relate font size, line height, and line width in a proportional way that maximizes beauty and makes reading easier and more pleasant. The question is: What proportion(s) will give us the best results? The golden ratio is often observed in nature where beauty and utility intersect; perhaps we can use this “divine” proportion to enhance these attributes in our typography."}
            </Text>

            <Text r#as={TextAs::P} size=2 color={AccentColor::Gray}>
                {"The goal of typography is to relate font size, line height, and line width in a proportional way that maximizes beauty and makes reading easier and more pleasant. The question is: What proportion(s) will give us the best results? The golden ratio is often observed in nature where beauty and utility intersect; perhaps we can use this “divine” proportion to enhance these attributes in our typography."}
            </Text>
        </>
    }
}
