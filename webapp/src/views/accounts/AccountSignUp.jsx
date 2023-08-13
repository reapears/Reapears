import { React, useState } from "react";

export function AccountSignUp() {
  const [user, setUser] = useState({
    firstName: "",
    lastName: "",
    email: "",
    password: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setUser((oldUser) => ({ ...oldUser, [key]: value }));
  };

  const submitForm = (event) => {
    signUpUser(user);
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
        <label htmlFor="email">Email:</label>
        <input
          value={user.email}
          name="email"
          onChange={onChange}
          type="email"
          required
        />
      </div>

      <div>
        <label htmlFor="password">Password:</label>
        <input
          value={user.password}
          id="password"
          name="password"
          onChange={onChange}
          type="password"
          required
        />
      </div>

      <button onClick={submitForm}>Sign Up</button>
      <pre>{JSON.stringify(user, true, 2)}</pre>
    </form>
  );
}

function signUpUser(user) {
  console.log(JSON.stringify(user));
}
