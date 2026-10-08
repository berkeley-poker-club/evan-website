use leptos::prelude::*;
use leptos_meta::Title;
use crate::components::OptimizedImage;

const KNOCKOUT_ORDER_FORM: &str = "https://docs.google.com/forms/d/e/1FAIpQLSdeLKN44sxTMiWJ_R1kvB-08sAhQ1lhbunSKOAUIFjGUNvPYg/viewform?usp=dialog";
const CATALOG_ORDER_FORM: &str = "https://docs.google.com/forms/d/e/1FAIpQLSfjmXWpN-px3H9o1AC0FTM8I643kMiuZk0vYQFgjRns4_ojxg/viewform?usp=publish-editor";
const SIZE_MATTERS_ORDER_FORM: &str = "https://docs.google.com/forms/d/e/1FAIpQLSeSAkRwARDVNgPpP42C87rNzB05Ge13OiLL-W3FKRgOynDtsQ/viewform?usp=dialog";

#[component]
pub fn MerchPage() -> impl IntoView {
    view! {
        <Title text="Merch | Poker at Berkeley" />
        <div class="min-h-screen bg-[#F5F0E8]">
            <div class="max-w-[500px] mx-auto px-6 pt-12">
                <OptimizedImage
                    src="public/images/merch/merchheader.png"
                    alt="Poker at Berkeley Merch"
                    class="w-full h-auto block"
                />
            </div>

            <section class="max-w-6xl mx-auto px-6 pt-20">
                <SectionHeading number="01" heading="Berkeley x Stanford" />
                <div class="grid grid-cols-1 md:grid-cols-2 gap-x-12 gap-y-16">
                    <a href=CATALOG_ORDER_FORM target="_blank" rel="noopener noreferrer" class="block">
                        <OptimizedImage
                            src="public/images/merch/catalog-drop.png"
                            alt="New Merch Drop"
                            class="w-full h-auto block"
                        />
                    </a>
                    <a href=CATALOG_ORDER_FORM target="_blank" rel="noopener noreferrer" class="block">
                        <OptimizedImage
                            src="public/images/merch/catalog-grid.png"
                            alt="Berkeley x Stanford Catalog"
                            class="w-full h-auto block"
                        />
                    </a>
                </div>
                <div class="flex justify-end border-t border-[#D9CBB8] mt-6 pt-4">
                    <a
                        href=CATALOG_ORDER_FORM
                        target="_blank"
                        rel="noopener noreferrer"
                        class="shrink-0 text-[#B8763F] text-lg uppercase tracking-[0.15em] border-b-2 border-[#B8763F]/50 hover:border-[#B8763F] pb-1 transition-colors whitespace-nowrap"
                    >
                        "Order Now"
                    </a>
                </div>
            </section>

            <section class="max-w-6xl mx-auto px-6 pt-20">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-x-12 gap-y-16">
                    <a href=KNOCKOUT_ORDER_FORM target="_blank" rel="noopener noreferrer" class="block">
                        <OptimizedImage
                            src="public/images/merch/3.png"
                            alt="The Knockout Tee"
                            class="w-full h-auto block"
                        />
                    </a>
                    <a href=SIZE_MATTERS_ORDER_FORM target="_blank" rel="noopener noreferrer" class="block">
                        <OptimizedImage
                            src="public/images/merch/7.png"
                            alt="The Size Matters Tee"
                            class="w-full h-auto block"
                        />
                    </a>
                </div>
            </section>

            <ProductSection
                number="02"
                heading="The Knockout Tee"
                image="public/images/merch/knockouttee.png"
                alt="Berkeley x Stanford Knockout Tee, unisex and women's"
                href=KNOCKOUT_ORDER_FORM
            />
            <ProductSection
                number="03"
                heading="The Size Matters Tee"
                image="public/images/merch/sizematters.png"
                alt="Size Matters Tee, unisex and women's"
                href=SIZE_MATTERS_ORDER_FORM
            />

            <div class="max-w-6xl mx-auto px-6 py-16 text-center">
                <p
                    class="text-[#B08B72] text-lg italic"
                    style="font-family: 'Cormorant Garamond', serif;"
                >
                    "More drops coming"
                </p>
            </div>
        </div>
    }
}

#[component]
fn ProductSection(
    number: &'static str,
    heading: &'static str,
    image: &'static str,
    alt: &'static str,
    href: &'static str,
) -> impl IntoView {
    view! {
        <section class="max-w-6xl mx-auto px-6 pt-20">
            <SectionHeading number=number heading=heading />

            <div class="max-w-4xl mx-auto">
                <a href=href target="_blank" rel="noopener noreferrer" class="block">
                    <OptimizedImage src=image alt=alt class="w-full h-auto block" />
                </a>
                <div class="flex justify-end border-t border-[#D9CBB8] mt-6 pt-4">
                    <a
                        href=href
                        target="_blank"
                        rel="noopener noreferrer"
                        class="shrink-0 text-[#B8763F] text-lg uppercase tracking-[0.15em] border-b-2 border-[#B8763F]/50 hover:border-[#B8763F] pb-1 transition-colors whitespace-nowrap"
                    >
                        "Order"
                    </a>
                </div>
            </div>
        </section>
    }
}

#[component]
fn SectionHeading(number: &'static str, heading: &'static str) -> impl IntoView {
    view! {
        <div class="flex items-baseline gap-4 mb-12">
            <span
                class="text-[#E0D2BE] leading-none select-none"
                style="font-family: 'Archivo Black', sans-serif; font-size: clamp(1.25rem, 2.5vw, 1.75rem);"
            >
                {number}
            </span>
            <h2
                class="text-[#3A2E24] leading-none"
                style="font-family: 'Archivo Black', sans-serif; font-weight: 900; font-size: clamp(2.25rem, 5vw, 3.75rem);"
            >
                {heading}
            </h2>
        </div>
    }
}
