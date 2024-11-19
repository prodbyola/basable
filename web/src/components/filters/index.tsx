import "./index.scss";

import CloseIcon from "@mui/icons-material/Close";
import {
  Box,
  Button,
  Dialog,
  DialogActions,
  DialogContent,
  DialogTitle,
  IconButton,
  Tab,
  Tabs,
} from "@mui/material";
import { ColumnTypeObject, FilterInput, sampleFilter } from "../../utils";
import { useState } from "react";
import ShowFilterList from "./ShowFilterList";
import FilterForm from "../forms/FilterForm";

interface TabPanelProps {
  children?: React.ReactNode;
  index: number;
  value: number;
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props;

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`simple-tabpanel-${index}`}
      aria-labelledby={`simple-tab-${index}`}
      {...other}
    >
      {value === index && <Box sx={{ p: 3 }}>{children}</Box>}
    </div>
  );
}

type RowFilteringProps = {
  open: boolean;
  columnNames: string[];
  tableFilters: FilterInput[];
  columnTypes: ColumnTypeObject[]
  onHideDialog: () => void;
  onUpdateFilters: (filters: FilterInput[]) => void;
};

const a11yProps = (index: number) => ({
  id: `filtering-tab-${index}`,
  "aria-controls": `filtering-tabpanel-${index}`,
});

const TableFiltering = ({
  open,
  columnNames,
  tableFilters,
  columnTypes,
  onHideDialog,
  onUpdateFilters,
}: RowFilteringProps) => {
  const [filters, setFilters] = useState<FilterInput[]>(tableFilters);
  const [tabValue, setTabValue] = useState(0);
  const changeTabValue = (_: React.SyntheticEvent, newValue: number) =>
    setTabValue(newValue);

  const insertFilter = (filter: FilterInput) => {
    setFilters([...filters, filter]);
    setTabValue(0);
  };

  const removeFilter = (index: number) => {
    filters.splice(index, 1);

    if (filters.length) {
      const first = filters[0];
      if (first.combinator !== "base") first.combinator = "base";
    }

    setFilters([...filters]);
  };

  const [defaultFilter, setDefaultFilter] = useState<FilterInput>(sampleFilter);

  return (
    <Dialog open={open}>
      <DialogTitle>Data Filtering</DialogTitle>
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
        <Tabs value={tabValue} onChange={changeTabValue}>
          <Tab label="Filters" {...a11yProps(0)} />
          <Tab label="Filtering" {...a11yProps(1)} />
        </Tabs>
        <TabPanel value={tabValue} index={0}>
          <ShowFilterList
            filters={filters}
            onCreateFilter={() => setTabValue(1)}
            onRequestNewFilter={(filter) => {
              setDefaultFilter(filter);
              setTabValue(1);
            }}
            onRemoveFilter={removeFilter}
          />
        </TabPanel>
        <TabPanel value={tabValue} index={1}>
          <FilterForm
            columnNames={columnNames}
            onInsertFilter={insertFilter}
            defaultFilter={defaultFilter}
            columnTypes={columnTypes}
          />
        </TabPanel>
      </DialogContent>
      {tabValue === 0 && (
        <DialogActions>
          {filters.length > 0 && (
            <Button
              onClick={() => {
                setFilters([]);
              }}
              variant="outlined"
              size="large"
            >
              Clear Filter
            </Button>
          )}
          <Button
            onClick={() => {
              onUpdateFilters(filters);
            }}
            variant="contained"
            size="large"
          >
            Update Filter
          </Button>
        </DialogActions>
      )}
    </Dialog>
  );
};

export default TableFiltering;
