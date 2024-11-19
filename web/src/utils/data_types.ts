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
  GREATER_THAN: "Gt",
  LESS_THAN: "Lt",
  GREATER_OR_EQUAL: "Gte",
  LESS_OR_EQUAL: "Lte",
  INCLUDES: "Includes",
  NOT_INCLUDE: "NotInclude",
  LIKE: "Like",
  NOT_LIKE: "NotLike",
  LIKE_SINGLE: "LikeSingle",
  NOT_LIKE_SINGLE: "NotLikeSingle",
  REGEX: "Regex",
  NOT_REGEX: "NotRegex",
  RANGE: "Btw",
  NOT_RANGE: "NotBtw",
  CONTAINS: "Contains",
  NOT_CONTAINS: "NotContains",
  NULL: "Null",
  NOT_NULL: "NotNull",
};

export type FilterOperatorLabel = keyof typeof TABLE_FILTER_OPERATORS;

export type FilterCombinator = "base" | "and" | "or"

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
  column: string,
  combinator: FilterCombinator,
  operatorLabel: FilterOperatorLabel,
  operatorValue: string
  endValue?: string
}
