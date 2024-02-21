<template>
    <div class="basable_auth">
        <div class="auth_card">
            <h3>Welcome! Please create a connection.</h3>
            <AuthInput v-model="dbConfig.username" placeholder="Username" />
            <AuthInput v-model="dbConfig.password" placeholder="Password" type="password" />
            <AuthInput v-model="dbConfig.host" placeholder="Host" />
            <AuthInput v-model="dbConfig.port" placeholder="Port" type="number" />
            <AuthInput v-model="dbConfig.db_name" placeholder="DB Name" />
            <AppButton label="Create Connection" class="action_btn" @click="connect" />
        </div>
    </div>
</template>
<script lang="ts" setup>
import AuthInput from '@/components/AppInput.vue';
import AppButton from '@/components/AppButton.vue';

import { reactive } from 'vue';
import { ApiService } from '@/request';

const svc = ApiService.find()

const dbConfig = reactive({
    username: 'root',
    password: '',
    host: 'localhost',
    port: 3306,
    db_name: ''
})

const connect = async() => {
    svc.request({
        path: 'app/connect',
        method: 'POST',
        data: dbConfig
    })
    .then((resp => console.log(resp)))
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
        width: 480px;

        .action_btn {
            width: 300px;
            height: 48px;
            border-radius: 8px;
            margin-top: 16px;
        }
    }
}
</style>