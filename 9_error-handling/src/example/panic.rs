pub fn run() {
    panic!("crash and burn");
    // thread 'main' panicked at 'crash and burn', src\example\panic.rs:3:9
    // [첫 번째 줄에는 작성해둔 패틱 메시지와 패닉이 발생한 소스 코드 지점을 보여줌
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // 에러 메시지를 통해 외부 코드 여부, 누가 작성한 코드인지 등 패닉이 발생한 원인을 파악할 수 있음
}

pub fn backtrace() {
    let v = vec![1, 2, 3];
    v[99]; // []의 사용은 어떤 요소의 반환을 가정하지만, 유효하지 않은 인덱스를 넘기게 되면 러스트가 반환할 올바른 요소가 없음
           // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src\example\panic.rs:12:9

    // RUST_BACKTRACE=1 에러 로그
    // thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src\example\panic.rs:12:9
    // stack backtrace:
    // 0: std::panicking::begin_panic_handler
    // at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library\std\src\panicking.rs:579
    // 1: core::panicking::panic_fmt
    // at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library\core\src\panicking.rs:64
    // 2: core::panicking::panic_bounds_check
    // at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library\core\src\panicking.rs:159
    // 3: core::slice::index::impl$2::index<i32>
    //     at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc\library\core\src\slice\index.rs:260
    // 4: core::slice::index::impl$0::index
    // at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc\library\core\src\slice\index.rs:18
    // 5: alloc::vec::impl$13::index<i32,usize,alloc::alloc::Global>
    //     at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc\library\alloc\src\vec\mod.rs:2703
    // 6: error_handling::example::panic::panic::backtrace
    // at .\src\example\panic.rs:12
    // 7: error_handling::main
    // at .\src\main.rs:4
    // 8: core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
    // at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc\library\core\src\ops\function.rs:250
}
