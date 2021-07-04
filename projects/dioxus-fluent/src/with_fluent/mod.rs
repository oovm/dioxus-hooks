use std::ops::Deref;
use dioxus::hooks::UseState;
use dioxus::prelude::UseSharedState;
use fluent::{FluentBundle, FluentValue, FluentResource, FluentArgs};

// Used to provide a locale for the bundle.
use unic_langid::LanguageIdentifier;


pub struct FluentContext {
    res: FluentResource,
}
pub struct UseFluent<'a, I> {
    bundle : &'a UseState<FluentBundle<I>>
}

impl UseFluent<I> {
    pub fn new(res: &FluentResource, locales: Vec<LanguageIdentifier>) {

        let res = FluentResource::try_new(ftl_string)
            .expect("Failed to parse an FTL string.");


        let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
        let mut bundle = FluentBundle::new(vec![langid_en]);

        bundle
            .add_resource(res)
            .expect("Failed to add FTL resources to the bundle.");
    }
}

// impl<'a, I> Deref for UseFluent<'a, I> {
//     type Target = FluentBundle<I>;
//
//     fn deref(&self) -> &Self::Target {
//         self.bundle.get()
//     }
// }


impl<'a, I>  UseFluent<'a, I> {
    pub fn get_message() {
        let msg = bundle.get_message("hello-world")
            .expect("Message doesn't exist.");




        let mut errors = vec![];
        let pattern = msg.value()
            .expect("Message has no value.");
        let value = bundle.format_pattern(&pattern, None, &mut errors);

        assert_eq!(&value, "Hello, world!");
    }
    pub fn get_format() {
        let mut args = FluentArgs::new();
        args.set("name", FluentValue::from("John"));

        let msg = bundle.get_message("intro")
            .expect("Message doesn't exist.");
        let mut errors = vec![];
        let pattern = msg.value().expect("Message has no value.");
        let value = bundle.format_pattern(&pattern, Some(&args), &mut errors);
    }
}

#[test]
fn example() {
    let ftl_string = String::from("
hello-world = Hello, world!
intro = Welcome, { $name }.
");
    let res = FluentResource::try_new(ftl_string)
        .expect("Failed to parse an FTL string.");


    let langid_en: LanguageIdentifier = "en-US".parse().expect("Parsing failed");
    let mut bundle = FluentBundle::new(vec![langid_en]);

    bundle
        .add_resource(res)
        .expect("Failed to add FTL resources to the bundle.");


    let bundle = use_fluent();







// The FSI/PDI isolation marks ensure that the direction of
// the text from the variable is not affected by the translation.
    assert_eq!(value, "Welcome, \u{2068}John\u{2069}.");
}



pub fn use_fluent() -> UseFluent<I> {

}

pub fn use_fluent_id() -> UseFluent<I> {

}
