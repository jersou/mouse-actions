import {
  EventTypes,
  EventTypeType,
} from "./config.type";
import ToggleButton from "@mui/material/ToggleButton";
import ToggleButtonGroup from "@mui/material/ToggleButtonGroup";
import { styled } from "@mui/material/styles";
import { startCase } from "lodash";

const StyledToggleButtonGroup = styled(ToggleButtonGroup)(({ theme }) => ({
  "& .MuiToggleButtonGroup-grouped": {
    textTransform: "none",
    "&.Mui-selected": {
      backgroundColor: "#7ac1ff",
      color: "#000",
    },
  },
}));

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
      onChange={(e, d) => setEventType?.(d)}
      color="primary"
      orientation="vertical"
      exclusive
    >
      {EventTypes.map((e) => e).map((evType) => (
        <ToggleButton key={evType} value={evType}>
          {startCase(evType)}
        </ToggleButton>
      ))}
    </StyledToggleButtonGroup>
  );
}
