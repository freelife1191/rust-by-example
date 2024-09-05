/*
    std::fmt포맷팅을 사용하려는 모든 유형은 traits인쇄 가능한 구현이 필요합니다
    자동 구현은 라이브러리와 같은 유형에만 제공됩니다 std. 다른 모든 것은 어떻게든 수동으로 구현 해야 합니다

    이것은 fmt::Debug trait매우 간단합니다. 모든 유형은 구현을 derive(자동으로) 만들 수 있습니다
    이는 수동으로 구현해야 하는 것에 fmt::Debug는 해당되지 않습니다 .fmt::Display
 */


// `Structure`에 대한 `fmt::Debug` 구현을 파생합니다. `Structure`
//는 단일 `i32`를 포함하는 구조체입니다.
#[derive(Debug)]
#[allow(warnings)]
struct Structure(i32);

// `Structure`를 구조체 `Deep` 내부에 넣습니다. 인쇄 가능하게 만듭니다.
// also.
#[derive(Debug)]
#[allow(dead_code)]
struct Deep(Structure);

/*
    모든 std도서관 유형은 {:?}다음과 같은 방법으로 자동으로 인쇄할 수 있습니다
 */
#[allow(unused)]
pub fn debug() {
    // `{:?}`로 인쇄하는 것은 `{}`로 인쇄하는 것과 비슷합니다.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure`가 인쇄 가능합니다!
    println!("Now {:?} will print!", Structure(3));

    // `derive`의 문제점은 결과가 어떻게 보일지 제어할 수 없다는 것입니다.
    // `7`만 표시하고 싶다면 어떻게 할까요?
    println!("Now {:?} will print!", Deep(Structure(7)));
}


/*
    그래서 fmt::Debug확실히 이것을 인쇄 가능하게 만들지만 우아함을 희생합니다. Rust는 또한 {:#?}
    fmt::Display디스플레이를 제어하기 위해 수동으로 구현할 수 있습니다

    또한 참조하세요:
    - attributes: https://doc.rust-lang.org/reference/attributes.html
    - derive: https://doc.rust-lang.org/rust-by-example/trait/derive.html
    - std::fmt: https://doc.rust-lang.org/std/fmt/
    - struct: https://doc.rust-lang.org/rust-by-example/custom_types/structs.html
 */
#[derive(Debug)]
#[allow(dead_code)]
struct Person<'a> {
    name: &'a str,
    age: u8
}
#[allow(unused)]
pub fn debug_person() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
}