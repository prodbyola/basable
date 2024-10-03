import { create } from 'zustand'
import { TableSummaryType } from './data_types'

type StoreType = {
    tables: TableSummaryType[];  // State: An array of TableSummaryType
    updateTables: (tables: TableSummaryType[]) => void;  // Action: A function to update tables
  };

export const useStore = create<StoreType>(set => ({
    tables: [],
    updateTables: (tables: TableSummaryType[]) => set({ tables })
}))