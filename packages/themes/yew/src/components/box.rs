use yew::prelude::*;

use crate::{
    components::box_props::{BoxAs, BoxAsProp, BoxDisplayProp},
    helpers::{extract_props::extract_props, merge_classes::merge_classes, merge_styles::Style},
    props::{
        height_props::{HeightProp, MaxHeightProp, MinHeightProp},
        layout_props::{
            BottomProp, FlexBasisProp, FlexGrowProp, FlexShrinkProp, GridAreaProp,
            GridColumnEndProp, GridColumnProp, GridColumnStartProp, GridRowEndProp, GridRowProp,
            GridRowStartProp, InsetProp, LeftProp, OverflowProp, OverflowXProp, OverflowYProp,
            PositionProp, RightProp, TopProp,
        },
        margin_props::{MProp, MbProp, MlProp, MrProp, MtProp, MxProp, MyProp},
        padding_props::{PProp, PbProp, PlProp, PrProp, PtProp, PxProp, PyProp},
        width_props::{MaxWidthProp, MinWidthProp, WidthProp},
    },
};

#[derive(PartialEq, Properties)]
pub struct BoxProps {
    #[prop_or_default]
    pub r#as: BoxAsProp,
    #[prop_or_default]
    pub display: BoxDisplayProp,
    #[prop_or_default]
    pub p: PProp,
    #[prop_or_default]
    pub px: PxProp,
    #[prop_or_default]
    pub py: PyProp,
    #[prop_or_default]
    pub pr: PrProp,
    #[prop_or_default]
    pub pt: PtProp,
    #[prop_or_default]
    pub pb: PbProp,
    #[prop_or_default]
    pub pl: PlProp,
    #[prop_or_default]
    pub width: WidthProp,
    #[prop_or_default]
    pub min_width: MinWidthProp,
    #[prop_or_default]
    pub max_width: MaxWidthProp,
    #[prop_or_default]
    pub height: HeightProp,
    #[prop_or_default]
    pub min_height: MinHeightProp,
    #[prop_or_default]
    pub max_height: MaxHeightProp,
    #[prop_or_default]
    pub position: PositionProp,
    #[prop_or_default]
    pub inset: InsetProp,
    #[prop_or_default]
    pub top: TopProp,
    #[prop_or_default]
    pub right: RightProp,
    #[prop_or_default]
    pub bottom: BottomProp,
    #[prop_or_default]
    pub left: LeftProp,
    #[prop_or_default]
    pub overflow: OverflowProp,
    #[prop_or_default]
    pub overflow_x: OverflowXProp,
    #[prop_or_default]
    pub overflow_y: OverflowYProp,
    #[prop_or_default]
    pub flex_basis: FlexBasisProp,
    #[prop_or_default]
    pub flex_shrink: FlexShrinkProp,
    #[prop_or_default]
    pub flex_grow: FlexGrowProp,
    #[prop_or_default]
    pub grid_area: GridAreaProp,
    #[prop_or_default]
    pub grid_column: GridColumnProp,
    #[prop_or_default]
    pub grid_column_start: GridColumnStartProp,
    #[prop_or_default]
    pub grid_column_end: GridColumnEndProp,
    #[prop_or_default]
    pub grid_row: GridRowProp,
    #[prop_or_default]
    pub grid_row_start: GridRowStartProp,
    #[prop_or_default]
    pub grid_row_end: GridRowEndProp,
    #[prop_or_default]
    pub m: MProp,
    #[prop_or_default]
    pub mx: MxProp,
    #[prop_or_default]
    pub my: MyProp,
    #[prop_or_default]
    pub mr: MrProp,
    #[prop_or_default]
    pub mt: MtProp,
    #[prop_or_default]
    pub mb: MbProp,
    #[prop_or_default]
    pub ml: MlProp,

    #[prop_or_default]
    pub node_ref: NodeRef,
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Style,
    #[prop_or_default]
    pub as_child: Option<Callback<BoxChildProps, Html>>,
    #[prop_or_default]
    pub children: Html,
}

#[derive(Clone, PartialEq)]
pub struct BoxChildProps {
    pub node_ref: NodeRef,
    pub id: Option<String>,
    pub class: String,
    pub style: Style,
    pub r#as: BoxAs,
}

impl BoxChildProps {
    pub fn render(self, children: Html) -> Html {
        html! {
            if self.r#as == BoxAs::Div {
                <div
                    ref={self.node_ref}
                    id={self.id}
                    class={self.class}
                    style={self.style.to_string()}
                >
                    {children}
                </div>
            } else {
                <span
                    ref={self.node_ref}
                    id={self.id}
                    class={self.class}
                    style={self.style.to_string()}
                >
                    {children}
                </span>
            }
        }
    }
}

#[function_component]
pub fn Box(props: &BoxProps) -> Html {
    let (class, style) = extract_props(
        &[
            &props.r#as,
            &props.display,
            &props.p,
            &props.px,
            &props.py,
            &props.pt,
            &props.pr,
            &props.pb,
            &props.pl,
            &props.width,
            &props.min_width,
            &props.max_width,
            &props.height,
            &props.min_height,
            &props.max_height,
            &props.position,
            &props.inset,
            &props.top,
            &props.right,
            &props.bottom,
            &props.left,
            &props.overflow,
            &props.overflow_x,
            &props.overflow_y,
            &props.flex_basis,
            &props.flex_shrink,
            &props.flex_grow,
            &props.grid_area,
            &props.grid_column,
            &props.grid_column_start,
            &props.grid_column_end,
            &props.grid_row,
            &props.grid_row_start,
            &props.grid_row_end,
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

    let child_props = BoxChildProps {
        node_ref: props.node_ref.clone(),
        id: props.id.clone(),
        class: merge_classes(&[&"rt-Box", &class]),
        style,
        r#as: props.r#as.0,
    };

    if let Some(as_child) = props.as_child.as_ref() {
        as_child.emit(child_props)
    } else {
        child_props.render(props.children.clone())
    }
}
