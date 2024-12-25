use std::collections::HashMap;

use glob::glob;
use proc_macro::TokenStream;
use quote::{format_ident, quote};

#[proc_macro]
pub fn import_solutions(_: TokenStream) -> TokenStream {
    let years_and_days = parse_solutions();

    let imports = years_and_days.keys().map(|year| {
        let module = format_ident!("year{year}");

        quote! {
            pub mod #module;
        }
    });

    let get_solution_match_arms = years_and_days.iter().flat_map(|(year, days)| {
        days.iter().flat_map(move |day| {
            (1_u8..=2).map(move |part| {
                let year_ident = format_ident!("year{year}");
                let day_ident = format_ident!("day{day:0>2}");
                let part_ident = format_ident!("part{part}");

                quote! {
                    (#year, #day, #part) => #year_ident::#day_ident::#part_ident
                }
            })
        })
    });

    quote! {
        #(#imports)*

        pub fn get_solution(year: u16, day: u8, part: u8) -> fn(&str) {
            match (year, day, part) {
                #(#get_solution_match_arms),*,
                _ => unimplemented!("No solution found for year {year} day {day} part {part}")
            }
        }
    }
    .into()
}

#[proc_macro]
pub fn import_days(input: TokenStream) -> TokenStream {
    let chosen_day = input.to_string().parse().unwrap();

    let years_and_days = parse_solutions();
    let days = &years_and_days[&chosen_day];

    let day_imports = days.iter().map(|day| {
        let day_ident = format_ident!("day{day:0>2}");

        quote! {
            pub mod #day_ident;
        }
    });

    quote! { #(#day_imports)* }.into()
}

fn parse_solutions() -> HashMap<u16, Vec<u8>> {
    // NOTE: use runtime-provided CARGO_MANIFEST_DIR.
    // env!() would give us our CARGO_MANIFEST_DIR instead of the caller's.
    let callsite = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not found");

    let mut paths = glob(&format!("{callsite}/src/year*/day*.rs")).expect("error reading yearXXXX/dayXX.rs files in callsite.");

    let mut years_and_days: HashMap<u16, Vec<u8>> = HashMap::new();

    while let Some(Ok(path)) = paths.next() {
        let year = path
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_start_matches("year")
            .parse()
            .unwrap();

        let day = path.file_stem().unwrap().to_str().unwrap().trim_start_matches("day").parse().unwrap();

        years_and_days.entry(year).or_default().push(day);
    }

    years_and_days
}
