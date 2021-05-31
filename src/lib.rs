use silang;

#[no_mangle]
pub extern "C" fn sil_load_lib(interpreter: &mut silang::Interpreter) {
    let current_scope = interpreter.context.current_scope();

    let mut pi = silang::Value::new();
    pi.identifier = Some("math.PI".to_owned());
    pi.float = Some(std::f64::consts::PI);
    pi.sil_type = silang::SILType::Float;
    interpreter.context.store_identifier(current_scope.scope_number, "math.PI", pi);
    let mut sin = silang::Value::new();
    sin.identifier = Some("math.sin".to_owned());
    sin.function = Some(MathExt::sin);
    sin.sil_type = silang::SILType::Float;
    interpreter.context.store_identifier(current_scope.scope_number, "math.sin", sin);
}

pub trait MathExt {
    fn sin(&mut self, args: &[silang::Value]) -> Result<silang::EvalReturn, String>;
}
impl MathExt for silang::Interpreter {
    fn sin(&mut self, args: &[silang::Value]) -> Result<silang::EvalReturn, String> {
        let mut evaluated_args = Vec::new();
        for arg in &args[1..] {
            match self.eval_value(arg, true) {
                Ok(result) => {
                    for v in result.values {
                        evaluated_args.push(v);
                    }
                },
                Err(e) => return Err(e),
            }
        }
        if evaluated_args.len() != 1 {
            return Err("sin: Argument length must be 1".to_owned())
        }
        let retval;
        match self.cast_value(&evaluated_args[0], silang::SILType::Float) {
            Ok(v) => {
                let mut val = v;
                val.float = Some(val.float.unwrap().sin());
                retval = val;
            },
            Err(e) => return Err(e),
        }
        Ok(
            silang::EvalReturn {
                result: silang::EvalResult::Normal,
                values: vec![retval],
            }
        )
    }
}
