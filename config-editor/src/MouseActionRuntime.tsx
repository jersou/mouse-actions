import { useEffect, useState } from "react";
import { Button } from "@mui/material";

export default function MouseActionRuntime() {
  const [statusIsOk, setStatusIsOk] = useState(false);

  const refreshStatus = async () => {
    const res = await fetch("http://localhost:8000/api/status");
    setStatusIsOk(await res.json());
  };

  useEffect(() => {
    refreshStatus();
  }, []);
  return (
    <div class="flex gap-2 w-full">
      <Button onClick={() => refreshStatus()}>
        status : {statusIsOk ? "OK" : "KO"} (refresh)
      </Button>
    </div>
  );
}
