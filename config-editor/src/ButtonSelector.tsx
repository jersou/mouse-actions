import { Buttons, ButtonType } from "./config.type";
import ToggleButton from "@mui/material/ToggleButton";
import ToggleButtonGroup from "@mui/material/ToggleButtonGroup";
import { styled } from "@mui/material/styles";
import { startCase } from "lodash";
import { MenuItem, Select } from "@mui/material";

const StyledToggleButtonGroup = styled(ToggleButtonGroup)(({ theme }) => ({
  "& .MuiToggleButtonGroup-grouped": {
    textTransform: "none",
    "&.Mui-selected": { backgroundColor: "#7ac1ff", color: "#000" },
  },
}));

export function ButtonSelector({
  button,
  setButton,
}: {
  button: ButtonType;
  setButton?: (evType: ButtonType) => unknown;
}) {
  return (
    <Select
      size="small"
      value={button}
      onChange={(e) => setButton?.(e.target.value as ButtonType)}
      color="primary"
    >
      {Buttons.map((e) => e).map((evType) => (
        <MenuItem key={evType} value={evType}>
          {startCase(evType)}
        </MenuItem>
      ))}
    </Select>
  );
}
