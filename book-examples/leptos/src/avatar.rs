use leptos::*;
use radix_leptos_avatar::{Avatar, AvatarFallback, AvatarImage};

#[component]
pub fn AvatarDemo() -> impl IntoView {
    view! {
        <div style:display="flex" style:gap="20px">
            <Avatar>
                <AvatarImage
                    src="https://images.unsplash.com/photo-1492633423870-43d1cd2775eb?&w=128&h=128&dpr=2&q=80"
                    attr:alt="Colm Tuite"
                />
                <AvatarFallback delay_ms=600>
                    CT
                </AvatarFallback>
            </Avatar>
            <Avatar>
                <AvatarImage
                    src="https://images.unsplash.com/photo-1511485977113-f34c92461ad9?ixlib=rb-1.2.1&w=128&h=128&dpr=2&q=80"
                    attr:alt="Pedro Duarte"
                />
                <AvatarFallback delay_ms=600>
                    PD
                </AvatarFallback>
            </Avatar>
            <Avatar>
                <AvatarFallback>PD</AvatarFallback>
            </Avatar>
        </div>
    }
}
