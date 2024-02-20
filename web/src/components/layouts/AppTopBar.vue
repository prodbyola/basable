<template>
    <div class="basable_topbar">
        <div class="search_bar">
            <AppInput placeholder="Search" class="search_input">
                <template v-slot:append>
                    <BasableIcon name="search" width="18px" />
                </template>
            </AppInput>
        </div>
        <BasableIcon name="notification" width="18px" />
        <BasableIcon name="message" width="18px" />
        <div class="basable_user">
            <div class="basable_avatar">
                <p>G</p>
            </div>
            <p class="basable_username">Guest User</p>
            <BasableIcon name="dropdown" width="15px" />
        </div>
    </div>
</template>
<script setup lang="ts">
import AppInput from '@/components/AppInput.vue';
import BasableIcon from '../BasableIcon.vue';
</script>
<style lang="scss" scoped>
@use '../../assets/variables' as vars;

$avatar_size: 42px;

.basable_topbar {
    display: flex;
    align-items: center;
    width: 100%;
    height: 90px;
    border-bottom: 1px solid vars.$line_color;
    width: 100%;
    column-gap: 32px;

    .search_bar {
        flex: 1;
        .search_input {
            width: 343px;
        }
    }

    .basable_user {
        display: inline-flex;
        align-items: center;
        column-gap: 12px;

        .basable_username {
            color: vars.$tertiary_color;
        }

        .basable_avatar {
            height: $avatar_size;
            width: $avatar_size;
            background-color: vars.$main_color;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            border-radius: 50%;
    
            p {
                color: white;
                text-transform: uppercase;
                font-size: 2rem;
                font-weight: bold;
            }
        }
    }


}
</style>