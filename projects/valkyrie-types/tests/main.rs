use nyar_error::SourceCache;
use std::{io::Write, path::Path};
use valkyrie_types::ResolveContext;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_hello_world() {
    let here = Path::new(env!("CARGO_MANIFEST_DIR"));

    let mut file_cache = SourceCache::default();
    let file = file_cache.load_local(here.join("tests/component.vk")).unwrap();
    let mut context = ResolveContext::new("std");
    let errors = context.parse(file, &mut file_cache);
    for error in errors {
        error.as_report().print(&file_cache).ok();
    }
    println!("{:#?}", context);

    let mut wat = std::fs::File::create(here.join("tests/component.wat")).unwrap();
    let source = context.resolve().unwrap();
    let wast = source.encode();
    wat.write_all(wast.as_bytes()).unwrap();
}
