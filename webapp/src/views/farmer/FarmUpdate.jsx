import { React, useState } from "react";

export function FarmUpdate() {
  const [farm, setFarm] = useState({
    name: "",
    contactNumber: "",
    contactEmail: "",
    foundedAt: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setFarm((oldFarm) => {
      return { ...oldFarm, [key]: value };
    });
  };

  const submitForm = (event) => {
    updateFarm(farm);
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

      <button onClick={submitForm}>Save</button>
      <pre>{JSON.stringify(farm, true, 2)}</pre>
    </form>
  );
}

function updateFarm(farm) {
  console.log(JSON.stringify(farm));
}
