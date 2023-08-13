import { React, useState } from "react";

export function FarmCreate() {
  const [farm, setFarm] = useState({
    name: "",
    contactNumber: "",
    contactEmail: "",
    foundedAt: "",
    location: {
      countryId: "",
      regionId: "",
      placeName: "",
      description: "",
      coords: {},
    },
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setFarm((oldFarm) => {
      if (oldFarm[key] !== undefined) {
        oldFarm[key] = value;
      } else {
        oldFarm["location"][key] = value;
      }
      return { ...oldFarm };
    });
  };

  const submitForm = (event) => {
    createFarm(farm);
    event.preventDefault();
  };

  return (
    <form style={{ display: "flex", flexDirection: "column" }}>
      <div>
        <label htmlFor="farm-name">Farm name:</label>
        <input
          id="farm-name"
          value={farm.name}
          name="name"
          onChange={onChange}
          type="text"
          required
        />
      </div>

      <div>
        <label htmlFor="contact-number">Contact number:</label>
        <input
          id="contact-number"
          value={farm.contactNumber}
          name="contactNumber"
          onChange={onChange}
          type="phone"
        />
      </div>

      <div>
        <label htmlFor="contact-number">Contact email:</label>
        <input
          id="contact-email"
          value={farm.contactEmail}
          name="contactEmail"
          onChange={onChange}
          type="email"
        />
      </div>

      <div>
        <label htmlFor="foundedAt">Founded at:</label>
        <input
          value={farm.foundedAt}
          name="foundedAt"
          onChange={onChange}
          type="date"
        />
      </div>

      <div>
        <label htmlFor="country">Country:</label>
        <select
          value={farm.location.countryId}
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
          value={farm.location.regionId}
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
          value={farm.location.placeName}
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
          value={farm.location.description}
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
          value={farm.location.coords}
          name="coords"
          onChange={onChange}
          type="text"
          required
        />
      </div>

      <button onClick={submitForm}>add your farm</button>
      <pre>{JSON.stringify(farm, true, 2)}</pre>
    </form>
  );
}

function createFarm(farm) {
  console.log(JSON.stringify(farm));
}
