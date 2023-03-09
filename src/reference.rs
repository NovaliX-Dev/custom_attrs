use proc_macro2::{Group, Ident, Span, TokenStream, TokenTree};
use proc_macro_error::{emit_error, SpanRange};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token,
};

pub struct ReferenceTokens {
    tokens: TokenStream,
    last_span: Option<Span>,
}

impl Parse for ReferenceTokens {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let (mut tokens, add_comma) = get_tokens(&input);

        // trick to have the correct span for the unexpected end error
        if tokens.is_empty() {
            let comma = token::Comma {
                spans: [input.span()],
            };
            comma.to_tokens(&mut tokens);

            return Ok(Self {
                tokens,
                last_span: None,
            });
        }

        let last_span = if add_comma { Some(input.span()) } else { None };

        Ok(Self { tokens, last_span })
    }
}

impl ToTokens for ReferenceTokens {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.tokens.to_tokens(tokens);

        // trick to have the correct span for the unexpected end error
        if let Some(last_span) = self.last_span {
            let comma = token::Comma { spans: [last_span] };
            comma.to_tokens(tokens);
        }
    }
}

fn get_tokens(input: &ParseStream) -> (TokenStream, bool) {
    input
        .step(|cursor| {
            let mut tokens = TokenStream::new();
            let mut add_comma = true;

            let mut cursor = *cursor;
            while let Some((tt, next)) = cursor.token_tree() {
                if let TokenTree::Punct(punct) = &tt {
                    if punct.as_char() == ',' {
                        add_comma = false;
                        break;
                    }
                }

                tt.to_tokens(&mut tokens);

                cursor = next;
            }

            Ok(((tokens, add_comma), cursor))
        })
        .unwrap()
}

pub struct ReferenceProcessor {
    tokens: TokenStream,
    references: Vec<ReferenceList>,
    span: SpanRange,
}

impl ReferenceProcessor {
    pub fn into_parts(self) -> (TokenStream, Vec<ReferenceList>, SpanRange) {
        (self.tokens, self.references, self.span)
    }

    pub fn parse(tokens: TokenStream) -> Result<Self, ()> {
        let span = SpanRange::from_tokens(&tokens);

        let mut references = Vec::new();
        let (new_tokens, errored) = parse_references(tokens, &mut references);
        if errored {
            return Err(());
        }

        Ok(Self {
            tokens: new_tokens,
            references,
            span,
        })
    }
}

impl ToTokens for ReferenceProcessor {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.tokens.to_tokens(tokens);
    }
}

pub struct ReferenceList {
    name: Reference,
    list: Vec<Reference>,
}

impl ReferenceList {
    pub fn name(&self) -> &Reference {
        &self.name
    }

    pub fn into_list(self) -> Vec<Reference> {
        self.list
    }

    fn new(name: Reference) -> Self {
        Self {
            name,
            list: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct Reference {
    name: String,
    spans: Vec<Span>,
}

impl Reference {
    fn new(name: String, span: Span) -> Self {
        Self {
            name,
            spans: vec![span],
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn emit_error(&self, msg: &str) {
        for span in &self.spans {
            emit_error!(span, msg);
        }
    }
}

enum State {
    None,
    ExpectingReference,
    ExpectingPoint,
    ExpectingIdent,
}

fn parse_references(
    tokens: TokenStream,
    references: &mut Vec<ReferenceList>,
) -> (TokenStream, bool) {
    let mut new_tokens = TokenStream::new();
    let mut state = State::None;

    let mut last_reference = String::new();
    let mut last_reference_index = 0_usize;

    let mut last_span = tokens.span();

    let mut errored = false;

    for tt in tokens {
        last_span = tt.span();

        match state {
            State::None => {
                if let TokenTree::Group(group) = &tt {
                    let (tokens2, errored2) = parse_references(group.stream(), references);

                    let mut new_group = Group::new(group.delimiter(), tokens2);
                    new_group.set_span(group.span());
                    new_group.to_tokens(&mut new_tokens);

                    errored = errored || errored2;
                    continue;
                }

                if let TokenTree::Punct(punct) = &tt {
                    if punct.as_char() == '#' {
                        state = State::ExpectingReference;
                        continue;
                    }
                }
            }
            State::ExpectingReference => {
                // pound are also used in attributes
                if let TokenTree::Group(_) = &tt {
                    state = State::None;
                    continue;
                }

                if let TokenTree::Ident(ident) = &tt {
                    let ident_str = ident.to_string();
                    match ident_str.as_str() {
                        "self" => {
                            last_reference = ident_str.to_owned();

                            let match_ = references
                                .iter()
                                .enumerate()
                                .find(|(_i, r)| r.name.name == "self");

                            if let Some((i, _)) = match_ {
                                last_reference_index = i;
                            } else {
                                last_reference_index = references.len();
                                references.push(ReferenceList::new(Reference::new(
                                    ident_str,
                                    ident.span(),
                                )));
                            }
                        }
                        _ => {
                            emit_error!(ident, "Unknown reference.");
                            errored = true;
                        }
                    }

                    state = State::ExpectingPoint;
                    continue;
                }

                emit_error!(tt.span(), "Expecting reference ident or `[`.");
                state = State::None;
                errored = true;
                continue;
            }
            State::ExpectingPoint => {
                if let TokenTree::Punct(punct) = &tt {
                    if punct.as_char() == '.' {
                        state = State::ExpectingIdent;
                        continue;
                    }
                }

                emit_error!(tt.span(), "Expecting `.`.");
                state = State::None;
                errored = true;
                continue;
            }
            State::ExpectingIdent => {
                let (ident_str, span) = match &tt {
                    TokenTree::Ident(ident) => {
                        ident.to_tokens(&mut new_tokens);

                        (ident.to_string(), ident.span())
                    }
                    TokenTree::Literal(lit) => {
                        let lit: syn::Lit = syn::parse2(lit.to_token_stream()).unwrap();
                        let lit_int = match lit {
                            syn::Lit::Int(int) => int,
                            _ => {
                                emit_error!(tt.span(), "Expecting ident or integer.");
                                state = State::None;
                                errored = true;
                                continue;
                            }
                        };

                        let lit_str = lit_int.to_string();
                        let field_str = format!("_{0}", lit_str);
                        let lit_int = Ident::new(&field_str, lit_int.span());

                        lit_int.to_tokens(&mut new_tokens);

                        (lit_str, lit_int.span())
                    }
                    _ => {
                        emit_error!(tt.span(), "Expecting ident.");
                        errored = true;
                        state = State::None;
                        continue;
                    }
                };

                let list = &mut references[last_reference_index].list;
                let match_ = list.iter_mut().find(|r| r.name == ident_str.as_str());

                if let Some(ref_) = match_ {
                    ref_.spans.push(span)
                } else {
                    list.push(Reference::new(ident_str, span))
                }

                state = State::None;
                continue;
            }
        }

        tt.to_tokens(&mut new_tokens);
    }

    match state {
        State::None => (),
        State::ExpectingReference => {
            emit_error!(last_span, "Expecting reference ident or `[`.");
            errored = true;
        }
        State::ExpectingPoint => {
            emit_error!(last_span, "Expecting `.`.");
            errored = true;
        }
        State::ExpectingIdent => {
            match last_reference.as_str() {
                "self" => {
                    emit_error!(last_span, "Expecting field ident.");
                }
                _ => panic!("Unknown reference."),
            }
            errored = true;
        }
    }

    (new_tokens, errored)
}
