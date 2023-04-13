import { HandlerContext } from "$fresh/server.ts";
import { flags } from "../../components/flags.ts";

export const handler = async (
  _req: Request,
  _ctx: HandlerContext,
): Promise<Response> => {
  const status = await Deno.run({
    cmd: flags.configPath
      ? [
        flags.mouseActionPath,
        "--config-path",
        flags.configPath,
        "status",
      ]
      : [flags.mouseActionPath, "status"],
  }).status();

  return new Response(`${status.success}`);
};
