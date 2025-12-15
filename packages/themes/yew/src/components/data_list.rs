use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent};
use yew_style::Style;

use crate::{
    components::{
        data_list_props::{
            DataListItemAlignProp, DataListLeadingTrimProp, DataListOrientationProp,
            DataListSizeProp,
        },
        text::{Text, TextChildProps},
    },
    helpers::extract_props::extract_props,
    props::{
        color_prop::ColorProp,
        high_contrast_prop::HighContrastProp,
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        width_props::{MaxWidthProp, MinWidthProp, WidthProp},
    },
};

#[derive(PartialEq, Properties)]
pub struct DataListProps {
    #[prop_or_default]
    pub orientation: DataListOrientationProp,
    #[prop_or_default]
    pub size: DataListSizeProp,
    #[prop_or_default]
    pub trim: DataListLeadingTrimProp,
    #[prop_or_default]
    pub m: MProp,
    #[prop_or_default]
    pub mx: MxProp,
    #[prop_or_default]
    pub my: MyProp,
    #[prop_or_default]
    pub mt: MtProp,
    #[prop_or_default]
    pub mr: MrProp,
    #[prop_or_default]
    pub mb: MbProp,
    #[prop_or_default]
    pub ml: MlProp,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "dl")]
pub struct DataListChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub data_accent_color: Option<String>,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn DataList(props: &DataListProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.orientation,
            &props.size,
            &props.trim,
            &props.m,
            &props.mx,
            &props.my,
            &props.mt,
            &props.mr,
            &props.mb,
            &props.ml,
        ],
        props.class.clone(),
        props.style.clone().into(),
    );

    html! {
        <Text
            class={class.to_string()}
            id={props.id.clone()}
            style={style}

            node_ref={props.node_ref.clone()}
            attributes={props.attributes.clone()}
            as_child={{
                let children = props.children.clone();

                move |TextChildProps {
                    node_ref,
                    attributes,

                    class,
                    data_accent_color,
                    id,
                    style,

                    ..
                } | {
                    let child_props = DataListChildProps {
                        node_ref,
                        attributes,

                        // Global attributes
                        class: classes!("rt-DataListRoot", class).to_string(),
                        data_accent_color,
                        id,
                        style,
                    };

                    child_props.render(children.clone())
                }
            }}
        />
    }
}

#[derive(PartialEq, Properties)]
pub struct DataListItemProps {
    #[prop_or_default]
    pub align: DataListItemAlignProp,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "div")]
pub struct DataListItemChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn DataListItem(props: &DataListItemProps) -> Html {
    let (class, style) = extract_props(
        &[&props.align],
        props.class.clone(),
        props.style.clone().into(),
    );

    let child_props = DataListItemChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-DataListItem", class).to_string(),
        id: props.id.clone(),
        style,
    };

    child_props.render(props.children.clone())
}

#[derive(PartialEq, Properties)]
pub struct DataListLabelProps {
    #[prop_or_default]
    pub width: WidthProp,
    #[prop_or_default]
    pub min_width: MinWidthProp,
    #[prop_or_default]
    pub max_width: MaxWidthProp,
    #[prop_or_default]
    pub color: ColorProp,
    #[prop_or_default]
    pub high_contrast: HighContrastProp,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "dt")]
pub struct DataListLabelChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub data_accent_color: Option<String>,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn DataListLabel(props: &DataListLabelProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.width,
            &props.min_width,
            &props.max_width,
            &props.color,
            &props.high_contrast,
        ],
        props.class.clone(),
        props.style.clone().into(),
    );

    let child_props = DataListLabelChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-DataListLabel", class).to_string(),
        data_accent_color: props.color.0.map(|color| color.to_string()),
        id: props.id.clone(),
        style,
    };

    child_props.render(props.children.clone())
}

#[derive(PartialEq, Properties)]
pub struct DataListValueProps {
    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "dd")]
pub struct DataListValueChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn DataListValue(props: &DataListValueProps) -> Html {
    let child_props = DataListValueChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-DataListValue", &props.class).to_string(),
        id: props.id.clone(),
        style: props.style.clone(),
    };

    child_props.render(props.children.clone())
}
