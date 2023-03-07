"use strict";

import { harvestIndexFromJson } from "./harvest";
import { get, post, put, del } from "./index";

// ---Cultivar api handlers---

const ENDPOINT = "cultivars";

// Get cultivar list from the server
async function getCultivarList() {
	let response = await get(ENDPOINT);
	let cultivars = response.map(cultivarIndexFromJson);
	return cultivars;
}

// Get cultivar detail from the server
async function getCultivar(id) {
	let json = await get(`${ENDPOINT}/${id}`);
	return cultivarFromJson(json);
}

// Create cultivar on the server
async function createCultivar(cultivar) {
	return await post(cultivar, ENDPOINT);
}

// Update cultivar on the server
async function updateCultivar(cultivar, id) {
	return await put(cultivar, `${ENDPOINT}/${id}`);
}

// Delete cultivar from the server
async function deleteCultivar() {
	return del(`${ENDPOINT}/${id}`);
}

// Upload cultivar image to the server
async function uploadCultivarImage() {
	return del(`${ENDPOINT}/${id}/photo`);
}

// Delete cultivar image from the server
async function deleteCultivarImage() {
	return del(`${ENDPOINT}/${id}/photo`);
}

// ---Forms---

// Cultivar create and update form
class CultivarForm {
	constructor(name, categoryId) {
		this.name = name;
		this.categoryId = categoryId;
	}
}

// ---Models---

// Cultivar detail class
class Cultivar {
	constructor(id, name, category, image, harvests) {
		this.id = id;
		this.name = name;
		this.category = category;
		this.image = image;
		this.harvests = harvests;
	}
}

// Converts json into cultivar
function cultivarFromJson(obj) {
	return new CultivarIndex(
		obj.id,
		obj.name,
		obj.category,
		obj.image,
		obj.harvests.map(harvestIndexFromJson),
	);
}

// Cultivar index class
class CultivarIndex {
	constructor(id, name, image, harvest_count) {
		this.id = id;
		this.name = name;
		this.image = image;
		this.harvest_count = harvest_count;
	}
}

// Converts json into cultivar index
function cultivarIndexFromJson(obj) {
	return Object.assign(new CultivarIndex(), obj);
}

// ---Exports---
export {
	Cultivar,
	CultivarIndex,
	getCultivar,
	getCultivarList,
	createCultivar,
	updateCultivar,
	deleteCultivar,
	uploadCultivarImage,
	deleteCultivarImage,
};
