use syn::{parse::Parse, punctuated::Punctuated, ExprStruct, Ident, Token};

pub struct OpCodeDefinition {
    pub name: Ident,
}

impl Parse for OpCodeDefinition {
    // parse OpCodeDefition as ExprStruct
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr_struct: ExprStruct = input.parse()?;

        // get name of the OpCode
        let name = expr_struct.path.segments.last().unwrap().ident.clone();

        Ok(Self { name })
    }
}

pub struct ScriptDefinition {
    pub name: Ident,
    pub op_codes: Punctuated<OpCodeDefinition, Token![,]>,
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
