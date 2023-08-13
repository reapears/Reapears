import { React, useState } from "react";

export function PasswordReset() {
  const [password, setPassword] = useState({
    new: "",
    confirm: "",
  });

  const onChange = (event) => {
    const key = event.target.name;
    const value = event.target.value;
    setPassword((oldPassword) => ({ ...oldPassword, [key]: value }));
  };

  const submitForm = (event) => {
    resetPassword(password);
    event.preventDefault();
  };

  return (
    <form style={{ display: "flex", flexDirection: "column" }}>
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

      <button onClick={submitForm}>Reset password</button>
      <pre>{JSON.stringify(password, true, 2)}</pre>
    </form>
  );
}

function resetPassword(info) {
  console.log(JSON.stringify(info));
}
