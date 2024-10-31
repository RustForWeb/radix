use radix_yew_avatar::*;
use yew::prelude::*;

#[function_component]
pub fn AvatarDemo() -> Html {
    html! {
        <div class="flex gap-5">
            <Avatar class="bg-blackA1 inline-flex h-[45px] w-[45px] select-none items-center justify-center overflow-hidden rounded-full align-middle">
                <AvatarImage
                    class="h-full w-full rounded-[inherit] object-cover"
                    src="https://images.unsplash.com/photo-1492633423870-43d1cd2775eb?&w=128&h=128&dpr=2&q=80"
                    alt="Colm Tuite"
                />
                <AvatarFallback
                    class="text-violet11 leading-1 flex h-full w-full items-center justify-center bg-white text-[15px] font-medium"
                    delay_ms=600
                >
                    {"CT"}
                </AvatarFallback>
            </Avatar>
            <Avatar class="bg-blackA1 inline-flex h-[45px] w-[45px] select-none items-center justify-center overflow-hidden rounded-full align-middle">
                <AvatarImage
                    src="https://images.unsplash.com/photo-1511485977113-f34c92461ad9?ixlib=rb-1.2.1&w=128&h=128&dpr=2&q=80"
                    alt="Pedro Duarte"
                />
                <AvatarFallback
                    class="text-violet11 leading-1 flex h-full w-full items-center justify-center bg-white text-[15px] font-medium"
                    delay_ms=600
                >
                    {"PD"}
                </AvatarFallback>
            </Avatar>
            <Avatar class="bg-blackA1 inline-flex h-[45px] w-[45px] select-none items-center justify-center overflow-hidden rounded-full align-middle">
                <AvatarFallback class="text-violet11 leading-1 flex h-full w-full items-center justify-center bg-white text-[15px] font-medium">
                    {"PD"}
                </AvatarFallback>
            </Avatar>
        </div>
    }
}
