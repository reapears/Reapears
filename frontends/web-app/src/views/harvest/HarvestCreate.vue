<!--  Harvest create form-->

<template>
    <form @submit.prevent="submitHarvest">
        <label>
            Email:
            <input type="email" v-model="user.email" />
        </label>
        <label>
            Name:
            <input type="text" v-model="user.name" />
        </label>
        <label>
            Password:
            <input type="password" v-model="user.password" />
        </label>
        <button type="submit">Submit</button>
    </form>
</template>

<script "allowsjs": true>
import { createHarvest } from '../../api-handlers/harvest';

export default {
    data() {
        return {
            harvest: {
                locationId: '',
                cultivarId: '',
                price: '',
                type: '',
                description: '',
                availableAt: ''
            }
        }
    },

    methods: {
        submitHarvest() {
            createHarvest(this.harvest)
        }
    }
}
</script>