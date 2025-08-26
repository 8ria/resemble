use resemble::{similarity::similarity_from_counts};
use resemble::ast::NodeCounter;
use syn::{ItemFn, visit::Visit};

#[test]
fn test_similarity_same_small_snippet() {
    let a = "fn foo() { let x = 1; println!(\"{}\", x); }";
    let b = "fn foo() { let x = 2; println!(\"{}\", x); }";

    let counts_a = syn::parse_str::<ItemFn>(a).map(|f| {
        let mut v = NodeCounter::default();
        v.visit_item_fn(&f);
        v.counts
    }).unwrap();

    let counts_b = syn::parse_str::<ItemFn>(b).map(|f| {
        let mut v = NodeCounter::default();
        v.visit_item_fn(&f);
        v.counts
    }).unwrap();

    let sim = similarity_from_counts(counts_a, counts_b);
    assert!(sim > 0.0);
}
