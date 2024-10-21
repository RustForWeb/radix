use leptos::*;
use radix_leptos_avatar::*;

#[component]
pub fn AvatarDemo() -> impl IntoView {
    view! {
        <div class="flex gap-5">
            <Avatar attr:class="bg-blackA1 inline-flex h-[45px] w-[45px] select-none items-center justify-center overflow-hidden rounded-full align-middle">
                <AvatarImage
                    attr:class="h-full w-full rounded-[inherit] object-cover"
                    src="https://images.unsplash.com/photo-1492633423870-43d1cd2775eb?&w=128&h=128&dpr=2&q=80"
                    attr:alt="Colm Tuite"
                />
                <AvatarFallback
                    attr:class="text-violet11 leading-1 flex h-full w-full items-center justify-center bg-white text-[15px] font-medium"
                    delay_ms=600
                >
                    CT
                </AvatarFallback>
            </Avatar>
            <Avatar attr:class="bg-blackA1 inline-flex h-[45px] w-[45px] select-none items-center justify-center overflow-hidden rounded-full align-middle">
                <AvatarImage
                    src="https://images.unsplash.com/photo-1511485977113-f34c92461ad9?ixlib=rb-1.2.1&w=128&h=128&dpr=2&q=80"
                    attr:alt="Pedro Duarte"
                />
                <AvatarFallback
                    attr:class="text-violet11 leading-1 flex h-full w-full items-center justify-center bg-white text-[15px] font-medium"
                    delay_ms=600
                >
                    PD
                </AvatarFallback>
            </Avatar>
            <Avatar attr:class="bg-blackA1 inline-flex h-[45px] w-[45px] select-none items-center justify-center overflow-hidden rounded-full align-middle">
                <AvatarFallback attr:class="text-violet11 leading-1 flex h-full w-full items-center justify-center bg-white text-[15px] font-medium">
                    PD
                </AvatarFallback>
            </Avatar>
        </div>
    }
}
