import { create } from 'zustand'
import { CurrentUser, TableConfig, TableSummaryType } from './data_types'

const defaultSnackbar = {
  showAlert: false,
  message: "",
  alertColor: "success" as "success" | "error" | "info" | "warning",
  loading: false,
}

type SnackBarOption = typeof defaultSnackbar

type StoreType = {
    tables: TableSummaryType[]; 
    tableConfigs: TableConfig[];
    currentUser: CurrentUser,
    snackBar: SnackBarOption,
    updateTables: (tables: TableSummaryType[]) => void;
    addTableConfig: (config: TableConfig) => void;
    updateTableConfig: (config: TableConfig) => void;
    showSnackBar: (opts: SnackBarOption) => void
    logout: () => void
};

export const userDefaults: CurrentUser = {
  name: 'Guest User',
  isLogged: false,
  role: 'Demo Account'
}

export const useStore = create<StoreType>((set, get) => ({
    tables: [],
    tableConfigs: [],
    currentUser: userDefaults,
    snackBar: defaultSnackbar,
    updateTables: (tables: TableSummaryType[]) => set({ tables }),
    addTableConfig: (config: TableConfig) => {
      const tableConfigs = get().tableConfigs
      tableConfigs.push(config)
      
      set({ tableConfigs })
    },
    updateTableConfig(config) {
      const tableConfigs = get().tableConfigs
      const cfg = tableConfigs.find(c => c.name === config.name)

      if(cfg) {
        const i = tableConfigs.indexOf(cfg)
        tableConfigs.splice(i, 1, config)
        set({ tableConfigs })
      }
    },
    showSnackBar: (opts: SnackBarOption) => set({ snackBar: opts }),
    logout() {
      set({currentUser: userDefaults})
    },
}))