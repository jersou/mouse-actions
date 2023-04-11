import { Head } from "$fresh/runtime.ts";
import MouseActionRuntime from "../islands/MouseActionRuntime.tsx";
import Config from "../islands/Config.tsx";

export default function Home() {
  return (
    <>
      <Head>
        <title>Mouse Action config editor</title>
      </Head>
      <div class="p-4">
        <div class="text-3xl flex justify-center items-center">
          <img src="/logo.svg" className="w-16 h-16 p-2" />
          Mouse Action config editor
        </div>
        <MouseActionRuntime />
        <Config />
      </div>
    </>
  );
}
