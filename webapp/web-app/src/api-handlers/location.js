"use strict";
// ---Location api handlers---

import { harvestIndexFromJson } from "./harvest";
import { get, post, put, del } from "./index";

const ENDPOINT = "locations";

// --Handlers---

// Get location list from the server
async function getLocationList() {
	let response = await get(ENDPOINT);
	let locations = response.map(locationIndexFromJson);
	return locations;
}

// Get location detail from the server
async function getLocation() {
	let json = await get(`${ENDPOINT}/${id}`);
	return locationFromJson(json);
}

// Create location on the server
async function createLocation(location, farm_id) {
	return await post(location, `$farms/${farm_id}/locations`);
}

// Update location on the server
async function updateLocation(location, id) {
	return await put(location, `${ENDPOINT}/${id}`);
}

// Delete location from the server
async function deleteLocation() {
	return del(`${ENDPOINT}/${id}`);
}

// ---Forms---

// Location create and update form
class LocationForm {
	constructor(placeName, regionId, countryId, description, coords) {
		this.placeName = placeName;
		this.regionId = regionId;
		this.countryId = countryId;
		this.description = description;
		this.coords = coords;
	}
}

// ---Models---

// Farm location detail class
class Location {
	constructor(
		id,
		placeName,
		farm,
		region,
		country,
		coords,
		description,
		harvests,
	) {
		this.id = id;
		this.placeName = placeName;
		this.farm = farm;
		this.region = region;
		this.country = country;
		this.coords = coords;
		this.description = description;
		this.harvests = harvests;
	}
}

// Converts json into Location
function locationFromJson(obj) {
	return new Location(
		obj.id,
		obj.placeName,
		obj.farm,
		obj.region,
		obj.country,
		obj.coords,
		obj.description,
		obj.harvests.map(harvestIndexFromJson),
	);
}

// Farm location index class
class LocationIndex {
	constructor(id, placeName, farm, region, country, coords, harvest_count) {
		this.id = id;
		this.placeName = placeName;
		this.farm = farm;
		this.region = region;
		this.country = country;
		this.coords = coords;
		this.harvest_count = harvest_count;
	}
}

// Converts json into Location
function locationIndexFromJson(obj) {
	return Object.assign(new LocationIndex(), obj);
}

// ---Exports---

export {
	Location,
	LocationIndex,
	getLocation,
	getLocationList,
	createLocation,
	updateLocation,
	deleteLocation,
	locationFromJson,
	locationIndexFromJson,
	LocationForm,
};
