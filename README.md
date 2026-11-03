# Orange Code (오렌지코드)

한국어로 코드를 작성하는 프로그래밍 언어 프로젝트입니다.

## 최신 개발: Orange Code Native 2.0

기존 1.x 실행기는 Python 변환 방식이었지만, 2.0부터는 **Rust로 구현한 독립 실행 엔진**을 개발합니다. Orange Code 소스는 자체 어휘 분석기(Lexer), 구문 분석기(Parser), AST, 실행 엔진을 거쳐 실행됩니다. Python 설치는 필요하지 않습니다.

### 현재 2.0 개발 기능
- 한국어 변수 선언 및 출력
- 숫자, 문자열, 참/거짓 값
- 사칙연산, 비교 연산, 논리 연산
- 조건문과 반복문
- 한국어 GUI 문법: 창, 글자, 버튼, 클릭 이벤트
- VS Code에서 네이티브 실행 파일 호출

### 네이티브 엔진 빌드
Windows에 Rust stable 도구 모음을 설치한 다음:

```powershell
cd native
cargo build --release
```

실행 파일은 `native\target\release\orange.exe`에 생성됩니다.

### 네이티브 예제 실행
```powershell
cargo run --release -- examples/hello.orange
cargo run --release -- examples/window.orange
```

GUI 예제:
```orange
창 만들기 "나의 첫 프로그램" {
    너비 720
    높이 460
    글자 "오렌지코드 네이티브 GUI"
    버튼 "확인" 클릭하면 {
        출력하기("버튼을 눌렀습니다!")
    }
}
```

### VS Code 연동
1. 네이티브 엔진을 빌드합니다.
2. `orange.exe`를 PATH에 등록하거나 VS Code 설정 `orange.executablePath`에 실행 파일의 전체 경로를 입력합니다.
3. `.orange` 파일을 열고 `Ctrl+Shift+F5`를 누릅니다.

VS Code 확장 패키지는 `vscode-extension` 폴더에서 만들 수 있습니다:
```powershell
cd vscode-extension
npm install
npx vsce package
```

## 기존 Orange Code 1.x
기존 `runtime/orange.py`와 기존 예제는 이전 버전 사용자를 위해 보존되어 있습니다. 이 실행기만 사용할 경우 Python 3.10 이상이 필요합니다. 새 네이티브 2.0 실행기는 Python을 사용하지 않습니다.

## 현재 개발 범위와 다음 단계
2.0 첫 단계는 네이티브 인터프리터와 기본 GUI를 구현하는 중입니다. 함수, 배열, 모듈, 파일/네트워크/데이터베이스 표준 라이브러리, 정적 타입 검사, 바이트코드 VM, 네이티브 기계어 컴파일은 후속 개발 항목입니다.

## 라이선스
MIT
