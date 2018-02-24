extern crate foo;
extern crate bar;

#[test]
fn it_works() {
	let _f: foo::Foo = bar::get_foo();
}
