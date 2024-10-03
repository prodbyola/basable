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

export type SessionCookie = {
  token: string,
  connID: string,
  isAuth: boolean
}

export type ServerDetails = {
  version: string
  db_size: string
  os: string
  comment: string
}
