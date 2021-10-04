pub mod sinsemilla;

#[test]
fn main() {
    println!("========================");
    let domain = sinsemilla::HashDomain::new("test");
    let message = [
        true, true, false, true, false, true, false, true, false, true, true,
    ]
    .iter()
    .cloned();
    let p = domain.hash_to_point(message);
    println!("Hello {:?}", p);
    println!("========================");
}
