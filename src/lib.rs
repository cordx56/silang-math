extern crate silang;

#[no_mangle]
pub extern "C" fn sil_load_lib(ctx: &mut silang::Context) {
    let current_scope = ctx.current_scope();

    let mut pi = silang::Factor::new();
    pi.kind = silang::FactorKind::Float;
    pi.float = Some(std::f64::consts::PI);
    ctx.identifier_storage[current_scope].insert(
        "math.PI".to_owned(),
        pi
    );
}
