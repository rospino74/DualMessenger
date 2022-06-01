import { dialog } from "@tauri-apps/api";

export function selectFile(options: dialog.OpenDialogOptions): Promise<string | string[]> {
    return dialog.open(options);
}

export async function selectAPK(): Promise<string> {
    const option = {
        title: "Seleziona un APK",
        filters: [
            { name: "Android Package", extensions: ["apk"]}
        ],
        multiple: false,
    };
    let result = await selectFile(option);

    if (Array.isArray(result)) {
        return result[0];
    } else {
        return result;
    }
}

