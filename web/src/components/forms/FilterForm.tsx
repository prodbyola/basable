import {
  Button,
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
  TextField,
} from "@mui/material";
import {
  BasableFilter,
  FILTER_OPERATOR_LABELS,
  FilterOperatorLabel,
} from "../../utils";
import { ChangeEvent, useState } from "react";
import AddIcon from '@mui/icons-material/Add';

type FilterFormProps = {
  columnNames: string[];
  onInsertFilter: (filter: BasableFilter) => void;
};

const FilterForm = ({ columnNames, onInsertFilter }: FilterFormProps) => {
  const defaultFilter = new BasableFilter(columnNames[0]);
  const [filter, setTableFilter] = useState(defaultFilter);

  const updateFilter = (filter: BasableFilter) =>
    setTableFilter(JSON.parse(JSON.stringify(filter)));

  const changeFilterColumn = (evt: SelectChangeEvent<string>) => {
    const {
      target: { value },
    } = evt;

    filter.column = value;
    updateFilter(filter);
  };

  const changeFilterOperator = (evt: SelectChangeEvent<string>) => {
    const {
      target: { value },
    } = evt;

    filter.operatorKey = value as FilterOperatorLabel;
    updateFilter(filter);
  };

  const onValueChange = (
    evt: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const target = evt.target as HTMLInputElement;
    const value = target.value;

    filter.filterValue = value;
    updateFilter(filter);
  };

  const createFilter = () => {
    onInsertFilter(filter)
    updateFilter(defaultFilter)
  }

  return (
    <>
      <FormControl className="tableConfigField" fullWidth>
        <InputLabel>Column</InputLabel>
        <Select
          id="opp-0"
          labelId="opp0"
          label="Column"
          value={filter.column}
          onChange={changeFilterColumn}
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
          value={filter.operatorKey}
          onChange={changeFilterOperator}
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
        value={filter.filterValue}
        onChange={onValueChange}
        label="Filter Value"
        fullWidth
      />
      <div className="formAction">
        <Button
          variant="contained"
          size="large"
          startIcon={<AddIcon />}
          onClick={ createFilter }
        >
          Create Filter
        </Button>
      </div>
    </>
  );
};

export default FilterForm;