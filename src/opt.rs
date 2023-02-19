use syn::{Expr, GenericArgument, Path, PathArguments, PathSegment};

const OPTION_DECLARATIONS_PATHS: [&str; 3] = ["Option|", "std|option|Option", "core|option|Option"];

const OPTION_PATHS: [&str; 6] = [
    "Some|",
    "None|",
    "std|option|Option|Some|",
    "std|option|Option|None|",
    "core|option|Option|Some|",
    "core|option|Option|None|",
];

pub fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
    match *ty {
        syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
        _ => None,
    }
}

fn extract_option_declaration_segment(path: &Path) -> Option<&PathSegment> {
    let idents_of_path = path.segments.iter().fold(String::new(), |mut acc, v| {
        acc.push_str(&v.ident.to_string());
        acc.push('|');
        acc
    });

    if OPTION_DECLARATIONS_PATHS.contains(&idents_of_path.as_str()) {
        path.segments.last()
    } else {
        None
    }
}

fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
    let idents_of_path = path.segments.iter().fold(String::new(), |mut acc, v| {
        acc.push_str(&v.ident.to_string());
        acc.push('|');
        acc
    });

    if OPTION_PATHS.contains(&idents_of_path.as_str()) {
        path.segments.last()
    } else {
        None
    }
}

pub fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
    extract_type_path(ty)
        .and_then(extract_option_declaration_segment)
        .and_then(|path_seg| {
            let type_params = &path_seg.arguments;
            // It should have only on angle-bracketed param ("<String>"):
            match *type_params {
                PathArguments::AngleBracketed(ref params) => params.args.first(),
                _ => None,
            }
        })
        .and_then(|generic_arg| match *generic_arg {
            GenericArgument::Type(ref ty) => Some(ty),
            _ => None,
        })
}

pub fn is_option_wrapped(expr: &syn::Expr) -> bool {
    match expr {
        Expr::Call(call) => {
            let mut func_name_valid = false;
            if let Expr::Path(path) = call.func.as_ref() {
                if let Some(segment) = extract_option_segment(&path.path) {
                    if segment.ident == "Some" {
                        func_name_valid = true
                    }
                }
            }

            return call.args.len() == 1 && func_name_valid;
        }
        Expr::Path(path) => {
            if path.qself.is_some() {
                return false;
            }

            if let Some(segment) = extract_option_segment(&path.path) {
                if segment.ident == "None" {
                    if let PathArguments::None = segment.arguments {
                        return true;
                    }
                }
            }
        }
        _ => (),
    };

    false
}
