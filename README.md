# 프로그래밍 경력자를 위한 러스트

ch01
* 기본문법: for, it, fn, println!, map, collect, match

ch02
* mutable
* 타입추론: 피보나치
* as
* 배열
* 시저암호만들기
* 3항 연산자: let var = if cond % 2 == 0 { 1 } else { 0};


ch03
* Result, Option
* 소유권/참조: for와 배열을 사용하면 소유권 이전이 발생한다 따라서 iter()를 사용해야함
* 배열/슬라이스
* 문자열
* 문자열에 for를 사용할때는 .chars()
* to_string()메소드
* format!() 매크로

ch04
* 구조체
* 생성자대신에 사용하는 new함수
* 메소드

ch05
* 구조체의 복사: Clone트레잇
* Trait
* Generic

ch06
* Enum
* 패턴매칭
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
ch07
* 매크로

ch08
* 리스트
* 메모리참조: rc, box, refcell, weak

ch09
* 

프로젝트1
* 어셈블리 파서

프로젝트2
* 파일시스템 모니터
