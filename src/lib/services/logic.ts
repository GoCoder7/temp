import { message } from "@tauri-apps/plugin-dialog";
import { commands } from "$lib/../bindings";
import type { Paths } from "$lib/../bindings";

export async function execute(paths: Paths) {
  commands.execute(paths).then((ok) => {
    if (ok) {
      message(`'${paths.xlPath}'경로에 파일이 생성되었습니다.`);
    } else {
      message(`파일 생성에 실패했습니다.`);
    }
  });
}
