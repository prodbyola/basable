import { Button, Dialog, DialogActions, DialogContent, DialogTitle, FormControl, InputLabel, MenuItem, Select, TextField } from "@mui/material";
import { BasableFilter, FILTER_OPERATOR_LABELS } from "../../utils";
import { useState } from "react";

type RowFilteringProps = {
  open: boolean;
  columnNames: string[],
  onHideDialog: () => void;
};

const TableFiltering = ({ open, columnNames, onHideDialog }: RowFilteringProps) => {
  const defaultFilter = new BasableFilter(columnNames[0], '')
  const [ tableFilter, setTableFilter ] = useState(defaultFilter)

  return (
    <Dialog open={open}>
      <DialogTitle>
        Data Filtering
      </DialogTitle>
      <DialogContent sx={{ minWidth: "420px", paddingTop: "32px !important" }}>
        <FormControl className="tableConfigField" fullWidth>
          <InputLabel>Column</InputLabel>
          <Select
            id="opp-0"
            labelId="opp0"
            label="Column"
            value={tableFilter.column}
          >
            {columnNames.map((col) => (
              <MenuItem key={col} value={col}>
                {col}
              </MenuItem>
            ))}
          </Select>
        </FormControl>
        <FormControl className="tableConfigField" fullWidth>
          <InputLabel>Operator</InputLabel>
          <Select
            id="opp-1"
            labelId="opp1"
            label="Operator"
            value={tableFilter.operatorKey}
          >
            {FILTER_OPERATOR_LABELS.map((label) => (
              <MenuItem key={label} value={label}>
                {label}
              </MenuItem>
            ))}
          </Select>
        </FormControl>
        <TextField
          id="filter value"
          className="tableConfigField"
          value={tableFilter.value}
          label="Filter Value"
          fullWidth
        />
      </DialogContent>
      <DialogActions>
        <Button onClick={onHideDialog} variant="outlined" size="large">Close</Button>
        <Button onClick={onHideDialog} variant="contained" size="large">Add Filter</Button>
      </DialogActions>
    </Dialog>
  );
};

export default TableFiltering;
