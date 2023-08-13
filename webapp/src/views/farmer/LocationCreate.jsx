import { React, useState } from "react";

export function LocationCreate() {
  const [location, setLocation] = useState({
    countryId: "",
    regionId: "",
    placeName: "",
    description: "",
    coords: {},
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setLocation((oldLocation) => ({ ...oldLocation, [key]: value }));
  };

  const submitForm = (event) => {
    createLocation(location);
    event.preventDefault();
  };

  return (
    <form style={{ display: "flex", flexDirection: "column" }}>
      <div>
        <label htmlFor="country">Country:</label>
        <select
          value={location.countryId}
          name="countryId"
          onChange={onChange}
          id="country"
        >
          <option value="namibia">Select country</option>
          <option value="namibia">Namibia</option>
        </select>
      </div>

      <div>
        <label htmlFor="region">Region:</label>
        <select
          value={location.regionId}
          name="regionId"
          onChange={onChange}
          id="region"
        >
          <option value="">Select region</option>
          <option value="omusati">Omusati</option>
          <option value="ohangwena">Ohangwena</option>
          <option value="kavango west">Kavango West</option>
        </select>
      </div>

      <div>
        <label htmlFor="place-name">Place name:</label>
        <input
          id="place-name"
          value={location.placeName}
          name="placeName"
          onChange={onChange}
          type="text"
          required
        />
      </div>

      <div>
        <label htmlFor="location-description">Location description:</label>
        <textarea
          name="description"
          value={location.description}
          onChange={onChange}
          id="location-description"
          cols="30"
          rows="5"
        ></textarea>
      </div>

      <div>
        <label htmlFor="geo-position">Geo position:</label>
        <input
          id="geo-position"
          value={location.coords}
          name="coords"
          onChange={onChange}
          type="text"
          required
        />
      </div>

      <button onClick={submitForm}>add a location</button>
      <pre>{JSON.stringify(location, true, 2)}</pre>
    </form>
  );
}

function createLocation(location) {
  console.log(JSON.stringify(location));
}
