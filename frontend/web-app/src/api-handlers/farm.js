"use strict";
// ---Farm api handlers---

import { locationFromJson, locationIndexFromJson } from "./location";
import { get, post, put, del } from "./index";

const ENDPOINT = "farms";

// ---Handlers---

// Get farm list from the server
async function getFarmList() {
	let response = await get(ENDPOINT);
	let farms = response.map(farmIndexFromJson);
	return farms;
}

// Get farm detail from the server
async function getFarm(id) {
	let json = await get(`${ENDPOINT}/${id}`);
	return farmFromJson(json);
}

// Create farm on the server
async function createFarm(farm) {
	return await post(farm, ENDPOINT);
}

// Update farm on the server
async function updateFarm(farm, id) {
	return await put(farm, `${ENDPOINT}/${id}`);
}

// Delete farm from the server
async function deleteFarm() {
	return del(`${ENDPOINT}/${id}`);
}

// ---Forms---

class FarmForm {
	constructor(name, location) {
		this.name = name;
		this.location = location;
	}
}

// ---Models---

// Farm detail class
class Farm {
	constructor(id, name, owner, registered_on, locations) {
		this.id = id;
		this.name = name;
		this.owner = owner;
		this.registered_on = registered_on;
		this.locations = locations;
	}
}

// Converts json into farm
function farmFromJson(obj) {
	return new Farm(
		obj.id,
		obj.name,
		obj.owner,
		obj.registered_on,
		obj.locations.map(locationFromJson),
	);
}

// Farm index class
class FarmIndex {
	constructor(id, name, owner, locations) {
		this.id = id;
		this.name = name;
		this.owner = owner;
		this.locations = locations;
	}
}

// Converts json into farm index
function farmIndexFromJson(obj) {
	return new FarmIndex(
		obj.id,
		obj.name,
		obj.owner,
		obj.locations.map(locationIndexFromJson),
	);
}

// ---Exports---
export {
	Farm,
	FarmIndex,
	getFarmList,
	getFarm,
	createFarm,
	updateFarm,
	deleteFarm,
	farmFromJson,
	farmIndexFromJson,
};
