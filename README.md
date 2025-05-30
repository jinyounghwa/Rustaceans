# Rustaceans
Goodnight, Rustaceans (2025)
# 🦀 Rust 개발자 되기 - 현실 기반 학습 플랜

## 🚩 전체 구성 개요

| 단계 | 목표 | 기간 추천 | 핵심 주제 |
|------|------|------------|------------|
| **1단계. 생존과 기반기술** | Rust 문법/구조 완전 습득 | 1~2개월 | 소유권, 패턴 매칭, 모듈화, CLI |
| **1.5단계. 실용 진입 브리지** | 실용 도구 제작 및 구조 감각 익히기 | 1~2개월 | CLI, 비동기, HTTP, 구조화 |
| **2단계. 응용과 도구화** | 유틸 제작 및 백엔드 기술 | 2~4개월 | 웹서버, DB, 비동기, 시스템 |
| **3단계. 실전과 아키텍처** | 서비스 구조 설계 및 운영 | 3개월~ | 프로젝트 구조화, 배포, 최적화 |

---

## 🧱 1단계 – 생존과 기반기술

### 🎯 목표
- Rust로 자신 있게 CLI 툴 작성
- 소유권/수명/패턴 자유자재

### 📚 학습 항목
- 소유권, 참조, 수명 `'a`
- 패턴 매칭 (`match`, `if let`)
- 구조체, 열거형, trait
- 에러 처리 (`Result`, `Option`, `?`)
- 모듈화, `mod`, `lib.rs`
- `clap`, `std::fs`, `std::io`

### 🧪 실습 예제
- 텍스트 필터 CLI
- 파일 통계 툴
- JSON 설정 파서

---

## 🪜 1.5단계 – 실용 진입 브리지

Rust 기초 → 비동기/시스템 응용으로 넘어가기 위한 중간 단계

### Step 1: CLI 유틸 구조화
- 예제: 디렉토리 정리 툴, 시스템 모니터
- 도구: `clap`, `walkdir`, `log`
- 학습: 모듈 분리, 에러 구조화, 기능별 분리 설계

### Step 2: 비동기 HTTP 감각 익히기
- 예제: `/hello`, `/echo`, `/time` API
- 도구: `axum`, `tokio`, `hyper`
- 학습: async/await, 핸들러 분리, Body 추출

### Step 3: 간단한 DB 연동
- 예제: TODO 리스트 API
- 도구: `sqlx`, SQLite/PostgreSQL
- 학습: 연결 풀, 쿼리 작성, 상태 공유 (`Arc<Pool>`)

### ✅ 체크리스트
- [ ] CLI 유틸 만들기 완료
- [ ] `async fn` 자유롭게 사용 가능
- [ ] JSON API 서버 구현
- [ ] DB와 CRUD 연동
- [ ] 최소 2~3개 GitHub 업로드

---

## 🔹 2단계 – 응용과 도구화

### 🎯 목표
- 웹서버 + DB + 비동기 연계 가능
- 시스템 감시 유틸 작성

### 📚 학습 항목
- 웹 프레임워크: Axum or Actix
- 비동기 흐름: `tokio`, `select!`
- DB 연동: `sqlx`, `diesel`
- 시스템 API: `sysinfo`, `notify`
- 테스트: `#[test]`, `criterion`

### 🧪 실습 예제
- 네트워크 감시 데몬
- 로그인 API 서버 (JWT)
- TODO 리스트 API + DB

---

## 🔺 3단계 – 실전과 아키텍처

### 🎯 목표
- 실용적인 Rust 기반 서비스 운영
- 배포/운영까지 마스터

### 📚 학습 항목
- 도메인 구조화 (DDD-lite)
- 서비스 구조 나누기 (`mod user`, `mod auth`)
- 배포 자동화 (Docker, cross compile)
- 성능 최적화 (`flamegraph`, `tokio-console`)
- 운영 로직: 로그, 상태 점검, 자동 재시작

### 🧪 실습 예제
- 키오스크 복구 시스템
- 로그 집계 도구
- 올인원 CLI + API 도구

---

## 📁 실습 예제 추천

### 📁 실습 1: “파일 정리 CLI 툴”
- 입력 디렉토리, 확장자 분류
- dry-run, 삭제 옵션

### 🌐 실습 2: “Axum 미니 API 서버”
- GET `/hello`
- POST `/echo`
- GET `/time`

### 🗃️ 실습 3: “SQLite 기반 TODO API”
- POST `/todo`
- GET `/todo`
- DELETE `/todo/:id`

---

## 📌 정리 요약

- 지금은 1단계 말기, 1.5단계 진입이 중요
- “작지만 실용적인 프로그램”을 직접 작성하면서 구조화 감각을 훈련
- 비동기 → 웹 → DB 순으로 확장
- 프로젝트 중심으로 학습 진행 + 정리하면서 반복 학습

---

## 🧠 내가 도와줄 수 있는 것

- 각 실습 예제의 설계 구조 및 코드 제공
- 네가 만드는 코드 구조 피드백
- 컨벤션, 에러 흐름, 프로젝트 설계 리뷰
