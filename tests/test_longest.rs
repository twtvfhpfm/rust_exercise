//binary crate can't have integration test
use ref_;

#[test]
fn longest()
{
    let a = "你好";
    let b = "hello";
    assert_eq!(ref_::longest(a,b), a);
}