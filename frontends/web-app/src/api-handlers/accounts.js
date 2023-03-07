"use strict";
// ---User session api handlers---

import { post, del } from "./index";

const ENDPOINT = "account";

// ---Handlers---

// Authenticate and logins user
async function login(form) {
	return await post(form, `${ENDPOINT}/login`);
}

// Deletes user session
async function logout() {
	return del(`${ENDPOINT}/logout`);
}

// User account registration
async function signup(form) {
	return await post(form, `${ENDPOINT}/signup`);
}

// User account registration
async function confirmAccount(url) {
	return await post({}, url);
}

// Deletes user account
async function deactivate() {
	return await del(`${ENDPOINT}/deactivate`);
}

// Forgot password handler
async function forgotPassword(form) {
	return await post(form, `${ENDPOINT}/forgot-password`);
}

// Reset password handler
async function resetPassword(form, url) {
	return await post(form, url);
}

// Reset password handler
async function emailExists(form) {
	let response = await post(form, `${ENDPOINT}/email-exists`);
	if (response.ok) {
		return true;
	} else {
		return false;
	}
}

// ---Forms---

// Signup form
class SignUpForm {
	constructor(firstName, lastName, email, password) {
		this.firstName = firstName;
		this.lastName = lastName;
		this.email = email;
		this.password = password;
	}
}

// Login form
class LoginForm {
	constructor(email, password) {
		this.email = email;
		this.password = password;
	}
}

// Email exists form
class EmailExistsForm {
	constructor(email) {
		this.email = email;
	}
}

// Forgot password form
class ForgotPasswordForm {
	constructor(email) {
		this.email = email;
	}
}

// Reset password form
class ResetPasswordForm {
	constructor(new_, confirm) {
		this.new = new_;
		this.confirm = confirm;
	}
}

// ---Exports---
export {
	emailExists,
	login,
	logout,
	confirmAccount,
	signup,
	deactivate,
	forgotPassword,
	resetPassword,
};
