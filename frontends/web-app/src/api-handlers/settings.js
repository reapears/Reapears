"use strict";
// ---Account settings api handlers---

import { get, post, put } from "./index";

const ENDPOINT = "account/settings";

// ---Handlers---

// Gets user personal infos
async function getUserPersonalInfo() {
	let json = await get(`${ENDPOINT}/personal-info`);
	return personalInfoFromJson(json);
}

// Updates user personal info
async function updatePersonalInfo(form) {
	return await put(form, `${ENDPOINT}/personal-info`);
}

// Change user email
async function changeEmail(form) {
	return await post(form, `${ENDPOINT}/change-email`);
}

// Verify user email
async function confirmEmail(form) {
	return await post(form, `${ENDPOINT}/confirm-email`);
}

// Change user email
async function changePassword(form) {
	return await post(form, `${ENDPOINT}/change-password`);
}

// Verify user is correct
async function verifyPassword(form) {
	return await post(form, `${ENDPOINT}/verify-password`);
}

// ---Forms---

// User personal info update form
class PersonalInfoForm {
	constructor(firstName, lastName, gender, dateOfBirth) {
		this.firstName = firstName;
		this.lastName = lastName;
		this.gender = gender;
		this.dateOfBirth = dateOfBirth;
	}
}

// Change email form
class ChangeEmailForm {
	constructor(email) {
		this.email = email;
	}
}

// Confirm email form
class ConfirmEmailForm {
	constructor(code) {
		this.code = code;
	}
}

// Change password form
class ChangePasswordForm {
	constructor(current, new_, confirm) {
		this.current = current;
		this.new = new_;
		this.confirm = confirm;
	}
}

// Verify password form
class VerifyPasswordForm {
	constructor(password) {
		this.password = password;
	}
}

// ---Models---

// User personal infos
class PersonalInfo {
	constructor(
		firstName,
		lastName,
		gender,
		dateOfBirth,
		governmentId,
		email,
		phone,
		dateJoined,
	) {
		this.firstName = firstName;
		this.lastName = lastName;
		this.gender = gender;
		this.dateOfBirth = dateOfBirth;
		this.governmentId = governmentId;
		this.email = email;
		this.phone = phone;
		this.dateJoined = dateJoined;
	}
}

// Converts json into PersonalInfo
function personalInfoFromJson(obj) {
	return Object.assign(new PersonalInfo(), obj);
}

// ---Exports---
export {
	getUserPersonalInfo,
	updatePersonalInfo,
	changeEmail,
	changePassword,
	confirmEmail,
	verifyPassword,
};
