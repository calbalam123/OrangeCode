# Orange Code Native 2.0

Python 소스 변환 없이 Orange Code 코드를 자체 Lexer, Parser, AST, Interpreter로 실행하는 독립형 한국어 프로그래밍 언어입니다.

## 빌드
Rust stable을 설치한 뒤 이 폴더에서:
```powershell
cargo build --release
cargo test
```
Windows 실행 파일은 `target\release\orange.exe`입니다.

## 실행 예제
```powershell
cargo run --release -- examples/hello.orange
cargo run --release -- examples/window.orange
cargo run --release -- examples/input_file.orange
cargo run --release -- examples/loop_control.orange
```

## 콘솔 입력 및 파일
```orange
입력받기 이름 "이름을 입력하세요:"
파일에쓰기 "name.txt" 이름
파일읽기 저장된이름 "name.txt"
출력하기(저장된이름)
```

## 반복문 제어
```orange
반복하기 5번 {
    출력하기("반복")
    중단
}
```
`중단`은 반복문을 끝내며 `계속`은 현재 반복의 남은 문장을 건너뜁니다.

## GUI
```orange
창 만들기 "내 프로그램" {
    너비 720
    높이 460
    글자 "안녕하세요"
    버튼 "확인" 클릭하면 {
        출력하기("버튼 클릭")
    }
}
```

## 지원 상태
구현 단계: 변수, 출력, 숫자/문자열/논리값, 연산, 조건문, 반복문, 중단/계속, 콘솔 입력, 텍스트 파일 읽기/쓰기, 기본 GUI 창·라벨·입력칸·체크상자·버튼. 함수, 배열, 모듈, 고급 GUI 위젯, 네트워크·데이터베이스 표준 라이브러리, 타입 시스템과 네이티브 기계어 컴파일은 후속 개발입니다.

실행 파일은 Rust로 한 번 빌드해야 하지만, 빌드된 Orange Code 프로그램 실행에는 Python이 필요하지 않습니다.
