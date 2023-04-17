import { Skeleton, Stack } from "@mui/material";
import { BindingSkeleton } from "./BindingSkeleton";

const sx = { margin: "2px" };

export function AppSkeleton() {
  return (
    <>
      <div
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          top: 0,
          bottom: 0,
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          justifyContent: "center",
        }}
      >
        <div className="lds-dual-ring"></div>
      </div>
      <Stack
        spacing={1}
        style={{
          position: "absolute",
          left: 0,
          right: 0,
          top: 0,
          bottom: 0,
          display: "flex",
          flexDirection: "column",
        }}
      >
        <div
          style={{
            backgroundColor: "#fff",
            display: "flex",
            flexDirection: "row",
            borderBottom: "solid #888 1px",
            padding: 10,
            zIndex: 10,
            boxShadow: "0 2px 5px rgb(152, 151, 151)",
            justifyContent: "space-between",
            marginBottom: 10,
          }}
        >
          <Stack spacing={1} direction="row">
            <Skeleton variant="rectangular" width={150} height={45} />
            <Skeleton variant="rectangular" width={100} height={45} />
          </Stack>
          <div style={{ flex: 1, display: "flex", justifyContent: "end" }}>
            <Skeleton variant="rectangular" width={100} height={45} sx={sx} />
            <Skeleton variant="rectangular" width={100} height={45} sx={sx} />
            <Skeleton variant="rectangular" width={100} height={45} sx={sx} />
            <Skeleton variant="rectangular" width={100} height={45} sx={sx} />
          </div>
        </div>
        <div
          style={{
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
            overflow: "auto",
          }}
        >
          <BindingSkeleton />
          <BindingSkeleton />
          <BindingSkeleton />
          <BindingSkeleton />
          <div>
            <Skeleton variant="rectangular" width={200} height={40} sx={sx} />
          </div>
        </div>
      </Stack>
    </>
  );
}
