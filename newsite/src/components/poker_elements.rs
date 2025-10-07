use leptos::prelude::*;
use rand::Rng;

#[component]
pub fn PokerCard(suit: &'static str, value: &'static str, color: &'static str) -> impl IntoView {
    let text_color = if color == "red" { "text-red-600" } else { "text-black" };
    
    view! {
        <div class="poker-card hover:animate-card-flip">
            <div class="absolute top-1 left-1 text-xs">
                <div class=format!("font-bold {}", text_color)>{value}</div>
                <div class=format!("{}", text_color)>{suit}</div>
            </div>
            <div class="absolute bottom-1 right-1 text-xs rotate-180">
                <div class=format!("font-bold {}", text_color)>{value}</div>
                <div class=format!("{}", text_color)>{suit}</div>
            </div>
            <div class="absolute inset-0 flex items-center justify-center">
                <span class=format!("text-4xl {}", text_color)>{suit}</span>
            </div>
        </div>
    }
}

#[component]
pub fn PokerChip(color: &'static str, value: &'static str) -> impl IntoView {
    let chip_class = match color {
        "red" => "poker-chip-red",
        "blue" => "poker-chip-blue",
        "green" => "poker-chip-green",
        "black" => "poker-chip-black",
        _ => "poker-chip-red"
    };

    view! {
        <div class=format!("{} hover:animate-chip-stack cursor-pointer", chip_class)>
            <div class="absolute inset-0 flex items-center justify-center">
                <span class="text-white font-bold text-xs">{value}</span>
            </div>
            <div class="absolute inset-2 border border-white/30 rounded-full"></div>
        </div>
    }
}

#[component]
pub fn FloatingCards() -> impl IntoView {
    let cards = vec![
        ("♠", "A", "black"),
        ("♥", "K", "red"),
        ("♦", "Q", "red"),
        ("♣", "J", "black"),
        ("♠", "10", "black"),
        ("♥", "9", "red"),
    ];

    view! {
        <div class="absolute inset-0 overflow-hidden pointer-events-none">
            {cards.into_iter().enumerate().map(|(i, (suit, value, color))| {
                let delay = i as f64 * 0.5;
                let left = rand::thread_rng().gen_range(10..80);
                let top = rand::thread_rng().gen_range(20..80);
                
                view! {
                    <div 
                        class="absolute animate-bounce opacity-30"
                        style=format!("left: {}%; top: {}%; animation-delay: {}s; animation-duration: {}s", 
                                    left, top, delay, 3.0 + (i as f64 * 0.3))
                    >
                        <div class="transform scale-75">
                            <PokerCard suit=suit value=value color=color />
                        </div>
                    </div>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
pub fn ChipStack() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center space-y-1">
            <PokerChip color="black" value="100" />
            <PokerChip color="green" value="25" />
            <PokerChip color="blue" value="10" />
            <PokerChip color="red" value="5" />
        </div>
    }
}