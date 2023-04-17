import { Modifiers, ModifierType } from "./config.type";
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

export function ModifiersSelector({
  modifiers,
  setModifiers,
}: {
  modifiers: ModifierType[];
  setModifiers?: (edges: ModifierType[]) => unknown;
}) {
  return (
    <StyledToggleButtonGroup
      size="small"
      value={modifiers}
      onChange={(e, d) => setModifiers?.(d)}
      color="primary"
      style={{ display: "flex" }}
    >
      {Modifiers.map((e) => e).map((modifier) => (
        <ToggleButton key={modifier} value={modifier} style={{ flex: 1 }}>
          {startCase(modifier)}
        </ToggleButton>
      ))}
    </StyledToggleButtonGroup>
  );
}
