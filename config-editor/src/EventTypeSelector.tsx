import { EventTypes, EventTypeType } from "./config.type";
import ToggleButton from "@mui/material/ToggleButton";
import ToggleButtonGroup from "@mui/material/ToggleButtonGroup";
import { styled } from "@mui/material/styles";
import { startCase } from "lodash";
import GestureIcon from "@mui/icons-material/Gesture";
import FileDownloadIcon from "@mui/icons-material/FileDownload";
import MouseIcon from "@mui/icons-material/Mouse";
import UploadIcon from "@mui/icons-material/Upload";

const StyledToggleButtonGroup = styled(ToggleButtonGroup)(({ theme }) => ({
  "& .MuiToggleButtonGroup-grouped": {
    textTransform: "none",
    "&.Mui-selected": {
      backgroundColor: "#7ac1ff",
      color: "#000",
    },
  },
}));

function EventTypeIcon({ evType }: { evType: EventTypeType }) {
  switch (evType) {
    case "Shape":
      return <GestureIcon />;
    case "Release":
      return <UploadIcon />;
    case "Press":
      return <FileDownloadIcon />;
    case "Click":
      return <MouseIcon />;
  }
  return null;
}

export function EventTypeSelector({
  eventType,
  setEventType,
}: {
  eventType: EventTypeType;
  setEventType?: (evType: EventTypeType) => unknown;
}) {
  return (
    <StyledToggleButtonGroup
      size="small"
      value={eventType}
      onChange={(e, d) => {
        if (d) {
          setEventType?.(d);
        }
      }}
      color="primary"
      orientation="vertical"
      exclusive
    >
      {EventTypes.map((e) => e).map((evType) => (
        <ToggleButton key={evType} value={evType}>
          <EventTypeIcon evType={evType} /> {startCase(evType)}
        </ToggleButton>
      ))}
    </StyledToggleButtonGroup>
  );
}
