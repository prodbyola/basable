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
  FilterInput,
  ColumnTypeObject,
  buildFilterQuery,
  extractColumnTypes,
  OrderByKey,
  TableQueryOpts,
  TableSearchOpts,
  DownloadFormat,
  downloadExport,
} from "../../utils";
import { Button, ButtonGroup, ThemeProvider, Typography } from "@mui/material";
import theme from "../../theme";

import ReportIcon from "@mui/icons-material/Report";
import FilterAltIcon from "@mui/icons-material/FilterAlt";
import SettingsIcon from "@mui/icons-material/Settings";
import SearchIcon from "@mui/icons-material/Search";
import SaveIcon from "@mui/icons-material/Save";
import DownloadIcon from "@mui/icons-material/Download";
import DeleteIcon from "@mui/icons-material/Delete";
import TableRefresh from "../../components/common/icons/RefreshIcon";
import TableConfigForm from "../../components/forms/TableConfigForm";
import TableFiltering from "../../components/filters";
import { isAxiosError } from "axios";
import TableNavigator from "../../components/table/Navigator";
import TableSearchForm from "../../components/forms/TableSearchForm";
import DownloadMenu from "../../components/table/DownloadMenu";
import DeleteItemsDialog from "../../components/DeleteItems";

const DatabaseTable = () => {
  const request = useNetworkRequest();
  const { tableID } = useParams();

  const tableConfigs = useStore((state) => state.tableConfigs);
  const [tableConfig, setTableConfig] = React.useState<Partial<TableConfig>>(
    {}
  );
  const showAlert = useStore((state) => state.showAlert);

  const defaultQueryOpts: TableQueryOpts = {
    table: tableID as string,
    offset: 0,
    row_count: tableConfig.items_per_page ?? 100,
    filters: [],
  };

  const [queryOpts, setQueryOpts] = React.useState<TableQueryOpts | undefined>(
    undefined
  );
  const [selectedRows, setSelectedRows] = React.useState<number[]>([]);
  const [openDeleteDialog, setOpenDeleteDialog] = React.useState(false);
  const [queryCount, setQueryCount] = React.useState(0);
  const [navPage, setNavPage] = React.useState(0);
  const totalPages = React.useMemo(() => {
    const ps = queryOpts ? queryCount / queryOpts.row_count : 0;

    return Math.ceil(ps);
  }, [queryCount, queryOpts]);

  const [tableLabel, setTableLabel] = React.useState("");
  const [hasUniqueColumn, setHasUniqueColumn] = React.useState(false);
  const [openTableConfig, setOpenTableConfig] = React.useState(false);
  const [openSearchForm, setOpenSearchForm] = React.useState(false);
  const [downloadMenuTarget, setDownloadMenuTarget] =
    React.useState<null | HTMLElement>(null);

  const [openFiltering, setOpenFiltering] = React.useState(false);
  const [filters, setFilters] = React.useState<FilterInput[] | undefined>(
    undefined
  );

  const [filteredColumns, setFilteredColumns] = React.useState<TableColumn[]>(
    []
  );
  const [allColumns, setAllColumns] = React.useState<TableColumn[]>([]);
  const [rows, setRows] = React.useState<TableRow[]>([]);
  const [columnTypes, setColumnTypes] = React.useState<ColumnTypeObject[]>([]);

  const defaultUTD: UpdateTableData = {
    columns: [],
    unique_values: [],
    input: [],
  };
  const [utd, setUTD] = React.useState(defaultUTD);

  const [tableLoading, setTableLoading] = React.useState(true);

  const selectRow = (event: React.MouseEvent, rowIndex: number) => {
    if (event.ctrlKey || event.metaKey) {
      if (selectedRows.includes(rowIndex)) {
        const i = selectedRows.indexOf(rowIndex);
        selectedRows.splice(i, 1);
        setSelectedRows([...selectedRows]);
        return;
      }

      setSelectedRows([...selectedRows, rowIndex]);
    } else {
      if (selectedRows.length) setSelectedRows([]);
    }
  };

  const navigateTable = (to: "prev" | "next") => {
    const rowCount = queryOpts ? queryOpts.row_count : 0;
    const dest = to === "next" ? navPage + 1 : navPage - 1;
    const offset = rowCount * dest;

    if (dest < 0 || dest === totalPages) {
      return;
    }

    setQueryOpts({
      ...(queryOpts as TableQueryOpts),
      offset,
    });

    setNavPage(dest);
  };

  const getColumnType = (columnName: string) => {
    const ct = columnTypes.find((ct) => ct[columnName] !== undefined);
    if (ct) return ct[columnName];
  };

  const getColumnValue = (name: string, row: TableRow) => {
    const o = row[name];
    if (row[name]) {
      const k = Object.keys(row[name])[0];
      return o[k] as string;
    }

    return "NULL";
  };

  const getInputLabel = (row: TableRow) => {
    if (hasUniqueColumn) {
      return getColumnValue(tableConfig.pk_column as string, row);
    }

    return "edit-table-input";
  };

  const updateOrderBy = (columnName: string) => {
    let key: OrderByKey = "ASC";

    if (queryOpts) {
      if (queryOpts.order_by) {
        const ob = queryOpts.order_by;
        key = Object.keys(ob)[0] as OrderByKey;
        const currentColumn = ob[key];

        if (currentColumn === columnName) {
          key = key === "ASC" ? "DESC" : "ASC";
        }
      }

      setQueryOpts({
        ...queryOpts,
        order_by: {
          [key]: columnName,
        },
      });
    }
  };

  /**
   * Update a specific cell for a given row and column.
   * @param evt - input event
   * @param column - name of column where the cell belongs.
   * @param rowIndex - index of current row being edited.
   */
  const updateRowCell = (
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

  /**
   * Updates `tableCOnfig` and all related dependencies.
   * @param config - The new config value.
   * @param isInit - Indicates the function is being called on component init.
   */
  const updateConfigStates = (config: TableConfig, isInit = false) => {
    setHasUniqueColumn(typeof config.pk_column === "string");
    setTableLabel(getTableLabel(config as TableConfig));
    setNavPage(0);

    let fcols = allColumns;
    const excluded = config.exclude_columns;
    let selection: string[] = [];

    if (excluded && excluded.length) {
      fcols = fcols.filter((col) => !excluded.includes(col.name));
      selection = allColumns.map((col) => col.name);
    }

    // Move unique column to left most
    if (config.pk_column) {
      const pkc = fcols.find((col) => col.name === config.pk_column);
      if (pkc) {
        const i = fcols.indexOf(pkc);
        fcols.splice(i, 1);
        fcols.splice(0, 0, pkc);
      }
    }

    setFilteredColumns([...fcols]);
    setTableConfig(config);

    if (!isInit) {
      const row_count = config.items_per_page ?? 100;
      setQueryOpts({
        ...(queryOpts as TableQueryOpts),
        columns: selection,
        row_count,
      });
    }

    if (typeof config.pk_column === "string") {
      setUTD({
        ...utd,
        unique_key: config.pk_column,
      });
    }
  };

  /**
   * Calls api endpoint for updating all data changes.
   * @returns
   */
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

  /**
   * The initial function we call to retrieve table columns and initialize
   * state preludes.
   */
  const loadColumns = async () => {
    setTableLoading(true);

    const cols = (await request({
      method: "get",
      path: "tables/columns/" + tableID,
    })) as TableColumn[];

    setAllColumns(cols);
  };

  /**
   * This function loads table data based on options set for `queryOpts`.
   */
  const loadData = async () => {
    setTableLoading(true);

    try {
      // Get row count
      let count = queryCount;
      if (navPage === 0 || !queryOpts?.offset) {
        count = (await request({
          method: "post",
          path: `tables/query-result-count/${tableID}`,
          data: queryOpts,
        })) as number;
      }

      // get rows
      const rows = (await request({
        method: "post",
        path: `tables/query-data/${tableID}`,
        data: queryOpts,
      })) as TableRow[];

      if (rows.length) {
        const cts = extractColumnTypes(rows[0]);
        setColumnTypes(cts);
      }

      setRows(rows);
      setQueryCount(count);
    } catch (err: any) {
      let msg = err.message;
      if (isAxiosError(err)) {
        msg = err.response?.data;
      }

      showAlert("error", msg);
    }

    setTableLoading(false);
  };

  const initiateDownload = async (format: DownloadFormat) => {
    const { data, filename, mimetype } = (await request({
      method: "post",
      path: `tables/data/export/${tableID}`,
      data: {
        query_opts: queryOpts,
        format,
      },
    })) as any;

    downloadExport(data, mimetype, filename);
  };

  // function we call page reload
  React.useEffect(() => {
    if (tableID) {
      // reset states
      setQueryOpts(undefined);
      setFilters(undefined);
      setNavPage(0);

      // load columns
      loadColumns();
    }
  }, [tableID]);

  // When `filters` is updated, update `queryOpts.filters` which then trigggers
  // `loadData` function.
  React.useEffect(() => {
    if (filters) {
      const fs = filters.map((f) => buildFilterQuery(f));
      setQueryOpts({
        ...(queryOpts as TableQueryOpts),
        filters: fs,
        search_opts: undefined,
      });
    }
  }, [filters]);

  // The effect should be triggered at component initiation after all columns is loaded.
  React.useEffect(() => {
    const tc = tableConfigs.find((c) => c.name === tableID);
    if (allColumns.length && tc && tableID) {
      setQueryOpts({
        ...defaultQueryOpts,
        table: tableID,
        row_count: tc.items_per_page ?? 100,
      });

      updateConfigStates(tc, true);
    }
  }, [allColumns]);

  // Everytime we update `queryOpts`, we trigger `loadData` function.
  React.useEffect(() => {
    if (queryOpts) {
      loadData();
    }
  }, [queryOpts]);

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
        {/* <div className="tableToolbar"> */}
        <ButtonGroup size="small">
          {selectedRows.length && (
            <Button onClick={ () => setOpenDeleteDialog(true) }>
              <DeleteIcon />
            </Button>
          )}
          <Button
            onClick={(event: React.MouseEvent<HTMLButtonElement>) =>
              setDownloadMenuTarget(event.currentTarget)
            }
          >
            <DownloadIcon />
          </Button>
          <DownloadMenu
            anchorEl={downloadMenuTarget}
            onClose={(format) => {
              setDownloadMenuTarget(null);
              if (format) initiateDownload(format);
            }}
          />
          <Button onClick={() => loadData()}>
            <TableRefresh color={theme.palette.primary.main} />
          </Button>
          <Button onClick={() => setOpenSearchForm(true)}>
            <SearchIcon />
          </Button>
          <Button onClick={() => setOpenFiltering(true)}>
            <FilterAltIcon />
          </Button>
          <Button
            sx={{
              backgroundColor: utd.unique_values.length
                ? theme.palette.primary.main
                : "",
              color: utd.unique_values.length ? "white" : "",
            }}
            onClick={updateData}
          >
            <SaveIcon />
          </Button>
        </ButtonGroup>
        {/* </div> */}
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

      <TableNavigator
        count={queryCount}
        totalPages={totalPages}
        currentPage={navPage}
        onNavigate={navigateTable}
      />
      <section className="displayTable dashboardDisplay databaseTable">
        <table>
          <thead>
            <tr>
              {filteredColumns.map((col) => (
                <th key={col.name} onClick={() => updateOrderBy(col.name)}>
                  {col.name}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {rows.length ? (
              rows.map((row, index) => (
                <tr
                  className={`editableRow ${
                    selectedRows.includes(index) ? "selectedRow" : ""
                  }`}
                  key={index}
                  onClick={(evt) => selectRow(evt, index)}
                >
                  {filteredColumns.map((col) => (
                    <td key={col.name}>
                      {
                        <input
                          name={getInputLabel(row)}
                          value={getColumnValue(col.name, row)}
                          onChange={(evt) =>
                            updateRowCell(evt, col.name, index)
                          }
                          disabled={!hasUniqueColumn}
                        />
                      }
                    </td>
                  ))}
                </tr>
              ))
            ) : (
              <tr className="editableRow emptyData">
                <td colSpan={filteredColumns.length}>
                  <div className="emptyDataBox">
                    <h3>No Data</h3>
                  </div>
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </section>
      <TableConfigForm
        config={tableConfig}
        open={openTableConfig}
        columns={filteredColumns.map((col) => col.name)}
        onHideDialog={() => setOpenTableConfig(false)}
        onConfigUpdated={(config) => {
          if (config !== tableConfig) {
            setOpenTableConfig(false);
            updateConfigStates(config as TableConfig);
          }
        }}
      />
      <TableFiltering
        open={openFiltering}
        columnNames={allColumns.map((col) => col.name)}
        tableFilters={filters ?? []}
        columnTypes={columnTypes}
        onHideDialog={() => setOpenFiltering(false)}
        onUpdateFilters={(newFilters) => {
          setOpenFiltering(false);
          if (newFilters !== filters) {
            setFilters(newFilters);
            setNavPage(0);
          }
        }}
      />
      <TableSearchForm
        open={openSearchForm}
        config={tableConfig}
        opts={queryOpts?.search_opts}
        columns={allColumns
          .filter((col) => getColumnType(col.name) === "Text")
          .map((col) => col.name)}
        onHideDialog={() => setOpenSearchForm(false)}
        onSearch={(opts) => {
          setOpenSearchForm(false);

          if (opts !== queryOpts?.search_opts) {
            if (
              opts.search_cols &&
              opts.search_cols.length &&
              opts.query &&
              opts.query.length
            ) {
              setNavPage(0);
              setQueryOpts({
                ...(queryOpts as TableQueryOpts),
                search_opts: opts as TableSearchOpts,
                offset: 0,
              });
            }
          }
        }}
        onClearSearch={() => {
          setOpenSearchForm(false);

          setNavPage(0);
          setQueryOpts({
            ...(queryOpts as TableQueryOpts),
            search_opts: undefined,
            offset: 0,
          });
        }}
      />
      <DeleteItemsDialog
        open={openDeleteDialog}
        title="Delete Items"
        content={`Delete ${selectedRows.length} item${ selectedRows.length > 1 ? 's' : '' } from ${tableID}? This operation is irreversible.`}
        onHideDialog={ () => setOpenDeleteDialog(false) }
        onDelete={() => console.log(selectedRows)}
      />
    </ThemeProvider>
  );
};

export default DatabaseTable;
