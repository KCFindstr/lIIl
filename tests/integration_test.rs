mod test_module;

use lIIl::module::CodeModule;
use test_module::TestModule;

fn exec_program(file: &str) -> CodeModule {
    let path = format!("tests/{}", file);
    let mut module = lIIl::parser::parse_file(&path, None).unwrap();
    {
        // Register test module.
        let ctx = module.ctx.clone();
        let global = ctx.borrow().get_global();
        let manager = &mut global.borrow_mut().builtin_modules;
        TestModule::register(manager);
    }
    module.exec().unwrap();
    module
}

#[test]
fn test_assert() {
    exec_program("assert.lIIl");
}
