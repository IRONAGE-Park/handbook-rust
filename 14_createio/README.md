# 14. 카고와 Crates.io 더 알아보기

## 14.1 릴리즈 프로필을 통한 빌드 커스터마이징하기

- 릴리즈 프로필(release profile): 미리 정의된 설정값을 가지고 있으며, 커스터마이징 가능한 프로필
  - 코드 컴파일을 위한 옵션 제어
  1. `build` 시 쓰이는 `dev` 프로필: 개발에 적합한 기본값
  2. `build --release` 시 쓰이는 `release` 프로필: 릴리즈 빌드용 설정

```bash
$ cargo build
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
$ cargo build --release
    Finished `release` profile [optimized] target(s) in 0.02s
```

- `Cargo.toml` 파일에 `[profile.*]` 섹션으로 커스터마이징 가능

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

- `opt-level` 설정: `Rust` 컴파일러가 코드에 적용할 최적화 수치(0~3)
  - 높은 최적화 수치는 컴파일 시간 증가 = 릴리즈
  - 낮은 최적화 수치는 컴파일 시간 감소 = 개발
- `opt-level`을 설정할 경우, `dev` 프로필에서 `optimized`를 표시하거나, `release` 프로필에서 `unoptimized`가 표시될 수 있음

```bash
$ cargo build
   Compiling createio v0.1.0 (C:\Users\ghooz\sources\Rust\handbook-rust\14_createio)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.31s

$ cargo build --release
   Compiling createio v0.1.0 (C:\Users\ghooz\sources\Rust\handbook-rust\14_createio)
    Finished `release` profile [unoptimized] target(s) in 0.23s
```

> [카고 공식 문서](https://doc.rust-lang.org/cargo/reference/profiles.html)

---
