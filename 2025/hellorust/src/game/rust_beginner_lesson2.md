# Rust 초급 두번째 강의: 간단한 게임 프로젝트 만들기

## 목차
1. [소개](#소개)
2. [프로젝트 설정](#프로젝트-설정)
3. [기본 게임 구조 만들기](#기본-게임-구조-만들기)
4. [사용자 입력 처리하기](#사용자-입력-처리하기)
5. [게임 로직 구현하기](#게임-로직-구현하기)
6. [게임 상태 관리하기](#게임-상태-관리하기)
7. [마무리 및 다음 단계](#마무리-및-다음-단계)

## 소개

이 강의에서는 Rust의 기본 개념을 활용하여 간단한 숫자 맞추기 게임을 만들어볼 것입니다. 이 프로젝트를 통해 다음과 같은 Rust의 핵심 개념을 배우게 됩니다:

- 구조체(Structs)와 열거형(Enums)
- 패턴 매칭(Pattern Matching)
- 오류 처리(Error Handling)
- 모듈 시스템(Module System)
- 사용자 입력 처리
- 랜덤 값 생성

## 프로젝트 설정

먼저 새로운 Rust 프로젝트를 만들어 보겠습니다:

```bash
cargo new number_guessing_game
cd number_guessing_game
```

`Cargo.toml` 파일에 필요한 의존성을 추가합니다:

```toml
[package]
name = "number_guessing_game"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
```

`rand` 크레이트는 랜덤 숫자를 생성하는 데 사용됩니다.

## 기본 게임 구조 만들기

이제 `src/main.rs` 파일에 기본 게임 구조를 만들어 보겠습니다:

```rust
use rand::Rng;
use std::io;

// 게임 상태를 나타내는 열거형
enum GameState {
    Playing,
    Won,
    Lost,
}

// 게임 설정을 위한 구조체
struct GameConfig {
    min_number: u32,
    max_number: u32,
    max_attempts: u32,
}

// 게임 진행 상황을 추적하는 구조체
struct Game {
    config: GameConfig,
    secret_number: u32,
    attempts: u32,
    state: GameState,
}

impl Game {
    // 새 게임 인스턴스 생성
    fn new(config: GameConfig) -> Game {
        let secret_number = rand::thread_rng().gen_range(config.min_number..=config.max_number);
        
        Game {
            config,
            secret_number,
            attempts: 0,
            state: GameState::Playing,
        }
    }
    
    // 게임 시작 메시지 출력
    fn print_welcome(&self) {
        println!("숫자 맞추기 게임에 오신 것을 환영합니다!");
        println!("{} 부터 {} 사이의 숫자를 맞춰보세요.", 
                 self.config.min_number, self.config.max_number);
        println!("최대 시도 횟수: {}", self.config.max_attempts);
    }
    
    // 게임의 메인 루프
    fn run(&mut self) {
        self.print_welcome();
        
        while let GameState::Playing = self.state {
            self.play_turn();
        }
        
        self.print_result();
    }
    
    // 게임 결과 출력
    fn print_result(&self) {
        match self.state {
            GameState::Won => {
                println!("축하합니다! {}번의 시도 끝에 숫자 {}를 맞추셨습니다!", 
                         self.attempts, self.secret_number);
            },
            GameState::Lost => {
                println!("아쉽게도 모든 기회를 사용하셨습니다. 정답은 {}였습니다.", 
                         self.secret_number);
            },
            _ => {}
        }
    }
    
    // 한 턴 진행
    fn play_turn(&mut self) {
        // 이 메서드는 다음 섹션에서 구현할 예정입니다.
    }
}

fn main() {
    let config = GameConfig {
        min_number: 1,
        max_number: 100,
        max_attempts: 7,
    };
    
    let mut game = Game::new(config);
    game.run();
}
```

## 사용자 입력 처리하기

이제 사용자 입력을 처리하는 코드를 추가해 보겠습니다. `play_turn` 메서드를 구현합니다:

```rust
fn play_turn(&mut self) {
    self.attempts += 1;
    
    println!("\n시도 {}/{}", self.attempts, self.config.max_attempts);
    println!("추측한 숫자를 입력하세요:");
    
    let mut guess = String::new();
    
    // 사용자 입력 읽기
    io::stdin()
        .read_line(&mut guess)
        .expect("입력을 읽는 데 실패했습니다.");
    
    // 입력을 숫자로 변환
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("유효한 숫자를 입력해주세요!");
            // 시도 횟수를 다시 줄임 (이번 시도는 카운트하지 않음)
            self.attempts -= 1;
            return;
        }
    };
    
    // 입력 범위 검증
    if guess < self.config.min_number || guess > self.config.max_number {
        println!("입력한 숫자는 {} 부터 {} 사이여야 합니다!", 
                 self.config.min_number, self.config.max_number);
        self.attempts -= 1;
        return;
    }
    
    // 게임 로직 처리
    self.process_guess(guess);
    
    // 최대 시도 횟수 확인
    if let GameState::Playing = self.state {
        if self.attempts >= self.config.max_attempts {
            self.state = GameState::Lost;
        }
    }
}
```

## 게임 로직 구현하기

이제 사용자의 추측을 처리하는 로직을 구현해 보겠습니다:

```rust
fn process_guess(&mut self, guess: u32) {
    match guess.cmp(&self.secret_number) {
        std::cmp::Ordering::Less => {
            println!("너무 작습니다!");
        },
        std::cmp::Ordering::Greater => {
            println!("너무 큽니다!");
        },
        std::cmp::Ordering::Equal => {
            println!("정확합니다!");
            self.state = GameState::Won;
        }
    }
}
```

## 게임 상태 관리하기

게임을 더 재미있게 만들기 위해 힌트 기능을 추가해 보겠습니다:

```rust
// Game 구조체에 새로운 필드 추가
struct Game {
    config: GameConfig,
    secret_number: u32,
    attempts: u32,
    state: GameState,
    last_guess: Option<u32>, // 마지막 추측 저장
}

// new 메서드 수정
fn new(config: GameConfig) -> Game {
    let secret_number = rand::thread_rng().gen_range(config.min_number..=config.max_number);
    
    Game {
        config,
        secret_number,
        attempts: 0,
        state: GameState::Playing,
        last_guess: None,
    }
}

// 힌트 제공 메서드 추가
fn give_hint(&self) {
    if let Some(last_guess) = self.last_guess {
        let distance = if last_guess > self.secret_number {
            last_guess - self.secret_number
        } else {
            self.secret_number - last_guess
        };
        
        if distance <= 5 {
            println!("아주 가깝습니다!");
        } else if distance <= 15 {
            println!("꽤 가깝습니다.");
        } else if distance <= 30 {
            println!("조금 멀었습니다.");
        } else {
            println!("많이 멀었습니다.");
        }
    }
}

// process_guess 메서드 수정
fn process_guess(&mut self, guess: u32) {
    self.last_guess = Some(guess);
    
    match guess.cmp(&self.secret_number) {
        std::cmp::Ordering::Less => {
            println!("너무 작습니다!");
            self.give_hint();
        },
        std::cmp::Ordering::Greater => {
            println!("너무 큽니다!");
            self.give_hint();
        },
        std::cmp::Ordering::Equal => {
            println!("정확합니다!");
            self.state = GameState::Won;
        }
    }
}
```

## 마무리 및 다음 단계

이제 완성된 게임을 실행해 볼 수 있습니다:

```bash
cargo run
```

이 간단한 게임 프로젝트를 통해 다음과 같은 Rust의 핵심 개념을 배웠습니다:

1. **구조체와 열거형**: `Game`, `GameConfig` 구조체와 `GameState` 열거형을 사용하여 데이터를 구조화했습니다.
2. **패턴 매칭**: `match` 표현식을 사용하여 다양한 상황을 처리했습니다.
3. **오류 처리**: `parse()` 메서드의 결과를 `match`로 처리하여 오류를 우아하게 처리했습니다.
4. **모듈 시스템**: 외부 크레이트(`rand`)를 사용했습니다.
5. **사용자 입력 처리**: 표준 입력을 사용하여 사용자 입력을 받았습니다.
6. **랜덤 값 생성**: `rand` 크레이트를 사용하여 랜덤 숫자를 생성했습니다.

### 다음 단계로 시도해 볼 수 있는 개선 사항:

1. **난이도 선택 기능**: 사용자가 게임 시작 시 난이도를 선택할 수 있게 합니다.
2. **게임 기록 저장**: 최고 점수나 게임 기록을 파일에 저장합니다.
3. **GUI 추가**: `ggez` 또는 `piston` 같은 크레이트를 사용하여 그래픽 인터페이스를 추가합니다.
4. **멀티플레이어 모드**: 두 명 이상의 플레이어가 번갈아가며 플레이할 수 있는 모드를 추가합니다.
5. **테스트 추가**: 게임 로직에 대한 단위 테스트를 작성합니다.

이 프로젝트를 확장하면서 Rust의 더 많은 기능을 탐색해 보세요!
