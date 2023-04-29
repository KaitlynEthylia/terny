use proc_macro::{Delimiter, Group, Ident, Spacing, Span, TokenStream, TokenTree};

enum ParseState {
	Normal,
	TrueClause,
	FalseClause,
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
	let iter = input.into_iter();
	let iif = Ident::new("if", Span::call_site());

	let mut output: Vec<TokenTree> = vec![];
	let mut true_clause: Vec<TokenTree> = vec![];
	let mut false_clause: Vec<TokenTree> = vec![];
	output.push(TokenTree::Ident(iif));

	let mut state = ParseState::Normal;

	for token in iter {
		if let TokenTree::Punct(ref punc) = token {
			if punc.spacing() == Spacing::Alone {
				match (punc.as_char(), &state) {
					('?', &ParseState::Normal) => {
						state = ParseState::TrueClause;
						continue;
					},
					(':', &ParseState::TrueClause) => {
						state = ParseState::FalseClause;
						continue;
					},
					_ => {},
				}
			}
		}

		match state {
			ParseState::Normal => output.push(token),
			ParseState::TrueClause => true_clause.push(token),
			ParseState::FalseClause => false_clause.push(token),
		}
	}
	let true_group = Group::new(
		Delimiter::Brace,
		TokenStream::from_iter(true_clause.into_iter()),
	);
	let ielse = Ident::new("else", Span::call_site());
	let false_group = Group::new(
		Delimiter::Brace,
		TokenStream::from_iter(false_clause.into_iter()),
	);

	output.push(TokenTree::Group(true_group));
	output.push(TokenTree::Ident(ielse));
	output.push(TokenTree::Group(false_group));

	TokenStream::from_iter(output.into_iter())
}
