pub mod commands; // 커맨드
pub mod types; // 모델(struct)
use specta_typescript::Typescript;
use tauri_specta::collect_commands;

// 빌더 생성하는 함수
pub fn init() -> tauri_specta::Builder {
    // 커맨드, 이벤트, 타입 등록하여 빌더 생성
    let builder = tauri_specta::Builder::<tauri::Wry>::new()
        .commands(collect_commands![
            commands::execute,
        ])
        .typ::<types::Paths>();

    // 빌더 정보를 통해, 프론트에서 사용할 ts파일 생성
    #[cfg(debug_assertions)]
    builder
	    // 경로와 이름은 임의로 지정 가능. src-tauri를 기준으로 함.
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");
    return builder;
}