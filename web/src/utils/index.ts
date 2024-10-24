import { TableConfig } from './data_types'

export * from './cookies'
export * from './data_types'
export * from './network'
export * from './store'
export * from './hooks'

export const getTableLabel = (c: TableConfig) => c.label ?? c.name