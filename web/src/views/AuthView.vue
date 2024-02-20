<template>
    <AuthInput v-model="dbConfig.username" />
    <AuthInput v-model="dbConfig.password" type="password" />
    <AuthInput v-model="dbConfig.host" />
    <AuthInput v-model="dbConfig.port" type="number" />
    <AuthInput v-model="dbConfig.db_name" />
</template>
<script lang="ts" setup>
import AuthInput from '@/components/AppInput.vue';
import { reactive } from 'vue';

const dbConfig = reactive({
    username: 'root',
    password: '',
    host: 'localhost',
    port: 3306,
    db_name: ''
})
</script>