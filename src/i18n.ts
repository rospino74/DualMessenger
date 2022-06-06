import { invoke } from "@tauri-apps/api/tauri";
import { register, init, getLocaleFromNavigator, isLoading } from "svelte-i18n";

export function initializeI18n() {
    return new Promise<void>(async (resolve, reject) => {
        register("en-US", () => import("../locale/en-US.json"));
        register("it", () => import("../locale/it.json"));

        const locale: string = await (invoke("get_locale").then((r: string) => r, getLocaleFromNavigator));

        init({
            fallbackLocale: "en-US",
            initialLocale: locale,
        });

        isLoading.subscribe((value) => {
            if (!value) {
                resolve();
            }
        });
    });
}