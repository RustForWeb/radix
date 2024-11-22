use yew::prelude::*;
use yew_router::prelude::*;

use crate::primitives::{
    arrow, aspect_ratio, avatar, checkbox, collection, dialog, focus_scope, label, popper, portal,
    presence, select, separator, switch, tooltip, visually_hidden,
};

#[derive(Clone, Copy, Debug, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Index,

    #[at("/arrow/styled")]
    ArrowStyled,
    #[at("/arrow/custom-sizes")]
    ArrowCustomSizes,
    #[at("/arrow/custom-arrow")]
    ArrowCustomArrow,

    #[at("/aspect-ratio/styled")]
    AspectRatioStyled,
    #[at("/aspect-ratio/custom-ratios")]
    AspectRatioCustomRatios,
    #[at("/aspect-ratio/chromatic")]
    AspectRatioChromatic,

    #[at("/avatar/styled")]
    AvatarStyled,
    #[at("/avatar/chromatic")]
    AvatarChromatic,

    #[at("/checkbox/styled")]
    CheckboxStyled,
    #[at("/checkbox/controlled")]
    CheckboxControlled,
    #[at("/checkbox/indeterminate")]
    CheckboxIndeterminate,
    #[at("/checkbox/within-form")]
    CheckboxWithinForm,
    #[at("/checkbox/animated")]
    CheckboxAnimated,
    #[at("/checkbox/chromatic")]
    CheckboxChromatic,

    #[at("/collection/basic")]
    CollectionBasic,
    #[at("/collection/with-elements-in-between")]
    CollectionWithElementsInBetween,
    #[at("/collection/with-wrapped-item")]
    CollectionWithWrappedItem,
    #[at("/collection/with-fragment")]
    CollectionWithFragment,
    #[at("/collection/dynamic-insertion")]
    CollectionDynamicInsertion,
    #[at("/collection/with-changing-item")]
    CollectionWithChangingItem,
    #[at("/collection/nested")]
    CollectionNested,

    #[at("/dialog/styled")]
    DialogStyled,
    #[at("/dialog/non-modal")]
    DialogNonModal,
    #[at("/dialog/controlled")]
    DialogControlled,
    #[at("/dialog/focus-trap")]
    DialogFocusTrap,
    #[at("/dialog/custom-focus")]
    DialogCustomFocus,
    #[at("/dialog/no-escape-dismiss")]
    DialogNoEscapeDismiss,
    #[at("/dialog/no-pointer-down-outside-dismiss")]
    DialogNoPointerDownOutsideDismiss,
    #[at("/dialog/with-portal-container")]
    DialogWithPortalContainer,
    #[at("/dialog/animated")]
    DialogAnimated,
    #[at("/dialog/forced-mount")]
    DialogForcedMount,
    #[at("/dialog/inner-scrollable")]
    DialogInnerScrollable,
    #[at("/dialog/outer-scrollable")]
    DialogOuterScrollable,
    #[at("/dialog/chromatic")]
    DialogChromatic,
    #[at("/dialog/cypress")]
    DialogCypress,

    #[at("/focus-scope/basic")]
    FocusScopeBasic,
    #[at("/focus-scope/multiple")]
    FocusScopeMultiple,
    #[at("/focus-scope/with-options")]
    FocusScopeWithOptions,

    #[at("/label/styled")]
    LabelStyled,
    #[at("/label/with-control")]
    LabelWithControl,
    #[at("/label/with-input-number")]
    LabelWithInputNumber,

    #[at("/popper/styled")]
    PopperStyled,
    #[at("/popper/with-custom-arrow")]
    PopperWithCustomArrow,
    #[at("/popper/animated")]
    PopperAnimated,
    #[at("/popper/with-portal")]
    PopperWithPortal,
    #[at("/popper/with-update-position-strategy-always")]
    PopperWithUpdatePositionStrategyAlways,
    #[at("/popper/chromatic")]
    PopperChromatic,

    #[at("/portal/base")]
    PortalBase,
    #[at("/portal/custom-container")]
    PortalCustomContainer,
    #[at("/portal/chromatic")]
    PortalChromatic,

    #[at("/presence/basic")]
    PresenceBasic,
    #[at("/presence/with-mount-animation")]
    PresenceWithMountAnimation,
    #[at("/presence/with-unmount-animation")]
    PresenceWithUnmountAnimation,
    #[at("/presence/with-multiple-mount-animations")]
    PresenceWithMultipleMountAnimations,
    #[at("/presence/with-open-and-close-animation")]
    PresenceWithOpenAndCloseAnimation,
    #[at("/presence/with-multiple-open-and-close-animations")]
    PresenceWithMultipleOpenAndCloseAnimations,
    #[at("/presence/with-deferred-mount-animation")]
    PresenceWithDeferredMountAnimation,

    #[at("/select/styled")]
    SelectStyled,
    #[at("/select/controlled")]
    SelectControlled,
    #[at("/select/position")]
    SelectPosition,
    #[at("/select/no-default-value")]
    SelectNoDefaultValue,
    #[at("/select/typeahead")]
    SelectTypeahead,
    #[at("/select/with-groups")]
    SelectWithGroups,
    #[at("/select/labelling")]
    SelectLabelling,
    #[at("/select/right-to-left")]
    SelectRightToLeft,
    #[at("/select/within-form")]
    SelectWithinForm,
    #[at("/select/disabled-within-form")]
    SelectDisabledWithinForm,
    #[at("/select/required-within-form")]
    SelectRequiredWithForm,
    #[at("/select/within-dialog")]
    SelectWithinDialog,
    #[at("/select/chromatic-short-options-padded-content")]
    SelectChromaticShortOptionsPaddedContent,
    #[at("/select/chromatic-short-options-padded-viewport")]
    SelectChromaticShortOptionsPaddedViewport,
    #[at("/select/chromatic-long-options-padded-content")]
    SelectChromaticLongOptionsPaddedContent,
    #[at("/select/chromatic-long-options-padded-viewport")]
    SelectChromaticLongOptionsPaddedViewport,
    #[at("/select/chromatic-top-first-padded-content")]
    SelectChromaticTopFirstPaddedContent,
    #[at("/select/chromatic-top-first-padded-viewport")]
    SelectChromaticTopFirstPaddedViewport,
    #[at("/select/chromatic-bottom-last-padded-content")]
    SelectChromaticBottomLastPaddedContent,
    #[at("/select/chromatic-bottom-last-padded-viewport")]
    SelectChromaticBottomLastPaddedViewport,
    #[at("/select/chromatic-no-default-value")]
    SelectChromaticNoDefaultValue,
    #[at("/select/cypress")]
    SelectCypress,

    #[at("/separator/styled")]
    SeparatorStyled,

    #[at("/switch/styled")]
    SwitchStyled,
    #[at("/switch/controlled")]
    SwitchControlled,
    #[at("/switch/within-form")]
    SwitchWithinForm,
    #[at("/switch/chromatic")]
    SwitchChromatic,

    #[at("/tooltip/styled")]
    TooltipStyled,
    #[at("/tooltip/controlled")]
    TooltipControlled,
    #[at("/tooltip/custom-durations")]
    TooltipCustomDurations,
    #[at("/tooltip/custom-content")]
    TooltipCustomContent,
    #[at("/tooltip/positions")]
    TooltipPositions,
    #[at("/tooltip/aria-label")]
    TooltipAriaLabel,
    #[at("/tooltip/with-text")]
    TooltipWithText,
    #[at("/tooltip/with-external-ref")]
    TooltipWithExternalRef,
    #[at("/tooltip/unmount")]
    TooltipUnmount,
    #[at("/tooltip/animated")]
    TooltipAnimated,
    #[at("/tooltip/slottable-content")]
    TooltipSlottableContent,
    #[at("/tooltip/within-dialog")]
    TooltipWithinDialog,
    #[at("/tooltip/keep-open-on-activation")]
    TooltipKeepOpenOnActivation,
    #[at("/tooltip/within-scrollable")]
    TooltipWithinScrollable,
    #[at("/tooltip/disable-hoverable-content")]
    TooltipDisableHoverableContent,
    #[at("/tooltip/chromatic")]
    TooltipChromatic,

    #[at("/visually-hidden/basic")]
    VisuallyHiddenBasic,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! { <Index /> },

        Route::ArrowStyled => html! { <arrow::Styled /> },
        Route::ArrowCustomSizes => html! { <arrow::CustomSizes /> },
        Route::ArrowCustomArrow => html! { <arrow::CustomArrow /> },

        Route::AspectRatioStyled => html! { <aspect_ratio::Styled /> },
        Route::AspectRatioCustomRatios => html! { <aspect_ratio::CustomRatios /> },
        Route::AspectRatioChromatic => html! { <aspect_ratio::Chromatic /> },

        Route::AvatarStyled => html! { <avatar::Styled /> },
        Route::AvatarChromatic => html! { <avatar::Chromatic /> },

        Route::CheckboxStyled => html! { <checkbox::Styled /> },
        Route::CheckboxControlled => html! { <checkbox::Controlled /> },
        Route::CheckboxIndeterminate => html! { <checkbox::Indeterminate /> },
        Route::CheckboxWithinForm => html! { <checkbox::WithinForm /> },
        Route::CheckboxAnimated => html! { <checkbox::Animated /> },
        Route::CheckboxChromatic => html! { <checkbox::Chromatic /> },

        Route::CollectionBasic => html! { <collection::Basic /> },
        Route::CollectionWithElementsInBetween => html! { <collection::WithElementsInBetween /> },
        Route::CollectionWithWrappedItem => html! { <collection::WithWrappedItem /> },
        Route::CollectionWithFragment => html! { <collection::WithFragment /> },
        Route::CollectionDynamicInsertion => html! { <collection::DynamicInsertion /> },
        Route::CollectionWithChangingItem => html! { <collection::WithChangingItem /> },
        Route::CollectionNested => html! { <collection::Nested /> },

        Route::DialogStyled => html! { <dialog::Styled /> },
        Route::DialogNonModal => html! { <dialog::NonModal /> },
        Route::DialogControlled => html! { <dialog::Controlled /> },
        Route::DialogFocusTrap => html! { <dialog::FocusTrap /> },
        Route::DialogCustomFocus => html! { <dialog::CustomFocus /> },
        Route::DialogNoEscapeDismiss => html! { <dialog::NoEscapeDismiss /> },
        Route::DialogNoPointerDownOutsideDismiss => {
            html! { <dialog::NoPointerDownOutsideDismiss /> }
        }
        Route::DialogWithPortalContainer => html! { <dialog::WithPortalContainer /> },
        Route::DialogAnimated => html! { <dialog::Animated /> },
        Route::DialogForcedMount => html! { <dialog::ForcedMount /> },
        Route::DialogInnerScrollable => html! { <dialog::InnerScrollable /> },
        Route::DialogOuterScrollable => html! { <dialog::OuterScrollable /> },
        Route::DialogChromatic => html! { <dialog::Chromatic /> },
        Route::DialogCypress => html! { <dialog::Cypress /> },

        Route::FocusScopeBasic => html! { <focus_scope::Basic /> },
        Route::FocusScopeMultiple => html! { <focus_scope::Multiple /> },
        Route::FocusScopeWithOptions => html! { <focus_scope::WithOptions /> },

        Route::LabelStyled => html! { <label::Styled /> },
        Route::LabelWithControl => html! { <label::WithControl /> },
        Route::LabelWithInputNumber => html! { <label::WithInputNumber /> },

        Route::PopperStyled => html! { <popper::Styled /> },
        Route::PopperWithCustomArrow => html! { <popper::WithCustomArrow /> },
        Route::PopperAnimated => html! { <popper::Animated /> },
        Route::PopperWithPortal => html! { <popper::WithPortal /> },
        Route::PopperWithUpdatePositionStrategyAlways => {
            html! { <popper::WithUpdatePositionStrategyAlways /> }
        }
        Route::PopperChromatic => html! { <popper::Chromatic /> },

        Route::PortalBase => html! { <portal::Base /> },
        Route::PortalCustomContainer => html! { <portal::CustomContainer /> },
        Route::PortalChromatic => html! { <portal::Chromatic /> },

        Route::PresenceBasic => html! { <presence::Basic /> },
        Route::PresenceWithMountAnimation => html! { <presence::WithMountAnimation /> },
        Route::PresenceWithUnmountAnimation => html! { <presence::WithUnmountAnimation /> },
        Route::PresenceWithMultipleMountAnimations => {
            html! { <presence::WithMultipleMountAnimations /> }
        }
        Route::PresenceWithOpenAndCloseAnimation => {
            html! { <presence::WithOpenAndCloseAnimation /> }
        }
        Route::PresenceWithMultipleOpenAndCloseAnimations => {
            html! { <presence::WithMultipleOpenAndCloseAnimations /> }
        }
        Route::PresenceWithDeferredMountAnimation => {
            html! { <presence::WithDeferredMountAnimation /> }
        }

        Route::SelectStyled => html! { <select::Styled /> },
        Route::SelectControlled => html! { <select::Controlled /> },
        Route::SelectPosition => html! { <select::Position /> },
        Route::SelectNoDefaultValue => html! { <select::NoDefaultValue /> },
        Route::SelectTypeahead => html! { <select::Typeahead /> },
        Route::SelectWithGroups => html! { <select::WithGroups /> },
        Route::SelectLabelling => html! { <select::Labelling /> },
        Route::SelectRightToLeft => html! { <select::RightToLeft /> },
        Route::SelectWithinForm => html! { <select::WithinForm /> },
        Route::SelectDisabledWithinForm => html! { <select::DisabledWithinForm /> },
        Route::SelectRequiredWithForm => html! { <select::RequiredWithForm /> },
        Route::SelectWithinDialog => html! { <select::WithinDialog /> },
        Route::SelectChromaticShortOptionsPaddedContent => {
            html! { <select::ChromaticShortOptionsPaddedContent /> }
        }
        Route::SelectChromaticShortOptionsPaddedViewport => {
            html! { <select::ChromaticShortOptionsPaddedViewport /> }
        }
        Route::SelectChromaticLongOptionsPaddedContent => {
            html! { <select::ChromaticLongOptionsPaddedContent /> }
        }
        Route::SelectChromaticLongOptionsPaddedViewport => {
            html! { <select::ChromaticLongOptionsPaddedViewport /> }
        }
        Route::SelectChromaticTopFirstPaddedContent => {
            html! { <select::ChromaticTopFirstPaddedContent /> }
        }
        Route::SelectChromaticTopFirstPaddedViewport => {
            html! { <select::ChromaticTopFirstPaddedViewport /> }
        }
        Route::SelectChromaticBottomLastPaddedContent => {
            html! { <select::ChromaticBottomLastPaddedContent /> }
        }
        Route::SelectChromaticBottomLastPaddedViewport => {
            html! { <select::ChromaticBottomLastPaddedViewport /> }
        }
        Route::SelectChromaticNoDefaultValue => html! { <select::ChromaticNoDefaultValue /> },
        Route::SelectCypress => html! { <select::Cypress /> },

        Route::SeparatorStyled => html! { <separator::Styled /> },

        Route::SwitchStyled => html! { <switch::Styled /> },
        Route::SwitchControlled => html! { <switch::Controlled /> },
        Route::SwitchWithinForm => html! { <switch::WithinForm /> },
        Route::SwitchChromatic => html! { <switch::Chromatic /> },

        Route::TooltipStyled => html! { <tooltip::Styled /> },
        Route::TooltipControlled => html! { <tooltip::Controlled /> },
        Route::TooltipCustomDurations => html! { <tooltip::CustomDurations /> },
        Route::TooltipCustomContent => html! { <tooltip::CustomContent /> },
        Route::TooltipPositions => html! { <tooltip::Positions /> },
        Route::TooltipAriaLabel => html! { <tooltip::AriaLabel /> },
        Route::TooltipWithText => html! { <tooltip::WithText /> },
        Route::TooltipWithExternalRef => html! { <tooltip::WithExternalRef /> },
        Route::TooltipUnmount => html! { <tooltip::Unmount /> },
        Route::TooltipAnimated => html! { <tooltip::Animated /> },
        Route::TooltipSlottableContent => html! { <tooltip::SlottableContent /> },
        Route::TooltipWithinDialog => html! { <tooltip::WithinDialog /> },
        Route::TooltipKeepOpenOnActivation => html! { <tooltip::KeepOpenOnActivation /> },
        Route::TooltipWithinScrollable => html! { <tooltip::WithinScrollable /> },
        Route::TooltipDisableHoverableContent => html! { <tooltip::DisableHoverableContent /> },
        Route::TooltipChromatic => html! { <tooltip::Chromatic /> },

        Route::VisuallyHiddenBasic => html! { <visually_hidden::Basic /> },
    }
}

#[derive(PartialEq, Properties)]
struct NavLinkProps<R>
where
    R: Routable + 'static,
{
    to: R,
    children: Html,
}

#[function_component]
fn NavLink<R>(props: &NavLinkProps<R>) -> Html
where
    R: Routable + 'static,
{
    // TODO: add class when active
    html! {
        <Link<R>
            classes={classes!("text-inherit", "decoration-inherit", "no-underline")}
            to={props.to.clone()}
        >
            {props.children.clone()}
        </Link<R>>
    }
}

#[function_component]
fn Index() -> Html {
    html! {
        <h1>{"Radix Yew Stories"}</h1>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <nav class="bg-slate-200 p-4 fixed top-0 bottom-0 start-0 w-64 box-border overflow-y-auto leading-normal">
                <ul class="list-none m-0 p-0">
                    <li>
                        <NavLink<Route> to={Route::Index}>{"Index"}</NavLink<Route>>
                    </li>
                    <li>
                        {"Arrow"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::ArrowStyled}>{"Styled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::ArrowCustomSizes}>{"Custom Sizes"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::ArrowCustomArrow}>{"Custom Arrow"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Aspect Ratio"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::AspectRatioStyled}>{"Styled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::AspectRatioCustomRatios}>{"Custom Ratios"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::AspectRatioChromatic}>{"Chromatic"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Avatar"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::AvatarStyled}>{"Styled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::AvatarChromatic}>{"Chromatic"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Checkbox"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::CheckboxStyled}>{"Styled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::CheckboxControlled}>{"Controlled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::CheckboxIndeterminate}>{"Indeterminate"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::CheckboxWithinForm}>{"Within Form"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::CheckboxAnimated}>{"Animated"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::CheckboxChromatic}>{"Chromatic"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Collection"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::CollectionBasic}>{"Basic"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::CollectionWithElementsInBetween}>{"With Elements In Between"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::CollectionWithWrappedItem}>{"With Wrapped Item"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::CollectionWithFragment}>{"With Fragment"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::CollectionDynamicInsertion}>{"Dynamic Insertion"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::CollectionWithChangingItem}>{"With Changing Item"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::CollectionNested}>{"Nested"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Dialog"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::DialogStyled}>{"Styled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::DialogNonModal}>{"Non Modal"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::DialogControlled}>{"Controlled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::DialogFocusTrap}>{"Focus Trap"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::DialogCustomFocus}>{"Custom Focus"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::DialogNoEscapeDismiss}>{"No Escape Dismiss"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::DialogNoPointerDownOutsideDismiss}>{"No Pointer Down Outside Dismiss"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::DialogWithPortalContainer}>{"With Portal Container"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::DialogAnimated}>{"Animated"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::DialogForcedMount}>{"Forced Mount"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::DialogInnerScrollable}>{"Inner Scrollable"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::DialogOuterScrollable}>{"Outer Scrollable"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::DialogChromatic}>{"Chromatic"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::DialogCypress}>{"Cypress"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Focus Scope"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::FocusScopeBasic}>{"Basic"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::FocusScopeMultiple}>{"Multiple"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::FocusScopeWithOptions}>{"With Options"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Label"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::LabelStyled}>{"Styled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::LabelWithControl}>{"With Control"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::LabelWithInputNumber}>{"With Input Number"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Popper"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::PopperStyled}>{"Styled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::PopperWithCustomArrow}>{"With Custom Arrow"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::PopperAnimated}>{"Animated"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::PopperWithPortal}>{"With Portal"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::PopperWithUpdatePositionStrategyAlways}>{"With Update Position Strategy Always"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::PopperChromatic}>{"Chromatic"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Portal"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::PortalBase}>{"Base"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::PortalCustomContainer}>{"Custom Container"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::PortalChromatic}>{"Chromatic"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Presence"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::PresenceBasic}>{"Basic"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::PresenceWithMountAnimation}>{"With Mount Animation"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::PresenceWithUnmountAnimation}>{"With Unmount Animation"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::PresenceWithMultipleMountAnimations}>{"With Multiple Mount Animations"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::PresenceWithOpenAndCloseAnimation}>{"With Open and Close Animation"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::PresenceWithMultipleOpenAndCloseAnimations}>{"With Multiple Open and Close Animations"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::PresenceWithDeferredMountAnimation}>{"With Deferred Mount Animation"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Select"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::SelectStyled}>{"Styled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectControlled}>{"Controlled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectPosition}>{"Position"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectNoDefaultValue}>{"No Default Value"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectTypeahead}>{"Typeahead"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectWithGroups}>{"With Groups"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectLabelling}>{"Labelling"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectRightToLeft}>{"Right To Left"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectWithinForm}>{"Within Form"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectDisabledWithinForm}>{"Disabled Within Form"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectRequiredWithForm}>{"Required With Form"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectWithinDialog}>{"Within Dialog"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectChromaticShortOptionsPaddedContent}>{"Chromatic Short Options Padded Content"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectChromaticShortOptionsPaddedViewport}>{"Chromatic Short Options Padded Viewport"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectChromaticLongOptionsPaddedContent}>{"Chromatic Long Options Padded Content"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectChromaticLongOptionsPaddedViewport}>{"Chromatic Long Options Padded Viewport"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectChromaticTopFirstPaddedContent}>{"Chromatic Top First Padded Content"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectChromaticTopFirstPaddedViewport}>{"Chromatic Top First Padded Viewport"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectChromaticBottomLastPaddedContent}>{"Chromatic Bottom Last Padded Content"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectChromaticBottomLastPaddedViewport}>{"Chromatic Bottom Last Padded Viewport"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectChromaticNoDefaultValue}>{"Chromatic No Default Value"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SelectCypress}>{"Cypress"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Separator"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::SeparatorStyled}>{"Styled"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Switch"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::SwitchStyled}>{"Styled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SwitchControlled}>{"Controlled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SwitchWithinForm}>{"Within Form"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::SwitchChromatic}>{"Chromatic"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Tooltip"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::TooltipStyled}>{"Styled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipControlled}>{"Controlled"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipCustomDurations}>{"Custom Durations"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipCustomContent}>{"Custom Content"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipPositions}>{"Positions"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipAriaLabel}>{"Aria Label"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipWithText}>{"With Text"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipWithExternalRef}>{"With External Ref"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipUnmount}>{"Unmount"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipAnimated}>{"Animated"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipSlottableContent}>{"Slottable Content"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipWithinDialog}>{"Within Dialog"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipKeepOpenOnActivation}>{"Keep Open On Activation"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipWithinScrollable}>{"Within Scrollable"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipDisableHoverableContent}>{"Disable Hoverable Content"}</NavLink<Route>></li>
                            <li><NavLink<Route> to={Route::TooltipChromatic}>{"Chromatic"}</NavLink<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Visually Hidden"}

                        <ul class="list-none m-0 ms-4 p-0">
                            <li><NavLink<Route> to={Route::VisuallyHiddenBasic}>{"Basic"}</NavLink<Route>></li>
                        </ul>
                    </li>
                </ul>
            </nav>
            <main class="ms-64 p-4">
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}
