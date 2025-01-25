use leptos::prelude::*;
use radix_leptos_avatar::*;

const COLM_IMG: &str = "https://images.unsplash.com/photo-1492633423870-43d1cd2775eb?&w=128&h=128&dpr=2&q=80";
const PEDRO_IMG: &str = "https://images.unsplash.com/photo-1511485977113-f34c92461ad9?ixlib=rb-1.2.1&w=128&h=128&dpr=2&q=80";
const BROKEN_IMG: &str = "https://broken.link.com/broken-pic.jpg";

const ROOT_CLASS: &str = "inline-flex items-center justify-center align-middle overflow-hidden select-none rounded-full w-[45px] h-[45px] bg-black/10";
const IMAGE_CLASS: &str = "w-full h-full object-cover rounded-[inherit]";
const FALLBACK_CLASS: &str = "w-full h-full flex items-center justify-center bg-white text-violet-900 text-[15px] leading-none font-medium";

#[component]
pub fn Styled() -> impl IntoView {
    view! {
       <section class="space-y-8">
           <div>
               <h2 class="text-lg font-medium mb-4">Basic</h2>
               <div class="flex gap-4">
                   <Avatar attr:class=ROOT_CLASS>
                       <AvatarImage src=COLM_IMG.to_string() attr:alt="CT" attr:class=IMAGE_CLASS />
                       <AvatarFallback delay_ms=600 attr:class=FALLBACK_CLASS>CT</AvatarFallback>
                   </Avatar>

                   <Avatar attr:class=ROOT_CLASS>
                       <AvatarImage src=PEDRO_IMG.to_string() attr:alt="PD" attr:class=IMAGE_CLASS />
                       <AvatarFallback delay_ms=600 attr:class=FALLBACK_CLASS>PD</AvatarFallback>
                   </Avatar>

                   <Avatar attr:class=ROOT_CLASS>
                       <AvatarFallback attr:class=FALLBACK_CLASS>PD</AvatarFallback>
                   </Avatar>
               </div>
           </div>

           <div>
               <h2 class="text-lg font-medium mb-4">Sizes</h2>
               <div class="flex items-end gap-4">
                   <Avatar attr:class="inline-flex items-center justify-center align-middle overflow-hidden select-none rounded-full w-8 h-8 bg-black/10">
                       <AvatarImage src=COLM_IMG.to_string() attr:alt="S" attr:class=IMAGE_CLASS />
                       <AvatarFallback attr:class=FALLBACK_CLASS>S</AvatarFallback>
                   </Avatar>

                   <Avatar attr:class=ROOT_CLASS>
                       <AvatarImage src=COLM_IMG.to_string() attr:alt="M" attr:class=IMAGE_CLASS />
                       <AvatarFallback attr:class=FALLBACK_CLASS>M</AvatarFallback>
                   </Avatar>

                   <Avatar attr:class="inline-flex items-center justify-center align-middle overflow-hidden select-none rounded-full w-16 h-16 bg-black/10">
                       <AvatarImage src=COLM_IMG.to_string() attr:alt="L" attr:class=IMAGE_CLASS />
                       <AvatarFallback attr:class=FALLBACK_CLASS>L</AvatarFallback>
                   </Avatar>
               </div>
           </div>
       </section>
   }
}

#[component]
pub fn Chromatic() -> impl IntoView {
    view! {
       <section class="space-y-8">
           <div>
               <h2 class="text-lg font-medium mb-4">Loading States</h2>
               <div class="flex gap-4">
                   <div>
                       <p class="text-sm mb-2">Instant fallback</p>
                       <Avatar attr:class=ROOT_CLASS>
                           <AvatarImage src=BROKEN_IMG.to_string() attr:alt="Avatar" attr:class=IMAGE_CLASS />
                           <AvatarFallback attr:class=FALLBACK_CLASS>IF</AvatarFallback>
                       </Avatar>
                   </div>

                   <div>
                       <p class="text-sm mb-2">With delay</p>
                       <Avatar attr:class=ROOT_CLASS>
                           <AvatarImage
                               src=BROKEN_IMG.to_string()
                               attr:alt="Avatar"
                               attr:class=IMAGE_CLASS
                               on_loading_status_change=move |status| log::info!("Loading status: {:?}", status)
                           />
                           <AvatarFallback delay_ms=600 attr:class=FALLBACK_CLASS>6D</AvatarFallback>
                       </Avatar>
                   </div>
               </div>
           </div>

           <div>
               <h2 class="text-lg font-medium mb-4">Fallback Types</h2>
               <div class="flex gap-4">
                   <Avatar attr:class=ROOT_CLASS>
                       <AvatarImage src=BROKEN_IMG.to_string() attr:alt="Avatar" attr:class=IMAGE_CLASS />
                       <AvatarFallback attr:class=FALLBACK_CLASS>
                           <AvatarIcon />
                       </AvatarFallback>
                   </Avatar>
               </div>
           </div>
       </section>
   }
}

#[component]
fn AvatarIcon() -> impl IntoView {
    view! {
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="42" height="42">
           <path
               d="M50 51.7a22.1 22.1 0 100-44.2 22.1 22.1 0 000 44.2zM87.9 69.3a27.8 27.8 0 00-21.2-16.1 4 4 0 00-2.8.7 23.5 23.5 0 01-27.6 0 4 4 0 00-2.8-.7 27.5 27.5 0 00-21.2 16.1c-.3.6-.2 1.3.1 1.8a52.8 52.8 0 007 8.9 43.4 43.4 0 0056.9 3.8 56.3 56.3 0 008.9-8.8c.9-1.2 1.8-2.5 2.6-3.9.3-.6.3-1.2.1-1.8z"
               fill="currentColor"
           />
       </svg>
    }
}
