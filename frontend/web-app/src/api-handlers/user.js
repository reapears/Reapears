"use strict";
// ---User api handlers---

import { get, put } from "./index";

const ENDPOINT = "account/users";

// ---Handlers---

// Gets user profile
async function getUserProfile(id) {
	let json = await get(`${ENDPOINT}/${id}/profile`);
	return userProfileFromJson(json);
}

// Gets user profile
async function getMyProfile() {
	let json = await get(`${ENDPOINT}/profile`);
	return userProfileFromJson(json);
}

// Updates user profile
async function updateMyProfile(form) {
	return await put(form, `${ENDPOINT}/profile`);
}

// // Uploads user profile photo to the server
// async function uploadProfilePhoto() {}

// ---Forms---

// User profile update form
class UserProfileForm {
	constructor(about, livesAt) {
		this.about = about;
		this.livesAt = livesAt;
	}
}

// ---Models---

// User profile
class UserProfile {
	constructor(user, about, livesAt, dateJoined, farms) {
		this.user = user;
		this.about = about;
		this.livesAt = livesAt;
		this.dateJoined = dateJoined;
		this.farms = farms;
	}
}

// Converts json into UserProfile
function userProfileFromJson(obj) {
	return Object.assign(new UserProfile(), obj);
}

// ---Exports---
export { getUserProfile, getMyProfile, updateMyProfile };
