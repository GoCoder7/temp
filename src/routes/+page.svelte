<script lang="ts">
  // import { extractPdfTable } from "@mkas3/pdf-table-parser";
  import Button from "$lib/shad/ui/button/button.svelte";
  import * as Card from "$lib/shad/ui/card";
  import type { FileSelecterProps } from "$lib/types/file";
  import FileSelectSection from "$lib/components/file-select-section.svelte";
  import { message, confirm, ask } from "@tauri-apps/plugin-dialog";
  import { execute } from "$lib/services/logic";
  import { info } from '@tauri-apps/plugin-log';


  let csvProps: FileSelecterProps = $state({
    title: "csv 파일 선택",
    extensions: ["csv"],
  });
  let pdfProps: FileSelecterProps = $state({
    title: "pdf 파일 선택",
    extensions: ["pdf"],
  });
  let xlProps: FileSelecterProps = $state({
    title: "저장할 엑셀파일 경로",
    extensions: ["xlsx"],
    save: true,
  });
  $effect(() => {
    csvProps;
    info('csvPath selected');
  })
  $effect(() => {
    pdfProps;
    info('pdfPath selected');
  })
  $effect(() => {
    xlProps;
    info('xlPath selected');
  })

</script>

<main class="flex flex-col items-center justify-center h-screen">
  <Card.Root>
    <Card.Header class="mb-4">
      <Card.Title>Diff Checker</Card.Title>
      <Card.Description>
        CSV파일과 PDF파일을 비교하여 엑셀파일로 저장합니다
      </Card.Description>
    </Card.Header>
    <hr />
    <Card.Content class="p-3">
      <div class="flex flex-col p-4 space-y-4">
        <FileSelectSection bind:props={csvProps} />
        <FileSelectSection bind:props={pdfProps} />
        <FileSelectSection bind:props={xlProps} />
      </div>
    </Card.Content>
    <Card.Footer>
      <Button
        class="w-full"
        disabled={!(csvProps.path && pdfProps.path && xlProps.path)}
        onclick={() => {
          execute({
            csvPath: csvProps.path!,
            pdfPath: pdfProps.path!,
            xlPath: xlProps.path!,
          });
        }}
      >
        실행
      </Button>
    </Card.Footer>
  </Card.Root>
</main>