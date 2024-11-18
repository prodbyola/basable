import * as React from "react";
import { useParams } from "react-router-dom";
import {
  useNetworkRequest,
  TableColumn,
  TableRow,
  TableConfig,
  useStore,
  getTableLabel,
  UpdateTableData,
  BasableFilter,
  TABLE_FILTER_OPERATORS,
  FilterInput,
} from "../../utils";
import { IconButton, ThemeProvider, Typography } from "@mui/material";
import theme from "../../theme";

import ReportIcon from "@mui/icons-material/Report";
import FilterAltIcon from "@mui/icons-material/FilterAlt";
import SettingsIcon from "@mui/icons-material/Settings";
import SearchIcon from "@mui/icons-material/Search";
import SaveIcon from "@mui/icons-material/Save";
import DownloadIcon from "@mui/icons-material/Download";
import TableRefresh from "../../components/common/icons/RefreshIcon";
import TableConfigForm from "../../components/forms/TableConfigForm";
import TableFiltering from "../../components/filters";
import { isAxiosError } from "axios";

const buildFilterQuery = (ft: FilterInput): BasableFilter => {
  const label = ft.operatorLabel;
  let value = ft.operatorValue;

  if (["LIKE", "NOT_LIKE"].includes(label)) value = `${value}%`;
  else if (["LIKE_SINGLE", "NOT_LIKE_SINGLE"].includes(label))
    value = `_${value}%`;
  else if (["RANGE", "NOT_RANGE"].includes(label))
    value = `('${value}' AND '${ft.endValue}')`;

  const operator = TABLE_FILTER_OPERATORS[label];
  return  {
    column: ft.column,
    combinator: ft.combinator,
    expression: {
      [operator.key]: operator.symbol
    }
  };
};

const DatabaseTable = () => {
  const request = useNetworkRequest();
  const { tableID } = useParams();

  const tableConfigs = useStore((state) => state.tableConfigs);
  const showAlert = useStore((state) => state.showAlert);

  const [tableConfig, setTableConfig] = React.useState<Partial<TableConfig>>(
    {}
  );
  const [tableLabel, setTableLabel] = React.useState("");
  const [hasUniqueColumn, setHasUniqueColumn] = React.useState(false);
  const [openTableConfig, setOpenTableConfig] = React.useState(false);

  const [openFiltering, setOpenFiltering] = React.useState(false);
  const [filters, setFilters] = React.useState<FilterInput[]>([]);

  const [filteredColumns, setFilteredColumns] = React.useState<TableColumn[]>(
    []
  );
  const [allColumns, setAllColumns] = React.useState<TableColumn[]>([]);
  const [rows, setRows] = React.useState<TableRow[]>([]);

  const defaultUTD: UpdateTableData = {
    columns: [],
    unique_values: [],
    input: [],
  };
  const [utd, setUTD] = React.useState(defaultUTD);

  const [tableLoading, setTableLoading] = React.useState(false);

  const getColumnValue = (name: string, row: TableRow) => {
    const o = row[name];
    const k = Object.keys(row[name])[0];
    return o[k] as string;
  };

  const getInputLabel = (row: TableRow) => {
    if (hasUniqueColumn) {
      return getColumnValue(tableConfig.pk_column as string, row);
    }

    return "edit-table-input";
  };

  const onInputChange = (
    evt: React.ChangeEvent<HTMLInputElement>,
    column: string,
    rowIndex: number
  ) => {
    const { name: uniqueValue, value } = evt.target;

    // update table rows
    const row = rows[rowIndex];
    row[column][0] = value;
    rows.splice(rowIndex, 1, row);
    setRows([...rows]);

    const uniqueValues = utd.unique_values;
    const exists = uniqueValues.find((uv) => uv === uniqueValue);

    // if column is not added yet, add it
    const columns = utd.columns;
    if (!columns.find((col) => col === column)) {
      columns.push(column);
      utd.columns = columns;
    }

    // if row exists update the row
    if (exists) {
      const i = uniqueValues.indexOf(exists);
      const row = utd.input[i];
      row[column] = value;

      utd.input.splice(i, 1, row);
    } else {
      utd.unique_values.push(uniqueValue);
      utd.input.push({ [column]: value });
    }

    setUTD({ ...utd });
  };

  const updateConfigStates = (config: TableConfig) => {
    setTableConfig(config);
    setHasUniqueColumn(typeof config.pk_column === "string");
    setTableLabel(getTableLabel(config as TableConfig));

    if (typeof config.pk_column === "string") {
      setUTD({
        ...utd,
        unique_key: config.pk_column,
      });
    }
  };

  const updateData = async () => {
    if (!utd.unique_values.length) return;

    try {
      await request({
        method: "patch",
        path: "tables/data/" + tableID,
        data: utd,
      });

      showAlert("success", "Table data saved successfully");

      setUTD({
        ...defaultUTD,
        unique_key: tableConfig.pk_column,
      });
    } catch (err: any) {
      showAlert("error", err.message);
    }
  };

  const loadData = async () => {
    setTableLoading(true);

    try {
      const cols = (await request({
        method: "get",
        path: "tables/columns/" + tableID,
      })) as TableColumn[];

      setAllColumns(cols);

      let fcols = cols;
      let dataQuery = `?table=${tableID}`;

      // Add excluded columns to query
      const tc = tableConfigs.find((c) => c.name === tableID);
      if (tc) {
        const excluded = tc.exclude_columns;
        if (excluded && excluded.length) {
          fcols = cols.filter((col) => !excluded.includes(col.name));
          const selection = cols.map((col) => col.name).join(",");

          dataQuery += "&columns=" + selection;
        }

        // if there's a unique column, always shift it to leftmost
        if (tc.pk_column) {
          const pkc = fcols.find((col) => col.name === tc.pk_column);
          if (pkc) {
            const i = fcols.indexOf(pkc);
            fcols.splice(i, 1);
            fcols.splice(0, 0, pkc);
          }
        }

        updateConfigStates(tc);
      }
      setFilteredColumns(fcols);

      // Add filters to query param
      if (filters.length) {
        const fs = filters.map((f) => buildFilterQuery(f));
        const fj = fs.join(",");
        console.log(fs, fj);
        dataQuery = dataQuery + `&filters=${fj}`;
      }

      const rows = (await request({
        method: "get",
        path: `tables/data/${tableID}` + dataQuery,
      })) as TableRow[];

      setRows(rows);
      setTableLoading(false);
    } catch (err: any) {
      setTableLoading(false);

      let msg = err.message;
      if (isAxiosError(err)) {
        msg = err.response?.data;
      }

      showAlert("error", msg);
    }
  };

  React.useEffect(() => {
    if (tableID) loadData();
  }, [request, tableID]);

  if (tableLoading) return <div>Loading</div>;

  return (
    <ThemeProvider theme={theme}>
      <div className="displayTableHeader">
        <div
          className="tableConfig"
          key={tableLabel}
          style={{
            backgroundColor: theme.palette.primary.main,
            color: "white",
          }}
          onClick={() => setOpenTableConfig(true)}
        >
          <SettingsIcon />
          <h3>{tableLabel}</h3>
        </div>
        <div className="tableToolbar">
          <IconButton>
            <DownloadIcon />
          </IconButton>
          <IconButton>
            <TableRefresh />
          </IconButton>
          <IconButton>
            <SearchIcon />
          </IconButton>
          <IconButton onClick={() => setOpenFiltering(true)}>
            <FilterAltIcon />
          </IconButton>
          <IconButton
            sx={{
              backgroundColor: utd.unique_values.length
                ? theme.palette.primary.main
                : "",
              color: utd.unique_values.length ? "white" : "",
            }}
            onClick={updateData}
          >
            <SaveIcon />
          </IconButton>
        </div>
      </div>
      {!hasUniqueColumn && (
        <div className="tableHeaderWarning">
          <ReportIcon />
          <Typography>
            No <strong>unique column</strong> is found for filter table. Table
            modification is impossible. You can manually set unique column by
            clicking the settings button above.
          </Typography>
        </div>
      )}
      <section className="displayTable dashboardDisplay databaseTable">
        <table>
          <thead>
            <tr>
              {filteredColumns.map((col) => (
                <th key={col.name}>{col.name}</th>
              ))}
            </tr>
          </thead>
          <tbody>
            {rows.map((row, index) => (
              <tr className="editableRow" key={index}>
                {filteredColumns.map((col) => (
                  <td key={col.name}>
                    {
                      <input
                        name={getInputLabel(row)}
                        value={getColumnValue(col.name, row)}
                        onChange={(evt) => onInputChange(evt, col.name, index)}
                        disabled={!hasUniqueColumn}
                      />
                    }
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      </section>
      <TableConfigForm
        config={tableConfig}
        open={openTableConfig}
        columns={allColumns.map((col) => col.name)}
        onHideDialog={() => setOpenTableConfig(false)}
        onConfigUpdated={(config) => {
          if (config !== tableConfig) loadData();
        }}
      />
      <TableFiltering
        open={openFiltering}
        columnNames={allColumns.map((col) => col.name)}
        tableFilters={filters}
        onHideDialog={() => setOpenFiltering(false)}
        onUpdateFilters={(newFilters) => {
          if (newFilters !== filters) {
            setFilters([...newFilters]);
            loadData();
          }
        }}
      />
    </ThemeProvider>
  );
};

export default DatabaseTable;
