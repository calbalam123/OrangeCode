use eframe::egui;
use std::{collections::HashMap, env, fs, io::{self, Write}, path::PathBuf};

#[cfg(windows)]
fn setup_windows_console() {
    use windows_sys::Win32::System::Console::{SetConsoleCP, SetConsoleOutputCP};
    unsafe { SetConsoleCP(65001); SetConsoleOutputCP(65001); }
}
#[cfg(not(windows))]
fn setup_windows_console() {}

#[derive(Debug, Clone, PartialEq)]
enum TokenKind { Ident, Number, String, Symbol, Eof }
#[derive(Debug, Clone)]
struct Token { kind: TokenKind, text: String, line: usize, col: usize }

fn lex(source: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = source.chars().collect();
    let (mut i, mut line, mut col) = (0, 1, 1);
    let mut out = Vec::new();
    while i < chars.len() {
        let c = chars[i];
        if c == '\n' { line += 1; col = 1; i += 1; continue; }
        if c.is_whitespace() { col += 1; i += 1; continue; }
        if c == '#' {
            while i < chars.len() && chars[i] != '\n' { i += 1; col += 1; }
            continue;
        }
        let start_line = line; let start_col = col;
        if c == '"' || c == '\'' {
            let quote = c; i += 1; col += 1;
            let mut value = String::new();
            while i < chars.len() && chars[i] != quote {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 1; col += 1;
                    value.push(match chars[i] { 'n' => '\n', 't' => '\t', other => other });
                } else { value.push(chars[i]); }
                if chars[i] == '\n' { line += 1; col = 0; }
                i += 1; col += 1;
            }
            if i >= chars.len() { return Err(format!("{start_line}행 {start_col}열: 문자열 따옴표가 닫히지 않았습니다.")); }
            i += 1; col += 1;
            out.push(Token { kind: TokenKind::String, text: value, line: start_line, col: start_col });
            continue;
        }
        if c.is_ascii_digit() {
            let start = i;
            while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') { i += 1; col += 1; }
            out.push(Token { kind: TokenKind::Number, text: chars[start..i].iter().collect(), line: start_line, col: start_col });
            continue;
        }
        if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') { i += 1; col += 1; }
            out.push(Token { kind: TokenKind::Ident, text: chars[start..i].iter().collect(), line: start_line, col: start_col });
            continue;
        }
        let two = if i + 1 < chars.len() { format!("{}{}", c, chars[i+1]) } else { String::new() };
        if ["==", "!=", "<=", ">="].contains(&two.as_str()) {
            out.push(Token { kind: TokenKind::Symbol, text: two, line, col }); i += 2; col += 2; continue;
        }
        if "+-*/%=<>(){},".contains(c) {
            out.push(Token { kind: TokenKind::Symbol, text: c.to_string(), line, col }); i += 1; col += 1; continue;
        }
        return Err(format!("{line}행 {col}열: 알 수 없는 문자 '{c}'"));
    }
    out.push(Token { kind: TokenKind::Eof, text: String::new(), line, col });
    Ok(out)
}

#[derive(Debug, Clone)]
enum Expr { Number(f64), Text(String), Bool(bool), Variable(String), Unary(String, Box<Expr>), Binary(Box<Expr>, String, Box<Expr>) }
#[derive(Debug, Clone)]
enum Widget { Label(String), TextInput { name: String, placeholder: String }, Checkbox { name: String, label: String }, Button { label: String, action: Vec<Stmt> } }
#[derive(Debug, Clone)]
enum Stmt {
    Let(String, Expr), Print(Expr), If(Expr, Vec<Stmt>, Vec<Stmt>),
    While(Expr, Vec<Stmt>), Repeat(Expr, Vec<Stmt>), Break, Continue,
    Window(String, usize, usize, Vec<Widget>),
    Input(String, String), WriteFile(Expr, Expr), ReadFile(String, Expr),
}

struct Parser { tokens: Vec<Token>, pos: usize }
impl Parser {
    fn new(tokens: Vec<Token>) -> Self { Self { tokens, pos: 0 } }
    fn current(&self) -> &Token { &self.tokens[self.pos] }
    fn check(&self, s: &str) -> bool { self.current().text == s }
    fn advance(&mut self) -> Token { let t = self.tokens[self.pos].clone(); if t.kind != TokenKind::Eof { self.pos += 1; } t }
    fn expect(&mut self, s: &str) -> Result<(), String> {
        if self.check(s) { self.advance(); Ok(()) } else { let t=self.current(); Err(format!("{}행 {}열: '{}'가 필요하지만 '{}'가 있습니다.",t.line,t.col,s,t.text)) }
    }
    fn identifier(&mut self) -> Result<String,String> {
        let t=self.advance(); if t.kind==TokenKind::Ident { Ok(t.text) } else { Err(format!("{}행 {}열: 이름이 필요합니다.",t.line,t.col)) }
    }
    fn block(&mut self) -> Result<Vec<Stmt>,String> {
        self.expect("{")?; let mut body=Vec::new();
        while !self.check("}") && self.current().kind!=TokenKind::Eof { body.push(self.statement()?); }
        self.expect("}")?; Ok(body)
    }
    fn statement(&mut self) -> Result<Stmt,String> {
        if self.check("변수") {
            self.advance(); let name=self.identifier()?; self.expect("=")?; return Ok(Stmt::Let(name,self.expression(0)?));
        }
        if self.check("출력하기") {
            self.advance(); self.expect("(")?; let e=self.expression(0)?; self.expect(")")?; return Ok(Stmt::Print(e));
        }
        if self.check("만약") {
            self.advance(); let condition=self.expression(0)?; let yes=self.block()?;
            let no=if self.check("아니면") { self.advance(); self.block()? } else { Vec::new() };
            return Ok(Stmt::If(condition,yes,no));
        }
        if self.check("동안") {
            self.advance(); let condition=self.expression(0)?; self.expect("반복하기")?; return Ok(Stmt::While(condition,self.block()?));
        }
        if self.check("반복하기") {
            self.advance(); let count=self.expression(0)?; self.expect("번")?; return Ok(Stmt::Repeat(count,self.block()?));
        }
        if self.check("입력받기") { self.advance(); let name=self.identifier()?; let prompt=self.advance(); if prompt.kind!=TokenKind::String{return Err("입력받기 뒤에는 안내 문구가 필요합니다.".into());} return Ok(Stmt::Input(name,prompt.text)); }
        if self.check("파일에쓰기") { self.advance(); let path=self.expression(0)?; let value=self.expression(0)?; return Ok(Stmt::WriteFile(path,value)); }
        if self.check("파일읽기") { self.advance(); let name=self.identifier()?; let path=self.expression(0)?; return Ok(Stmt::ReadFile(name,path)); }
        if self.check("중단") { self.advance(); return Ok(Stmt::Break); }
        if self.check("계속") { self.advance(); return Ok(Stmt::Continue); }
        if self.check("창") {
            self.advance(); self.expect("만들기")?;
            let title=self.advance(); if title.kind!=TokenKind::String { return Err(format!("{}행: 창 제목은 문자열이어야 합니다.",title.line)); }
            self.expect("{")?; let mut width=640; let mut height=420; let mut widgets=Vec::new();
            while !self.check("}") && self.current().kind!=TokenKind::Eof {
                if self.check("너비") { self.advance(); width=self.advance().text.parse().map_err(|_|"너비는 숫자여야 합니다.")?; }
                else if self.check("높이") { self.advance(); height=self.advance().text.parse().map_err(|_|"높이는 숫자여야 합니다.")?; }
                else if self.check("글자") { self.advance(); let t=self.advance(); if t.kind!=TokenKind::String{return Err("글자 뒤에는 문자열이 필요합니다.".into());} widgets.push(Widget::Label(t.text)); }
                else if self.check("입력칸") { self.advance(); let name=self.identifier()?; let t=self.advance(); if t.kind!=TokenKind::String{return Err("입력칸 뒤에는 안내 문구가 필요합니다.".into());} widgets.push(Widget::TextInput{name,placeholder:t.text}); }
                else if self.check("체크상자") { self.advance(); let name=self.identifier()?; let t=self.advance(); if t.kind!=TokenKind::String{return Err("체크상자 뒤에는 설명이 필요합니다.".into());} widgets.push(Widget::Checkbox{name,label:t.text}); }
                else if self.check("버튼") {
                    self.advance(); let t=self.advance(); if t.kind!=TokenKind::String{return Err("버튼 뒤에는 문자열이 필요합니다.".into());}
                    let action=if self.check("클릭하면") { self.advance(); self.block()? } else { Vec::new() };
                    widgets.push(Widget::Button{label:t.text,action});
                } else { let t=self.current(); return Err(format!("{}행 {}열: GUI 안에서는 너비, 높이, 글자, 입력칸, 체크상자, 버튼만 사용할 수 있습니다.",t.line,t.col)); }
            }
            self.expect("}")?; return Ok(Stmt::Window(title.text,width,height,widgets));
        }
        let t=self.current(); Err(format!("{}행 {}열: 알 수 없는 명령어 '{}'",t.line,t.col,t.text))
    }
    fn expression(&mut self, min_prec:u8) -> Result<Expr,String> {
        let mut left = if self.check("-") || self.check("아니다") {
            let op=self.advance().text; Expr::Unary(op,Box::new(self.expression(7)?))
        } else if self.check("(") {
            self.advance(); let e=self.expression(0)?; self.expect(")")?; e
        } else {
            let t=self.advance();
            match t.kind {
                TokenKind::Number => Expr::Number(t.text.parse().map_err(|_|format!("{}행: 잘못된 숫자입니다.",t.line))?),
                TokenKind::String => Expr::Text(t.text),
                TokenKind::Ident if t.text=="참" => Expr::Bool(true),
                TokenKind::Ident if t.text=="거짓" => Expr::Bool(false),
                TokenKind::Ident => Expr::Variable(t.text),
                _ => return Err(format!("{}행 {}열: 올바른 값이 필요합니다.",t.line,t.col))
            }
        };
        loop {
            let op=self.current().text.clone();
            let prec=match op.as_str() { "또는"=>1, "그리고"=>2, "=="|"!="|"<"|">"|"<="|">="=>3, "+"|"-"=>4, "*"|"/"|"%"=>5, _=>0 };
            if prec==0 || prec<min_prec { break; }
            self.advance(); let right=self.expression(prec+1)?;
            left=Expr::Binary(Box::new(left),op,Box::new(right));
        }
        Ok(left)
    }
    fn parse(&mut self) -> Result<Vec<Stmt>,String> {
        let mut out=Vec::new(); while self.current().kind!=TokenKind::Eof { out.push(self.statement()?); } Ok(out)
    }
}

#[derive(Debug, Clone)]
enum Value { Number(f64), Text(String), Bool(bool), Null }
impl Value {
    fn truthy(&self)->bool { match self { Value::Bool(v)=>*v, Value::Number(n)=>*n!=0.0, Value::Text(s)=>!s.is_empty(), Value::Null=>false } }
    fn show(&self)->String { match self { Value::Number(n)=>if n.fract()==0.0 {format!("{}",*n as i64)} else {n.to_string()}, Value::Text(s)=>s.clone(), Value::Bool(b)=>if *b {"참".into()} else {"거짓".into()}, Value::Null=>"없음".into() } }
}
#[derive(Clone, Copy, PartialEq)]
enum LoopSignal { Break, Continue }
#[derive(Clone,Default)]
struct Interpreter { vars: HashMap<String,Value>, signal: Option<LoopSignal> }
impl Interpreter {
    fn eval(&self,e:&Expr)->Result<Value,String>{
        match e {
            Expr::Number(n)=>Ok(Value::Number(*n)), Expr::Text(s)=>Ok(Value::Text(s.clone())), Expr::Bool(b)=>Ok(Value::Bool(*b)),
            Expr::Variable(n)=>self.vars.get(n).cloned().ok_or_else(||format!("정의되지 않은 변수 '{n}'")),
            Expr::Unary(op,v)=>{let x=self.eval(v)?; match op.as_str(){"-"=>if let Value::Number(n)=x{Ok(Value::Number(-n))}else{Err("숫자에만 - 연산을 사용할 수 있습니다.".into())},"아니다"=>Ok(Value::Bool(!x.truthy())),_=>Err("알 수 없는 단항 연산자".into())}},
            Expr::Binary(a,op,b)=>{
                let x=self.eval(a)?; let y=self.eval(b)?;
                match op.as_str(){
                    "+"=>match (x,y){(Value::Number(a),Value::Number(b))=>Ok(Value::Number(a+b)),(a,b)=>Ok(Value::Text(a.show()+&b.show()))},
                    "-"=>num_op(x,y,|a,b|a-b), "*"=>num_op(x,y,|a,b|a*b), "/"=>num_op(x,y,|a,b|a/b), "%"=>num_op(x,y,|a,b|a%b),
                    "=="=>Ok(Value::Bool(x.show()==y.show())), "!="=>Ok(Value::Bool(x.show()!=y.show())),
                    ">"|"<"|">="|"<="=>{let (a,b)=match (x,y){(Value::Number(a),Value::Number(b))=>(a,b),_=>return Err("비교 연산에는 숫자를 사용하세요.".into())}; Ok(Value::Bool(match op.as_str(){">"=>a>b,"<"=>a<b,">="=>a>=b,_=>a<=b}))},
                    "그리고"=>Ok(Value::Bool(x.truthy()&&y.truthy())), "또는"=>Ok(Value::Bool(x.truthy()||y.truthy())), _=>Err("알 수 없는 연산자".into())
                }
            }
        }
    }
    fn execute(&mut self,body:&[Stmt])->Result<(),String>{
        for stmt in body { if self.signal.is_some(){break;} match stmt {
            Stmt::Let(n,e)=>{self.vars.insert(n.clone(),self.eval(e)?);},
            Stmt::Print(e)=>println!("{}",self.eval(e)?.show()),
            Stmt::If(c,a,b)=>{if self.eval(c)?.truthy(){self.execute(a)?}else{self.execute(b)?}},
            Stmt::Input(name,prompt)=>{print!("{prompt} ");io::stdout().flush().map_err(|e|e.to_string())?;let mut line=String::new();io::stdin().read_line(&mut line).map_err(|e|e.to_string())?;self.vars.insert(name.clone(),Value::Text(line.trim_end().to_string()));},
            Stmt::WriteFile(path,value)=>{let p=self.eval(path)?.show();let v=self.eval(value)?.show();fs::write(&p,v).map_err(|e|format!("파일 저장 실패: {e}"))?;},
            Stmt::ReadFile(name,path)=>{let p=self.eval(path)?.show();let v=fs::read_to_string(&p).map_err(|e|format!("파일 읽기 실패: {e}"))?;self.vars.insert(name.clone(),Value::Text(v));},
            Stmt::While(c,b)=>{let mut guard=0;while self.eval(c)?.truthy(){guard+=1;if guard>1_000_000{return Err("반복 횟수 제한을 초과했습니다.".into())}self.execute(b)?;match self.signal.take(){Some(LoopSignal::Break)=>break,Some(LoopSignal::Continue)=>continue,None=>()}}},
            Stmt::Repeat(e,b)=>{let count=self.eval(e)?;let n=if let Value::Number(v)=count{v.max(0.0).min(1_000_000.0) as usize}else{return Err("반복 횟수는 숫자여야 합니다.".into())};for _ in 0..n{self.execute(b)?;match self.signal.take(){Some(LoopSignal::Break)=>break,Some(LoopSignal::Continue)=>continue,None=>()}}},
            Stmt::Break=>self.signal=Some(LoopSignal::Break),
            Stmt::Continue=>self.signal=Some(LoopSignal::Continue),
            Stmt::Window(title,w,h,widgets)=>{let app=OrangeGui{title:title.clone(),width:*w,height:*h,widgets:widgets.clone(),interpreter:self.clone(),fonts_loaded:false};let options=eframe::NativeOptions{viewport:egui::ViewportBuilder::default().with_inner_size([*w as f32,*h as f32]),..Default::default()};eframe::run_native(title,options,Box::new(|_|Ok(Box::new(app)))).map_err(|e|format!("GUI 실행 실패: {e}"))?;}
        }} Ok(())
    }
}
fn num_op(a:Value,b:Value,f:impl Fn(f64,f64)->f64)->Result<Value,String>{if let (Value::Number(x),Value::Number(y))=(a,b){if y==0.0 && (f(1.0,y).is_infinite() || f(1.0,y).is_nan()){return Err("0으로 나눌 수 없습니다.".into())}Ok(Value::Number(f(x,y)))}else{Err("산술 연산에는 숫자를 사용하세요.".into())}}

struct OrangeGui { title:String,width:usize,height:usize,widgets:Vec<Widget>,interpreter:Interpreter,fonts_loaded:bool }

fn install_korean_font(ctx:&egui::Context) {
    let candidates = [
        r"C:\\Windows\\Fonts\\malgun.ttf",
        r"C:\\Windows\\Fonts\\gulim.ttc",
        r"C:\\Windows\\Fonts\\batang.ttc",
    ];
    let bytes = candidates.iter().find_map(|path| fs::read(path).ok());
    if let Some(bytes) = bytes {
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert("orange_korean".to_owned(), egui::FontData::from_owned(bytes));
        for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
            fonts.families.entry(family).or_default().insert(0, "orange_korean".to_owned());
        }
        ctx.set_fonts(fonts);
    }
}
impl eframe::App for OrangeGui {
    fn update(&mut self,ctx:&egui::Context,_frame:&mut eframe::Frame){
        egui::CentralPanel::default().show(ctx,|ui|{
            ui.heading(&self.title); ui.add_space(12.0);
            for widget in &self.widgets {
                match widget {
                    Widget::Label(s)=>{ui.label(s);}
                    Widget::TextInput{name,placeholder}=>{
                        ui.label(placeholder);
                        let value=self.interpreter.vars.entry(name.clone()).or_insert_with(||Value::Text(String::new()));
                        if let Value::Text(text)=value { ui.text_edit_singleline(text); }
                    }
                    Widget::Checkbox{name,label}=>{
                        let value=self.interpreter.vars.entry(name.clone()).or_insert(Value::Bool(false));
                        if let Value::Bool(checked)=value { ui.checkbox(checked,label); }
                    }
                    Widget::Button{label,action}=>{if ui.button(label).clicked(){if let Err(e)=self.interpreter.execute(action){eprintln!("[오렌지코드 오류] {e}");}}}
                }
                ui.add_space(6.0);
            }
        });
    }
}

fn run_file(path:PathBuf)->Result<(),String>{
    let source=fs::read_to_string(&path).map_err(|e|format!("파일을 읽을 수 없습니다: {e}"))?;
    let tokens=lex(&source)?; let program=Parser::new(tokens).parse()?;
    Interpreter::default().execute(&program)
}
fn main(){
    let args:Vec<String>=env::args().skip(1).collect();
    if args.iter().any(|a|a=="--version"||a=="-V"){println!("Orange Code 2.0.0-native");return;}
    if args.is_empty(){println!("Orange Code 2.0.0 - 한국어 네이티브 프로그래밍 언어\n사용법: orange <파일.orange>");return;}
    if let Err(e)=run_file(PathBuf::from(&args[0])){eprintln!("[오렌지코드 오류] {e}");std::process::exit(1);}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_recognizes_korean_keywords_and_strings() {
        let tokens = lex("변수 이름 = \"오렌지\"").unwrap();
        assert_eq!(tokens[0].text, "변수");
        assert_eq!(tokens[1].text, "이름");
        assert_eq!(tokens[3].text, "오렌지");
    }

    #[test]
    fn parser_respects_arithmetic_precedence() {
        let tokens = lex("변수 합계 = 5 + 2 * 3").unwrap();
        let program = Parser::new(tokens).parse().unwrap();
        let mut vm = Interpreter::default();
        vm.execute(&program).unwrap();
        assert_eq!(vm.vars.get("합계").unwrap().show(), "11");
    }

    #[test]
    fn parser_accepts_if_else_and_repeat_blocks() {
        let source = "변수 값 = 2\n만약 값 > 1 {\n출력하기(\"참\")\n} 아니면 {\n출력하기(\"거짓\")\n}\n반복하기 2번 {\n출력하기(\"반복\")\n}";
        let program = Parser::new(lex(source).unwrap()).parse().unwrap();
        assert_eq!(program.len(), 3);
    }

    #[test]
    fn parser_accepts_console_input_and_file_io() {
        let source = "입력받기 사용자 \"이름을 입력하세요\"\n파일에쓰기 \"sample.txt\" \"저장 내용\"\n파일읽기 다시읽기 \"sample.txt\"";
        let program = Parser::new(lex(source).unwrap()).parse().unwrap();
        assert_eq!(program.len(), 3);
        assert!(matches!(program[0], Stmt::Input(_, _)));
        assert!(matches!(program[1], Stmt::WriteFile(_, _)));
        assert!(matches!(program[2], Stmt::ReadFile(_, _)));
    }

    #[test]
    fn parser_accepts_loop_control_commands() {
        let source = "반복하기 3번 {\n중단\n}";
        let program = Parser::new(lex(source).unwrap()).parse().unwrap();
        assert!(matches!(program.first(), Some(Stmt::Repeat(_, _))));
    }

    #[test]
    fn parser_accepts_korean_gui_syntax() {
        let source = "창 만들기 \"테스트\" {\n너비 640\n높이 480\n글자 \"안녕\"\n버튼 \"확인\" 클릭하면 {\n출력하기(\"클릭\")\n}\n}";
        let program = Parser::new(lex(source).unwrap()).parse().unwrap();
        assert!(matches!(program.first(), Some(Stmt::Window(_, 640, 480, _))));
    }
}
