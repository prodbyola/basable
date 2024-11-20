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
  ColumnType,
  ColumnTypeObject,
  FILTER_OPERATOR_LABELS,
  FilterInput,
  FilterOperatorLabel,
} from "../../utils";
import { ChangeEvent, useState } from "react";
import AddIcon from "@mui/icons-material/Add";

type FilterFormProps = {
  columnNames: string[];
  defaultFilter: FilterInput;
  columnTypes: ColumnTypeObject[];
  onInsertFilter: (filter: FilterInput) => void;
};

const FilterForm = ({
  columnNames,
  defaultFilter,
  columnTypes,
  onInsertFilter,
}: FilterFormProps) => {
  const [filter, setTableFilter] = useState(defaultFilter);
  const [fieldType, setFieldType] = useState("text");

  const updateFilter = (filter: FilterInput) =>
    setTableFilter(JSON.parse(JSON.stringify(filter)));

  const getColumnType = (columnName: string) => {
    const ct = columnTypes.find((ct) => ct[columnName] !== undefined);
    if (ct) return ct[columnName];
  };

  const getFieldType = (ct: ColumnType) => {
    if (["Int", "UInt"].includes(ct)) return "number";
    return "text";
  };

  const changeFilterColumn = (evt: SelectChangeEvent<string>) => {
    const {
      target: { value },
    } = evt;

    const columnType = getColumnType(value);
    if (columnType) setFieldType(getFieldType(columnType));

    filter.column = value;
    updateFilter(filter);
  };

  const changeFilterOperator = (evt: SelectChangeEvent<string>) => {
    const {
      target: { value },
    } = evt;

    filter.operatorLabel = value as FilterOperatorLabel;
    updateFilter(filter);
  };

  const onValueChange = (
    evt: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>
  ) => {
    const target = evt.target as HTMLInputElement;
    const value = target.value;

    filter.operatorValue = value;
    updateFilter(filter);
  };

  const createFilter = () => {
    onInsertFilter(filter);
    updateFilter(defaultFilter);
  };

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
          value={filter.operatorLabel}
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
        value={filter.operatorValue}
        onChange={onValueChange}
        label="Filter Value"
        type={fieldType}
        fullWidth
      />
      <div className="formAction">
        <Button
          variant="contained"
          size="large"
          startIcon={<AddIcon />}
          onClick={createFilter}
        >
          Create Filter
        </Button>
      </div>
    </>
  );
};

export default FilterForm;
