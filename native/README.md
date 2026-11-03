# Orange Code Native 2.0

Orange Code Native는 Python으로 소스를 변환하지 않는 독립 실행형 한국어 프로그래밍 언어 구현입니다.

## 기술 구성
- Rust로 작성한 네이티브 실행기
- 자체 Lexer(어휘 분석기), Parser(재귀 하향 구문 분석기), AST, Interpreter
- eframe 기반 데스크톱 GUI
- 소스 확장자: `.orange`

## 빌드
Rust stable 도구 모음을 설치한 뒤 `native` 폴더에서 실행합니다.

```powershell
cargo build --release
```

실행 파일:
- Windows: `target\release\orange.exe`
- Linux/macOS: `target/release/orange`

## 실행
```powershell
cargo run --release -- examples/hello.orange
cargo run --release -- examples/window.orange
```

## 기본 문법
```orange
변수 이름 = "오렌지"
변수 합계 = 10 + 20
출력하기("안녕, " + 이름)

만약 합계 >= 30 {
    출력하기("조건이 참입니다.")
} 아니면 {
    출력하기("조건이 거짓입니다.")
}

반복하기 3번 {
    출력하기("반복 실행")
}
```

## GUI 문법
```orange
창 만들기 "내 프로그램" {
    너비 720
    높이 460
    글자 "첫 번째 네이티브 창"
    버튼 "확인" 클릭하면 {
        출력하기("버튼을 눌렀습니다.")
    }
}
```

## 지원 상태
현재 2.0 개발 첫 단계입니다. 변수, 숫자/문자열/논리값, 사칙연산, 비교/논리 연산, 조건문, 반복문, 출력, 기본 데스크톱 GUI 위젯을 구현했습니다. 함수, 배열, 모듈, 파일/네트워크/데이터베이스 표준 라이브러리, 정적 타입 검사, 네이티브 기계어 컴파일은 이후 단계입니다.

이 구현은 Python 런타임을 사용하지 않습니다. Rust로 빌드된 Orange 실행 파일만 있으면 Orange Code 프로그램을 실행할 수 있습니다.
