<template>
    <div class="basable_topbar">
        <AppInput placeholder="Search" class="search_input">
            <template v-slot:append>
                <BasableIcon name="search" size="18px" />
            </template>
        </AppInput>
    </div>
</template>
<script setup lang="ts">
import AppInput from '@/components/AppInput.vue';
import BasableIcon from '../BasableIcon.vue';
</script>
<style lang="scss" scoped>
@use '../../assets/variables' as vars;

.basable_topbar {
    display: flex;
    align-items: center;
    width: 100%;
    height: 90px;
    border-bottom: 1px solid vars.$line_color;
    width: 100%;

    .search_input {
        width: 343px;
    }
}
</style>