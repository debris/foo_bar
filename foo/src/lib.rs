pub struct Foo;

#[cfg(test)]
mod tests {
	extern crate bar;
	use self::bar::get_foo;

    #[test]
    fn it_works() {
		let _f = get_foo();
    }
}
