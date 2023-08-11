use syn::*;

#[proc_macro_derive(Builder)]
pub fn builder_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    dbg!(derive_builder::derive(input)
        .unwrap_or_else(|err| err.into_compile_error())
        .into())
}

mod derive_builder {
    use proc_macro2::*;
    use quote::quote;
    use syn::punctuated::Punctuated;
    use syn::token::Comma;
    use syn::*;

    pub fn derive(input: DeriveInput) -> Result<TokenStream> {
        let crate_name: syn::Path = parse_quote!(::comptime_builder);
        let fields = match input.data {
            Data::Struct(DataStruct {
                fields: Fields::Named(FieldsNamed { named, .. }),
                ..
            }) => named,
            _ => {
                return Err(syn::Error::new_spanned(
                    input,
                    "expected a struct with named fields",
                ))
            }
        };

        let struct_name = &input.ident;
        let builder_struct_name = Ident::new(&format!("{}Builder", struct_name), Span::call_site());

        // Use Generic type T{0..} to represent the fields
        let mut state = 1u32;
        let generic_fields = fields.into_iter().scan(&mut state, |state, mut field| {
            let gtype = format!("T{}", state);
            **state += 1;
            field.ty = syn::Type::Path(TypePath {
                qself: None,
                path: syn::Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter(vec![PathSegment {
                        ident: syn::Ident::new(&gtype, Span::call_site()),
                        arguments: PathArguments::None,
                    }]),
                },
            });
            Some(field)
        });
        let generic_fields: Punctuated<Field, Comma> = generic_fields.collect();
        let builder_generics = syn::Generics {
            lt_token: Token![<](Span::call_site()).into(),
            params: (1..state)
                .map(|f| {
                    GenericParam::Type(TypeParam {
                        attrs: vec![],
                        ident: syn::Ident::new(&format!("T{}", f), Span::call_site()),
                        colon_token: None,
                        bounds: Punctuated::new(),
                        eq_token: None,
                        default: None,
                    })
                })
                .collect(),
            gt_token: Token![>](Span::call_site()).into(),
            where_clause: None,
        };

        let builder_struct = syn::ItemStruct {
            attrs: vec![],
            vis: Visibility::Inherited,
            struct_token: Token![struct](Span::call_site()),
            ident: builder_struct_name.clone(),
            fields: Fields::Named(FieldsNamed {
                brace_token: token::Brace::default(),
                named: generic_fields,
            }),
            generics: builder_generics,
            semi_token: None,
        };

        let empty_builder_type: syn::Type = syn::Type::Path(TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: Punctuated::from_iter(vec![PathSegment {
                    ident: builder_struct_name.clone(),
                    arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                        colon2_token: None,
                        lt_token: Token![<](Span::call_site()),
                        args: core::iter::repeat::<GenericArgument>(parse_quote!(
                            #crate_name::Empty
                        ))
                        .take(state as usize - 1)
                        .collect(),
                        gt_token: Token![>](Span::call_site()),
                    }),
                }]),
            },
        });

        let empty_builder = syn::ExprStruct {
            attrs: vec![],
            qself: None,
            path: parse_quote!(#builder_struct_name),
            brace_token: token::Brace::default(),
            fields: core::iter::repeat::<FieldValue>(parse_quote!(
                _field: #crate_name::Empty
            ))
            .take(state as usize - 1)
            .collect(),
            dot2_token: None,
            rest: None,
        };

        let empty_builder_fn = syn::ImplItemFn {
            attrs: vec![],
            vis: Visibility::Inherited,
            defaultness: None,
            sig: parse_quote!(fn builder() -> #empty_builder_type),
            block: parse_quote! {{
                #empty_builder
            }},
        };

        let ge = input.generics.params.clone();
        let self_ty = parse_quote!(#struct_name<#ge>);
        let impl_empty_builder_fn = syn::ItemImpl {
            attrs: vec![],
            defaultness: None,
            unsafety: None,
            impl_token: Token![impl](Span::call_site()),
            generics: input.generics.clone(),
            trait_: None,
            self_ty: Box::new(self_ty),
            brace_token: token::Brace::default(),
            items: vec![ImplItem::Fn(empty_builder_fn)],
        };

        Ok(quote! {
            #impl_empty_builder_fn
            #builder_struct
        })
    }
}
