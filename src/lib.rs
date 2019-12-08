#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;
extern crate darling;
extern crate proc_macro;
use darling::FromMeta;

#[derive(Debug, FromMeta)]
struct MacroArgs {
    #[darling(default)]
    print: Option<bool>,
    #[darling(default)]
    prefix: Option<String>,
    #[darling(default)]
    suffix: Option<String>,
}

#[proc_macro_attribute]
pub fn measure_time(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr_args = parse_macro_input!(metadata as syn::AttributeArgs);
    let input_fn: syn::ItemFn = parse_macro_input!(input as syn::ItemFn);
    let args: MacroArgs = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let visibility = input_fn.vis;
    let ident = input_fn.ident;
    let inputs = input_fn.decl.inputs;
    let output = input_fn.decl.output;
    let block = input_fn.block;
    let mut print_str = "".to_string();
    if let Some(pre) = args.prefix {
        print_str.push_str(&format!("{}::", pre));
    }
    print_str.push_str(&ident.to_string());
    if let Some(suffix) = args.suffix {
        print_str.push_str(&format!("::{}", suffix));
    }
    if args.print.unwrap_or(true) {
        (quote!(
            #visibility fn #ident(#inputs) #output {
                let start_time = std::time::Instant::now();
                let r = || { #block };
                let r = r();
                let total_time = std::time::Instant::now() - start_time;
                println!("Time {}: {} mills", #print_str, total_time.as_millis());
                r
            }
        ))
        .into()
    } else {
        quote!(input).into()
    }
}

