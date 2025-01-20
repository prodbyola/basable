export type GraphDataType = {
  label: string;
  value: number;
};

export type TableSummaryType = {
  name: string;
  row_count: number;
  col_count: number;
  created: string;
  updated: string;
};

export type AuthTokenType = {
  token: string;
  exp: number;
};

export type CurrentUser = {
  name: string;
  dp?: string;
  role: string;
  isLogged: boolean;
};

export type SessionCookie = {
  token: string;
  connID: string;
  isAuth: boolean;
};

export type ServerDetails = {
  version: string;
  db_size: number;
  os: string;
  comment: string;
};

export type TableConfig = {
  events?: unknown;
  label: string;
  name: string;
  pk_column?: string;
  created_column?: string;
  update_column?: unknown;
  special_columns?: unknown;
  items_per_page?: number;
  exclude_columns?: string[];
};

export type TableColumn = {
  col_type: string;
  name: string;
  nullable: boolean;
  primary: boolean;
  unique: boolean;
  default_value: unknown;
};

export type TableRow = {
  [key: string]: {
    [key: string]: unknown;
  };
};

export type UpdateTableData = {
  unique_key?: string;
  columns: string[];
  unique_values: string[];
  input: { [key: string]: string }[];
};

export const TABLE_FILTER_OPERATORS = {
  EQUAL: "Eq",
  NOT_EQUAL: "NotEq",
  CONTAINS: "Contains",
  NOT_CONTAINS: "NotContains",
  GREATER_THAN: "Gt",
  LESS_THAN: "Lt",
  GREATER_OR_EQUAL: "Gte",
  LESS_OR_EQUAL: "Lte",
  INCLUDES: "Includes",
  NOT_INCLUDE: "NotInclude",
  REGEX: "Regex",
  NOT_REGEX: "NotRegex",
  RANGE: "Btw",
  NOT_RANGE: "NotBtw",
  NULL: "Null",
  NOT_NULL: "NotNull",
};

export type FilterOperatorLabel = keyof typeof TABLE_FILTER_OPERATORS;

export type FilterCombinator = "base" | "and" | "or";

/**
 * A list of labels for each filter operator
 */
export const FILTER_OPERATOR_LABELS = Object.keys(
  TABLE_FILTER_OPERATORS
) as FilterOperatorLabel[];

/**
 * Abstraction of query filtering in Basable
 */
export type BasableFilter = {
  combinator: string;
  column: string;
  expression: { [key: string]: string };
};

export type FilterInput = {
  column: string;
  combinator: FilterCombinator;
  operatorLabel: FilterOperatorLabel;
  operatorValue: string;
  endValue?: string;
};

/**
 * Options for querying table
 */
export type TableQueryOpts = {
  table: string;
  offset: number;
  row_count: number;
  filters?: BasableFilter[];
  columns?: string[];
  order_by?: {
    [key: string]: string;
  };
  search_opts?: TableSearchOpts;
};

export type TableSearchOpts = {
  search_cols: string[];
  query: string;
};

export const COLUMN_TYPES = [
  "NULL",
  "Text",
  "Int",
  "UInt",
  "Float",
  "Double",
  "Date",
  "Time",
] as const;
export type ColumnType = (typeof COLUMN_TYPES)[number];
export type ColumnTypeObject = {
  [key: string]: ColumnType;
};

export type OrderByKey = "ASC" | "DESC";

export const downloadFormats = [
  "CSV",
  "TSV",
  "PSV",
  "TEXT",
  "JSON",
  "HTML",
  "XML",
] as const;

export type DownloadFormat = (typeof downloadFormats)[number];

export type NavSubmenu = {
  label: string;
  value: string;
};
