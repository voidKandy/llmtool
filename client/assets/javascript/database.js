import { Surreal } from "./surreal";
import { surrealdbWasmEngines } from "./surrealwasm";

const db = new Surreal({
    engines: surrealdbWasmEngines(),
});
console.log(db);
