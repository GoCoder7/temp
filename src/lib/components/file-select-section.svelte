<script lang="ts">
  import Button from "$lib/shad/ui/button/button.svelte";
  import Label from "$lib/shad/ui/label/label.svelte";
  import type { FileSelecterProps } from "$lib/types/file";
  import { open, save } from "@tauri-apps/plugin-dialog";

  let { props = $bindable() }: { props: FileSelecterProps } = $props();

  let pathText: string = $derived(
    props.path ??
      (props.save ? "저장할 파일 경로를 지정해주세요" : "파일을 선택해주세요")
  );
  let buttonMsg: string = $derived(
    props.path
      ? props.save
        ? "저장경로 변경"
        : "선택파일 변경"
      : props.save
        ? "저장경로 지정"
        : "파일 선택"
  );

  async function showFileDialog(props: FileSelecterProps) {
    let path: string | null;
    if (props.save) {
      path = await save({
        filters: [
          {
            name: props.extensions.join(", "),
            extensions: props.extensions,
          },
        ],
      });
    } else {
      path = await open({
        filters: [
          {
            name: props.extensions.join(", "),
            extensions: props.extensions,
          },
        ],
      });
    }
    if (path) props.path = path;
  }
</script>

<div class="space-y-1.5">
  <Label
    ><div>
      {props.title}
    </div></Label
  >
  <div class="flex justify-between items-center">
    <span
      class={"p-2 mr-2 " + (props.path ? "border rounded" : "text-slate-400")}
    >
      {pathText}
    </span>
    <Button onclick={() => showFileDialog(props)}>
      {buttonMsg}
    </Button>
  </div>
</div>
