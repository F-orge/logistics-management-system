import "@fontsource/inter";
import "flyonui/dist/js/index.js";
import { createIcons, icons } from "lucide";
import type { IStaticMethods } from "flyonui/flyonui";

declare global {
	interface Window {
		HSStaticMethods: IStaticMethods;
	}
}

window.HSStaticMethods.autoInit();
createIcons({ icons });
console.log("Hello, world!");
