"use strict";

const HOST = "http://localhost:3000";

// Return api endpoint url
function api_url(endpoint) {
	return `${HOST}/${endpoint}`;
}

// Performs a GET request
async function get(endpoint) {
	let response = await fetch(api_url(endpoint));
	let json = await response.json();
	return json;
}

// Performs a POST request
async function post(data, endpoint) {
	let response = await fetch(api_url(endpoint), {
		method: "POST",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify(data),
	});
	return response;
}

// Performs a PUT request
async function put(data, endpoint) {
	let response = await fetch(api_url(endpoint), {
		method: "PUT",
		headers: {
			"Content-Type": "application/json",
		},
		body: JSON.stringify(data),
	});
	return response;
}

// Performs a DELETe request
async function del(endpoint) {
	let response = await fetch(api_url(endpoint), {
		method: "DELETE",
	});
	return response;
}

// Exports
export { api_url, get, post, put, del };
