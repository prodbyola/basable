import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogContentText,
  DialogTitle,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
  TextField,
} from "@mui/material";
import { TableConfig } from "../../utils";
import { ChangeEvent, useState } from "react";

type ConfigProps = {
  config: Partial<TableConfig>;
  open: boolean;
  onHideDialog: () => void;
  columns: string[];
};

const TableConfigForm = ({
  config,
  open,
  onHideDialog,
  columns,
}: ConfigProps) => {
  const [formData, setFormData] = useState(config);
  const columnList = ["None", ...columns];

  const handlePkChange = (evt: SelectChangeEvent<string>) => {
    const pk_column = evt.target.value;
    setFormData({
      ...formData,
      pk_column,
    });
  };

  const onValueChange = (
    evt: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const target = evt.target as HTMLInputElement;
    const id = target.id;

    const key = id.split("-")[1];
    const value = target.value;

    setFormData({
      ...formData,
      [key]: value,
    });
  };

  const saveConfig = () => {
    console.log(formData)
  }

  return (
    <Dialog open={open}>
      <DialogTitle>{config.label} Configuration</DialogTitle>
      <DialogContent sx={{ minWidth: "420px", marginTop: "12px" }}>
        <TextField
          id="tcf-label"
          className="tableConfigField"
          value={formData.label}
          label="Table Label"
          onChange={onValueChange}
          fullWidth
        />
        <FormControl className="tableConfigField" fullWidth>
          <InputLabel>Unique Column</InputLabel>
          <Select
            id="tcf-pk_column"
            labelId="pk_column"
            value={
              typeof formData.pk_column === "string"
                ? formData.pk_column
                : "None"
            }
            label="Unique Column"
            onChange={handlePkChange}
          >
            {columnList.map((col) => (
              <MenuItem key={col} value={col}>
                {col}
              </MenuItem>
            ))}
          </Select>
        </FormControl>
        <TextField
          id="tcf-items_per_page"
          className="tableConfigField"
          value={formData.items_per_page ?? 100}
          label="Items Per Page"
          type="number"
          onChange={onValueChange}
          fullWidth
        />
      </DialogContent>
      <DialogActions>
        <Button onClick={onHideDialog}>Cancel</Button>
        <Button onClick={saveConfig} variant="contained">Update</Button>
      </DialogActions>
    </Dialog>
  );
};

export default TableConfigForm;
