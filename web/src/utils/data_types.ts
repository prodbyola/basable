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
  EQUAL: "=",
  NOT_EQUAL: "!=",
  GREATER_THAN: ">",
  LESS_THAN: "<",
  GREATER_OR_EQUAL: ">=",
  LESS_OR_EQUAL: "<=",
  LIKE: "LIKE",
  NOT_LIKE: "NOT LIKE",
  LIKE_SINGLE: "LIKE",
  NOT_LIKE_SINGLE: "NOT LIKE",
  REGEX: "REGEXP",
  NOT_REGEX: "NOT REGEXP",
  RANGE: "BETWEEN",
  NOT_RANGE: "NOT BETWEEN",
  CONTAINS: "IN",
  NOT_CONTAINS: "NOT IN",
  NULL: "IS NULL",
  NOT_NULL: "IS NOT NULL",
};

export type FilterOperatorLabel = keyof typeof TABLE_FILTER_OPERATORS;
export const FILTER_OPERATOR_LABELS = Object.keys(
  TABLE_FILTER_OPERATORS
) as FilterOperatorLabel[];
export class BasableFilter {
  constructor(
    public column: string,
    public value: string,
    public operatorKey: FilterOperatorLabel = FILTER_OPERATOR_LABELS[0],
    public filterType: "base" | "and" | "or" = "base",
  ) {}

  
  public get operatorValue() : typeof TABLE_FILTER_OPERATORS[FilterOperatorLabel] {
    return TABLE_FILTER_OPERATORS[this.operatorKey]
  }
  
}
