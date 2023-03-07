"use strict";
// ---Harvest api handlers---

import { get, post, put, del } from "./index";

const ENDPOINT = "harvests";

// ---Handlers---

// Get harvests list from the server
async function getHarvestList() {
	let response = await get(ENDPOINT);
	let harvests = response.map(harvestIndexFromJson);
	return harvests;
}

// Get the harvest detail from the server
async function getHarvest(id) {
	let json = await get(`${ENDPOINT}/${id}`);
	return harvestFromJson(json);
}

// Create harvest on the server
async function createHarvest(harvest) {
	return await post(harvest, ENDPOINT);
}

// Update harvest on the server
async function updateHarvest(harvest, id) {
	return await put(harvest, `${ENDPOINT}/${id}`);
}

// Delete harvest on the server
async function deleteHarvest(id) {
	return del(`${ENDPOINT}/${id}`);
}

// Uploads harvest images to the server
async function uploadHarvestImages() {}

// Uploads harvest images from the server
async function deleteHarvestImages() {
	return del(`${ENDPOINT}/${id}/photos`);
}

// ---Forms---

// Harvest create and update form
class HarvestForm {
	constructor(locationId, cultivarId, price, type, description, availableAt) {
		this.locationId = locationId;
		this.cultivarId = cultivarId;
		this.price = price;
		this.type = type;
		this.description = description;
		this.availableAt = availableAt;
	}
}

// ---Models---

// Harvest detail class
class Harvest {
	constructor(
		id,
		name,
		cultivar,
		farm,
		UserIndex,
		price,
		type,
		description,
		images,
		availableAt,
		createdAt,
		HarvestLocation,
	) {
		this.id = id;
		this.name = name;
		this.cultivar = cultivar;
		this.farm = farm;
		this.farm_owner = UserIndex;
		this.price = parsePrice(price);
		this.type = type;
		this.description = description;
		this.images = images;
		this.availableAt = parseAvailableAt(availableAt);
		this.createdAt = createdAt;
		this.location = HarvestLocation;
	}
}

// Converts json into harvest
function harvestFromJson(obj) {
	return Object.assign(new Harvest(), obj);
}

// Harvest index class
class HarvestIndex {
	constructor(
		id,
		name,
		farm,
		price,
		images,
		availableAt,
		placeName,
		region,
		country,
	) {
		this.id = id;
		this.name = name;
		this.farm = farm;
		this.price = parsePrice(price);
		this.images = images;
		this.availableAt = parseAvailableAt(availableAt);
		this.placeName = placeName;
		this.region = region;
		this.country = country;
	}
}

// Converts json into harvest index
function harvestIndexFromJson(obj) {
	return Object.assign(new HarvestIndex(), obj);
}

// // Harvest location class
// class HarvestLocation {
// 	constructor(id, placeName, region, country) {
// 		this.id = id;
// 		this.placeName = placeName;
// 		this.region = region;
// 		this.country = country;
// 	}
// }

// Parses price for pretty print
function parsePrice(price) {
	return price;
}

// Parses available at date for pretty print
function parseAvailableAt(availableAt) {
	return availableAt;
}

// ---Exports---
export {
	Harvest,
	HarvestIndex,
	getHarvest,
	getHarvestList,
	createHarvest,
	updateHarvest,
	deleteHarvest,
	harvestIndexFromJson,
	uploadHarvestImages,
	deleteHarvestImages,
};
