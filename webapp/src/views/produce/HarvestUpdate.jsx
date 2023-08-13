import React, { useState } from "react";

export function HarvestUpdate() {
  const [harvest, setHarvest] = useState({
    locationId: "",
    cultivarId: "",
    price: { amount: null, unit: null },
    type: "",
    description: "",
    availableAt: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setHarvest((oldHarvest) => ({ ...oldHarvest, [key]: value }));
  };

  const submitForm = (event) => {
    updateHarvest(harvest);
    event.preventDefault();
  };

  return (
    <form style={{ display: "flex", flexDirection: "column" }}>
      <div>
        <label htmlFor="location">Location:</label>
        <select
          value={harvest.locationId}
          name="locationId"
          onChange={onChange}
          id="location"
        >
          <option value="namibia">Select location</option>
          <option value="location">User farm location here</option>
        </select>
      </div>

      <div>
        <label htmlFor="cultivar">Cultivar:</label>
        <select
          value={harvest.cultivarId}
          name="cultivarId"
          onChange={onChange}
          id="cultivar"
        >
          <option value="">Select cultivar</option>
          <option value="tomato">Tomatoes</option>
          <option value="onion">Onions</option>
          <option value="bell pepper">Bell pepper</option>
        </select>
      </div>

      <div>
        <label htmlFor="cultivar-type">Cultivar type:</label>
        <input
          id="cultivar-type"
          value={harvest.type}
          name="type"
          onChange={onChange}
          type="text"
          required
        />
      </div>

      <div>
        <label htmlFor="harvest-description">Harvest description:</label>
        <textarea
          name="description"
          value={harvest.description}
          onChange={onChange}
          id="harvest-description"
          cols="30"
          rows="5"
        ></textarea>
      </div>

      <div>
        <label htmlFor="availableAt">Available date:</label>
        <input
          id="availableAt"
          value={harvest.coords}
          name="availableAt"
          onChange={onChange}
          type="date"
          required
        />
      </div>

      <div>
        <label htmlFor="price">Price:</label>
        <input
          id="price"
          value={harvest.price}
          name="price"
          onChange={onChange}
          type="date"
          required
        />
      </div>

      <button onClick={submitForm}>list a harvest</button>
      <pre>{JSON.stringify(harvest, true, 2)}</pre>
    </form>
  );
}

function updateHarvest(harvest) {
  console.log(JSON.stringify(harvest));
}
