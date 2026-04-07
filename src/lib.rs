#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
use darling::{Error, FromMeta};
use proc_macro2::TokenStream as TokenStream2;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
enum OutputBackend {
    Auto,
    Stdout,
    Tracing,
}

impl FromMeta for OutputBackend {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value {
            "auto" => Ok(Self::Auto),
            "stdout" => Ok(Self::Stdout),
            "tracing" => Ok(Self::Tracing),
            _ => Err(Error::unknown_value(value)),
        }
    }
}

impl Default for OutputBackend {
    fn default() -> Self {
        Self::Auto
    }
}

#[derive(Clone, Copy, Debug, Default)]
enum EventLevel {
    Trace,
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

impl FromMeta for EventLevel {
    fn from_string(value: &str) -> darling::Result<Self> {
        match value {
            "trace" => Ok(Self::Trace),
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            _ => Err(Error::unknown_value(value)),
        }
    }
}

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
    backend: Option<OutputBackend>,
    #[darling(default)]
    level: Option<EventLevel>,
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

        let emit_stmt = emit_statement(
            &label,
            args.backend.unwrap_or_default(),
            args.level.unwrap_or_default(),
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
                    #emit_stmt
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
                    #emit_stmt
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

fn emit_statement(
    label: &str,
    backend: OutputBackend,
    level: EventLevel,
    unit: TimeUnit,
    log_over: Option<DurationThreshold>,
    warn_over: Option<DurationThreshold>,
) -> TokenStream2 {
    let normal_emit = backend_emit_statement(label, backend, level, unit, false);
    let warn_emit = backend_emit_statement(label, backend, EventLevel::Warn, unit, true);
    let has_thresholds = log_over.is_some() || warn_over.is_some();
    let log_over = threshold_expression(log_over.map(DurationThreshold::as_nanos));
    let warn_over = threshold_expression(warn_over.map(DurationThreshold::as_nanos));

    if !has_thresholds {
        quote! {
            #normal_emit
        }
    } else {
        quote! {
            let elapsed_nanos = elapsed.as_nanos();
            if let Some(threshold) = #warn_over {
                if elapsed_nanos >= threshold {
                    #warn_emit
                } else if let Some(threshold) = #log_over {
                    if elapsed_nanos >= threshold {
                        #normal_emit
                    }
                }
            } else if let Some(threshold) = #log_over {
                if elapsed_nanos >= threshold {
                    #normal_emit
                }
            }
        }
    }
}

fn backend_emit_statement(
    label: &str,
    backend: OutputBackend,
    _level: EventLevel,
    unit: TimeUnit,
    _warn: bool,
) -> TokenStream2 {
    match backend {
        OutputBackend::Auto => auto_backend_emit_statement(label, _level, unit, _warn),
        OutputBackend::Stdout => stdout_emit_statement(label, unit, _warn),
        OutputBackend::Tracing => tracing_emit_statement(label, _level, unit),
    }
}

fn auto_backend_emit_statement(
    label: &str,
    _level: EventLevel,
    unit: TimeUnit,
    _warn: bool,
) -> TokenStream2 {
    #[cfg(feature = "tracing")]
    {
        tracing_emit_statement(label, _level, unit)
    }

    #[cfg(not(feature = "tracing"))]
    {
        stdout_emit_statement(label, unit, _warn)
    }
}

fn stdout_emit_statement(label: &str, unit: TimeUnit, warn: bool) -> TokenStream2 {
    let body = format_message_body(label, unit);

    if warn {
        quote! {
            eprintln!("[exec_time][warn] {}", #body);
        }
    } else {
        quote! {
            println!("[exec_time] {}", #body);
        }
    }
}

#[cfg(feature = "tracing")]
fn tracing_emit_statement(label: &str, level: EventLevel, unit: TimeUnit) -> TokenStream2 {
    let body = format_message_body(label, unit);
    let level = tracing_level_tokens(level);
    let elapsed_value = elapsed_value_expression(unit);
    let elapsed_unit = unit.as_str();

    quote! {
        ::tracing::event!(
            target: "exec_time",
            #level,
            label = #label,
            elapsed_ns = elapsed.as_nanos(),
            elapsed_unit = #elapsed_unit,
            elapsed_value = #elapsed_value,
            "{}",
            #body
        );
    }
}

#[cfg(not(feature = "tracing"))]
fn tracing_emit_statement(_label: &str, _level: EventLevel, _unit: TimeUnit) -> TokenStream2 {
    quote! {
        compile_error!("`backend = \"tracing\"` requires the `tracing` feature on `exec_time` and a direct `tracing` dependency in the consuming crate.");
    }
}

fn format_message_body(label: &str, unit: TimeUnit) -> TokenStream2 {
    match unit {
        TimeUnit::Ns => quote! {
            format!("{} took {} ns", #label, elapsed.as_nanos())
        },
        TimeUnit::Us => quote! {
            format!("{} took {} us", #label, elapsed.as_micros())
        },
        TimeUnit::Ms => quote! {
            format!("{} took {} ms", #label, elapsed.as_millis())
        },
        TimeUnit::S => quote! {
            format!("{} took {:.3} s", #label, elapsed.as_secs_f64())
        },
    }
}

#[cfg(feature = "tracing")]
fn tracing_level_tokens(level: EventLevel) -> TokenStream2 {
    match level {
        EventLevel::Trace => quote! { ::tracing::Level::TRACE },
        EventLevel::Debug => quote! { ::tracing::Level::DEBUG },
        EventLevel::Info => quote! { ::tracing::Level::INFO },
        EventLevel::Warn => quote! { ::tracing::Level::WARN },
        EventLevel::Error => quote! { ::tracing::Level::ERROR },
    }
}

#[cfg(feature = "tracing")]
fn elapsed_value_expression(unit: TimeUnit) -> TokenStream2 {
    match unit {
        TimeUnit::Ns => quote! { elapsed.as_nanos() },
        TimeUnit::Us => quote! { elapsed.as_micros() },
        TimeUnit::Ms => quote! { elapsed.as_millis() },
        TimeUnit::S => quote! { elapsed.as_secs_f64() },
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

#[cfg(feature = "tracing")]
impl TimeUnit {
    fn as_str(self) -> &'static str {
        match self {
            TimeUnit::Ns => "ns",
            TimeUnit::Us => "us",
            TimeUnit::Ms => "ms",
            TimeUnit::S => "s",
        }
    }
}
