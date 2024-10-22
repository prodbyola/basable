import {
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
  TextField,
} from "@mui/material";
import LoadingButton from "@mui/lab/LoadingButton";

import {
  getTableLabel,
  TableConfig,
  useNetworkRequest,
  useStore,
} from "../../utils";
import { ChangeEvent, useState } from "react";

type ConfigProps = {
  config: Partial<TableConfig>;
  open: boolean;
  columns: string[];
  onHideDialog: () => void;
  onConfigUpdated: (config: Partial<TableConfig>) => void 
};

const TableConfigForm = ({
  config,
  open,
  columns,
  onHideDialog,
  onConfigUpdated,
}: ConfigProps) => {
  const request = useNetworkRequest();
  const columnList = ["None", ...columns];

  const snackBar = useStore((state) => state.snackBar);
  const updateSnackBar = useStore((state) => state.showSnackBar);
  const updateTableConfig = useStore((state) => state.updateTableConfig);

  const [formData, setFormData] = useState(config);
  const [loading, setLoading] = useState(false);

  const handlePkChange = (evt: SelectChangeEvent<string>) => {
    let pk_column: string | undefined = evt.target.value;
    if (pk_column === "None") pk_column = undefined;

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

  const saveConfig = async () => {
    setLoading(true);
    try {
      await request({
        method: "put",
        path: "tables/configurations/" + config.name,
        data: formData,
      });

      updateSnackBar({
        ...snackBar,
        showAlert: true,
        loading: false,
        alertColor: "success",
        message: "Table configuration updated successfully!",
      });

      updateTableConfig(formData as TableConfig);
      onConfigUpdated(formData)
    } catch (err: any) {
      updateSnackBar({
        ...snackBar,
        showAlert: true,
        loading: false,
        alertColor: "error",
        message: err.message,
      });
    }

    setLoading(false);
  };

  return (
    <Dialog open={open}>
      <DialogTitle>
        {getTableLabel(config as TableConfig)} Configuration
      </DialogTitle>
      <DialogContent sx={{ minWidth: "420px", paddingTop: "32px !important" }}>
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
        <LoadingButton
          loading={loading}
          onClick={saveConfig}
          variant="contained"
        >
          Update
        </LoadingButton>
      </DialogActions>
    </Dialog>
  );
};

export default TableConfigForm;
