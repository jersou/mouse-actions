import { Skeleton } from "@mui/material";

const sx = { margin: "2px" };

export function BindingSkeleton() {
  return (
    <div
      style={{
        marginBottom: 20,
        padding: 10,
        maxWidth: 1000,
        display: "grid",
        gap: 10,
        // FIXME
        gridTemplateColumns: "30px 90px 1fr 170px",
        borderBottom: "solid #000 1px",
      }}
    >
      <div style={{ display: "flex", flexDirection: "column" }}>
        <Skeleton variant="circular" width={30} height={30} sx={sx} />
        <Skeleton variant="circular" width={30} height={30} sx={sx} />
      </div>
      <div style={{ display: "flex", flexDirection: "column" }}>
        {[...Array(4).keys()].map((i) => (
          <Skeleton key={i} variant="rounded" width={80} height={45} sx={sx} />
        ))}
      </div>

      <div style={{ flexDirection: "column", flex: 1, display: "flex" }}>
        <div style={{ display: "flex" }}>
          <Skeleton variant="rounded" width={100} height={50} sx={sx} />
          <Skeleton
            variant="rounded"
            width={100}
            height={50}
            style={{ flex: 1 }}
            sx={sx}
          />
        </div>
        <div style={{ display: "flex", flex: 0 }}>
          <Skeleton
            variant="rounded"
            width={100}
            height={50}
            style={{ flex: 1, marginTop: 10, marginBottom: 10 }}
            sx={sx}
          />
        </div>
        <div style={{ display: "flex" }}>
          {[...Array(7).keys()].map((i) => (
            <Skeleton
              key={i}
              variant="rounded"
              width={50}
              height={60}
              style={{ flex: 1, marginTop: 10, marginBottom: 10 }}
              sx={sx}
            />
          ))}
        </div>
      </div>
      <div>
        <Skeleton variant="rounded" width={178} height={123} />
      </div>
    </div>
  );
}
