import { React, useState } from "react";

export function UserProfileUpdate() {
  const [user, setUser] = useState({
    about: "",
    livesAt: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setUser((oldUser) => ({ ...oldUser, [key]: value }));
  };

  const submitForm = (event) => {
    updateUserProfile(user);
    event.preventDefault();
  };
  return (
    <form style={{ display: "flex", flexDirection: "column" }}>
      <div>
        <label htmlFor="about">About:</label>
        <input
          value={user.about}
          name="about"
          onChange={onChange}
          type="text"
          required
        />
      </div>

      <div>
        <label htmlFor="livesAt">Lives at:</label>
        <input
          value={user.livesAt}
          name="livesAt"
          onChange={onChange}
          type="text"
        />
      </div>

      <button onClick={submitForm}>Save changes</button>
      <pre>{JSON.stringify(user, true, 2)}</pre>
    </form>
  );
}

function updateUserProfile(user) {
  console.log(JSON.stringify(user));
}
