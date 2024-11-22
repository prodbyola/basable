import {
  Box,
  Button,
  Chip,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  FormControl,
  InputLabel,
  MenuItem,
  OutlinedInput,
  Select,
  SelectChangeEvent,
  TextField,
  Theme,
  useTheme,
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
  onConfigUpdated: (config: Partial<TableConfig>) => void;
};

const ITEM_HEIGHT = 48;
const ITEM_PADDING_TOP = 8;
const MenuProps = {
  PaperProps: {
    style: {
      maxHeight: ITEM_HEIGHT * 4.5 + ITEM_PADDING_TOP,
      width: 250,
    },
  },
};

const getStyles = (
  item: string,
  list: readonly string[] | undefined,
  theme: Theme
) => {
  return {
    fontWeight: list && list.includes(item)
      ? theme.typography.fontWeightMedium
      : theme.typography.fontWeightRegular,
  };
};

const TableConfigForm = ({
  config,
  open,
  columns,
  onHideDialog,
  onConfigUpdated,
}: ConfigProps) => {
  const theme = useTheme();
  const request = useNetworkRequest();
  const columnList = ["None", ...columns];

  const showAlert = useStore((state) => state.showAlert);
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

  const onAddSelectColumns = (evt: SelectChangeEvent<string[]>) => {
    const {
      target: { value },
    } = evt;

    const exclude_columns =
      typeof value === "string" ? value.split(",") : value;

    setFormData({
      ...formData,
      exclude_columns,
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
        method: "patch",
        path: "tables/configurations/" + config.name,
        data: formData,
      });

      showAlert("success", "Table configuration updated successfully!");

      updateTableConfig(formData as TableConfig);
      onConfigUpdated(formData);
    } catch (err: any) {
      showAlert("error", err.message);
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
        <FormControl className="tableConfigField" fullWidth>
          <InputLabel id="demo-multiple-chip-label">Exclude Columns</InputLabel>
          <Select
            labelId="tcf-multiple-chip-label"
            id="tcf-multiple-chip"
            multiple
            value={formData.exclude_columns ?? []}
            onChange={onAddSelectColumns}
            input={
              <OutlinedInput
                id="select-multiple-chip"
                label="Exclude Columns"
              />
            }
            renderValue={(selected) => (
              <Box sx={{ display: "flex", flexWrap: "wrap", gap: 0.5 }}>
                {selected.map((value) => (
                  <Chip key={value} label={value} />
                ))}
              </Box>
            )}
            MenuProps={MenuProps}
          >
            {columns.map((name) => (
              <MenuItem
                key={name}
                value={name}
                style={getStyles(name, formData.exclude_columns, theme)}
              >
                {name}
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
