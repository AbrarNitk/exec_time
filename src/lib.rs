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
    print: Option<String>,
    #[darling(default)]
    prefix: Option<String>,
    #[darling(default)]
    suffix: Option<String>,
}

#[proc_macro_attribute]
pub fn exec_time(
    metadata: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let attr_args = parse_macro_input!(metadata as syn::AttributeArgs);
    let args: MacroArgs = match MacroArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return e.write_errors().into();
        }
    };

    let print_arg = args.print.unwrap_or("always".to_string());

    if print_arg.eq(&"always".to_string())
        || (print_arg.eq(&"debug".to_string()) && cfg!(debug_assertions))
    {
        let input_fn: syn::ItemFn = parse_macro_input!(input as syn::ItemFn);
        let visibility = input_fn.vis;
        let ident = input_fn.sig.ident;
        let inputs = input_fn.sig.inputs;
        let output = input_fn.sig.output;
        let generics = &input_fn.sig.generics;
        let where_clause = &input_fn.sig.generics.where_clause;
        let block = input_fn.block;
        let asyncness = input_fn.sig.asyncness;
        let mut print_str = "".to_string();
        if let Some(pre) = args.prefix {
            print_str.push_str(&format!("{}::", pre));
        }
        print_str.push_str(&ident.to_string());
        if let Some(suffix) = args.suffix {
            print_str.push_str(&format!("::{}", suffix));
        }

        if asyncness.is_some() {
            (quote!(
            #visibility #asyncness fn #ident #generics (#inputs) #output #where_clause {
                let start_time = std::time::Instant::now();
                let f = || async { #block };
                let r = f().await;
                println!("Time {}: {} mills", #print_str, (std::time::Instant::now() - start_time).as_millis());
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
                println!("Time {}: {} mills", #print_str, (std::time::Instant::now() - start_time).as_millis());
                r
            }
        ))
                .into()
        }
    } else {
        proc_macro::TokenStream::from(input).into()
    }
}
