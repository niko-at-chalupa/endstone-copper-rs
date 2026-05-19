use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemImpl};

/// Usage:
/// ```rust
/// use endstone::{Plugin, endstone_plugin};
///
/// struct MyPlugin;
///
/// #[endstone_plugin(
///     name = "my-plugin",
///     version = "0.1.0",
///     description = "Does stuff",
///     author = "you",
/// )]
/// impl Plugin for MyPlugin {
///     fn on_enable(&mut self) {
///         // ...
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn endstone_plugin(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    let attr_str = attr.to_string();
    let name        = extract_attr(&attr_str, "name")       .unwrap_or("unnamed");
    let version     = extract_attr(&attr_str, "version")    .unwrap_or("0.0.0");
    let description = extract_attr(&attr_str, "description");
    let author      = extract_attr(&attr_str, "author");

    let self_ty = &input.self_ty;

    let desc_tokens = match description {
        Some(d) => quote! { Some(#d) },
        None    => quote! { None },
    };
    let author_tokens = match author {
        Some(a) => quote! { Some(#a) },
        None    => quote! { None },
    };

    let expanded = quote! {
        #input

        #[unsafe(no_mangle)]
        pub extern "C" fn endstone_rs_init() {
            endstone_copper::plugin::register_plugin(
                <#self_ty as Default>::default(),
                endstone_copper::PluginMeta {
                    name:        #name,
                    version:     #version,
                    description: #desc_tokens,
                    author:      #author_tokens,
                },
            );
        }
    };

    expanded.into()
}

fn extract_attr<'a>(attr: &'a str, key: &str) -> Option<&'a str> {
    let needle = format!("{} = \"", key);
    let start  = attr.find(&needle)? + needle.len();
    let end    = attr[start..].find('"')? + start;
    Some(&attr[start..end])
}