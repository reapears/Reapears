import { React, useState } from "react";

export function PersonalInfoUpdate() {
  const [user, setUser] = useState({
    firstName: "",
    lastName: "",
    gender: "",
    dateOfBirth: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setUser((oldUser) => ({ ...oldUser, [key]: value }));
  };

  const submitForm = (event) => {
    updatePersonalInfo(user);
    event.preventDefault();
  };

  return (
    <form style={{ display: "flex", flexDirection: "column" }}>
      <div>
        <label htmlFor="firstName">First name:</label>
        <input
          value={user.firstName}
          name="firstName"
          onChange={onChange}
          type="text"
          required
        />
      </div>

      <div>
        <label htmlFor="lastName">Last name:</label>
        <input
          value={user.lastName}
          name="lastName"
          onChange={onChange}
          type="text"
        />
      </div>

      <div>
        <label htmlFor="gender">Gender:</label>
        <select
          value={user.gender}
          name="gender"
          onChange={onChange}
          id="gender"
        >
          <option value="">Not specified</option>
          <option value="male">Male</option>
          <option value="female">Female</option>
        </select>
      </div>

      <div>
        <label htmlFor="dateOfBirth">Date of birth:</label>
        <input
          value={user.dateOfBirth}
          name="dateOfBirth"
          onChange={onChange}
          type="date"
        />
      </div>

      <button onClick={submitForm}>Save changes</button>
      <pre>{JSON.stringify(user, true, 2)}</pre>
    </form>
  );
}

function updatePersonalInfo(user) {
  console.log(JSON.stringify(user));
}
