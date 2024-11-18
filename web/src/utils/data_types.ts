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
  EQUAL: {
    symbol: "=",
    key: "Eq",
  },
  NOT_EQUAL: {
    symbol: "!=",
    key: "NotEq",
  },
  GREATER_THAN: {
    symbol: ">",
    key: "Gt",
  },
  LESS_THAN: {
    symbol: "<",
    key: "Lt",
  },
  GREATER_OR_EQUAL: {
    symbol: ">=",
    key: "Gte",
  },
  LESS_OR_EQUAL: {
    symbol: "<=",
    key: "Lte",
  },
  LIKE: {
    symbol: "LIKE",
    key: "Like",
  },
  NOT_LIKE: {
    symbol: "NOT LIKE",
    key: "NotLike",
  },
  LIKE_SINGLE: {
    symbol: "LIKE",
    key: "LikeSingle",
  },
  NOT_LIKE_SINGLE: {
    symbol: "NOT LIKE",
    key: "NotLikeSingle",
  },
  REGEX: {
    symbol: "REGEXP",
    key: "Regex",
  },
  NOT_REGEX: {
    symbol: "NOT REGEXP",
    key: "NotRegex",
  },
  RANGE: {
    symbol: "BETWEEN",
    key: "Btw",
  },
  NOT_RANGE: {
    symbol: "NOT BETWEEN",
    key: "NotBtw",
  },
  CONTAINS: {
    symbol: "IN",
    key: "Contains",
  },
  NOT_CONTAINS: {
    symbol: "NOT IN",
    key: "NotContains",
  },
  NULL: {
    symbol: "IS NULL",
    key: "Null",
  },
  NOT_NULL: {
    symbol: "IS NOT NULL",
    key: "NotNull",
  },
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
  combinator: FilterCombinator;
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
