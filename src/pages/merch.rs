use leptos::prelude::*;
use leptos_meta::Title;
use crate::components::OptimizedImage;

const KNOCKOUT_ORDER_FORM: &str = "https://docs.google.com/forms/d/e/1FAIpQLSdeLKN44sxTMiWJ_R1kvB-08sAhQ1lhbunSKOAUIFjGUNvPYg/viewform?usp=dialog";
const SIZE_MATTERS_ORDER_FORM: &str = "https://docs.google.com/forms/d/e/1FAIpQLSeSAkRwARDVNgPpP42C87rNzB05Ge13OiLL-W3FKRgOynDtsQ/viewform?usp=dialog";

struct Product {
    image: &'static str,
    title: &'static str,
    description: &'static str,
    href: &'static str,
}

const KNOCKOUT_PRODUCTS: &[Product] = &[
    Product {
        image: "public/images/merch/m-ko.png",
        title: "Berkeley x Stanford Knockout Tee",
        description: "Unisex textured waffle washed boxy tee · 250 GSM",
        href: KNOCKOUT_ORDER_FORM,
    },
    Product {
        image: "public/images/merch/w-ko.png",
        title: "Berkeley x Stanford Knockout Tee",
        description: "Women's boat neck baby tee · 260 GSM",
        href: KNOCKOUT_ORDER_FORM,
    },
];

const SIZE_MATTERS_PRODUCTS: &[Product] = &[
    Product {
        image: "public/images/merch/M-sizematters.png",
        title: "Size Matters Tee",
        description: "Unisex mid-weight cotton tee in off-white · 260 GSM",
        href: SIZE_MATTERS_ORDER_FORM,
    },
    Product {
        image: "public/images/merch/W-sizematters.png",
        title: "Size Matters Tee",
        description: "Women's slim-fit cropped tee in off-white · 220 GSM",
        href: SIZE_MATTERS_ORDER_FORM,
    },
];

#[component]
pub fn MerchPage() -> impl IntoView {
    view! {
        <Title text="Merch | Poker at Berkeley" />
        <div class="min-h-screen bg-[#F5F0E8]">
            <OptimizedImage
                src="public/images/merch/merchbanner.png"
                alt="Poker at Berkeley Merch"
                class="w-full h-auto block"
            />

            <ProductSection number="01" heading="The Knockout Tee" products=KNOCKOUT_PRODUCTS />
            <ProductSection number="02" heading="The Size Matters Tee" products=SIZE_MATTERS_PRODUCTS />

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
    products: &'static [Product],
) -> impl IntoView {
    view! {
        <section class="max-w-6xl mx-auto px-6 pt-20">
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

            <div class="grid grid-cols-1 md:grid-cols-2 gap-x-12 gap-y-16">
                {products.iter().map(|p| view! { <ProductCard product=p /> }).collect::<Vec<_>>()}
            </div>
        </section>
    }
}

#[component]
fn ProductCard(product: &'static Product) -> impl IntoView {
    view! {
        <div>
            <OptimizedImage
                src=product.image
                alt=product.title
                class="w-full h-auto block"
            />
            <div class="flex items-end justify-between gap-4 border-t border-[#D9CBB8] mt-6 pt-4">
                <div>
                    <h3 class="text-[#3A2E24] text-sm uppercase tracking-[0.15em] font-semibold">{product.title}</h3>
                    <p class="text-[#8A7A68] text-xs mt-1" style="font-family: 'JetBrains Mono', monospace;">{product.description}</p>
                </div>
                <a
                    href=product.href
                    target="_blank"
                    rel="noopener noreferrer"
                    class="shrink-0 text-[#B8763F] text-xs uppercase tracking-[0.15em] border-b border-[#B8763F]/50 hover:border-[#B8763F] pb-0.5 transition-colors whitespace-nowrap"
                >
                    "Order"
                </a>
            </div>
        </div>
    }
}
