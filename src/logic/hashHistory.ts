import { createHashHistory as createBrowserHashHistory } from "history";
import { createHistory } from "svelte-navigator";

export default function createHashHistory() {
    const history = createBrowserHashHistory();
    let listeners: Function[] = [];

    history.listen((location) => {
        if (history.action === "POP") listeners.forEach((listener) => listener(location));
    });

    return createHistory({
        // @ts-ignore
        get location(): Location { return history.location as Location },
        addEventListener(name: string, handler: Function) {
            if (name === "popstate") listeners.push(handler);
        },
        removeEventListener(name: string, handler: Function) {
            if (name === "popstate") listeners = listeners.filter((fn) => fn !== handler);
        },
        history: {
            get state() { return history.location.state as any },
            pushState(state: any, _: string, uri: string) {
                history.push(uri, state);
            },
            replaceState(state: any, _: string, uri: string) {
                history.replace(uri, state);
            },
            go(to: number) {
                history.go(to);
            },
        },
    });
};
