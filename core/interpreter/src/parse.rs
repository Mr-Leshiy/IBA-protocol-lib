use syn::{ExprStruct, Ident, Token, parse::Parse, punctuated::Punctuated};

pub struct ScriptDefinition {
    pub name: Ident,
    pub op_codes: Punctuated<ExprStruct, Token![,]>,
}

impl Parse for ScriptDefinition {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // parse the name of the variable
        let name = input.parse()?;
        // parse ',' identifier if present
        match input.parse::<Option<Token![,]>>()? {
            // parse op_codes
            Some(_) => Ok(Self {
                name,
                op_codes: Punctuated::parse_separated_nonempty(input)?,
            }),
            // initialize with the empty op_codes
            None => Ok(Self {
                name,
                op_codes: Punctuated::new(),
            }),
        }
    }
}
