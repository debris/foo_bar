extern crate foo;

pub fn get_foo() -> foo::Foo {
	foo::Foo
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
