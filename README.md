# Tasker

Rust Tauri와 SvelteKit 기반의 개인 일정 관리 및 할 일 관리(To-do) 웹 앱입니다.

## 주요 기능

- **할 일 관리**: 할 일 추가, 토글(완료/미완료), 삭제 기능.
- **데이터 영속성**: `rusqlite`를 사용하여 로컬 SQLite 데이터베이스에 데이터를 안전하게 저장.
- **필터링**: 전체, 진행 중인 작업, 완료된 작업별 필터링 기능.
- **캘린더 뷰**: 월간 일정 확인을 위한 캘린더 인터페이스 제공.
- **현대적인 UI**: Tailwind CSS v4와 Lucide 아이콘을 사용한 깔끔하고 반응형인 디자인.

## 기술 스택

- **Frontend**: SvelteKit (Svelte 5), TypeScript
- **Styling**: Tailwind CSS v4
- **Backend**: Rust, Tauri v2
- **Database**: SQLite (via `rusqlite`)
- **Icons**: Lucide Svelte

## 시작하기

### 사전 준비

- [Rust](https://www.rust-lang.org/tools/install) 설치
- [Node.js](https://nodejs.org/) 설치 (v18 이상 권장)

### 의존성 설치

```bash
npm install
```

### 환경 변수 설정

Google 연동 기능을 사용하려면 프로젝트 루트에 `.env` 파일을 생성하고 발급받은 API 키를 설정해야 합니다.

1. [Google Cloud Console](https://console.cloud.google.com/)에서 프로젝트를 생성합니다.
2. **Google Tasks API** 및 **Google Calendar API**를 활성화합니다.
3. **OAuth 2.0 클라이언트 ID**를 생성합니다 (애플리케이션 유형: 데스크톱 앱).
4. 프로젝트 루트에 `.env` 파일을 생성하고 아래 내용을 입력합니다.

```env
GOOGLE_CLIENT_ID=your_client_id_here
GOOGLE_CLIENT_SECRET=your_client_secret_here
```

### 개발 모드 실행

```bash
npm run tauri dev
```

### 빌드

```bash
npm run tauri build
```

## 프로젝트 구조

- `src/`: SvelteKit 프론트엔드 코드
  - `routes/`: 페이지 및 레이아웃
  - `app.css`: Tailwind CSS 설정
- `src-tauri/`: Rust 백엔드 코드
  - `src/main.rs` & `lib.rs`: Tauri 설정 및 명령어 정의
  - `src/db.rs`: SQLite 데이터베이스 로직
- `GEMINI.md`: 프로젝트 상세 요구사항 및 기획서

## 라이선스

MIT License