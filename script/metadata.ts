import { meta } from "./meta.ts";

console.log(JSON.stringify(await meta(), "", "  "));
