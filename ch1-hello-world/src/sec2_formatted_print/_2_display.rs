/*
    fmt::Debug거의 컴팩트하고 깔끔해 보이지 않으므로 출력 모양을 사용자 정의하는 것이 종종 유리합니다
    이는 인쇄 마커를 fmt::Display사용하는 를 수동으로 구현하여 수행됩니다 {}. 구현은 다음과 같습니다.
 */

// `fmt` 모듈을 가져와서(`use`를 통해) 사용 가능하게 합니다.
use std::fmt;

// `fmt::Display`가 구현될 구조체를 정의합니다. 이것은
// `i32`를 포함하는 `Structure`라는 튜플 구조체입니다.
struct Structure(i32);

// `{}` 마커를 사용하려면 `fmt::Display` 트레이트를
// 해당 유형에 대해 수동으로 구현해야 합니다.
impl fmt::Display for Structure {
    // 이 트레이트에는 이 정확한 서명이 있는 `fmt`가 필요합니다.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 제공된 출력에 첫 번째 요소를 엄격하게 씁니다.
        // 스트림: `f`. `fmt::Result`를 반환합니다. 이는 다음 중 어느 것인지 나타냅니다.
        // 작업이 성공했거나 실패했습니다. `write!`는 다음과 같은 구문을 사용합니다.
        //는 `println!`와 매우 유사합니다.
        write!(f, "{}", self.0)
    }
}

// 두 개의 숫자를 보관하는 구조. `Debug`가 파생되어 결과를
// `Display`와 대조할 수 있습니다.
#[derive(Debug)]
struct MinMax(i64, i64);

// `MinMax`에 대해 `Display`를 구현합니다.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `self.number`를 사용하여 각 위치 데이터 포인트를 참조합니다.
        write!(f, "({}, {})", self.0, self.1)
    }
}


/*
    fmt::Display보다 깔끔할 수 있지만 라이브러리 fmt::Debug에 문제가 std됩니다. 모호한 유형은 어떻게 표시해야 합니까?
    예를 들어 std라이브러리가 모든 에 대해 단일 스타일을 구현한 경우 Vec<T>어떤 스타일이어야 합니까? 이 두 가지 중 하나일까요?

    Vec<path>: /:/etc:/home/username:/bin(분할 :)
    Vec<number>: 1,2,3(분할 ,)
    아니요. 모든 유형에 이상적인 스타일이란 없고, std라이브러리가 어떤 스타일을 지시한다고 가정하지도 않기 때문입니다
    는 다른 일반 컨테이너에 fmt::Display구현되지 않았습니다
    따라서 이러한 일반적인 경우에는 를 사용해야 합니다.Vec<T>fmt::Debug

    하지만 이것은 문제가 되지 않습니다. 왜냐하면 일반적 이지 않은 새로운 컨테이너 유형은 구현될 수 있기 때문입니다.fmt::Display
 */

// 비교를 위해 필드 이름을 지정할 수 있는 구조를 정의합니다.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// 마찬가지로 `Point2D`에 대해 `Display`를 구현합니다.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `x`와 `y`만 표시되도록 맞춤설정합니다.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // 오류. `Debug`와 `Display`가 모두 구현되었지만 `{:b}`
    // 구현하려면 `fmt::Binary`가 필요합니다. 이것은 작동하지 않습니다.
    // println!("Point2D는 바이너리: {:b}?", point);
}