<template>
  <div class="basable_auth">
    <div class="auth_card">
      <template v-if="stage === 1">
        <h3>Welcome! Please create a connection.</h3>
        <AuthInput v-model="dbConfig.username" placeholder="Username" label="Username" />
        <AuthInput
          v-model="dbConfig.password"
          placeholder="Password"
          label="Password"
          type="password"
        />
        <AuthInput v-model="dbConfig.host" placeholder="Host" label="Host" />
        <AuthInput v-model="dbConfig.port" placeholder="Port" label="Port" type="number" />
        <AuthInput v-model="dbConfig.db_name" placeholder="DB Name" label="DB Name" />
        <AppButton label="Create Connection" class="action_btn" @click="connect" />
      </template>
      <template v-else-if="stage === 2">
        <AuthInput
          :model-value="appState.currentTable"
          label="Select Table"
          type="select"
          :options="appState.tables"
          hint="Select a table you would like to load from your database."
          @update:model-value="createTable"
        />
        <AuthInput
          :model-value="appState.table?.createdAtColumn"
          label="Date Column"
          type="select"
          :options="appState.table?.columnList"
          @update:model-value="updateDateColumn"
          hint="Select a column that helps you track when a record was added to the table. This is typically a 'created_at' or 'joined_on' column."
        />
        <AppButton label="Continue" class="action_btn" @click="loadDashboard" />
      </template>
    </div>
  </div>
</template>
<script lang="ts" setup>
import AuthInput from '@/components/AppInput.vue'
import AppButton from '@/components/AppButton.vue'

import { reactive, ref } from 'vue'
import { ApiService } from '@/request'
import { useState } from '@/stores'

const svc = ApiService.find()
const stage = ref(1)
const appState = useState()

const dbConfig = reactive({
  username: 'root',
  password: '',
  host: 'localhost',
  port: 3306,
  db_name: ''
})

const connect = async () => {
  svc
    .request({
      path: 'connect',
      method: 'POST',
      data: dbConfig
    })
    .then((resp) => {
      appState.tables = resp?.data
      stage.value += 1
    })
}

const createTable = (tb: string) => {
  appState.currentTable = tb
  appState.table?.loadColumList()
}

const updateDateColumn = (col: string) => {
  if (appState.table) appState.table.createdAtColumn = col
}

const loadDashboard = () => {
  appState.table?.loadDashboard()
}
</script>
<style lang="scss" scoped>
.basable_auth {
  display: flex;
  min-width: 100vw;
  min-height: 100vh;
  justify-content: center;
  align-items: center;

  .auth_card {
    display: inline-flex;
    flex-direction: column;
    align-items: center;
    width: 428px;

    .action_btn {
      width: 300px;
      height: 48px;
      border-radius: 8px;
      margin-top: 16px;
    }
  }
}
</style>
