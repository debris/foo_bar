pub struct Foo;

#[cfg(test)]
mod tests {
	extern crate bar;
	use self::bar::get_foo;
	use super::Foo;

    #[test]
    fn it_works() {
		let _f: Foo = get_foo();
    }
}
