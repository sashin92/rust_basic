// 초기화 할 때부터 if문으로 값을 넣을 수 있다.
pub fn if_else() {
    let num = 5;
    let var = if num % 3 == 0 {
        3
    } else {
        if num % 5 == 0 {
            5
        } else {
            0
        }
    };
    println!("var = {}", var);
}
/*
Rust에서는 다양한 함수형 프로그래밍 기법을 사용하고 있기 때문에
if-else를 사용할 일이 거의 없다. 대신 변수 초기화 할 때에나 if-else가 쓰이는
셈이다. 그 외에는 match같은 패턴처리, map, filter 등을 훨씬 많이 쓴다.
만약 if-else가 많은 코드를 짜고 있다면 조금 더 다른 방향으로
'rust스럽게'만들어보려고 노력할 필요가 있다.
 */