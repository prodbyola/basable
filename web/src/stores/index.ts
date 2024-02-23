import { computed, reactive, ref } from 'vue'
import { defineStore } from 'pinia'
import { Table } from '@/data'
import { ApiService } from '@/request'

export const useState = defineStore('useState', () => {
  const tables = ref<string[]>([])
  const currentTable = ref<string | undefined>()

  const table = computed(() => {
    const t = currentTable.value
    if(t) return reactive(new Table(t, ApiService.find()))

    return undefined
  })

  return { tables, currentTable, table }
})
