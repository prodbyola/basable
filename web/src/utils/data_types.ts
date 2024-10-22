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
  token: string,
  exp: number
}

export type CurrentUser = {
  name: string
  dp?: string
  role: string;
  isLogged: boolean
}

export type SessionCookie = {
  token: string,
  connID: string,
  isAuth: boolean
}

export type ServerDetails = {
  version: string
  db_size: number
  os: string
  comment: string
}

export type TableConfig = {
  events?: unknown
  label: string
  name: string
  pk_column?: string
  created_column?: string
  update_column?: unknown
  special_columns?: unknown
  items_per_page?: number
}

export type TableColumn = {
  col_type: string
  name: string
  nullable: boolean
  primary: boolean
  unique: boolean
  default_value: unknown
}

export type TableRow = {
  [key: string]: {
    [key: string]: unknown
  }
}