use leptos::prelude::*;
use crate::components::OptimizedImage;

const MERCH_ORDER_FORM: &str = "https://docs.google.com/forms/d/e/1FAIpQLSdeLKN44sxTMiWJ_R1kvB-08sAhQ1lhbunSKOAUIFjGUNvPYg/viewform?usp=dialog";

#[component]
pub fn MerchPage() -> impl IntoView {
    view! {
        <div class="relative">
            <OptimizedImage
                src="public/images/merchbanner.png"
                alt="Poker at Berkeley Merch"
                class="w-full h-auto block"
            />
            <a
                href=MERCH_ORDER_FORM
                target="_blank"
                rel="noopener noreferrer"
                aria-label="Place an order"
                class="absolute group"
                style="left: 4.2%; top: 81.9%; width: 20%; height: 6.3%;"
            >
                <img
                    src="public/images/herebtn.png"
                    alt=""
                    class="w-full h-full object-contain group-hover:opacity-0 transition-opacity duration-150"
                />
                <img
                    src="public/images/hereinverse.png"
                    alt=""
                    class="absolute inset-0 w-full h-full object-contain opacity-0 group-hover:opacity-100 transition-opacity duration-150"
                />
            </a>
        </div>
    }
}
