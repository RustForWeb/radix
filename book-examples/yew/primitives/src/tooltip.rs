use radix_yew_icons::PlusIcon;
use radix_yew_tooltip::*;
use yew::prelude::*;

#[function_component]
pub fn TooltipDemo() -> Html {
    html! {
        <TooltipProvider>
            <Tooltip>
                <TooltipTrigger
                    as_child={Callback::from(|TooltipTriggerChildProps {
                        node_ref,
                        aria_describedby,
                        data_state,
                        onblur,
                        onclick,
                        onfocus,
                        onpointerdown,
                        onpointerleave,
                        onpointermove,
                        ..
                    }| html! {
                        <button
                            ref={node_ref}
                            aria-describedby={aria_describedby}
                            class={"inline-flex size-[35px] items-center justify-center rounded-full bg-white text-violet11 shadow-[0_2px_10px] shadow-blackA4 outline-none hover:bg-violet3 focus:shadow-[0_0_0_2px] focus:shadow-black"}
                            data-state={data_state}
                            onblur={onblur}
                            onclick={onclick}
                            onfocus={onfocus}
                            onpointerdown={onpointerdown}
                            onpointerleave={onpointerleave}
                            onpointermove={onpointermove}
                        >
                            <PlusIcon />
                        </button>
                    })}
                >
                </TooltipTrigger>
                <TooltipPortal>
                    <TooltipContent
                        class="select-none rounded bg-white px-[15px] py-2.5 text-[15px] leading-none text-violet11 shadow-[hsl(206_22%_7%_/_35%)_0px_10px_38px_-10px,_hsl(206_22%_7%_/_20%)_0px_10px_20px_-15px] will-change-[transform,opacity] data-[state=delayed-open]:data-[side=bottom]:animate-slideUpAndFade data-[state=delayed-open]:data-[side=left]:animate-slideRightAndFade data-[state=delayed-open]:data-[side=right]:animate-slideLeftAndFade data-[state=delayed-open]:data-[side=top]:animate-slideDownAndFade"
                        side_offset=5.0
                    >
                        {"Add to library"}
                        <TooltipArrow class="fill-white" />
                    </TooltipContent>
                </TooltipPortal>
            </Tooltip>
        </TooltipProvider>
    }
}
