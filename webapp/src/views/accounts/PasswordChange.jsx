import { React, useState } from "react";

export function PasswordChange() {
  const [password, setPassword] = useState({
    current: "",
    new: "",
    confirm: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setPassword((oldPassword) => ({ ...oldPassword, [key]: value }));
  };

  const submitForm = (event) => {
    changePassword(password);
    event.preventDefault();
  };

  return (
    <form style={{ display: "flex", flexDirection: "column" }}>
      <div>
        <label htmlFor="current">Current password:</label>
        <input
          value={password.current}
          name="current"
          onChange={onChange}
          type="password"
          required
        />
      </div>

      <div>
        <label htmlFor="new">New password:</label>
        <input
          value={password.new}
          name="new"
          onChange={onChange}
          type="password"
          required
        />
      </div>

      <div>
        <label htmlFor="confirm">Confirm password:</label>
        <input
          value={password.confirm}
          name="confirm"
          onChange={onChange}
          type="password"
          required
        />
      </div>

      <button onClick={submitForm}>Change password</button>
      <pre>{JSON.stringify(password, true, 2)}</pre>
    </form>
  );
}

function changePassword(password) {
  console.log(JSON.stringify(password));
}
