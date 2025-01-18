import {
  Box,
  Button,
  Chip,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  FormControl,
  IconButton,
  InputLabel,
  MenuItem,
  OutlinedInput,
  Select,
  SelectChangeEvent,
  TextField,
  Theme,
  useTheme,
} from "@mui/material";
import CloseIcon from "@mui/icons-material/Close";

import { getTableLabel, TableConfig, TableSearchOpts } from "../../utils";
import { ChangeEvent, useState } from "react";

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
    fontWeight:
      list && list.includes(item)
        ? theme.typography.fontWeightMedium
        : theme.typography.fontWeightRegular,
  };
};

type TableSearchProps = {
  open: boolean;
  columns: string[];
  opts?: Partial<TableSearchOpts>;
  config: Partial<TableConfig>;
  onHideDialog: () => void;
  onSearch: (opts: Partial<TableSearchOpts>) => void
  onClearSearch: () => void
};

const TableSearchForm = ({ open, config, opts, columns, onHideDialog, onSearch, onClearSearch }: TableSearchProps) => {
  const theme = useTheme();
  const [formData, setFormData] = useState(opts ?? {});

  const updateSearchQuery = (
    evt: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const {
      target: { value },
    } = evt;

    setFormData({
      ...formData,
      query: value,
    });
  };

  const addSelectColumns = (evt: SelectChangeEvent<string[]>) => {
    const {
      target: { value },
    } = evt;

    const searchColumns = typeof value === "string" ? value.split(",") : value;

    setFormData({
      ...formData,
      search_cols: searchColumns,
    });
  };

  return (
    <Dialog open={open}>
      <DialogTitle>Search {getTableLabel(config as TableConfig)}</DialogTitle>
      <IconButton
        aria-label="close"
        onClick={onHideDialog}
        sx={(theme) => ({
          position: "absolute",
          right: 8,
          top: 8,
          color: theme.palette.grey[500],
        })}
      >
        <CloseIcon />
      </IconButton>
      <DialogContent sx={{ minWidth: "420px", paddingTop: "32px !important" }}>
        <FormControl className="tableConfigField" fullWidth>
          <InputLabel id="demo-multiple-chip-label">Columns</InputLabel>
          <Select
            labelId="tcf-multiple-chip-label"
            id="tcf-multiple-chip"
            multiple
            value={formData.search_cols ?? []}
            onChange={addSelectColumns}
            input={<OutlinedInput id="select-multiple-chip" label="Columns" />}
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
                style={getStyles(name, formData.search_cols, theme)}
              >
                {name}
              </MenuItem>
            ))}
          </Select>
        </FormControl>
        <TextField
          id="tcf-items_per_page"
          className="tableConfigField"
          value={formData.query ?? ""}
          label="Query"
          onChange={updateSearchQuery}
          fullWidth
        />
      </DialogContent>
      <DialogActions>
        <Button onClick={onClearSearch}>Clear</Button>
        <Button
          onClick={() => onSearch(formData)}
          variant="contained"
        >
          Search
        </Button>
      </DialogActions>
    </Dialog>
  );
};

export default TableSearchForm;
