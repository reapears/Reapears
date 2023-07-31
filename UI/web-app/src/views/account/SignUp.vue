<!--  Harvest create form-->

<template>

    <form @submit.prevent="submitSignUpForm" class="form-floating border rounded px-4 pt-0 pb-4 mt-0 ">
        <h3 class="mt-3 mb-5 text-dark ">Sign up with your email</h3>

        <div class="form-floating mb-3">
            <input type="email" class="form-control" id="floatingInputValue" placeholder="Email address"
                v-model="form.email">
            <label for="floatingInputValue">Email address</label>
        </div>

        <div class="form-floating mb-3">
            <input type="text" class="form-control" id="floatingInputValue" placeholder="First Name"
                v-model="form.firstName">
            <label for="floatingInputValue">First Name</label>
        </div>

        <div class="form-floating mb-3">
            <input type="text" class="form-control" id="floatingInputValue" placeholder="Last Name"
                v-model="form.lastName">
            <label for="floatingInputValue">Last Name</label>
        </div>

        <div class="form-floating mb-3">
            <input type="password" class="form-control" id="floatingPassword" placeholder="Password"
                v-model="form.password">
            <label for="floatingPassword">Password</label>
        </div>

        <div class="form-floating mb-1">
            <button type="submit" class="btn btn-primary mb-3">Sign up</button>
        </div>

        <div class="form-floating text-dark">
            Already have a Reapears account? Log in
        </div>

    </form>

</template>

<script >

import { signup } from "../../api-handlers/accounts";

export default {
    data() {
        return {
            form: {
                email: '',
                firstName: '',
                lastName: '',
                password: '',
            }
        }
    },

    methods: {
        submitSignUpForm() {
            signup(this.form)
        }
    }
}
</script>