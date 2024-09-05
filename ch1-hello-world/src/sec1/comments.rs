

/*
모든 프로그램에는 주석이 필요하며 Rust는 몇 가지 다른 종류를 지원합니다.

컴파일러가 무시하는 일반적인 주석 :
// Line comments which go to the end of the line.
/* Block comments which go to the closing delimiter. */
HTML 라이브러리 문서 로 구문 분석되는 문서 주석 :
/// Generate library docs for the following item.
//! Generate library docs for the enclosing item.
 */
#[allow(unused)]
pub fn comments() {
    // 이것은 줄 주석의 예입니다.
    // 줄의 시작 부분에 슬래시 두 개가 있습니다.
    // 그리고 이 뒤에 쓰여진 것은 컴파일러가 읽지 않습니다.

    // println!("Hello, world!");

    // 실행해 보세요. 알겠죠? 이제 두 개의 슬래시를 삭제하고 다시 실행해 보세요.

    /*
    * 이것은 또 다른 유형의 주석, 블록 주석입니다. 일반적으로
    * 줄 주석은 권장되는 주석 스타일입니다. 하지만 블록 주석은
    * 코드 청크를 일시적으로 비활성화하는 데 매우 유용합니다.
    * /* 블록 주석은 /* 중첩될 수 있으므로 */ */ 이 main() 함수의 모든 것을
    * 주석 처리하는 데 몇 번의 키 입력만 있으면 됩니다.
    * /*/*/* 직접 시도해 보세요! */*/*/
    */

    /*
    참고: 이전 열의 `*`는 전적으로 스타일을 위한 것이었습니다.
    실제로 필요하지 않습니다.
    */

    // 줄 주석보다 블록 주석으로 표현식을 더 쉽게 조작할 수 있습니다.
    // 주석 구분 기호를 삭제해 보세요
    // 결과를 변경하려면:
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}