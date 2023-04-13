import { parseFlags } from "https://deno.land/x/cliffy@v0.25.7/flags/mod.ts";

export const { flags } = parseFlags(Deno.args, {
  stopEarly: true,
  flags: [{
    name: "config-path",
    type: "string",
    default: "../ign/mouse-actions.json",
  }, {
    name: "mouse-action-path",
    type: "string",
    default: "../target/debug/mouse_actions",
  }],
});
