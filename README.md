# 프로그래밍 경력자를 위한 러스트

ch01: 다른 언어에도 있는 문법
* 기본문법: for, if, fn, println!, match, map, collect, loop
* 3항 연산자: let var = if cond % 2 == 0 { 1 } else { 0};
* 함수 포인터
* 클로저
* 

ch02: 러스트에만 있는 문법
* mutable
* 타입추론: 예제)피보나치
* as: 예제) 시저암호만들기
* Enum
* 패턴매칭(https://doc.rust-lang.org/book/ch18-00-patterns.html)
```
let gen = match age {
    0..20 => "MZ",
    20..50 => "X",
    50..100 => "A",
    _ => "?",
};
for i in 1..=100 {}
    let msg = match i {
        n if n % 15 == 0 => "FizzBizz".to_string(),
        n if n % 3 == 0 => "Fizz".to_string(),
        n if n % 5 == 0 => "Buzz".to_string(),
        _ => format!("{}", i),
    };
}
```


ch03: 다른 언어에도 있는 개념들
* 배열/슬라이스
* 문자열
* iterator: 문자열에 for를 사용할때는 .chars()
* to_string()메소드
* format!() 매크로
* 구조체
* 생성자대신에 사용하는 new함수
* 메소드
* crate, use: 파일 나누기(https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

ch04: 러스트에만 있는 개념들
* Result, Option
* ? 연산자와 unwrap 계열 함수들
* 소유권/참조: for와 배열을 사용하면 소유권 이전이 발생한다 따라서 iter()를 사용해야함, iter()와 into_iter()의 차이 예제? (https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
* slice type: https://doc.rust-lang.org/book/ch04-03-slices.html
* 구조체의 복사: Clone트레잇
* Trait
* Generic, life time (https://doc.rust-lang.org/book/ch10-00-generics.html)

ch05
* std:collections
* 매크로

ch06
* 리스트
* 메모리참조: rc, box, refcell, weak

ch07
* 동기처리: 아토믹, 뮤텍스, 스핀락, 세마포어, 베리어, 메세지(std::sync::mpsc)
* async
* 멀티쓰레드

프로젝트2
* 파일시스템 모니터

프로젝트1
* 어셈블리 파서 (rust에 yacc같은 프로젝트가 있나 찾아보자)
