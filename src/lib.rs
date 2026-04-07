#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
use darling::{Error, FromMeta};
use proc_macro2::TokenStream as TokenStream2;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default)]
enum TimeUnit {
    Ns,
    Us,
    #[default]
    Ms,
    S,
}

impl FromMeta for TimeUnit {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value {
            "ns" => Ok(Self::Ns),
            "us" => Ok(Self::Us),
            "ms" => Ok(Self::Ms),
            "s" => Ok(Self::S),
            _ => Err(Error::unknown_value(value)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct DurationThreshold(u128);

impl DurationThreshold {
    fn as_nanos(self) -> u128 {
        self.0
    }
}

impl FromMeta for DurationThreshold {
    fn from_string(value: &str) -> darling::Result<Self> {
        parse_duration_threshold(value).map(|duration| Self(duration.as_nanos()))
    }
}

#[derive(Debug, FromMeta)]
#[darling(derive_syn_parse)]
struct MacroArgs {
    #[darling(default)]
    print: Option<String>,
    #[darling(default)]
    prefix: Option<String>,
    #[darling(default)]
    suffix: Option<String>,
    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    unit: Option<TimeUnit>,
    #[darling(default)]
    log_over: Option<DurationThreshold>,
    #[darling(default)]
    warn_over: Option<DurationThreshold>,
}

#[proc_macro_attribute]
pub fn exec_time(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args: MacroArgs = match syn::parse(metadata) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().into();
        }
    };

    let print_arg = args.print.as_deref().unwrap_or("always");

    if print_arg == "always" || (print_arg == "debug" && cfg!(debug_assertions)) {
        let input_fn: syn::ItemFn = parse_macro_input!(input as syn::ItemFn);
        let visibility = input_fn.vis;
        let ident = input_fn.sig.ident;
        let inputs = input_fn.sig.inputs;
        let output = input_fn.sig.output;
        let generics = &input_fn.sig.generics;
        let where_clause = &input_fn.sig.generics.where_clause;
        let block = input_fn.block;
        let asyncness = input_fn.sig.asyncness;
        let label = build_label(&ident, &args);

        let print_stmt = print_statement(
            &label,
            args.unit.unwrap_or_default(),
            args.log_over,
            args.warn_over,
        );

        if asyncness.is_some() {
            (quote!(
                #visibility #asyncness fn #ident #generics (#inputs) #output #where_clause {
                    let start_time = std::time::Instant::now();
                    let f = || async { #block };
                    let r = f().await;
                    let elapsed = start_time.elapsed();
                    #print_stmt
                    r
                }
            ))
            .into()
        } else {
            (quote!(
                #visibility fn #ident #generics (#inputs) #output #where_clause {
                    let start_time = std::time::Instant::now();
                    let f = || { #block };
                    let r = f();
                    let elapsed = start_time.elapsed();
                    #print_stmt
                    r
                }
            ))
            .into()
        }
    } else {
        proc_macro::TokenStream::from(input).into()
    }
}

fn build_label(ident: &syn::Ident, args: &MacroArgs) -> String {
    if let Some(name) = &args.name {
        return name.clone();
    }

    let mut label = String::new();
    if let Some(prefix) = &args.prefix {
        label.push_str(prefix);
        label.push_str("::");
    }
    label.push_str(&ident.to_string());
    if let Some(suffix) = &args.suffix {
        label.push_str("::");
        label.push_str(suffix);
    }

    label
}

fn print_statement(
    label: &str,
    unit: TimeUnit,
    log_over: Option<DurationThreshold>,
    warn_over: Option<DurationThreshold>,
) -> TokenStream2 {
    let message = format_message(label, unit);
    let has_thresholds = log_over.is_some() || warn_over.is_some();
    let log_over = threshold_expression(log_over.map(DurationThreshold::as_nanos));
    let warn_over = threshold_expression(warn_over.map(DurationThreshold::as_nanos));

    if !has_thresholds {
        quote! {
            println!("{}", #message);
        }
    } else {
        quote! {
            let elapsed_nanos = elapsed.as_nanos();
            if let Some(threshold) = #warn_over {
                if elapsed_nanos >= threshold {
                    eprintln!("[exec_time][warn] {}", #message);
                } else if let Some(threshold) = #log_over {
                    if elapsed_nanos >= threshold {
                        println!("{}", #message);
                    }
                }
            } else if let Some(threshold) = #log_over {
                if elapsed_nanos >= threshold {
                    println!("{}", #message);
                }
            }
        }
    }
}

fn format_message(label: &str, unit: TimeUnit) -> TokenStream2 {
    match unit {
        TimeUnit::Ns => quote! {
            format!("[exec_time] {} took {} ns", #label, elapsed.as_nanos())
        },
        TimeUnit::Us => quote! {
            format!("[exec_time] {} took {} us", #label, elapsed.as_micros())
        },
        TimeUnit::Ms => quote! {
            format!("[exec_time] {} took {} ms", #label, elapsed.as_millis())
        },
        TimeUnit::S => quote! {
            format!("[exec_time] {} took {:.3} s", #label, elapsed.as_secs_f64())
        },
    }
}

fn threshold_expression(value: Option<u128>) -> TokenStream2 {
    match value {
        Some(value) => quote! { Some(#value) },
        None => quote! { None::<u128> },
    }
}

fn parse_duration_threshold(value: &str) -> darling::Result<Duration> {
    let trimmed = value.trim();
    let units = ["ns", "us", "ms", "s"];

    for unit in units {
        if let Some(number) = trimmed.strip_suffix(unit) {
            let amount = number.trim();
            if amount.is_empty() {
                return Err(Error::unknown_value(value));
            }

            return match unit {
                "ns" => amount
                    .parse::<u64>()
                    .map(Duration::from_nanos)
                    .map_err(|_| Error::unknown_value(value)),
                "us" => amount
                    .parse::<u64>()
                    .map(Duration::from_micros)
                    .map_err(|_| Error::unknown_value(value)),
                "ms" => amount
                    .parse::<u64>()
                    .map(Duration::from_millis)
                    .map_err(|_| Error::unknown_value(value)),
                "s" => amount
                    .parse::<f64>()
                    .map(Duration::from_secs_f64)
                    .map_err(|_| Error::unknown_value(value)),
                _ => Err(Error::unknown_value(value)),
            };
        }
    }

    Err(Error::unknown_value(value))
}
