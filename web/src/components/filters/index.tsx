import "./index.scss";

import CloseIcon from '@mui/icons-material/Close';
import {
  Box,
  Dialog,
  DialogContent,
  DialogTitle,
  IconButton,
  Tab,
  Tabs,
} from "@mui/material";
import { BasableFilter } from "../../utils";
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
  onHideDialog: () => void;
};

const a11yProps = (index: number) => ({
  id: `filtering-tab-${index}`,
  "aria-controls": `filtering-tabpanel-${index}`,
});

const TableFiltering = ({
  open,
  columnNames,
  onHideDialog,
}: RowFilteringProps) => {
  const [filters, setFilters] = useState<BasableFilter[]>([])
  const [tabValue, setTabValue] = useState(0);
  const changeTabValue = (_: React.SyntheticEvent, newValue: number) =>
    setTabValue(newValue);

  const insertFilter = (filter: BasableFilter) => {
    filters.push(filter)
    setFilters([...filters])
    setTabValue(0)
  } 

  return (
    <Dialog open={open}>
      <DialogTitle>Data Filtering</DialogTitle>
      <IconButton
          aria-label="close"
          onClick={onHideDialog}
          sx={(theme) => ({
            position: 'absolute',
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
          <ShowFilterList filters={filters} onCreateFilter={() => setTabValue(1)} />
        </TabPanel>
        <TabPanel value={tabValue} index={1}>
          <FilterForm columnNames={columnNames} onInsertFilter={ insertFilter } />
        </TabPanel>
      </DialogContent>
      {/* <DialogActions>
        <Button onClick={onHideDialog} variant="outlined" size="large">
          Close
        </Button>
        <Button onClick={onHideDialog} variant="contained" size="large">
          Add Filter
        </Button>
      </DialogActions> */}
    </Dialog>
  );
};

export default TableFiltering;
