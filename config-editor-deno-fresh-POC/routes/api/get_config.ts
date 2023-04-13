import { HandlerContext } from "$fresh/server.ts";
import { flags } from "../../components/flags.ts";

export const handler = async (
  req: Request,
  _ctx: HandlerContext,
): Promise<Response> => {
  if (req.method == "GET") {
    const out = await Deno.run({
      cmd: flags.configPath
        ? [
          flags.mouseActionPath,
          "--config-path",
          flags.configPath,
          "show-config",
        ]
        : [flags.mouseActionPath, "show-config"],
      stdout: "piped",
    }).output();
    return new Response(out);
  }
};
