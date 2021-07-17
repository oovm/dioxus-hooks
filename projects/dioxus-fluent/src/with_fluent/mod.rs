use dioxus::{
    core::ScopeState,
    hooks::{use_context, use_context_provider, UseState},
    prelude::UseSharedState,
};
use fluent::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use log::warn;
use std::{borrow::Cow, ops::Deref};

// Used to provide a locale for the bundle.
use unic_langid::LanguageIdentifier;

pub struct FluentContext {
    res: FluentResource,
}

pub struct UseFluent<'a> {
    bundle: &'a UseState<FluentBundle<FluentResource>>,
}

impl FluentContext {
    pub fn new(cx: &ScopeState) {
        use_context_provider(cx, Self::provider())
    }
    fn provider() -> FluentBundle<FluentResource> {}
}

impl UseFluent {
    pub fn new(cx: &ScopeState, res: &FluentResource, locales: Vec<LanguageIdentifier>) -> Option<Self> {
        let ctx = use_context::<FluentBundle<FluentResource>>(cx)?;

        Self { bundle: &ctx };

        let res = FluentResource::try_new(ftl_string)?;
        let langid_en: LanguageIdentifier = "en-US".parse()?;
        let mut bundle = FluentBundle::new(vec![langid_en]);
        bundle.add_resource(res)?;
    }
}

// impl<'a, I> Deref for UseFluent<'a, I> {
//     type Target = FluentBundle<I>;
//
//     fn deref(&self) -> &Self::Target {
//         self.bundle.get()
//     }
// }

impl<'a> UseFluent<'a> {
    pub fn get_message(&self, id: &str) -> Option<Cow<str>> {
        let bundle = self.bundle.get();
        let mut errors = vec![];
        let msg = bundle.get_message(id)?;
        let value = bundle.format_pattern(msg.value()?, None, &mut errors);
        for e in errors {
            warn!("{}", e);
        }
        Some(value)
    }
    pub fn get_format(&self, id: &str, args: FluentArgs) -> Option<Cow<str>> {
        let bundle = self.bundle.get();
        let mut errors = vec![];
        let msg = bundle.get_message(id)?;
        let value = bundle.format_pattern(msg.value()?, Some(&args), &mut errors);
        for e in errors {
            warn!("{}", e);
        }
        Some(value)
    }
}

#[test]
fn example() {
    let ftl_string = String::from(
        "
hello-world = Hello, world!
intro = Welcome, { $name }.
",
    );
    let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");

    let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
    let mut bundle = FluentBundle::new(vec![langid_en]);

    bundle.add_resource(res).expect("Failed to add FTL resources to the bundle.");

    let bundle = use_fluent();

    // The FSI/PDI isolation marks ensure that the direction of
    // the text from the variable is not affected by the translation.
    assert_eq!(value, "Welcome, \u{2068}John\u{2069}.");
}

pub fn use_fluent() -> UseFluent<I> {}

pub fn use_fluent_id() -> UseFluent<I> {}
