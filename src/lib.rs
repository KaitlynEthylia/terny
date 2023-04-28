use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Token};
use syn::{Expr, Result};

struct Ternary {
	condition: Expr,
	truthy: Expr,
	falsey: Expr,
}

impl Parse for Ternary {
	fn parse(input: ParseStream) -> Result<Self> {
		let condition: Expr = input.parse()?;
		input.parse::<Token![?]>()?;
		let truthy: Expr = input.parse()?;
		input.parse::<Token![:]>()?;
		let falsey: Expr = input.parse()?;

		Ok(Ternary {
			condition,
			truthy,
			falsey,
		})
	}
}

/// The only macro exported by the `terny` crate.
/// A simple, C-like, ternary operator for cleaner syntax.
///
/// # Examples
///
/// ```
/// let left = iff!("some text" == "some other text"
///  ? "That's true!"
///  : "Not quite :(");
///
/// let right = if "some text" == "some other text" {
///     "That's true!"
/// } else {
///     "Not quite :("
/// };
///
/// assert_eq!(left, right);
/// ```
#[proc_macro]
pub fn iff(input: TokenStream) -> TokenStream {
	let Ternary {
		condition,
		truthy,
		falsey,
	} = parse_macro_input!(input as Ternary);

	let expanded = quote!(
	if #condition { #truthy } else { #falsey }
	);
	TokenStream::from(expanded)
}
