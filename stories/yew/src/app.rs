use yew::prelude::*;
use yew_router::prelude::*;

use crate::primitives::{
    arrow, aspect_ratio, avatar, collection, focus_scope, label, popper, portal, presence, select,
    separator, switch, tooltip, visually_hidden,
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

        Route::CollectionBasic => html! { <collection::Basic /> },
        Route::CollectionWithElementsInBetween => html! { <collection::WithElementsInBetween /> },
        Route::CollectionWithWrappedItem => html! { <collection::WithWrappedItem /> },
        Route::CollectionWithFragment => html! { <collection::WithFragment /> },
        Route::CollectionDynamicInsertion => html! { <collection::DynamicInsertion /> },
        Route::CollectionWithChangingItem => html! { <collection::WithChangingItem /> },
        Route::CollectionNested => html! { <collection::Nested /> },

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

#[function_component]
fn Index() -> Html {
    html! {
        <h1>{ "Radix Yew Stories" }</h1>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <nav class="bg-slate-200 p-4 fixed top-0 bottom-0 start-0 w-64 overflow-y-auto">
                <ul>
                    <li>
                        <Link<Route> to={Route::Index}>{ "Index" }</Link<Route>>
                    </li>
                    <li>
                        {"Arrow"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::ArrowStyled}>{"Styled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::ArrowCustomSizes}>{"Custom Sizes"}</Link<Route>></li>
                            <li><Link<Route> to={Route::ArrowCustomArrow}>{"Custom Arrow"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Aspect Ratio"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::AspectRatioStyled}>{"Styled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::AspectRatioCustomRatios}>{"Custom Ratios"}</Link<Route>></li>
                            <li><Link<Route> to={Route::AspectRatioChromatic}>{"Chromatic"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Avatar"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::AvatarStyled}>{"Styled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::AvatarChromatic}>{"Chromatic"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Collection"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::CollectionBasic}>{"Basic"}</Link<Route>></li>
                            <li><Link<Route> to={Route::CollectionWithElementsInBetween}>{"With Elements In Between"}</Link<Route>></li>
                            <li><Link<Route> to={Route::CollectionWithWrappedItem}>{"With Wrapped Item"}</Link<Route>></li>
                            <li><Link<Route> to={Route::CollectionWithFragment}>{"With Fragment"}</Link<Route>></li>
                            <li><Link<Route> to={Route::CollectionDynamicInsertion}>{"Dynamic Insertion"}</Link<Route>></li>
                            <li><Link<Route> to={Route::CollectionWithChangingItem}>{"With Changing Item"}</Link<Route>></li>
                            <li><Link<Route> to={Route::CollectionNested}>{"Nested"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Focus Scope"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::FocusScopeBasic}>{"Basic"}</Link<Route>></li>
                            <li><Link<Route> to={Route::FocusScopeMultiple}>{"Multiple"}</Link<Route>></li>
                            <li><Link<Route> to={Route::FocusScopeWithOptions}>{"With Options"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Label"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::LabelStyled}>{"Styled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::LabelWithControl}>{"With Control"}</Link<Route>></li>
                            <li><Link<Route> to={Route::LabelWithInputNumber}>{"With Input Number"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Popper"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::PopperStyled}>{"Styled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PopperWithCustomArrow}>{"With Custom Arrow"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PopperAnimated}>{"Animated"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PopperWithPortal}>{"With Portal"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PopperWithUpdatePositionStrategyAlways}>{"With Update Position Strategy Always"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PopperChromatic}>{"Chromatic"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Portal"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::PortalBase}>{"Base"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PortalCustomContainer}>{"Custom Container"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PortalChromatic}>{"Chromatic"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Presence"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::PresenceBasic}>{"Basic"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PresenceWithMountAnimation}>{"With Mount Animation"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PresenceWithUnmountAnimation}>{"With Unmount Animation"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PresenceWithMultipleMountAnimations}>{"With Multiple Mount Animations"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PresenceWithOpenAndCloseAnimation}>{"With Open and Close Animation"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PresenceWithMultipleOpenAndCloseAnimations}>{"With Multiple Open and Close Animations"}</Link<Route>></li>
                            <li><Link<Route> to={Route::PresenceWithDeferredMountAnimation}>{"With Deferred Mount Animation"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Select"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::SelectStyled}>{"Styled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectControlled}>{"Controlled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectPosition}>{"Position"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectNoDefaultValue}>{"No Default Value"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectTypeahead}>{"Typeahead"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectWithGroups}>{"With Groups"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectLabelling}>{"Labelling"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectRightToLeft}>{"Right To Left"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectWithinForm}>{"Within Form"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectDisabledWithinForm}>{"Disabled Within Form"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectRequiredWithForm}>{"Required With Form"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectWithinDialog}>{"Within Dialog"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectChromaticShortOptionsPaddedContent}>{"Chromatic Short Options Padded Content"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectChromaticShortOptionsPaddedViewport}>{"Chromatic Short Options Padded Viewport"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectChromaticLongOptionsPaddedContent}>{"Chromatic Long Options Padded Content"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectChromaticLongOptionsPaddedViewport}>{"Chromatic Long Options Padded Viewport"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectChromaticTopFirstPaddedContent}>{"Chromatic Top First Padded Content"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectChromaticTopFirstPaddedViewport}>{"Chromatic Top First Padded Viewport"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectChromaticBottomLastPaddedContent}>{"Chromatic Bottom Last Padded Content"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectChromaticBottomLastPaddedViewport}>{"Chromatic Bottom Last Padded Viewport"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectChromaticNoDefaultValue}>{"Chromatic No Default Value"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SelectCypress}>{"Cypress"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Separator"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::SeparatorStyled}>{"Styled"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Switch"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::SwitchStyled}>{"Styled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SwitchControlled}>{"Controlled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SwitchWithinForm}>{"Within Form"}</Link<Route>></li>
                            <li><Link<Route> to={Route::SwitchChromatic}>{"Chromatic"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Tooltip"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::TooltipStyled}>{"Styled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipControlled}>{"Controlled"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipCustomDurations}>{"Custom Durations"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipCustomContent}>{"Custom Content"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipPositions}>{"Positions"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipAriaLabel}>{"Aria Label"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipWithText}>{"With Text"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipWithExternalRef}>{"With External Ref"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipUnmount}>{"Unmount"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipAnimated}>{"Animated"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipSlottableContent}>{"Slottable Content"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipWithinDialog}>{"Within Dialog"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipKeepOpenOnActivation}>{"Keep Open On Activation"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipWithinScrollable}>{"Within Scrollable"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipDisableHoverableContent}>{"Disable Hoverable Content"}</Link<Route>></li>
                            <li><Link<Route> to={Route::TooltipChromatic}>{"Chromatic"}</Link<Route>></li>
                        </ul>
                    </li>
                    <li>
                        {"Visually Hidden"}

                        <ul class="ms-4">
                            <li><Link<Route> to={Route::VisuallyHiddenBasic}>{"Basic"}</Link<Route>></li>
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
