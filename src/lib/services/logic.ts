import { message } from "@tauri-apps/plugin-dialog";

export function execute({
  csvPath,
  pdfPath,
  xlPath,
}: {
  csvPath: string;
  pdfPath: string;
  xlPath: string;
}) {
  message(`csvPath: ${csvPath}\npdfPath: ${pdfPath}\nxlPath: ${xlPath}`);
}
