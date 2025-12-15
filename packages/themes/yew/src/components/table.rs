use yew::prelude::*;
use yew_struct_component::{Attributes, StructComponent};
use yew_style::Style;

use crate::{
    components::table_props::{
        TableCellJustifyProp, TableLayoutProp, TableRowAlignProp, TableSizeProp, TableVariantProp,
    },
    helpers::extract_props::extract_props,
    props::{
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        padding_props::{PProp, PbProp, PlProp, PrProp, PtProp, PxProp, PyProp},
        width_props::{MaxWidthProp, MinWidthProp, WidthProp},
    },
};

#[derive(PartialEq, Properties)]
pub struct TableProps {
    #[prop_or_default]
    pub size: TableSizeProp,
    #[prop_or_default]
    pub variant: TableVariantProp,
    #[prop_or_default]
    pub layout: TableLayoutProp,
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
#[struct_component(tag = "div")]
pub struct TableChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn Table(props: &TableProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.size,
            &props.variant,
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

    let (table_layout_class, table_layout_style) = extract_props(&[&props.layout], None, None);

    let child_props = TableChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-TableRoot", class).to_string(),
        id: props.id.clone(),
        style,
    };

    child_props.render(html! {
        // TODO: ScrollArea
        <table class={classes!("rt-TableRootTable", table_layout_class)} style={table_layout_style}>
            {props.children.clone()}
        </table>
    })
}

#[derive(PartialEq, Properties)]
pub struct TableHeaderProps {
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
#[struct_component(tag = "thead")]
pub struct TableHeaderChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn TableHeader(props: &TableHeaderProps) -> Html {
    let child_props = TableHeaderChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-TableHeader", &props.class).to_string(),
        id: props.id.clone(),
        style: props.style.clone(),
    };

    child_props.render(props.children.clone())
}

#[derive(PartialEq, Properties)]
pub struct TableBodyProps {
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
#[struct_component(tag = "tbody")]
pub struct TableBodyChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn TableBody(props: &TableBodyProps) -> Html {
    let child_props = TableBodyChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-TableBody", &props.class).to_string(),
        id: props.id.clone(),
        style: props.style.clone(),
    };

    child_props.render(props.children.clone())
}

#[derive(PartialEq, Properties)]
pub struct TableRowProps {
    #[prop_or_default]
    pub align: TableRowAlignProp,

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
#[struct_component(tag = "tr")]
pub struct TableRowChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub id: Option<String>,
    pub style: Style,
}

#[function_component]
pub fn TableRow(props: &TableRowProps) -> Html {
    let (class, style) = extract_props(
        &[&props.align],
        props.class.clone(),
        props.style.clone().into(),
    );

    let child_props = TableRowChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-TableRow", class).to_string(),
        id: props.id.clone(),
        style,
    };

    child_props.render(props.children.clone())
}

#[derive(PartialEq, Properties)]
pub struct TableCellProps {
    #[prop_or_default]
    pub justify: TableCellJustifyProp,
    #[prop_or_default]
    pub width: WidthProp,
    #[prop_or_default]
    pub min_width: MinWidthProp,
    #[prop_or_default]
    pub max_width: MaxWidthProp,
    #[prop_or_default]
    pub p: PProp,
    #[prop_or_default]
    pub px: PxProp,
    #[prop_or_default]
    pub py: PyProp,
    #[prop_or_default]
    pub pt: PtProp,
    #[prop_or_default]
    pub pr: PrProp,
    #[prop_or_default]
    pub pb: PbProp,
    #[prop_or_default]
    pub pl: PlProp,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `td`
    #[prop_or_default]
    pub colspan: Option<String>,
    #[prop_or_default]
    pub headers: Option<String>,
    #[prop_or_default]
    pub rowspan: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "td")]
pub struct TableCellChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub id: Option<String>,
    pub style: Style,

    // Attributes from `td`
    pub colspan: Option<String>,
    pub headers: Option<String>,
    pub rowspan: Option<String>,
}

#[function_component]
pub fn TableCell(props: &TableCellProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.justify,
            &props.width,
            &props.min_width,
            &props.max_width,
            &props.p,
            &props.px,
            &props.py,
            &props.pt,
            &props.pr,
            &props.pb,
            &props.pl,
        ],
        props.class.clone(),
        props.style.clone().into(),
    );

    let child_props = TableCellChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-TableCell", class).to_string(),
        id: props.id.clone(),
        style,

        // Attributes from `td`
        colspan: props.colspan.clone(),
        headers: props.headers.clone(),
        rowspan: props.rowspan.clone(),
    };

    child_props.render(props.children.clone())
}

#[derive(PartialEq, Properties)]
pub struct TableColumnHeaderCellProps {
    #[prop_or_default]
    pub justify: TableCellJustifyProp,
    #[prop_or_default]
    pub width: WidthProp,
    #[prop_or_default]
    pub min_width: MinWidthProp,
    #[prop_or_default]
    pub max_width: MaxWidthProp,
    #[prop_or_default]
    pub p: PProp,
    #[prop_or_default]
    pub px: PxProp,
    #[prop_or_default]
    pub py: PyProp,
    #[prop_or_default]
    pub pt: PtProp,
    #[prop_or_default]
    pub pr: PrProp,
    #[prop_or_default]
    pub pb: PbProp,
    #[prop_or_default]
    pub pl: PlProp,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `th`
    #[prop_or_default]
    pub abbr: Option<String>,
    #[prop_or_default]
    pub colspan: Option<String>,
    #[prop_or_default]
    pub headers: Option<String>,
    #[prop_or_default]
    pub rowspan: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "th")]
pub struct TableColumnHeaderCellChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub id: Option<String>,
    pub style: Style,

    // Attributes from `th`
    pub abbr: Option<String>,
    pub colspan: Option<String>,
    pub headers: Option<String>,
    pub rowspan: Option<String>,
    pub scope: String,
}

#[function_component]
pub fn TableColumnHeaderCell(props: &TableColumnHeaderCellProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.justify,
            &props.width,
            &props.min_width,
            &props.max_width,
            &props.p,
            &props.px,
            &props.py,
            &props.pt,
            &props.pr,
            &props.pb,
            &props.pl,
        ],
        props.class.clone(),
        props.style.clone().into(),
    );

    let child_props = TableColumnHeaderCellChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Globla attributes
        class: classes!("rt-TableCell", "rt-TableColumnHeaderCell", class).to_string(),
        id: props.id.clone(),
        style,

        // Attributes from `th`
        abbr: props.abbr.clone(),
        colspan: props.colspan.clone(),
        headers: props.headers.clone(),
        rowspan: props.rowspan.clone(),
        scope: "col".to_owned(),
    };

    child_props.render(props.children.clone())
}

#[derive(PartialEq, Properties)]
pub struct TableRowHeaderCellProps {
    #[prop_or_default]
    pub justify: TableCellJustifyProp,
    #[prop_or_default]
    pub width: WidthProp,
    #[prop_or_default]
    pub min_width: MinWidthProp,
    #[prop_or_default]
    pub max_width: MaxWidthProp,
    #[prop_or_default]
    pub p: PProp,
    #[prop_or_default]
    pub px: PxProp,
    #[prop_or_default]
    pub py: PyProp,
    #[prop_or_default]
    pub pt: PtProp,
    #[prop_or_default]
    pub pr: PrProp,
    #[prop_or_default]
    pub pb: PbProp,
    #[prop_or_default]
    pub pl: PlProp,

    // Global attributes
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub style: Style,

    // Attributes from `th`
    #[prop_or_default]
    pub abbr: Option<String>,
    #[prop_or_default]
    pub colspan: Option<String>,
    #[prop_or_default]
    pub headers: Option<String>,
    #[prop_or_default]
    pub rowspan: Option<String>,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub attributes: Attributes,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq, StructComponent)]
#[struct_component(tag = "th")]
pub struct TableRowHeaderCellChildProps {
    pub node_ref: NodeRef,
    pub attributes: Attributes,

    // Global attributes
    pub class: String,
    pub id: Option<String>,
    pub style: Style,

    // Attributes from `th`
    pub abbr: Option<String>,
    pub colspan: Option<String>,
    pub headers: Option<String>,
    pub rowspan: Option<String>,
    pub scope: String,
}

#[function_component]
pub fn TableRowHeaderCell(props: &TableRowHeaderCellProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.justify,
            &props.width,
            &props.min_width,
            &props.max_width,
            &props.p,
            &props.px,
            &props.py,
            &props.pt,
            &props.pr,
            &props.pb,
            &props.pl,
        ],
        props.class.clone(),
        props.style.clone().into(),
    );

    let child_props = TableRowHeaderCellChildProps {
        node_ref: props.node_ref.clone(),
        attributes: props.attributes.clone(),

        // Global attributes
        class: classes!("rt-TableCell", "rt-TableRowHeaderCell", class).to_string(),
        id: props.id.clone(),
        style,

        // Attributes from `th`
        abbr: props.abbr.clone(),
        colspan: props.colspan.clone(),
        headers: props.headers.clone(),
        rowspan: props.rowspan.clone(),
        scope: "row".to_owned(),
    };

    child_props.render(props.children.clone())
}
