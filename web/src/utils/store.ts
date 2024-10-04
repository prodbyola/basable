import { create } from 'zustand'
import { CurrentUser, TableSummaryType } from './data_types'

type StoreType = {
    tables: TableSummaryType[]; 
    currentUser: CurrentUser,
    updateTables: (tables: TableSummaryType[]) => void;
};

export const userDefaults: CurrentUser = {
  name: 'Guest User',
  isLogged: false,
  role: 'Demo Account'
}

export const useStore = create<StoreType>(set => ({
    tables: [],
    currentUser: userDefaults,
    updateTables: (tables: TableSummaryType[]) => set({ tables })
}))