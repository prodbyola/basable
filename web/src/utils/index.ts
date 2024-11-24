import {
  BasableFilter,
  ColumnTypeObject,
  FilterInput,
  TABLE_FILTER_OPERATORS,
  TableConfig,
  TableRow,
} from "./data_types";

export * from "./cookies";
export * from "./data_types";
export * from "./network";
export * from "./store";
export * from "./hooks";

export const getTableLabel = (c: TableConfig) => c.label ?? c.name;

export const sampleFilter: FilterInput = {
  combinator: "base",
  column: "",
  operatorLabel: "EQUAL",
  operatorValue: "",
};

export const extractColumnTypes = (row: TableRow): ColumnTypeObject[] => {
  const columnNames = Object.keys(row);
  const types = columnNames.map((col) => {
    const f = row[col];
    const t = Object.keys(f)[0];

    return { [col]: t } as ColumnTypeObject;
  });

  return types;
};

export const buildFilterQuery = (ft: FilterInput): BasableFilter => {
  const label = ft.operatorLabel;
  let value = ft.operatorValue;

  if (["LIKE", "NOT_LIKE"].includes(label)) value = `${value}%`;
  else if (["LIKE_SINGLE", "NOT_LIKE_SINGLE"].includes(label))
    value = `_${value}%`;
  else if (["RANGE", "NOT_RANGE"].includes(label))
    value = `('${value}' AND '${ft.endValue}')`;

  const operator = TABLE_FILTER_OPERATORS[label];
  const combinator = ft.combinator.toUpperCase();
  return {
    column: ft.column,
    combinator,
    expression: {
      [operator]: value,
    },
  };
};

export const downloadExport = (data: string, mimetype: string, filename: string) => {
  const blob = new Blob([data], { type: mimetype });
  const url = URL.createObjectURL(blob);
  // Download the Blob as a file
  const link = document.createElement("a");
  link.href = url;
  link.download = filename; // Specify the file name
  link.click();

  // Clean up the Blob URL
  URL.revokeObjectURL(url);
};
